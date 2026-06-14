use crate::lzcomp::bitio::*;
use crate::lzcomp::mtxmem::*;
use crate::lzcomp::ahuff::*;

extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __jmp_buf = [::core::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: ::core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mem_struct {
    pub pointermem: *mut ::core::ffi::c_void,
    pub pointersize: ::core::ffi::c_long,
}

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
    pub ptr1: *mut ::core::ffi::c_uchar,
    pub ptr1_IsSizeLimited: ::core::ffi::c_char,
    pub rlComp: Box<RUNLENGTHCOMP>,
    pub usingRunLength: bool,
    pub out_len: ::core::ffi::c_long,
    pub num_DistRanges: ::core::ffi::c_long,
    pub dist_max: ::core::ffi::c_long,
    pub DUP2: ::core::ffi::c_long,
    pub DUP4: ::core::ffi::c_long,
    pub DUP6: ::core::ffi::c_long,
    pub NUM_SYMS: ::core::ffi::c_long,
    pub maxCopyDistance: ::core::ffi::c_long,
    pub dist_ecoder: AHUFF,
    pub len_ecoder: AHUFF,
    pub sym_ecoder: AHUFF,
    pub bio: BITIO,
    pub mem: *mut MTX_MemHandler,
}

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const ERR_LZCOMP_Decode_bounds: ::core::ffi::c_int = 3354 as ::core::ffi::c_int;
#[no_mangle]
pub static mut max_2Byte_Dist: ::core::ffi::c_long = 512 as ::core::ffi::c_long;

