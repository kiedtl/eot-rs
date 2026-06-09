extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
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
pub type size_t = usize;
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
#[no_mangle]
pub unsafe extern "C" fn MTX_mem_malloc(
    mut t: *mut MTX_MemHandler,
    mut size: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    return (*t).malloc.expect("non-null function pointer")(size as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn MTX_mem_realloc(
    mut t: *mut MTX_MemHandler,
    mut p: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    return (*t).realloc.expect("non-null function pointer")(p, size as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn MTX_mem_free(
    mut t: *mut MTX_MemHandler,
    mut deadObject: *mut ::core::ffi::c_void,
) {
    (*t).free.expect("non-null function pointer")(deadObject);
}
#[no_mangle]
pub unsafe extern "C" fn MTX_mem_Create(
    mut mptr: MTX_MALLOCPTR,
    mut rptr: MTX_REALLOCPTR,
    mut fptr: MTX_FREEPTR,
) -> *mut MTX_MemHandler {
    let mut t: *mut MTX_MemHandler = malloc(
        ::core::mem::size_of::<MTX_MemHandler>() as size_t,
    ) as *mut MTX_MemHandler;
    *t = MTX_MemHandler {
        mem_pointers: ::core::ptr::null_mut::<mem_struct>(),
        mem_maxPointers: 0,
        mem_numPointers: 0,
        mem_numNewCalls: 0,
        malloc: None,
        realloc: None,
        free: None,
        env: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
    };
    (*t).malloc = mptr;
    (*t).realloc = rptr;
    (*t).free = fptr;
    return t;
}
