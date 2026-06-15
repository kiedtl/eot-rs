use crate::core::Error;
use crate::lzcomp::bitio::*;
use crate::lzcomp::ahuff::*;
use crate::stream::Stream;

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
    pub count: u8,
    pub state: RunLengthCompState,
}

impl RUNLENGTHCOMP {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            escape: 0,
            count: 0,
            state: Default::default(),
        })
    }

    /* Use this method to decompress the data transparantly */
    /* as it goes to the memory. */
    pub fn save_bytes(&mut self, value: u8, dataOut: &mut Vec<u8>) {
        self.state = match self.state {
            RunLengthCompState::Normal => {
                if value == self.escape {
                    RunLengthCompState::SeenEscape
                } else {
                    dataOut.push(value);
                    self.state
                }
            }
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
    pub ptr1_IsSizeLimited: bool,
    pub rlComp: Box<RUNLENGTHCOMP>,
    pub usingRunLength: bool,
    pub out_len: i64,
    pub num_DistRanges: i64,
    pub dist_max: i64,
    pub DUP2: i64,
    pub DUP4: i64,
    pub DUP6: i64,
    pub NUM_SYMS: i64,
    pub maxCopyDistance: usize,
    pub dist_ecoder: AHUFF,
    pub len_ecoder: AHUFF,
    pub sym_ecoder: AHUFF,
    pub bio: BITIO<'a>,
}

const len_width: i64 = 3;
const preLoadSize: usize = 2 * 32 * 96 + 4 * 256;
const max_2Byte_Dist: usize = 512;

fn SetDistRange(mut t: &mut LZCOMP, mut length: i64) {
    let dist_min = 1i64;
    let dist_width = 3i64;
    t.num_DistRanges = 1i64;
    t.dist_max = dist_min + (1i64 << dist_width * t.num_DistRanges) - 1;
    while t.dist_max < length {
        t.num_DistRanges += 1;
        t.dist_max = dist_min + (1i64 << dist_width * t.num_DistRanges) - 1;
    }
    t.DUP2 = 256 + (1i64 << len_width) * t.num_DistRanges;
    t.DUP4 = t.DUP2 + 1;
    t.DUP6 = t.DUP4 + 1;
    t.NUM_SYMS = t.DUP6 + 1;
}

fn DecodeLength(t: &mut LZCOMP, symbol: ::core::ffi::c_int, numDistRanges: &mut i64) -> Result<i64, Error> {
    const len_min: i64 = 2;
    const bit_Range: u64 = 3 - 1; /* == len_width - 1 */

    let mut done: i64 = 0;
    let mut bits: i64 = 0;
    let mut firstTime = symbol >= 0;
    let mut value: i64 = 0 as i64;
    let mask = 1u64 << bit_Range as u64;

    loop {
        if firstTime {
            bits = (symbol - 256) as i64;
            firstTime = false;
            assert!(bits >= 0);
            *numDistRanges = bits / (1i64 << len_width) + 1;
            assert!(*numDistRanges >= 1 && *numDistRanges <= t.num_DistRanges);
            bits %= 1i64 << len_width;
        } else {
            bits = t.len_ecoder.read_symbol(&mut t.bio)? as i64;
        }
        done = (bits as ::core::ffi::c_ulong & mask == 0 as ::core::ffi::c_ulong)
            as ::core::ffi::c_int as i64;
        bits = (bits as ::core::ffi::c_ulong & !mask) as i64;
        value <<= bit_Range;
        value |= bits;
        if !(done == 0) {
            break;
        }
    }
    value += len_min;
    Ok(value)
}

fn DecodeDistance2(t: &mut LZCOMP, distRanges: i64) -> Result<i64, Error> {
    let mut bits: i64 = 0;
    let mut value: i64 = 0 as i64;
    let dist_min: i64 = 1 as i64;
    let dist_width: i64 = 3 as i64;
    let mut i:  i64 =  distRanges;
    while i > 0 as i64 {
        bits = t.dist_ecoder.read_symbol(&mut t.bio)? as i64;
        value <<= dist_width;
        value |= bits;
        i -= 1;
    }
    value += dist_min;
    Ok(value)
}

/*
 * Initializes our hashTable and also pre-loads some data so that there is a chance that bytes in
 * the beginning of the file might use copy items.
 */
fn InitializeModel(t: &mut LZCOMP) {
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
        if i >= preLoadSize {
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

    assert!(i as usize == preLoadSize);
}

fn Decode(t: &mut LZCOMP) -> Result<Vec<u8>, Error> {
    let mut symbol = 0;
    let mut length = 0usize;
    let mut distance = 0usize;
    let mut start = 0i64;
    let mut numDistRanges = 0;
    let mut value = 0u8;
    let mut usingRunLength = t.usingRunLength;
    let mut pos = 0usize;

    let mut dataOut = Vec::new();
    InitializeModel(t);
    if !t.ptr1_IsSizeLimited {
        pos = 0;
        while pos < t.out_len as usize {
            symbol = t.sym_ecoder.read_symbol(&mut t.bio)? as ::core::ffi::c_int;
            if symbol < 256 as ::core::ffi::c_int {
                value = symbol as ::core::ffi::c_uchar;
            } else if symbol as i64 == t.DUP2 {
                value = t.ptr1[(preLoadSize as i64 + (pos as i64 - 2)) as usize];
            } else if symbol as i64 == t.DUP4 {
                value = t.ptr1[(preLoadSize as i64 + (pos as i64 - 4)) as usize];
            } else if symbol as i64 == t.DUP6 {
                value = t.ptr1[(preLoadSize as i64 + (pos as i64 - 6)) as usize];
            } else {
                length = DecodeLength(t, symbol, &mut numDistRanges)? as usize;
                distance = DecodeDistance2(t, numDistRanges)? as usize;
                if distance >= max_2Byte_Dist {
                    length += 1;
                }
                start = pos as i64 - distance as i64 - length as i64 + 1;
                for j in 0..length {
                    value = t.ptr1[(preLoadSize as i64 + (start + j as i64)) as usize];
                    let fresh3 = pos;
                    pos = pos + 1;
                    t.ptr1[preLoadSize + fresh3 as usize] = value;
                    if usingRunLength {
                        t.rlComp.save_bytes(value, &mut dataOut);
                    } else {
                        dataOut.push(value);
                    }
                }
                continue;
            }
            let fresh5 = pos;
            pos = pos + 1;
            t.ptr1[preLoadSize + fresh5 as usize] = value;
            if usingRunLength {
                t.rlComp.save_bytes(value, &mut dataOut);
            } else {
                dataOut.push(value);
            }
        }
    } else {
        let mut src = 0;
        let mut dst = preLoadSize;
        assert!(t.maxCopyDistance > preLoadSize);
        pos = 0;
        while pos < t.out_len as usize {
            symbol = t.sym_ecoder.read_symbol(&mut t.bio)? as ::core::ffi::c_int;
            if symbol < 256 as ::core::ffi::c_int {
                value = symbol as ::core::ffi::c_uchar;
            } else if symbol as i64 == t.DUP2 {
                src = dst.checked_sub(2)
                    .unwrap_or(dst + t.maxCopyDistance - 2);
                value = t.ptr1[src as usize];
            } else if symbol as i64 == t.DUP4 {
                src = dst.checked_sub(4)
                    .unwrap_or(dst + t.maxCopyDistance - 4);
                value = t.ptr1[src as usize];
            } else if symbol as i64 == t.DUP6 {
                src = dst.checked_sub(6)
                    .unwrap_or(dst + t.maxCopyDistance - 6);
                value = t.ptr1[src as usize];
            } else {
                length = DecodeLength(t, symbol, &mut numDistRanges)? as usize;
                distance = DecodeDistance2(t, numDistRanges)? as usize;
                if distance >= max_2Byte_Dist {
                    length += 1;
                }
                start = dst as i64 - distance as i64 - length as i64 + 1;
                assert!(distance + length - 1 <= t.maxCopyDistance);
                for j in 0..length {
                    src = (start + j as i64)
                        .try_into()
                        .unwrap_or((start + j as i64 + t.maxCopyDistance as i64) as usize);
                    value = t.ptr1[src as usize];
                    t.ptr1[dst as usize] = value;
                    dst = (dst + 1) % t.maxCopyDistance as usize;
                    pos += 1;
                    if usingRunLength {
                        t.rlComp.save_bytes(value, &mut dataOut);
                    } else {
                        dataOut.push(value);
                    }
                }
                continue;
            }
            t.ptr1[dst as usize] = value;
            dst = (dst + 1) % t.maxCopyDistance as usize;
            pos += 1;
            if usingRunLength {
                t.rlComp.save_bytes(value, &mut dataOut);
            } else {
                dataOut.push(value);
            }
        }
    }

    assert!(t.usingRunLength || dataOut.len() == t.out_len as usize);

    if pos != t.out_len as usize {
        return Err(Error::LZCOMP_ERROR);
    }

    Ok(dataOut)
}

pub fn MTX_LZCOMP_UnPackMemory(dataIn: &[u8], version: u8) -> Result<Vec<u8>, Error> {
    let dist_width: i64 = 3 as i64;

    let mut t = LZCOMP {
        ptr1: Vec::new(),
        ptr1_IsSizeLimited: false,
        rlComp: RUNLENGTHCOMP::new(),
        usingRunLength: false,
        out_len: 0,
        num_DistRanges: 0,
        dist_max: 0,
        DUP2: 0,
        DUP4: 0,
        DUP6: 0,
        NUM_SYMS: 0,
        maxCopyDistance: 0x7fffffff,
        dist_ecoder: AHUFF::new((1i64 << dist_width) as i16),
        len_ecoder: AHUFF::new((1i64 << len_width) as i16),
        sym_ecoder: AHUFF::PLACEHOLDER,
        bio: BITIO::new(dataIn),
    };

    t.usingRunLength = if version == 1 { false } else { t.bio.input_bit()? != 0 };
    t.out_len = t.bio.read_value(24 as i64)? as i64;
    let out_len = t.out_len;
    SetDistRange(&mut t, out_len);

    let mut maxOutSize =  t.out_len as usize + preLoadSize;
    if t.maxCopyDistance < maxOutSize  {
        t.ptr1_IsSizeLimited = true;
        t.ptr1.resize(t.maxCopyDistance as _, 0);
    } else {
        t.ptr1.resize(maxOutSize as _, 0);
    }

    t.sym_ecoder = AHUFF::new(t.NUM_SYMS as i16);

    let dataOut = Decode(&mut t)?; // do the work!
    assert!(t.usingRunLength || dataOut.len() < maxOutSize);
    Ok(dataOut)
}

pub fn unpackMtx(buf: &mut Stream, mut _size: u32) -> Result<[Vec<u8>; 3], Error> {
    let versionMagic = buf.be_read_u8()
        .map_err(|_| Error::MTX_ERROR)?;
    let _copyLimit = buf.be_read_u24()
        .map_err(|_| Error::MTX_ERROR)?;

    let mut offsets = [10usize, 0, 0];
    for i in 1 /* sic */ ..3 {
        offsets[i] = buf.be_read_u24()
            .map_err(|_| Error::MTX_ERROR)? as usize;
    }

    let sizes: [usize; 3] = [
        offsets[1] - offsets[0],
        offsets[2] - offsets[1],
        buf.buf.len() - offsets[2],
    ];

    for i in 0..3 {
        if offsets[i] + sizes[i] > buf.buf.len() {
            return Err(Error::MTX_ERROR);
        }
    }

    let bufs = [
        MTX_LZCOMP_UnPackMemory(
            &buf.buf[offsets[0]..offsets[1]],
            versionMagic as u8,
        )?,
        MTX_LZCOMP_UnPackMemory(
            &buf.buf[offsets[1]..offsets[2]],
            versionMagic as u8,
        )?,
        MTX_LZCOMP_UnPackMemory(
            &buf.buf[offsets[2]..],
            versionMagic as u8,
        )?,
    ];

    Ok(bufs)
}
