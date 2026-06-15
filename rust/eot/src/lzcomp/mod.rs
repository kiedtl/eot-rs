mod ahuff;
mod bitio;

use ahuff::*;
use bitio::*;

use crate::{ core::Error, stream::Stream };

#[derive(Copy, Clone, Default)]
pub enum RunLengthCompState {
    #[default]
    Initial,
    Normal,
    SeenEscape,
    NeedByte,
}

#[derive(Copy, Clone)]
pub struct RUNLENGTHCOMP {
    pub escape: u8,
    pub count:  u8,
    pub state:  RunLengthCompState,
}

impl RUNLENGTHCOMP {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            escape: 0,
            count:  0,
            state:  Default::default(),
        })
    }

    // Use this method to decompress the data transparantly
    // as it goes to the memory.
    pub fn save_bytes(&mut self, value: u8, dataOut: &mut Vec<u8>) {
        self.state = match self.state {
            RunLengthCompState::Normal =>
                if value == self.escape {
                    RunLengthCompState::SeenEscape
                } else {
                    dataOut.push(value);
                    self.state
                },
            RunLengthCompState::SeenEscape => {
                self.count = value;
                if self.count == 0 {
                    dataOut.push(self.escape);
                    RunLengthCompState::Normal
                } else {
                    RunLengthCompState::NeedByte
                }
            }
            RunLengthCompState::NeedByte => {
                for _ in 0..self.count {
                    dataOut.push(value);
                }
                RunLengthCompState::Normal
            }
            RunLengthCompState::Initial => {
                self.escape = value;
                RunLengthCompState::Normal
            }
        };
    }
}

pub struct LZCOMP<'a> {
    pub ptr1: Vec<u8>,
    pub ptr1_is_size_limited: bool,
    pub rl_comp: Box<RUNLENGTHCOMP>,
    pub using_run_length: bool,
    pub out_len: i64,
    pub num_dist_ranges: i64,
    pub dist_max: i64,
    pub dup2: i64,
    pub dup4: i64,
    pub dup6: i64,
    pub num_syms: i64,
    pub max_copy_distance: usize,
    pub dist_ecoder: AHUFF,
    pub len_ecoder: AHUFF,
    pub sym_ecoder: AHUFF,
    pub bio: BITIO<'a>,
}

const LEN_WIDTH: i64 = 3;
const PRE_LOAD_SIZE: usize = 2 * 32 * 96 + 4 * 256;
const MAX_2_BYTE_DIST: usize = 512;

fn set_dist_range(t: &mut LZCOMP, length: i64) {
    let dist_min = 1i64;
    let dist_width = 3i64;
    t.num_dist_ranges = 1i64;
    t.dist_max = dist_min + (1i64 << (dist_width * t.num_dist_ranges)) - 1;
    while t.dist_max < length {
        t.num_dist_ranges += 1;
        t.dist_max = dist_min + (1i64 << (dist_width * t.num_dist_ranges)) - 1;
    }
    t.dup2 = 256 + (1i64 << LEN_WIDTH) * t.num_dist_ranges;
    t.dup4 = t.dup2 + 1;
    t.dup6 = t.dup4 + 1;
    t.num_syms = t.dup6 + 1;
}

fn decode_length(t: &mut LZCOMP, symbol: ::core::ffi::c_int, numDistRanges: &mut i64) -> Result<i64, Error> {
    const LEN_MIN: i64 = 2;
    const BIT_RANGE: u64 = 3 - 1; /* == len_width - 1 */

    let mut done: i64 = 0;
    let mut bits: i64 = 0;
    let mut first_time = symbol >= 0;
    let mut value: i64 = 0_i64;
    let mask = 1u64 << BIT_RANGE;

    loop {
        if first_time {
            bits = (symbol - 256) as i64;
            first_time = false;
            assert!(bits >= 0);
            *numDistRanges = bits / (1i64 << LEN_WIDTH) + 1;
            assert!(*numDistRanges >= 1 && *numDistRanges <= t.num_dist_ranges);
            bits %= 1i64 << LEN_WIDTH;
        } else {
            bits = t.len_ecoder.read_symbol(&mut t.bio)? as i64;
        }
        done =
            (bits as ::core::ffi::c_ulong & mask == 0 as ::core::ffi::c_ulong) as ::core::ffi::c_int as i64;
        bits = (bits as ::core::ffi::c_ulong & !mask) as i64;
        value <<= BIT_RANGE;
        value |= bits;
        if done != 0 {
            break;
        }
    }
    value += LEN_MIN;
    Ok(value)
}

