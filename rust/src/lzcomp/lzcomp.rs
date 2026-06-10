extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
    fn MTX_mem_malloc(
        t: *mut MTX_MemHandler,
        size: ::core::ffi::c_ulong,
    ) -> *mut ::core::ffi::c_void;
    fn MTX_mem_realloc(
        t: *mut MTX_MemHandler,
        p: *mut ::core::ffi::c_void,
        size: ::core::ffi::c_ulong,
    ) -> *mut ::core::ffi::c_void;
    fn MTX_mem_free(t: *mut MTX_MemHandler, deadObject: *mut ::core::ffi::c_void);
    fn MTX_BITIO_ReadValue(
        t: *mut BITIO,
        numberOfBits: ::core::ffi::c_long,
    ) -> ::core::ffi::c_ulong;
    fn MTX_BITIO_input_bit(t: *mut BITIO) -> ::core::ffi::c_short;
    fn MTX_BITIO_Create(
        mem: *mut MTX_MemHandler,
        memPtr: *mut ::core::ffi::c_void,
        memSize: ::core::ffi::c_long,
        param: ::core::ffi::c_char,
    ) -> *mut BITIO;
    fn MTX_BITIO_Destroy(t: *mut BITIO);
    fn MTX_AHUFF_ReadSymbol(t: *mut AHUFF) -> ::core::ffi::c_short;
    fn MTX_AHUFF_Create(
        mem: *mut MTX_MemHandler,
        bio: *mut BITIO,
        range: ::core::ffi::c_short,
    ) -> *mut AHUFF;
    fn MTX_AHUFF_Destroy(t: *mut AHUFF);
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
pub type MTX_MALLOCPTR = Option<
    unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void,
