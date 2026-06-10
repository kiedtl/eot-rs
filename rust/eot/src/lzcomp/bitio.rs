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
pub const ERR_BITIO_end_of_file: ::core::ffi::c_int = 3304 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn MTX_BITIO_WriteValue(
    mut t: *mut BITIO,
    mut value: ::core::ffi::c_ulong,
    mut numberOfBits: ::core::ffi::c_long,
) {
    
    let mut i:  ::core::ffi::c_long =  numberOfBits - 1 as ::core::ffi::c_long;
    while i >= 0 as ::core::ffi::c_long {
        MTX_BITIO_output_bit(
            t,
            value & ((1 as ::core::ffi::c_long) << i) as ::core::ffi::c_ulong,
        );
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MTX_BITIO_ReadValue(
    mut t: *mut BITIO,
    mut numberOfBits: ::core::ffi::c_long,
) -> ::core::ffi::c_ulong {
    
    
    
    let mut value:  ::core::ffi::c_ulong =  0 as ::core::ffi::c_ulong;let mut i:  ::core::ffi::c_long =  numberOfBits - 1 as ::core::ffi::c_long;
    while i >= 0 as ::core::ffi::c_long {
        value <<= 1 as ::core::ffi::c_int;
        if MTX_BITIO_input_bit(t) != 0 {
            value |= 1 as ::core::ffi::c_ulong;
        }
        i -= 1;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_BITIO_input_bit(mut t: *mut BITIO) -> ::core::ffi::c_short {
    let fresh1 = (*t).input_bit_count;
    (*t).input_bit_count = (*t).input_bit_count.wrapping_sub(1);
    if fresh1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let fresh2 = (*t).mem_index;
        (*t).mem_index = (*t).mem_index + 1;
        (*t).input_bit_buffer = *(*t).mem_bytes.offset(fresh2 as isize)
            as ::core::ffi::c_ushort;
        if (*t).mem_index > (*t).mem_size {
            longjmp(
                &raw mut (*(*t).mem).env as *mut __jmp_buf_tag,
                ERR_BITIO_end_of_file,
            );
        }
        (*t).bytes_in += 1;
        (*t).input_bit_count = 7 as ::core::ffi::c_ushort;
    }
    (*t).input_bit_buffer = (((*t).input_bit_buffer as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    return ((*t).input_bit_buffer as ::core::ffi::c_int & 0x100 as ::core::ffi::c_int)
        as ::core::ffi::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_BITIO_output_bit(
    mut t: *mut BITIO,
    mut bit: ::core::ffi::c_ulong,
) {
    (*t).output_bit_buffer = (((*t).output_bit_buffer as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    if bit != 0 {
        (*t).output_bit_buffer = ((*t).output_bit_buffer as ::core::ffi::c_int
            | 1 as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    }
    (*t).output_bit_count = (*t).output_bit_count.wrapping_add(1);
    if (*t).output_bit_count as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
        if (*t).mem_index >= (*t).mem_size {
            (*t).mem_size += (*t).mem_size / 2 as ::core::ffi::c_long;
            (*t).mem_bytes = MTX_mem_realloc(
                (*t).mem,
                (*t).mem_bytes as *mut ::core::ffi::c_void,
                (*t).mem_size as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_uchar;
        }
        let fresh0 = (*t).mem_index;
        (*t).mem_index = (*t).mem_index + 1;
        *(*t).mem_bytes.offset(fresh0 as isize) = (*t).output_bit_buffer
            as ::core::ffi::c_uchar;
        (*t).output_bit_count = 0 as ::core::ffi::c_ushort;
        (*t).bytes_out += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MTX_BITIO_flush_bits(mut t: *mut BITIO) {
    if (*t).ReadOrWrite as ::core::ffi::c_int == 'w' as i32 {} else {
        __assert_fail(
            b"t->ReadOrWrite == 'w'\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/bitio.c\0" as *const u8 as *const ::core::ffi::c_char,
            79 as ::core::ffi::c_uint,
            b"void MTX_BITIO_flush_bits(BITIO *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    if (*t).output_bit_count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if (*t).mem_index >= (*t).mem_size {
            (*t).mem_size = (*t).mem_index + 1 as ::core::ffi::c_long;
            (*t).mem_bytes = MTX_mem_realloc(
                (*t).mem,
                (*t).mem_bytes as *mut ::core::ffi::c_void,
                (*t).mem_size as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_uchar;
        }
        let fresh3 = (*t).mem_index;
        (*t).mem_index = (*t).mem_index + 1;
        *(*t).mem_bytes.offset(fresh3 as isize) = (((*t).output_bit_buffer
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int - (*t).output_bit_count as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        (*t).output_bit_count = 0 as ::core::ffi::c_ushort;
        (*t).bytes_out += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MTX_BITIO_GetMemoryPointer(
    mut t: *mut BITIO,
) -> *mut ::core::ffi::c_uchar {
    return (*t).mem_bytes;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_BITIO_GetBytesOut(
    mut t: *mut BITIO,
) -> ::core::ffi::c_long {
    if (*t).ReadOrWrite as ::core::ffi::c_int == 'w' as i32 {} else {
        __assert_fail(
            b"t->ReadOrWrite == 'w'\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/bitio.c\0" as *const u8 as *const ::core::ffi::c_char,
            102 as ::core::ffi::c_uint,
            b"long MTX_BITIO_GetBytesOut(BITIO *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    return (*t).bytes_out;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_BITIO_GetBytesIn(mut t: *mut BITIO) -> ::core::ffi::c_long {
    if (*t).ReadOrWrite as ::core::ffi::c_int == 'r' as i32 {} else {
        __assert_fail(
            b"t->ReadOrWrite == 'r'\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/lzcomp/bitio.c\0" as *const u8 as *const ::core::ffi::c_char,
            109 as ::core::ffi::c_uint,
            b"long MTX_BITIO_GetBytesIn(BITIO *)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
    return (*t).bytes_out;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_BITIO_Create(
    mut mem: *mut MTX_MemHandler,
    mut memPtr: *mut ::core::ffi::c_void,
    mut memSize: ::core::ffi::c_long,
    param: ::core::ffi::c_char,
) -> *mut BITIO {
    let mut t: *mut BITIO = MTX_mem_malloc(
        mem,
        ::core::mem::size_of::<BITIO>() as ::core::ffi::c_ulong,
    ) as *mut BITIO;
    (*t).mem = mem;
    (*t).mem_bytes = memPtr as *mut ::core::ffi::c_uchar;
    (*t).mem_index = 0 as ::core::ffi::c_long;
    (*t).mem_size = memSize;
    (*t).ReadOrWrite = param;
    (*t).input_bit_count = 0 as ::core::ffi::c_ushort;
    (*t).input_bit_buffer = 0 as ::core::ffi::c_ushort;
    (*t).bytes_in = 0 as ::core::ffi::c_long;
    (*t).output_bit_count = 0 as ::core::ffi::c_ushort;
    (*t).output_bit_buffer = 0 as ::core::ffi::c_ushort;
    (*t).bytes_out = 0 as ::core::ffi::c_long;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn MTX_BITIO_Destroy(mut t: *mut BITIO) {
    if (*t).ReadOrWrite as ::core::ffi::c_int == 'w' as i32 {
        MTX_BITIO_flush_bits(t);
        if (*t).mem_index == (*t).bytes_out {} else {
            __assert_fail(
                b"t->mem_index == t->bytes_out\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"src/lzcomp/bitio.c\0" as *const u8 as *const ::core::ffi::c_char,
                141 as ::core::ffi::c_uint,
                b"void MTX_BITIO_Destroy(BITIO *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        };
    }
    MTX_mem_free((*t).mem, t as *mut ::core::ffi::c_void);
}