fn decode_distance(t: &mut LZCOMP, distRanges: i64) -> Result<i64, Error> {
    let mut bits: i64;
    let mut value: i64 = 0_i64;
    let dist_min: i64 = 1_i64;
    let dist_width: i64 = 3_i64;
    for i in 0..distRanges {
        bits = t.dist_ecoder.read_symbol(&mut t.bio)? as i64;
        value <<= dist_width;
        value |= bits;
    }
    value += dist_min;
    Ok(value)
}

// Initializes our hashTable and also pre-loads some data so that there is a chance that bytes in
// the beginning of the file might use copy items.
fn initialize_model(t: &mut LZCOMP) {
    let mut i = 0;

    for k in 0u8..32 {
        for j in 0u8..96 {
            t.ptr1[i] = k;
            i += 1;
            t.ptr1[i] = j;
            i += 1;
        }
    }

    for j in 0u8..=255 {
        if i >= PRE_LOAD_SIZE {
            break;
        }
        t.ptr1[i] = j;
        i += 1;
        t.ptr1[i] = j;
        i += 1;
        t.ptr1[i] = j;
        i += 1;
        t.ptr1[i] = j;
        i += 1;
    }

    assert!(i == PRE_LOAD_SIZE);
}

fn decode(t: &mut LZCOMP) -> Result<Vec<u8>, Error> {
    let mut symbol = 0;
    let mut length: usize;
    let mut distance: usize;
    let mut num_dist_ranges = 0;
    let mut value = 0u8;
    let using_run_length = t.using_run_length;
    let mut pos: usize;

    let mut data_out = Vec::new();
    initialize_model(t);

    if !t.ptr1_is_size_limited {
        pos = 0;
        while pos < t.out_len as usize {
            symbol = t.sym_ecoder.read_symbol(&mut t.bio)? as ::core::ffi::c_int;
            if symbol < 256 as ::core::ffi::c_int {
                value = symbol as ::core::ffi::c_uchar;
            } else if symbol as i64 == t.dup2 {
                value = t.ptr1[(PRE_LOAD_SIZE as i64 + (pos as i64 - 2)) as usize];
            } else if symbol as i64 == t.dup4 {
                value = t.ptr1[(PRE_LOAD_SIZE as i64 + (pos as i64 - 4)) as usize];
            } else if symbol as i64 == t.dup6 {
                value = t.ptr1[(PRE_LOAD_SIZE as i64 + (pos as i64 - 6)) as usize];
            } else {
                length = decode_length(t, symbol, &mut num_dist_ranges)? as usize;
                distance = decode_distance(t, num_dist_ranges)? as usize;
                if distance >= MAX_2_BYTE_DIST {
                    length += 1;
                }
                let start = pos as i64 - distance as i64 - length as i64 + 1;
                for j in 0..length {
                    value = t.ptr1[(PRE_LOAD_SIZE as i64 + (start + j as i64)) as usize];
                    let fresh3 = pos;
                    pos += 1;
                    t.ptr1[PRE_LOAD_SIZE + fresh3] = value;
                    if using_run_length {
                        t.rl_comp.save_bytes(value, &mut data_out);
                    } else {
                        data_out.push(value);
                    }
                }
                continue;
            }
            let fresh5 = pos;
            pos += 1;
            t.ptr1[PRE_LOAD_SIZE + fresh5] = value;
            if using_run_length {
                t.rl_comp.save_bytes(value, &mut data_out);
            } else {
                data_out.push(value);
            }
        }
    } else {
        let mut src;
        let mut dst = PRE_LOAD_SIZE;
        assert!(t.max_copy_distance > PRE_LOAD_SIZE);
        pos = 0;
        while pos < t.out_len as usize {
            symbol = t.sym_ecoder.read_symbol(&mut t.bio)? as ::core::ffi::c_int;
            if symbol < 256 as ::core::ffi::c_int {
                value = symbol as ::core::ffi::c_uchar;
            } else if symbol as i64 == t.dup2 {
                src = dst.checked_sub(2).unwrap_or(dst + t.max_copy_distance - 2);
                value = t.ptr1[src];
            } else if symbol as i64 == t.dup4 {
                src = dst.checked_sub(4).unwrap_or(dst + t.max_copy_distance - 4);
                value = t.ptr1[src];
            } else if symbol as i64 == t.dup6 {
                src = dst.checked_sub(6).unwrap_or(dst + t.max_copy_distance - 6);
                value = t.ptr1[src];
            } else {
                length = decode_length(t, symbol, &mut num_dist_ranges)? as usize;
                distance = decode_distance(t, num_dist_ranges)? as usize;
                if distance >= MAX_2_BYTE_DIST {
                    length += 1;
                }
                let start = dst as i64 - distance as i64 - length as i64 + 1;
                assert!(distance + length - 1 <= t.max_copy_distance);
                for j in 0..length {
                    src = (start + j as i64)
                        .try_into()
                        .unwrap_or((start + j as i64 + t.max_copy_distance as i64) as usize);
                    value = t.ptr1[src];
                    t.ptr1[dst] = value;
                    dst = (dst + 1) % t.max_copy_distance;
                    pos += 1;
                    if using_run_length {
                        t.rl_comp.save_bytes(value, &mut data_out);
                    } else {
                        data_out.push(value);
                    }
                }
                continue;
            }
            t.ptr1[dst] = value;
            dst = (dst + 1) % t.max_copy_distance;
            pos += 1;
            if using_run_length {
                t.rl_comp.save_bytes(value, &mut data_out);
            } else {
                data_out.push(value);
            }
        }
    }

    assert!(t.using_run_length || data_out.len() == t.out_len as usize);

    if pos != t.out_len as usize {
        return Err(Error::LZCOMP_ERROR);
    }

    Ok(data_out)
}