unsafe extern "C" fn SetDistRange(mut t: &mut LZCOMP, mut length: ::core::ffi::c_long) {
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

unsafe extern "C" fn DecodeLength(
    mut t: &mut LZCOMP,
    mut symbol: ::core::ffi::c_int,
    mut numDistRanges: *mut ::core::ffi::c_long,
) -> ::core::ffi::c_long {
    
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
            if bits >= 0 as ::core::ffi::c_long {} else {
                __assert_fail(
                    b"bits >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
                    260 as ::core::ffi::c_uint,
                    b"long DecodeLength(LZCOMP *, int, long *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
            *numDistRanges = bits / ((1 as ::core::ffi::c_long) << len_width)
                + 1 as ::core::ffi::c_long;
            if *numDistRanges >= 1 as ::core::ffi::c_long
                && *numDistRanges <= (*t).num_DistRanges
            {} else {
                __assert_fail(
                    b"*numDistRanges >= 1 && *numDistRanges <= t->num_DistRanges\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
                    263 as ::core::ffi::c_uint,
                    b"long DecodeLength(LZCOMP *, int, long *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
            bits = bits % ((1 as ::core::ffi::c_long) << len_width);
        } else {
            bits = t.len_ecoder.read_symbol(&mut t.bio) as ::core::ffi::c_long;
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
    return value;
}

unsafe extern "C" fn UpdateModel(mut _t: &mut LZCOMP, mut _index: ::core::ffi::c_long) {}
unsafe extern "C" fn DecodeDistance2(
    mut t: &mut LZCOMP,
    mut distRanges: ::core::ffi::c_long,
) -> ::core::ffi::c_long {
    
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
        bits = t.dist_ecoder.read_symbol(&mut t.bio) as ::core::ffi::c_long;
        value <<= dist_width;
        value |= bits;
        i -= 1;
    }
    value += dist_min;
    return value;
}

unsafe fn InitializeModel(t: &mut LZCOMP, mut compress: ::core::ffi::c_int) {
    let mut j: ::core::ffi::c_long = 0;
    let mut k: ::core::ffi::c_long = 0;
    let preLoadSize: ::core::ffi::c_long = (2 as ::core::ffi::c_int
        * 32 as ::core::ffi::c_int * 96 as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as ::core::ffi::c_long;
    let mut i:  ::core::ffi::c_long =  0 as ::core::ffi::c_long;
    if preLoadSize > 0 as ::core::ffi::c_long {} else {
        __assert_fail(
            b"preLoadSize > 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            444 as ::core::ffi::c_uint,
            b"void InitializeModel(LZCOMP *, int)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if compress != 0 {
        k = 0 as ::core::ffi::c_long;
        while k < 32 as ::core::ffi::c_long {
            j = 0 as ::core::ffi::c_long;
            while j < 96 as ::core::ffi::c_long {
                *t.ptr1.offset(i as isize) = k as ::core::ffi::c_uchar;
                let fresh9 = i;
                i = i + 1;
                UpdateModel(t, fresh9);
                *t.ptr1.offset(i as isize) = j as ::core::ffi::c_uchar;
                let fresh10 = i;
                i = i + 1;
                UpdateModel(t, fresh10);
                j += 1;
            }
            k += 1;
        }
        j = 0 as ::core::ffi::c_long;
        while i < preLoadSize && j < 256 as ::core::ffi::c_long {
            *t.ptr1.offset(i as isize) = j as ::core::ffi::c_uchar;
            let fresh11 = i;
            i = i + 1;
            UpdateModel(t, fresh11);
            *t.ptr1.offset(i as isize) = j as ::core::ffi::c_uchar;
            let fresh12 = i;
            i = i + 1;
            UpdateModel(t, fresh12);
            *t.ptr1.offset(i as isize) = j as ::core::ffi::c_uchar;
            let fresh13 = i;
            i = i + 1;
            UpdateModel(t, fresh13);
            *t.ptr1.offset(i as isize) = j as ::core::ffi::c_uchar;
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
                *t.ptr1.offset(fresh15 as isize) = k as ::core::ffi::c_uchar;
                let fresh16 = i;
                i = i + 1;
                *t.ptr1.offset(fresh16 as isize) = j as ::core::ffi::c_uchar;
                j += 1;
            }
            k += 1;
        }
        j = 0 as ::core::ffi::c_long;
        while i < preLoadSize && j < 256 as ::core::ffi::c_long {
            let fresh17 = i;
            i = i + 1;
            *t.ptr1.offset(fresh17 as isize) = j as ::core::ffi::c_uchar;
            let fresh18 = i;
            i = i + 1;
            *t.ptr1.offset(fresh18 as isize) = j as ::core::ffi::c_uchar;
            let fresh19 = i;
            i = i + 1;
            *t.ptr1.offset(fresh19 as isize) = j as ::core::ffi::c_uchar;
            let fresh20 = i;
            i = i + 1;
            *t.ptr1.offset(fresh20 as isize) = j as ::core::ffi::c_uchar;
            j += 1;
        }
    }
    if j == 256 as ::core::ffi::c_long {} else {
        __assert_fail(
            b"j == 256\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            483 as ::core::ffi::c_uint,
            b"void InitializeModel(LZCOMP *, int)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if i == preLoadSize {} else {
        __assert_fail(
            b"i == preLoadSize\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            484 as ::core::ffi::c_uint,
            b"void InitializeModel(LZCOMP *, int)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
}

unsafe fn Decode(t: &mut LZCOMP) -> Vec<u8> {
    let mut symbol: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_long = 0;
    let mut length: ::core::ffi::c_long = 0;
    let mut distance: ::core::ffi::c_long = 0;
    let mut start: ::core::ffi::c_long = 0;
    let mut pos: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut numDistRanges: ::core::ffi::c_long = 0;
    let mut ptr1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
        ::core::ffi::c_uchar,
    >();
    let mut value: ::core::ffi::c_uchar = 0;
    let mut usingRunLength = t.usingRunLength;
    
    let preLoadSize: ::core::ffi::c_long = (2 as ::core::ffi::c_int
        * 32 as ::core::ffi::c_int * 96 as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as ::core::ffi::c_long;

    let mut dataOut = Vec::new();
    InitializeModel(t, false_0);
    if t.ptr1_IsSizeLimited == 0 {
        ptr1 = t.ptr1.offset(preLoadSize as isize);
        pos = 0 as ::core::ffi::c_long;
        while pos < t.out_len {
            symbol = t.sym_ecoder.read_symbol(&mut t.bio) as ::core::ffi::c_int;
            if symbol < 256 as ::core::ffi::c_int {
                value = symbol as ::core::ffi::c_uchar;
            } else if symbol as ::core::ffi::c_long == t.DUP2 {
                value = *ptr1.offset((pos - 2 as ::core::ffi::c_long) as isize);
            } else if symbol as ::core::ffi::c_long == t.DUP4 {
                value = *ptr1.offset((pos - 4 as ::core::ffi::c_long) as isize);
            } else if symbol as ::core::ffi::c_long == t.DUP6 {
                value = *ptr1.offset((pos - 6 as ::core::ffi::c_long) as isize);
            } else {
                length = DecodeLength(t, symbol, &raw mut numDistRanges);
                distance = DecodeDistance2(t, numDistRanges);
                if distance >= max_2Byte_Dist {
                    length += 1;
                }
                start = pos - distance - length + 1 as ::core::ffi::c_long;
                j = 0 as ::core::ffi::c_long;
                while j < length {
                    value = *ptr1.offset((start + j) as isize);
                    let fresh3 = pos;
                    pos = pos + 1;
                    *ptr1.offset(fresh3 as isize) = value;
                    if usingRunLength {
                        t.rlComp.save_bytes(value, &mut dataOut);
                    } else {
                        dataOut.push(value);
                    }
                    j += 1;
                }
                continue;
            }
            let fresh5 = pos;
            pos = pos + 1;
            *ptr1.offset(fresh5 as isize) = value;
            if usingRunLength {
                t.rlComp.save_bytes(value, &mut dataOut);
            } else {
                dataOut.push(value);
            }
        }
    } else {
        let mut src: ::core::ffi::c_long = 0;
        let mut dst: ::core::ffi::c_long = preLoadSize;
        ptr1 = t.ptr1;
        if t.maxCopyDistance > preLoadSize {} else {
            __assert_fail(
                b"t->maxCopyDistance > preLoadSize\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
                854 as ::core::ffi::c_uint,
                b"unsigned char *Decode(LZCOMP *, long *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
        pos = 0 as ::core::ffi::c_long;
        while pos < t.out_len {
            symbol = t.sym_ecoder.read_symbol(&mut t.bio) as ::core::ffi::c_int;
            if symbol < 256 as ::core::ffi::c_int {
                value = symbol as ::core::ffi::c_uchar;
            } else if symbol as ::core::ffi::c_long == t.DUP2 {
                src = dst - 2 as ::core::ffi::c_long;
                if src < 0 as ::core::ffi::c_long {
                    src = src + t.maxCopyDistance;
                }
                value = *ptr1.offset(src as isize);
            } else if symbol as ::core::ffi::c_long == t.DUP4 {
                src = dst - 4 as ::core::ffi::c_long;
                if src < 0 as ::core::ffi::c_long {
                    src = src + t.maxCopyDistance;
                }
                value = *ptr1.offset(src as isize);
            } else if symbol as ::core::ffi::c_long == t.DUP6 {
                src = dst - 6 as ::core::ffi::c_long;
                if src < 0 as ::core::ffi::c_long {
                    src = src + t.maxCopyDistance;
                }
                value = *ptr1.offset(src as isize);
            } else {
                length = DecodeLength(t, symbol, &raw mut numDistRanges);
                distance = DecodeDistance2(t, numDistRanges);
                if distance >= max_2Byte_Dist {
                    length += 1;
                }
                start = dst - distance - length + 1 as ::core::ffi::c_long;
                if distance + length - 1 as ::core::ffi::c_long <= t.maxCopyDistance
                {} else {
                    __assert_fail(
                        b"distance + length - 1 <= t->maxCopyDistance\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"src/lzcomp/lzcomp.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        885 as ::core::ffi::c_uint,
                        b"unsigned char *Decode(LZCOMP *, long *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                };
                j = 0 as ::core::ffi::c_long;
                while j < length {
                    src = start + j;
                    if src < 0 as ::core::ffi::c_long {
                        src = src + t.maxCopyDistance;
                    }
                    value = *ptr1.offset(src as isize);
                    *ptr1.offset(dst as isize) = value;
                    dst = (dst + 1 as ::core::ffi::c_long) % t.maxCopyDistance;
                    pos += 1;
                    if usingRunLength {
                        t.rlComp.save_bytes(value, &mut dataOut);
                    } else {
                        dataOut.push(value);
                    }
                    j += 1;
                }
                continue;
            }
            *ptr1.offset(dst as isize) = value;
            dst = (dst + 1 as ::core::ffi::c_long) % t.maxCopyDistance;
            pos += 1;
            if usingRunLength {
                t.rlComp.save_bytes(value, &mut dataOut);
            } else {
                dataOut.push(value);
            }
        }
    }
    assert!(pos == t.out_len);
    eprintln!("data len: {}, out_len: {}", dataOut.len(), t.out_len);
    assert!(t.usingRunLength || dataOut.len() == t.out_len as usize);
    if pos != t.out_len {
        longjmp(
            &raw mut (*t.mem).env as *mut __jmp_buf_tag,
            ERR_LZCOMP_Decode_bounds,
        );
    }

    dataOut
}

pub unsafe fn MTX_LZCOMP_UnPackMemory(
    mut dataIn: *mut ::core::ffi::c_void,
    mut dataInSize: ::core::ffi::c_long,
    mut version: ::core::ffi::c_uchar,
) -> Vec<u8> {
    let len_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    let dist_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    let preLoadSize: ::core::ffi::c_long = (2 as ::core::ffi::c_int
        * 32 as ::core::ffi::c_int * 96 as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as ::core::ffi::c_long;

    let mut mem: *mut MTX_MemHandler = MTX_mem_Create(
        Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
        Some(realloc as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void),
        Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    assert!(!mem.is_null());

    let NUM_SYMS = 0i64;

    let mut t = LZCOMP {
        ptr1: core::ptr::null_mut(),
        ptr1_IsSizeLimited: 0,
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
        mem,
    };

    t.usingRunLength = if version == 1 { false } else { t.bio.input_bit() != 0 };
    t.out_len = t.bio.read_value(24 as ::core::ffi::c_long) as i64;
    assert!(!dataIn.is_null());
    let out_len = t.out_len;
    SetDistRange(&mut t, out_len);

    let mut maxOutSize:  ::core::ffi::c_long =  t.out_len + preLoadSize;
    t.ptr1 = MTX_mem_malloc(
        t.mem,
        (::core::mem::size_of::<::core::ffi::c_uchar>() as ::core::ffi::c_ulong)
            .wrapping_mul(
                (if t.maxCopyDistance < maxOutSize {
                    t.ptr1_IsSizeLimited = true_0 as ::core::ffi::c_char;
                    t.maxCopyDistance
                } else {
                    maxOutSize
                }) as ::core::ffi::c_ulong,
            ),
    ) as *mut ::core::ffi::c_uchar;

    t.sym_ecoder = AHUFF::new(t.NUM_SYMS as i16);

    let dataOut = Decode(&mut t); // do the work!
    assert!(t.usingRunLength || (dataOut.len() as i64) < maxOutSize);
    free(mem as *mut ::core::ffi::c_void);
    dataOut
}
