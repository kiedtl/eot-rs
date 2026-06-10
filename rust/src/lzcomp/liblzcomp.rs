extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn BEReadU8(s: *mut Stream, out: *mut uint8_t) -> StreamResult;
    fn BEReadU24(s: *mut Stream, out: *mut uint32_t) -> StreamResult;
    fn MTX_mem_Create(
        mptr: MTX_MALLOCPTR,
        rptr: MTX_REALLOCPTR,
        fptr: MTX_FREEPTR,
    ) -> *mut MTX_MemHandler;
    fn MTX_LZCOMP_UnPackMemory(
        t: *mut LZCOMP,
        dataIn: *mut ::core::ffi::c_void,
        dataInSize: ::core::ffi::c_long,
        sizeOut: *mut ::core::ffi::c_long,
        version: ::core::ffi::c_uchar,
    ) -> *mut ::core::ffi::c_uchar;
    fn MTX_LZCOMP_Create1(mem: *mut MTX_MemHandler) -> *mut LZCOMP;
    fn MTX_LZCOMP_Destroy(t: *mut LZCOMP);
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type EOTError = ::core::ffi::c_uint;
pub const EOT_WARN_NOT_ENOUGH_GLYPHS: EOTError = 1002;
pub const EOT_WARN_BAD_VERSION: EOTError = 1001;
pub const EOT_WARN_NOT_ENOUGH_SPACE_RESERVED: EOTError = 1000;
pub const EOT_MALFORMED_HEAD_TABLE: EOTError = 19;
pub const EOT_MTX_ERROR: EOTError = 18;
pub const EOT_UNKNOWN_BUFFER_WRITE_ERROR: EOTError = 17;
pub const EOT_CORRUPT_HOPCODE_DATA: EOTError = 16;
pub const EOT_NO_HDMX_TABLE: EOTError = 15;
pub const EOT_NO_HMTX_TABLE: EOTError = 14;
pub const EOT_NO_HEAD_TABLE: EOTError = 13;
pub const EOT_NO_MAXP_TABLE: EOTError = 12;
pub const EOT_LOGIC_ERROR: EOTError = 11;
pub const EOT_COMPRESSION_NOT_YET_IMPLEMENTED: EOTError = 10;
pub const EOT_FWRITE_ERROR: EOTError = 9;
pub const EOT_OTHER_STDLIB_ERROR: EOTError = 8;
pub const EOT_CANT_ALLOCATE_MEMORY: EOTError = 7;
pub const EOT_THIRD_STREAM_INCOMPLETE: EOTError = 6;
pub const EOT_SECOND_STREAM_INCOMPLETE: EOTError = 5;
pub const EOT_CORRUPT_FILE: EOTError = 4;
pub const EOT_BOGUS_STRING_SIZE: EOTError = 3;
pub const EOT_HEADER_TOO_BIG: EOTError = 2;
pub const EOT_INSUFFICIENT_BYTES: EOTError = 1;
pub const EOT_SUCCESS: EOTError = 0;
pub type StreamResult = ::core::ffi::c_uint;
pub const EOT_OFF_BYTE_BOUNDARY: StreamResult = 7;
pub const EOT_VALUE_OUT_OF_BOUNDS: StreamResult = 6;
pub const EOT_OUT_OF_RESERVED_SPACE: StreamResult = 5;
pub const EOT_CANT_ALLOCATE_MEMORY_FOR_STREAM: StreamResult = 4;
pub const EOT_SEEK_PAST_EOS: StreamResult = 3;
pub const EOT_NEGATIVE_SEEK: StreamResult = 2;
pub const EOT_NOT_ENOUGH_DATA: StreamResult = 1;
pub const EOT_STREAM_OK: StreamResult = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub buf: *mut uint8_t,
    pub size: ::core::ffi::c_uint,
    pub reserved: ::core::ffi::c_uint,
    pub pos: ::core::ffi::c_uint,
    pub bitPos: ::core::ffi::c_uint,
}
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub unsafe extern "C" fn be24ToCpu(mut buf: *const uint8_t) -> ::core::ffi::c_uint {
    return *buf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
        | (*buf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            << 8 as ::core::ffi::c_int
        | (*buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            << 16 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn unpackMtx(
    mut buf: *mut Stream,
    mut _size: ::core::ffi::c_uint,
    mut bufsOut: *mut *mut uint8_t,
    mut bufSizesOut: *mut ::core::ffi::c_uint,
) -> EOTError {
    let mut versionMagic: uint8_t = 0;
    let mut offsets: [uint32_t; 3] = [0; 3];
    let mut copyLimit: uint32_t = 0;
    let mut sizes: [::core::ffi::c_uint; 3] = [0; 3];
    let mut current_block: u64;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < 3 as ::core::ffi::c_uint {
        let ref mut fresh0 = *bufsOut.offset(i as isize);
        *fresh0 = ::core::ptr::null_mut::<uint8_t>();
        i = i.wrapping_add(1);
    }
    let mut sResult: StreamResult = EOT_STREAM_OK;
    let mut returnedStatus: EOTError = EOT_SUCCESS;
    let mut lzcomp: *mut LZCOMP = ::core::ptr::null_mut::<LZCOMP>();
    let mut mem: *mut MTX_MemHandler = MTX_mem_Create(
        Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
        Some(
            realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    if !mem.is_null() {
        lzcomp = MTX_LZCOMP_Create1(mem);
        if !lzcomp.is_null() {
            versionMagic = 0;
            offsets = [0; 3];
            offsets[0 as ::core::ffi::c_int as usize] = 10 as uint32_t;
            copyLimit = 0;
            sResult = BEReadU8(buf, &raw mut versionMagic);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                returnedStatus = EOT_MTX_ERROR;
            } else {
                sResult = BEReadU24(buf, &raw mut copyLimit);
                if sResult as ::core::ffi::c_uint
                    != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    returnedStatus = EOT_MTX_ERROR;
                } else {
                    let mut i_0: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
                    loop {
                        if !(i_0 < 3 as ::core::ffi::c_uint) {
                            current_block = 2668756484064249700;
                            break;
                        }
                        sResult = BEReadU24(
                            buf,
                            (&raw mut offsets as *mut uint32_t).offset(i_0 as isize)
                                as *mut uint32_t,
                        );
                        if sResult as ::core::ffi::c_uint
                            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            returnedStatus = EOT_MTX_ERROR;
                            current_block = 4245893770312551903;
                            break;
                        } else {
                            i_0 = i_0.wrapping_add(1);
                        }
                    }
                    match current_block {
                        4245893770312551903 => {}
                        _ => {
                            sizes = [
                                offsets[1 as ::core::ffi::c_int as usize]
                                    .wrapping_sub(offsets[0 as ::core::ffi::c_int as usize]),
                                offsets[2 as ::core::ffi::c_int as usize]
                                    .wrapping_sub(offsets[1 as ::core::ffi::c_int as usize]),
                                ((*buf).size as uint32_t)
                                    .wrapping_sub(offsets[2 as ::core::ffi::c_int as usize]),
                            ];
                            let mut i_1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                            while i_1 < 3 as ::core::ffi::c_uint {
                                if offsets[i_1 as usize].wrapping_add(sizes[i_1 as usize])
                                    > (*buf).size as uint32_t
                                {
                                    returnedStatus = EOT_MTX_ERROR;
                                    break;
                                } else {
                                    let mut sizeOut: ::core::ffi::c_long = 0;
                                    let ref mut fresh1 = *bufsOut.offset(i_1 as isize);
                                    *fresh1 = MTX_LZCOMP_UnPackMemory(
                                        lzcomp,
                                        (*buf).buf.offset(offsets[i_1 as usize] as isize)
                                            as *mut ::core::ffi::c_void,
                                        sizes[i_1 as usize] as ::core::ffi::c_long,
                                        &raw mut sizeOut,
                                        versionMagic as ::core::ffi::c_uchar,
                                    ) as *mut uint8_t;
                                    *bufSizesOut.offset(i_1 as isize) = sizeOut
                                        as ::core::ffi::c_uint;
                                    if (*bufsOut.offset(i_1 as isize)).is_null() {
                                        returnedStatus = EOT_MTX_ERROR;
                                        break;
                                    } else {
                                        i_1 = i_1.wrapping_add(1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !lzcomp.is_null() {
        MTX_LZCOMP_Destroy(lzcomp);
    }
    free(mem as *mut ::core::ffi::c_void);
    return returnedStatus;
}