pub fn mtx_lzcomp_unpack_memory(dataIn: &[u8], version: u8) -> Result<Vec<u8>, Error> {
    let dist_width: i64 = 3_i64;

    let mut t = LZCOMP {
        ptr1: Vec::new(),
        ptr1_is_size_limited: false,
        rl_comp: RUNLENGTHCOMP::new(),
        using_run_length: false,
        out_len: 0,
        num_dist_ranges: 0,
        dist_max: 0,
        dup2: 0,
        dup4: 0,
        dup6: 0,
        num_syms: 0,
        max_copy_distance: 0x7fffffff,
        dist_ecoder: AHUFF::new((1i64 << dist_width) as i16),
        len_ecoder: AHUFF::new((1i64 << LEN_WIDTH) as i16),
        sym_ecoder: AHUFF::PLACEHOLDER,
        bio: BITIO::new(dataIn),
    };

    t.using_run_length = if version == 1 {
        false
    } else {
        t.bio.input_bit()? != 0
    };
    t.out_len = t.bio.read_value(24_i64)? as i64;
    let out_len = t.out_len;
    set_dist_range(&mut t, out_len);

    let max_out_size = t.out_len as usize + PRE_LOAD_SIZE;
    if t.max_copy_distance < max_out_size {
        t.ptr1_is_size_limited = true;
        t.ptr1.resize(t.max_copy_distance as _, 0);
    } else {
        t.ptr1.resize(max_out_size as _, 0);
    }

    t.sym_ecoder = AHUFF::new(t.num_syms as i16);

    let data_out = decode(&mut t)?; // do the work!
    assert!(t.using_run_length || data_out.len() < max_out_size);
    Ok(data_out)
}

pub fn unpack_mtx(buf: &mut Stream, mut _size: u32) -> Result<[Vec<u8>; 3], Error> {
    let version_magic = buf.be_read_u8().map_err(|_| Error::MTX_ERROR)?;
    let _copy_limit = buf.be_read_u24().map_err(|_| Error::MTX_ERROR)?;

    let mut offsets = [10usize, 0, 0];
    for i in 1 /* sic */ ..3 {
        offsets[i] = buf.be_read_u24().map_err(|_| Error::MTX_ERROR)? as usize;
    }

    let sizes: [usize; 3] = [offsets[1] - offsets[0], offsets[2] - offsets[1], buf.buf.len() - offsets[2]];

    for i in 0..3 {
        if offsets[i] + sizes[i] > buf.buf.len() {
            return Err(Error::MTX_ERROR);
        }
    }

    let bufs = [
        mtx_lzcomp_unpack_memory(&buf.buf[offsets[0]..offsets[1]], version_magic)?,
        mtx_lzcomp_unpack_memory(&buf.buf[offsets[1]..offsets[2]], version_magic)?,
        mtx_lzcomp_unpack_memory(&buf.buf[offsets[2]..], version_magic)?,
    ];

    Ok(bufs)
}