>;
pub type MTX_REALLOCPTR = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
>;
pub type MTX_FREEPTR = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MTX_MemHandler {
    pub mem_pointers: *mut mem_struct,
    pub mem_maxPointers: ::core::ffi::c_long,
    pub mem_numPointers: ::core::ffi::c_long,
    pub mem_numNewCalls: ::core::ffi::c_long,
    pub malloc: MTX_MALLOCPTR,
    pub realloc: MTX_REALLOCPTR,
    pub free: MTX_FREEPTR,
    pub env: jmp_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BITIO {
    pub mem_bytes: *mut ::core::ffi::c_uchar,
    pub mem_index: ::core::ffi::c_long,
    pub mem_size: ::core::ffi::c_long,
    pub input_bit_count: ::core::ffi::c_ushort,
    pub input_bit_buffer: ::core::ffi::c_ushort,
    pub bytes_in: ::core::ffi::c_long,
    pub output_bit_count: ::core::ffi::c_ushort,
    pub output_bit_buffer: ::core::ffi::c_ushort,
    pub bytes_out: ::core::ffi::c_long,
    pub ReadOrWrite: ::core::ffi::c_char,
    pub mem: *mut MTX_MemHandler,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodeType {
    pub up: ::core::ffi::c_short,
    pub left: ::core::ffi::c_short,
    pub right: ::core::ffi::c_short,
    pub code: ::core::ffi::c_short,
    pub weight: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AHUFF {
    pub tree: *mut nodeType,
    pub symbolIndex: *mut ::core::ffi::c_short,
    pub bitCount: ::core::ffi::c_long,
    pub bitCount2: ::core::ffi::c_long,
    pub range: ::core::ffi::c_long,
    pub bio: *mut BITIO,
    pub mem: *mut MTX_MemHandler,
    pub maxSymbol: ::core::ffi::c_int,
    pub countA: ::core::ffi::c_long,
    pub countB: ::core::ffi::c_long,
    pub sym_count: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RUNLENGTHCOMP {
    pub escape: ::core::ffi::c_uchar,
    pub count: ::core::ffi::c_uchar,
    pub state: ::core::ffi::c_uchar,
    pub mem: *mut MTX_MemHandler,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZCOMP {
    pub ptr1: *mut ::core::ffi::c_uchar,
    pub ptr1_IsSizeLimited: ::core::ffi::c_char,
    pub filler1: ::core::ffi::c_char,
    pub filer2: ::core::ffi::c_char,
    pub filler3: ::core::ffi::c_char,
    pub rlComp: *mut RUNLENGTHCOMP,
    pub usingRunLength: ::core::ffi::c_short,
    pub length1: ::core::ffi::c_long,
    pub out_len: ::core::ffi::c_long,
    pub maxIndex: ::core::ffi::c_long,
    pub num_DistRanges: ::core::ffi::c_long,
    pub dist_max: ::core::ffi::c_long,
    pub DUP2: ::core::ffi::c_long,
    pub DUP4: ::core::ffi::c_long,
    pub DUP6: ::core::ffi::c_long,
    pub NUM_SYMS: ::core::ffi::c_long,
    pub maxCopyDistance: ::core::ffi::c_long,
    pub dist_ecoder: *mut AHUFF,
    pub len_ecoder: *mut AHUFF,
    pub sym_ecoder: *mut AHUFF,
    pub bitIn: *mut BITIO,
    pub bitOut: *mut BITIO,
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
unsafe extern "C" fn SetDistRange(mut t: *mut LZCOMP, mut length: ::core::ffi::c_long) {
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
    mut t: *mut LZCOMP,
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
            bits = MTX_AHUFF_ReadSymbol((*t).len_ecoder) as ::core::ffi::c_long;
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
unsafe extern "C" fn UpdateModel(mut _t: *mut LZCOMP, mut _index: ::core::ffi::c_long) {}
unsafe extern "C" fn DecodeDistance2(
    mut t: *mut LZCOMP,
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
        bits = MTX_AHUFF_ReadSymbol((*t).dist_ecoder) as ::core::ffi::c_long;
        value <<= dist_width;
        value |= bits;
        i -= 1;
    }
    value += dist_min;
    return value;
}
unsafe extern "C" fn InitializeModel(
    mut t: *mut LZCOMP,
    mut compress: ::core::ffi::c_int,
) {
    
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
                *(*t).ptr1.offset(i as isize) = k as ::core::ffi::c_uchar;
                let fresh9 = i;
                i = i + 1;
                UpdateModel(t, fresh9);
                *(*t).ptr1.offset(i as isize) = j as ::core::ffi::c_uchar;
                let fresh10 = i;
                i = i + 1;
                UpdateModel(t, fresh10);
                j += 1;
            }
            k += 1;
        }
        j = 0 as ::core::ffi::c_long;
        while i < preLoadSize && j < 256 as ::core::ffi::c_long {
            *(*t).ptr1.offset(i as isize) = j as ::core::ffi::c_uchar;
            let fresh11 = i;
            i = i + 1;
            UpdateModel(t, fresh11);
            *(*t).ptr1.offset(i as isize) = j as ::core::ffi::c_uchar;
            let fresh12 = i;
            i = i + 1;
            UpdateModel(t, fresh12);
            *(*t).ptr1.offset(i as isize) = j as ::core::ffi::c_uchar;
            let fresh13 = i;
            i = i + 1;
            UpdateModel(t, fresh13);
            *(*t).ptr1.offset(i as isize) = j as ::core::ffi::c_uchar;
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
                *(*t).ptr1.offset(fresh15 as isize) = k as ::core::ffi::c_uchar;
                let fresh16 = i;
                i = i + 1;
                *(*t).ptr1.offset(fresh16 as isize) = j as ::core::ffi::c_uchar;
                j += 1;
            }
            k += 1;
        }
        j = 0 as ::core::ffi::c_long;
        while i < preLoadSize && j < 256 as ::core::ffi::c_long {
            let fresh17 = i;
            i = i + 1;
            *(*t).ptr1.offset(fresh17 as isize) = j as ::core::ffi::c_uchar;
            let fresh18 = i;
            i = i + 1;
            *(*t).ptr1.offset(fresh18 as isize) = j as ::core::ffi::c_uchar;
            let fresh19 = i;
            i = i + 1;
            *(*t).ptr1.offset(fresh19 as isize) = j as ::core::ffi::c_uchar;
            let fresh20 = i;
            i = i + 1;
            *(*t).ptr1.offset(fresh20 as isize) = j as ::core::ffi::c_uchar;
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
unsafe extern "C" fn Decode(
    mut t: *mut LZCOMP,
    mut size: *mut ::core::ffi::c_long,
) -> *mut ::core::ffi::c_uchar {
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
    let mut usingRunLength: ::core::ffi::c_int = (*t).usingRunLength
        as ::core::ffi::c_int;
    
    let mut index: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    
    let preLoadSize: ::core::ffi::c_long = (2 as ::core::ffi::c_int
        * 32 as ::core::ffi::c_int * 96 as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as ::core::ffi::c_long;
    
    let mut dataOutSize:  ::core::ffi::c_long =  (*t).out_len;let mut dataOut:  *mut ::core::ffi::c_uchar =
     MTX_mem_malloc((*t).mem, dataOutSize as ::core::ffi::c_ulong)
        as *mut ::core::ffi::c_uchar;
    InitializeModel(t, false_0);
    if (*t).ptr1_IsSizeLimited == 0 {
        ptr1 = (*t).ptr1.offset(preLoadSize as isize);
        pos = 0 as ::core::ffi::c_long;
        while pos < (*t).out_len {
            symbol = MTX_AHUFF_ReadSymbol((*t).sym_ecoder) as ::core::ffi::c_int;
            if symbol < 256 as ::core::ffi::c_int {
                value = symbol as ::core::ffi::c_uchar;
            } else if symbol as ::core::ffi::c_long == (*t).DUP2 {
                value = *ptr1.offset((pos - 2 as ::core::ffi::c_long) as isize);
            } else if symbol as ::core::ffi::c_long == (*t).DUP4 {
                value = *ptr1.offset((pos - 4 as ::core::ffi::c_long) as isize);
            } else if symbol as ::core::ffi::c_long == (*t).DUP6 {
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
                    if usingRunLength != 0 {
                        MTX_RUNLENGTHCOMP_SaveBytes(
                            (*t).rlComp,
                            value,
                            &raw mut dataOut,
                            &raw mut dataOutSize,
                            &raw mut index,
                        );
                    } else {
                        if index <= dataOutSize {} else {
                            __assert_fail(
                                b"index <= dataOutSize\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"src/lzcomp/lzcomp.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                822 as ::core::ffi::c_uint,
                                b"unsigned char *Decode(LZCOMP *, long *)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        };
                        if index >= dataOutSize {
                            dataOutSize += dataOutSize >> 1 as ::core::ffi::c_int;
                            dataOut = MTX_mem_realloc(
                                (*t).mem,
                                dataOut as *mut ::core::ffi::c_void,
                                dataOutSize as ::core::ffi::c_ulong,
                            ) as *mut ::core::ffi::c_uchar;
                        }
                        let fresh4 = index;
                        index = index + 1;
                        *dataOut.offset(fresh4 as isize) = value;
                    }
                    j += 1;
                }
                continue;
            }
            let fresh5 = pos;
            pos = pos + 1;
            *ptr1.offset(fresh5 as isize) = value;
            if usingRunLength != 0 {
                MTX_RUNLENGTHCOMP_SaveBytes(
                    (*t).rlComp,
                    value,
                    &raw mut dataOut,
                    &raw mut dataOutSize,
                    &raw mut index,
                );
            } else {
                if index <= dataOutSize {} else {
                    __assert_fail(
                        b"index <= dataOutSize\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"src/lzcomp/lzcomp.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        840 as ::core::ffi::c_uint,
                        b"unsigned char *Decode(LZCOMP *, long *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                };
                if index >= dataOutSize {
                    dataOutSize += dataOutSize >> 1 as ::core::ffi::c_int;
                    dataOut = MTX_mem_realloc(
                        (*t).mem,
                        dataOut as *mut ::core::ffi::c_void,
                        dataOutSize as ::core::ffi::c_ulong,
                    ) as *mut ::core::ffi::c_uchar;
                }
                let fresh6 = index;
                index = index + 1;
                *dataOut.offset(fresh6 as isize) = value;
            }
        }
    } else {
        let mut src: ::core::ffi::c_long = 0;
        let mut dst: ::core::ffi::c_long = preLoadSize;
        ptr1 = (*t).ptr1;
        if (*t).maxCopyDistance > preLoadSize {} else {
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
        while pos < (*t).out_len {
            symbol = MTX_AHUFF_ReadSymbol((*t).sym_ecoder) as ::core::ffi::c_int;
            if symbol < 256 as ::core::ffi::c_int {
                value = symbol as ::core::ffi::c_uchar;
            } else if symbol as ::core::ffi::c_long == (*t).DUP2 {
                src = dst - 2 as ::core::ffi::c_long;
                if src < 0 as ::core::ffi::c_long {
                    src = src + (*t).maxCopyDistance;
                }
                value = *ptr1.offset(src as isize);
            } else if symbol as ::core::ffi::c_long == (*t).DUP4 {
                src = dst - 4 as ::core::ffi::c_long;
                if src < 0 as ::core::ffi::c_long {
                    src = src + (*t).maxCopyDistance;
                }
                value = *ptr1.offset(src as isize);
            } else if symbol as ::core::ffi::c_long == (*t).DUP6 {
                src = dst - 6 as ::core::ffi::c_long;
                if src < 0 as ::core::ffi::c_long {
                    src = src + (*t).maxCopyDistance;
                }
                value = *ptr1.offset(src as isize);
            } else {
                length = DecodeLength(t, symbol, &raw mut numDistRanges);
                distance = DecodeDistance2(t, numDistRanges);
                if distance >= max_2Byte_Dist {
                    length += 1;
                }
                start = dst - distance - length + 1 as ::core::ffi::c_long;
                if distance + length - 1 as ::core::ffi::c_long <= (*t).maxCopyDistance
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
                        src = src + (*t).maxCopyDistance;
                    }
                    value = *ptr1.offset(src as isize);
                    *ptr1.offset(dst as isize) = value;
                    dst = (dst + 1 as ::core::ffi::c_long) % (*t).maxCopyDistance;
                    pos += 1;
                    if usingRunLength != 0 {
                        MTX_RUNLENGTHCOMP_SaveBytes(
                            (*t).rlComp,
                            value,
                            &raw mut dataOut,
                            &raw mut dataOutSize,
                            &raw mut index,
                        );
                    } else {
                        if index <= dataOutSize {} else {
                            __assert_fail(
                                b"index <= dataOutSize\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"src/lzcomp/lzcomp.c\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                898 as ::core::ffi::c_uint,
                                b"unsigned char *Decode(LZCOMP *, long *)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        };
                        if index >= dataOutSize {
                            dataOutSize += dataOutSize >> 1 as ::core::ffi::c_int;
                            dataOut = MTX_mem_realloc(
                                (*t).mem,
                                dataOut as *mut ::core::ffi::c_void,
                                dataOutSize as ::core::ffi::c_ulong,
                            ) as *mut ::core::ffi::c_uchar;
                        }
                        let fresh7 = index;
                        index = index + 1;
                        *dataOut.offset(fresh7 as isize) = value;
                    }
                    j += 1;
                }
                continue;
            }
            *ptr1.offset(dst as isize) = value;
            dst = (dst + 1 as ::core::ffi::c_long) % (*t).maxCopyDistance;
            pos += 1;
            if usingRunLength != 0 {
                MTX_RUNLENGTHCOMP_SaveBytes(
                    (*t).rlComp,
                    value,
                    &raw mut dataOut,
                    &raw mut dataOutSize,
                    &raw mut index,
                );
            } else {
                if index <= dataOutSize {} else {
                    __assert_fail(
                        b"index <= dataOutSize\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"src/lzcomp/lzcomp.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        918 as ::core::ffi::c_uint,
                        b"unsigned char *Decode(LZCOMP *, long *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                };
                if index >= dataOutSize {
                    dataOutSize += dataOutSize >> 1 as ::core::ffi::c_int;
                    dataOut = MTX_mem_realloc(
                        (*t).mem,
                        dataOut as *mut ::core::ffi::c_void,
                        dataOutSize as ::core::ffi::c_ulong,
                    ) as *mut ::core::ffi::c_uchar;
                }
                let fresh8 = index;
                index = index + 1;
                *dataOut.offset(fresh8 as isize) = value;
            }
        }
    }
    if pos == (*t).out_len {} else {
        __assert_fail(
            b"pos == t->out_len\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            930 as ::core::ffi::c_uint,
            b"unsigned char *Decode(LZCOMP *, long *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (*t).usingRunLength as ::core::ffi::c_int != 0 || index == (*t).out_len {} else {
        __assert_fail(
            b"t->usingRunLength || index == t->out_len\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            931 as ::core::ffi::c_uint,
            b"unsigned char *Decode(LZCOMP *, long *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if pos != (*t).out_len {
        longjmp(
            &raw mut (*(*t).mem).env as *mut __jmp_buf_tag,
            ERR_LZCOMP_Decode_bounds,
        );
    }
    *size = index;
    if dataOutSize >= *size {} else {
        __assert_fail(
            b"dataOutSize >= *size\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            935 as ::core::ffi::c_uint,
            b"unsigned char *Decode(LZCOMP *, long *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (*t).usingRunLength != 0 {
        dataOut = MTX_mem_realloc(
            (*t).mem,
            dataOut as *mut ::core::ffi::c_void,
            *size as ::core::ffi::c_ulong,
        ) as *mut ::core::ffi::c_uchar;
    }
    return dataOut;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_LZCOMP_UnPackMemory(
    mut t: *mut LZCOMP,
    mut dataIn: *mut ::core::ffi::c_void,
    mut dataInSize: ::core::ffi::c_long,
    mut sizeOut: *mut ::core::ffi::c_long,
    mut version: ::core::ffi::c_uchar,
) -> *mut ::core::ffi::c_uchar {
    
    
    let len_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    let dist_width: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
    let preLoadSize: ::core::ffi::c_long = (2 as ::core::ffi::c_int
        * 32 as ::core::ffi::c_int * 96 as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as ::core::ffi::c_long;
    if !dataIn.is_null() {} else {
        __assert_fail(
            b"dataIn != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            1063 as ::core::ffi::c_uint,
            b"unsigned char *MTX_LZCOMP_UnPackMemory(LZCOMP *, void *, long, long *, unsigned char)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if !(*t).ptr1.is_null() {
        MTX_mem_free((*t).mem, (*t).ptr1 as *mut ::core::ffi::c_void);
    }
    (*t).ptr1 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*t).rlComp = MTX_RUNLENGTHCOMP_Create((*t).mem);
    (*t).bitIn = MTX_BITIO_Create(
        (*t).mem,
        dataIn,
        dataInSize,
        'r' as i32 as ::core::ffi::c_char,
    );
    if !(*t).bitIn.is_null() {} else {
        __assert_fail(
            b"t->bitIn != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            1073 as ::core::ffi::c_uint,
            b"unsigned char *MTX_LZCOMP_UnPackMemory(LZCOMP *, void *, long, long *, unsigned char)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    };
    if version as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        (*t).usingRunLength = false_0 as ::core::ffi::c_short;
    } else {
        (*t).usingRunLength = MTX_BITIO_input_bit((*t).bitIn);
    }
    (*t).dist_ecoder = MTX_AHUFF_Create(
        (*t).mem,
        (*t).bitIn,
        ((1 as ::core::ffi::c_long) << dist_width) as ::core::ffi::c_short,
    );
    if !(*t).dist_ecoder.is_null() {} else {
        __assert_fail(
            b"t->dist_ecoder != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            1082 as ::core::ffi::c_uint,
            b"unsigned char *MTX_LZCOMP_UnPackMemory(LZCOMP *, void *, long, long *, unsigned char)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    };
    (*t).len_ecoder = MTX_AHUFF_Create(
        (*t).mem,
        (*t).bitIn,
        ((1 as ::core::ffi::c_long) << len_width) as ::core::ffi::c_short,
    );
    if !(*t).len_ecoder.is_null() {} else {
        __assert_fail(
            b"t->len_ecoder != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            1084 as ::core::ffi::c_uint,
            b"unsigned char *MTX_LZCOMP_UnPackMemory(LZCOMP *, void *, long, long *, unsigned char)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    };
    (*t).out_len = MTX_BITIO_ReadValue((*t).bitIn, 24 as ::core::ffi::c_long)
        as ::core::ffi::c_long;
    SetDistRange(t, (*t).out_len);
    let mut maxOutSize:  ::core::ffi::c_long =  (*t).out_len + preLoadSize;
    (*t).ptr1 = MTX_mem_malloc(
        (*t).mem,
        (::core::mem::size_of::<::core::ffi::c_uchar>() as ::core::ffi::c_ulong)
            .wrapping_mul(
                (if (*t).maxCopyDistance < maxOutSize {
                    (*t).ptr1_IsSizeLimited = true_0 as ::core::ffi::c_char;
                    (*t).maxCopyDistance
                } else {
                    maxOutSize
                }) as ::core::ffi::c_ulong,
            ),
    ) as *mut ::core::ffi::c_uchar;
    (*t).sym_ecoder = MTX_AHUFF_Create(
        (*t).mem,
        (*t).bitIn,
        (*t).NUM_SYMS as ::core::ffi::c_short,
    );
    if !(*t).sym_ecoder.is_null() {} else {
        __assert_fail(
            b"t->sym_ecoder != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            1097 as ::core::ffi::c_uint,
            b"unsigned char *MTX_LZCOMP_UnPackMemory(LZCOMP *, void *, long, long *, unsigned char)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    };
    let mut dataOut:  *mut ::core::ffi::c_uchar =  Decode(t, sizeOut);
    MTX_AHUFF_Destroy((*t).dist_ecoder);
    (*t).dist_ecoder = ::core::ptr::null_mut::<AHUFF>();
    MTX_AHUFF_Destroy((*t).len_ecoder);
    (*t).len_ecoder = ::core::ptr::null_mut::<AHUFF>();
    MTX_AHUFF_Destroy((*t).sym_ecoder);
    (*t).sym_ecoder = ::core::ptr::null_mut::<AHUFF>();
    MTX_BITIO_Destroy((*t).bitIn);
    (*t).bitIn = ::core::ptr::null_mut::<BITIO>();
    MTX_RUNLENGTHCOMP_Destroy((*t).rlComp);
    (*t).rlComp = ::core::ptr::null_mut::<RUNLENGTHCOMP>();
    if (*t).usingRunLength as ::core::ffi::c_int != 0 || *sizeOut < maxOutSize {} else {
        __assert_fail(
            b"t->usingRunLength || *sizeOut < maxOutSize\0" as *const u8
                as *const ::core::ffi::c_char,
            b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
            1112 as ::core::ffi::c_uint,
            b"unsigned char *MTX_LZCOMP_UnPackMemory(LZCOMP *, void *, long, long *, unsigned char)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    };
    return dataOut;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_LZCOMP_Create1(
    mut mem: *mut MTX_MemHandler,
) -> *mut LZCOMP {
    //let hNodeAllocSize: ::core::ffi::c_short = 4095 as ::core::ffi::c_short;
    let mut t: *mut LZCOMP = MTX_mem_malloc(
        mem,
        ::core::mem::size_of::<LZCOMP>() as ::core::ffi::c_ulong,
    ) as *mut LZCOMP;
    (*t).mem = mem;
    (*t).ptr1 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*t).maxCopyDistance = 0x7fffffff as ::core::ffi::c_long;
    (*t).ptr1_IsSizeLimited = false_0 as ::core::ffi::c_char;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_LZCOMP_Create2(
    mut mem: *mut MTX_MemHandler,
    mut maxCopyDistance: ::core::ffi::c_long,
) -> *mut LZCOMP {
    let preLoadSize: ::core::ffi::c_long = (2 as ::core::ffi::c_int
        * 32 as ::core::ffi::c_int * 96 as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int * 256 as ::core::ffi::c_int) as ::core::ffi::c_long;
    //let hNodeAllocSize: ::core::ffi::c_short = 4095 as ::core::ffi::c_short;
    let mut t: *mut LZCOMP = MTX_mem_malloc(
        mem,
        ::core::mem::size_of::<LZCOMP>() as ::core::ffi::c_ulong,
    ) as *mut LZCOMP;
    (*t).mem = mem;
    (*t).ptr1 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*t).maxCopyDistance = maxCopyDistance;
    if (*t).maxCopyDistance < preLoadSize + 64 as ::core::ffi::c_long {
        (*t).maxCopyDistance = preLoadSize + 64 as ::core::ffi::c_long;
    }
    (*t).ptr1_IsSizeLimited = false_0 as ::core::ffi::c_char;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_LZCOMP_Destroy(mut t: *mut LZCOMP) {
    MTX_mem_free((*t).mem, (*t).ptr1 as *mut ::core::ffi::c_void);
    MTX_mem_free((*t).mem, t as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub static mut initialState: ::core::ffi::c_uchar = 100 as ::core::ffi::c_uchar;
#[no_mangle]
pub static mut normalState: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
#[no_mangle]
pub static mut seenEscapeState: ::core::ffi::c_uchar = 1 as ::core::ffi::c_uchar;
#[no_mangle]
pub static mut needByteState: ::core::ffi::c_uchar = 2 as ::core::ffi::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn MTX_RUNLENGTHCOMP_SaveBytes(
    mut t: *mut RUNLENGTHCOMP,
    mut value: ::core::ffi::c_uchar,
    mut dataOutRef: *mut *mut ::core::ffi::c_uchar,
    mut dataOutSizeRef: *mut ::core::ffi::c_long,
    mut indexRef: *mut ::core::ffi::c_long,
) {
    let mut dataOut: *mut ::core::ffi::c_uchar = *dataOutRef;
    let mut dataOutSize: ::core::ffi::c_long = *dataOutSizeRef;
    let mut index: ::core::ffi::c_long = *indexRef;
    if (*t).state as ::core::ffi::c_int == normalState as ::core::ffi::c_int {
        if value as ::core::ffi::c_int == (*t).escape as ::core::ffi::c_int {
            (*t).state = seenEscapeState;
        } else {
            if index <= dataOutSize {} else {
                __assert_fail(
                    b"index <= dataOutSize\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1280 as ::core::ffi::c_uint,
                    b"void MTX_RUNLENGTHCOMP_SaveBytes(RUNLENGTHCOMP *, unsigned char, unsigned char **, long *, long *)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            };
            if index >= dataOutSize {
                dataOutSize += dataOutSize >> 1 as ::core::ffi::c_int;
                dataOut = MTX_mem_realloc(
                    (*t).mem,
                    dataOut as *mut ::core::ffi::c_void,
                    dataOutSize as ::core::ffi::c_ulong,
                ) as *mut ::core::ffi::c_uchar;
            }
            let fresh0 = index;
            index = index + 1;
            *dataOut.offset(fresh0 as isize) = value;
        }
    } else if (*t).state as ::core::ffi::c_int == seenEscapeState as ::core::ffi::c_int {
        (*t).count = value;
        if (*t).count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if index <= dataOutSize {} else {
                __assert_fail(
                    b"index <= dataOutSize\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1291 as ::core::ffi::c_uint,
                    b"void MTX_RUNLENGTHCOMP_SaveBytes(RUNLENGTHCOMP *, unsigned char, unsigned char **, long *, long *)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            };
            if index >= dataOutSize {
                dataOutSize += dataOutSize >> 1 as ::core::ffi::c_int;
                dataOut = MTX_mem_realloc(
                    (*t).mem,
                    dataOut as *mut ::core::ffi::c_void,
                    dataOutSize as ::core::ffi::c_ulong,
                ) as *mut ::core::ffi::c_uchar;
            }
            let fresh1 = index;
            index = index + 1;
            *dataOut.offset(fresh1 as isize) = (*t).escape;
            (*t).state = normalState;
        } else {
            (*t).state = needByteState;
        }
    } else if (*t).state as ::core::ffi::c_int == needByteState as ::core::ffi::c_int {
        
        if index + (*t).count as ::core::ffi::c_long > dataOutSize {
            dataOutSize = index + (*t).count as ::core::ffi::c_long
                + (dataOutSize >> 1 as ::core::ffi::c_int);
            dataOut = MTX_mem_realloc(
                (*t).mem,
                dataOut as *mut ::core::ffi::c_void,
                dataOutSize as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_uchar;
        }
        let mut i:  ::core::ffi::c_int =  (*t).count as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int {
            let fresh2 = index;
            index = index + 1;
            *dataOut.offset(fresh2 as isize) = value;
            i -= 1;
        }
        if index <= dataOutSize {} else {
            __assert_fail(
                b"index <= dataOutSize\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
                1316 as ::core::ffi::c_uint,
                b"void MTX_RUNLENGTHCOMP_SaveBytes(RUNLENGTHCOMP *, unsigned char, unsigned char **, long *, long *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        };
        (*t).state = normalState;
    } else {
        if (*t).state as ::core::ffi::c_int == initialState as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"t->state == initialState\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/lzcomp/lzcomp.c\0" as *const u8 as *const ::core::ffi::c_char,
                1319 as ::core::ffi::c_uint,
                b"void MTX_RUNLENGTHCOMP_SaveBytes(RUNLENGTHCOMP *, unsigned char, unsigned char **, long *, long *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        };
        (*t).escape = value;
        (*t).state = normalState;
    }
    *dataOutRef = dataOut;
    *dataOutSizeRef = dataOutSize;
    *indexRef = index;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_RUNLENGTHCOMP_Create(
    mut mem: *mut MTX_MemHandler,
) -> *mut RUNLENGTHCOMP {
    let mut t: *mut RUNLENGTHCOMP = MTX_mem_malloc(
        mem,
        ::core::mem::size_of::<RUNLENGTHCOMP>() as ::core::ffi::c_ulong,
    ) as *mut RUNLENGTHCOMP;
    (*t).mem = mem;
    (*t).state = initialState;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_RUNLENGTHCOMP_Destroy(mut t: *mut RUNLENGTHCOMP) {
    MTX_mem_free((*t).mem, t as *mut ::core::ffi::c_void);
}
