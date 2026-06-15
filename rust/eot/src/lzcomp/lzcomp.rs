use crate::core::Error;
use crate::lzcomp::bitio::*;
use crate::lzcomp::ahuff::*;

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

pub struct LZCOMP {
    pub ptr1: Vec<u8>,
    pub ptr1_IsSizeLimited: bool,
    pub rlComp: Box<RUNLENGTHCOMP>,
    pub usingRunLength: bool,
    pub out_len: ::core::ffi::c_long,
    pub num_DistRanges: ::core::ffi::c_long,
    pub dist_max: ::core::ffi::c_long,
    pub DUP2: ::core::ffi::c_long,
    pub DUP4: ::core::ffi::c_long,
    pub DUP6: ::core::ffi::c_long,
    pub NUM_SYMS: ::core::ffi::c_long,
    pub maxCopyDistance: usize,
    pub dist_ecoder: AHUFF,
    pub len_ecoder: AHUFF,
    pub sym_ecoder: AHUFF,
    pub bio: BITIO,
}

const preLoadSize: usize = 2 * 32 * 96 + 4 * 256;
const max_2Byte_Dist: usize = 512;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const ERR_LZCOMP_Decode_bounds: ::core::ffi::c_int = 3354 as ::core::ffi::c_int;

unsafe fn SetDistRange(mut t: &mut LZCOMP, mut length: ::core::ffi::c_long) {
    // let len_min: ::core::ffi::c_long = 2 as ::core::ffi::c_long;
    // let len_min3: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    let len_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    // let bit_Range: ::core::ffi::c_long = (3 as ::core::ffi::c_int
    //     - 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    let dist_min: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
    let dist_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    (*t).num_DistRanges = 1 as ::core::ffi::c_long;
    (*t).dist_max = dist_min
        + ((1 as ::core::ffi::c_long) << dist_width * (*t).num_DistRanges)
        - 1 as ::core::ffi::c_long;
    while (*t).dist_max < length {
        (*t).num_DistRanges += 1;
        (*t).dist_max = dist_min
            + ((1 as ::core::ffi::c_long) << dist_width * (*t).num_DistRanges)
            - 1 as ::core::ffi::c_long;
    }
    (*t).DUP2 = 256 as ::core::ffi::c_long
        + ((1 as ::core::ffi::c_long) << len_width) * (*t).num_DistRanges;
    (*t).DUP4 = (*t).DUP2 + 1 as ::core::ffi::c_long;
    (*t).DUP6 = (*t).DUP4 + 1 as ::core::ffi::c_long;
    (*t).NUM_SYMS = (*t).DUP6 + 1 as ::core::ffi::c_long;
}

unsafe fn DecodeLength(
    mut t: &mut LZCOMP,
    mut symbol: ::core::ffi::c_int,
    mut numDistRanges: *mut ::core::ffi::c_long,
) -> Result<::core::ffi::c_long, Error> {
    let mut done: ::core::ffi::c_long = 0;
    let mut bits: ::core::ffi::c_long = 0;
    let mut firstTime: ::core::ffi::c_long = (symbol >= 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_long;
    let mut value: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let len_min: ::core::ffi::c_long = 2 as ::core::ffi::c_long;
    // let len_min3: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    let len_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    let bit_Range: ::core::ffi::c_long = (3 as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    // let dist_min: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
    // let dist_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    let mut mask:  ::core::ffi::c_ulong =
     ((1 as ::core::ffi::c_long) << bit_Range) as ::core::ffi::c_ulong;
    loop {
        if firstTime != 0 {
            bits = (symbol - 256 as ::core::ffi::c_int) as ::core::ffi::c_long;
            firstTime = false_0 as ::core::ffi::c_long;
            assert!(bits >= 0);
            *numDistRanges = bits / ((1 as ::core::ffi::c_long) << len_width)
                + 1 as ::core::ffi::c_long;
            assert!(*numDistRanges >= 1 && *numDistRanges <= t.num_DistRanges);
            bits = bits % ((1 as ::core::ffi::c_long) << len_width);
        } else {
            bits = t.len_ecoder.read_symbol(&mut t.bio)? as ::core::ffi::c_long;
        }
        done = (bits as ::core::ffi::c_ulong & mask == 0 as ::core::ffi::c_ulong)
            as ::core::ffi::c_int as ::core::ffi::c_long;
        bits = (bits as ::core::ffi::c_ulong & !mask) as ::core::ffi::c_long;
        value <<= bit_Range;
        value |= bits;
        if !(done == 0) {
            break;
        }
    }
    value += len_min;
    Ok(value)
}

unsafe fn UpdateModel(mut _t: &mut LZCOMP, mut _index: ::core::ffi::c_long) {}
unsafe fn DecodeDistance2(
    mut t: &mut LZCOMP,
    mut distRanges: ::core::ffi::c_long,
) -> Result<::core::ffi::c_long, Error> {
    let mut bits: ::core::ffi::c_long = 0;
    let mut value: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    // let len_min: ::core::ffi::c_long = 2 as ::core::ffi::c_long;
    // let len_min3: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    // let len_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    // let bit_Range: ::core::ffi::c_long = (3 as ::core::ffi::c_int
    //     - 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    let dist_min: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
    let dist_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    let mut i:  ::core::ffi::c_long =  distRanges;
    while i > 0 as ::core::ffi::c_long {
        bits = t.dist_ecoder.read_symbol(&mut t.bio)? as ::core::ffi::c_long;
        value <<= dist_width;
        value |= bits;
        i -= 1;
    }
    value += dist_min;
    Ok(value)
}

unsafe fn InitializeModel(t: &mut LZCOMP, mut compress: ::core::ffi::c_int) {
    let mut j: ::core::ffi::c_long = 0;
    let mut k: ::core::ffi::c_long = 0;
    let mut i:  ::core::ffi::c_long =  0 as ::core::ffi::c_long;
    if compress != 0 {
        k = 0;
        while k < 32 {
            j = 0;
            while j < 96 {
                t.ptr1[i as usize] = k as u8;
                let fresh9 = i;
                i = i + 1;
                UpdateModel(t, fresh9);
                t.ptr1[i as usize] = j as u8;
                let fresh10 = i;
                i = i + 1;
                UpdateModel(t, fresh10);
                j += 1;
            }
            k += 1;
        }
        j = 0;
        while i < preLoadSize  as i64 && j < 256 {
            t.ptr1[i as usize] = j as u8;
            let fresh11 = i;
            i = i + 1;
            UpdateModel(t, fresh11);
            t.ptr1[i as usize] = j as u8;
            let fresh12 = i;
            i = i + 1;
            UpdateModel(t, fresh12);
            t.ptr1[i as usize] = j as u8;
            let fresh13 = i;
            i = i + 1;
            UpdateModel(t, fresh13);
            t.ptr1[i as usize] = j as u8;
            let fresh14 = i;
            i = i + 1;
            UpdateModel(t, fresh14);
            j += 1;
        }
    } else {
        k = 0 as ::core::ffi::c_long;
        while k < 32 as ::core::ffi::c_long {
            j = 0 as ::core::ffi::c_long;
            while j < 96 as ::core::ffi::c_long {
                let fresh15 = i;
                i = i + 1;
                t.ptr1[fresh15 as usize] = k as u8;
                let fresh16 = i;
                i = i + 1;
                t.ptr1[fresh16 as usize] = j as u8;
                j += 1;
            }
            k += 1;
        }
        j = 0 as ::core::ffi::c_long;
        while i < preLoadSize  as i64 && j < 256 as ::core::ffi::c_long {
            let fresh17 = i;
            i = i + 1;
            t.ptr1[fresh17 as usize] = j as u8;
            let fresh18 = i;
            i = i + 1;
            t.ptr1[fresh18 as usize] = j as u8;
            let fresh19 = i;
            i = i + 1;
            t.ptr1[fresh19 as usize] = j as u8;
            let fresh20 = i;
            i = i + 1;
            t.ptr1[fresh20 as usize] = j as u8;
            j += 1;
        }
    }
    assert!(j == 256);
    assert!(i as usize == preLoadSize);
}

unsafe fn Decode(t: &mut LZCOMP) -> Result<Vec<u8>, Error> {
    let mut symbol = 0;
    let mut length = 0usize;
    let mut distance = 0usize;
    let mut start = 0i64;
    let mut numDistRanges = 0;
    let mut value = 0u8;
    let mut usingRunLength = t.usingRunLength;
    let mut pos = 0usize;

    let mut dataOut = Vec::new();
    InitializeModel(t, false_0);
    if !t.ptr1_IsSizeLimited {
        pos = 0;
        while pos < t.out_len as usize {
            symbol = t.sym_ecoder.read_symbol(&mut t.bio)? as ::core::ffi::c_int;
            if symbol < 256 as ::core::ffi::c_int {
                value = symbol as ::core::ffi::c_uchar;
            } else if symbol as ::core::ffi::c_long == t.DUP2 {
                value = t.ptr1[(preLoadSize as i64 + (pos as i64 - 2)) as usize];
            } else if symbol as ::core::ffi::c_long == t.DUP4 {
                value = t.ptr1[(preLoadSize as i64 + (pos as i64 - 4)) as usize];
            } else if symbol as ::core::ffi::c_long == t.DUP6 {
                value = t.ptr1[(preLoadSize as i64 + (pos as i64 - 6)) as usize];
            } else {
                length = DecodeLength(t, symbol, &raw mut numDistRanges)? as usize;
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
            } else if symbol as ::core::ffi::c_long == t.DUP2 {
                src = dst.checked_sub(2)
                    .unwrap_or(dst + t.maxCopyDistance - 2);
                value = t.ptr1[src as usize];
            } else if symbol as ::core::ffi::c_long == t.DUP4 {
                src = dst.checked_sub(4)
                    .unwrap_or(dst + t.maxCopyDistance - 4);
                value = t.ptr1[src as usize];
            } else if symbol as ::core::ffi::c_long == t.DUP6 {
                src = dst.checked_sub(6)
                    .unwrap_or(dst + t.maxCopyDistance - 6);
                value = t.ptr1[src as usize];
            } else {
                length = DecodeLength(t, symbol, &raw mut numDistRanges)? as usize;
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

    assert!(pos == t.out_len as usize);
    assert!(t.usingRunLength || dataOut.len() == t.out_len as usize);

    if pos != t.out_len as usize {
        return Err(Error::LZCOMP_ERROR);
    }

    Ok(dataOut)
}

pub unsafe fn MTX_LZCOMP_UnPackMemory(
    mut dataIn: *mut ::core::ffi::c_void,
    mut dataInSize: ::core::ffi::c_long,
    mut version: ::core::ffi::c_uchar,
) -> Result<Vec<u8>, Error> {
    let len_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    let dist_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;

    let NUM_SYMS = 0i64;

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
        NUM_SYMS,
        maxCopyDistance: 0x7fffffff,
        dist_ecoder: AHUFF::new((1i64 << dist_width) as i16),
        len_ecoder: AHUFF::new((1i64 << len_width) as i16),
        sym_ecoder: AHUFF::PLACEHOLDER,
        bio: BITIO::new(dataIn, dataInSize),
    };

    t.usingRunLength = if version == 1 { false } else { t.bio.input_bit()? != 0 };
    t.out_len = t.bio.read_value(24 as ::core::ffi::c_long)? as i64;
    assert!(!dataIn.is_null());
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
