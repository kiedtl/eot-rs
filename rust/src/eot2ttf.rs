use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn err(__status: ::core::ffi::c_int, __format: *const ::core::ffi::c_char, ...) -> !;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn EOTfreeMetadata(toFree: *mut EOTMetadata);
    fn EOT2ttf_file(
        font: *const uint8_t,
        fontSize: ::core::ffi::c_uint,
        metadataOut: *mut EOTMetadata,
        out: *mut FILE,
    ) -> EOTError;
    fn EOTprintError(_: EOTError, out: *mut FILE);
    fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off_t,
    ) -> *mut ::core::ffi::c_void;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __syscall_ulong_t = ::core::ffi::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EUDCInfo {
    pub exists: bool,
    pub codePage: uint32_t,
    pub flags: uint32_t,
    pub fontDataSize: uint32_t,
    pub fontData: *mut uint8_t,
}
pub type EOTVersion = ::core::ffi::c_uint;
pub const VERSION_3: EOTVersion = 3;
pub const VERSION_2: EOTVersion = 2;
pub const VERSION_1: EOTVersion = 1;
pub type EOTCharset = ::core::ffi::c_uint;
pub const OEM_CHARSET: EOTCharset = 255;
pub const EASTEUROPE_CHARSET: EOTCharset = 238;
pub const THAI_CHARSET: EOTCharset = 222;
pub const RUSSIAN_CHARSET: EOTCharset = 204;
pub const BALTIC_CHARSET: EOTCharset = 186;
pub const ARABIC_CHARSET: EOTCharset = 178;
pub const HEBREW_CHARSET: EOTCharset = 177;
pub const VIETNAMESE_CHARSET: EOTCharset = 163;
pub const TURKISH_CHARSET: EOTCharset = 162;
pub const GREEK_CHARSET: EOTCharset = 161;
pub const CHINESEBIG5_CHARSET: EOTCharset = 136;
pub const GB2312_CHARSET: EOTCharset = 134;
pub const HANGUL_CHARSET: EOTCharset = 131;
pub const JOHAB_CHARSET: EOTCharset = 130;
pub const SHIFTJIS_CHARSET: EOTCharset = 128;
pub const MAC_CHARSET: EOTCharset = 77;
pub const SYMBOL_CHARSET: EOTCharset = 2;
pub const DEFAULT_CHARSET: EOTCharset = 1;
pub const ANSI_CHARSET: EOTCharset = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EOTRootStringInfo {
    pub rootStringSize: uint16_t,
    pub rootString: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EOTMetadata {
    pub totalSize: uint32_t,
    pub version: EOTVersion,
    pub flags: uint32_t,
    pub panose: [uint8_t; 10],
    pub charset: EOTCharset,
    pub italic: bool,
    pub weight: uint32_t,
    pub permissions: uint16_t,
    pub unicodeRange: [uint32_t; 4],
    pub codePageRange: [uint32_t; 2],
    pub checkSumAdjustment: uint32_t,
    pub familyNameSize: uint16_t,
    pub familyName: *mut uint16_t,
    pub styleNameSize: uint16_t,
    pub styleName: *mut uint16_t,
    pub versionNameSize: uint16_t,
    pub versionName: *mut uint16_t,
    pub fullNameSize: uint16_t,
    pub fullName: *mut uint16_t,
    pub numRootStrings: ::core::ffi::c_uint,
    pub rootStrings: *mut EOTRootStringInfo,
    pub fontDataSize: uint32_t,
    pub fontDataOffset: ::core::ffi::c_uint,
    pub eudcInfo: EUDCInfo,
    pub do_not_use_size: uint16_t,
    pub do_not_use: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atime: __time_t,
    pub st_atimensec: __syscall_ulong_t,
    pub st_mtime: __time_t,
    pub st_mtimensec: __syscall_ulong_t,
    pub st_ctime: __time_t,
    pub st_ctimensec: __syscall_ulong_t,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAP_SHARED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAP_FAILED: *mut ::core::ffi::c_void = -(1 as ::core::ffi::c_int)
    as *mut ::core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn usage(mut progName: *mut ::core::ffi::c_char) {
    fprintf(
        stderr,
        b"Usage: %s myfont.eot out.ttf\n\0" as *const u8 as *const ::core::ffi::c_char,
        progName,
    );
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if argc != 3 as ::core::ffi::c_int {
        usage(*argv.offset(0 as ::core::ffi::c_int as isize));
        return 1 as ::core::ffi::c_int;
    }
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atime: 0,
        st_atimensec: 0,
        st_mtime: 0,
        st_mtimensec: 0,
        st_ctime: 0,
        st_ctimensec: 0,
        __glibc_reserved: [0; 3],
    };
    if stat(*argv.offset(1 as ::core::ffi::c_int as isize), &raw mut st)
        != 0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"The file %s could not be opened.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            *argv.offset(1 as ::core::ffi::c_int as isize),
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut fildes: ::core::ffi::c_int = open(
        *argv.offset(1 as ::core::ffi::c_int as isize),
        O_RDONLY,
    );
    if fildes == -(1 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            b"The file %s could not be opened.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            *argv.offset(1 as ::core::ffi::c_int as isize),
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut outFileName: *const ::core::ffi::c_char = *argv
        .offset(2 as ::core::ffi::c_int as isize);
    let mut outFile: *mut FILE = fopen(
        outFileName,
        b"wb\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if outFile.is_null() {
        fprintf(
            stderr,
            b"The file %s could not be opened for writing.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            outFileName,
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut font: *const uint8_t = mmap(
        NULL,
        st.st_size as size_t,
        PROT_READ,
        MAP_SHARED,
        fildes,
        0 as __off_t,
    ) as *const uint8_t;
    if font == MAP_FAILED as *const uint8_t {
        err(1 as ::core::ffi::c_int, ::core::ptr::null::<::core::ffi::c_char>());
    }
    let mut out: EOTMetadata = EOTMetadata {
        totalSize: 0,
        version: 0 as EOTVersion,
        flags: 0,
        panose: [0; 10],
        charset: ANSI_CHARSET,
        italic: false,
        weight: 0,
        permissions: 0,
        unicodeRange: [0; 4],
        codePageRange: [0; 2],
        checkSumAdjustment: 0,
        familyNameSize: 0,
        familyName: ::core::ptr::null_mut::<uint16_t>(),
        styleNameSize: 0,
        styleName: ::core::ptr::null_mut::<uint16_t>(),
        versionNameSize: 0,
        versionName: ::core::ptr::null_mut::<uint16_t>(),
        fullNameSize: 0,
        fullName: ::core::ptr::null_mut::<uint16_t>(),
        numRootStrings: 0,
        rootStrings: ::core::ptr::null_mut::<EOTRootStringInfo>(),
        fontDataSize: 0,
        fontDataOffset: 0,
        eudcInfo: EUDCInfo {
            exists: false,
            codePage: 0,
            flags: 0,
            fontDataSize: 0,
            fontData: ::core::ptr::null_mut::<uint8_t>(),
        },
        do_not_use_size: 0,
        do_not_use: ::core::ptr::null_mut::<uint16_t>(),
    };
    let mut result: EOTError = EOT2ttf_file(
        font,
        st.st_size as ::core::ffi::c_uint,
        &raw mut out,
        outFile,
    );
    if result as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        EOTprintError(result, stderr);
        return 1 as ::core::ffi::c_int;
    }
    EOTfreeMetadata(&raw mut out);
    fclose(outFile);
    return 0;
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(
            main_0(
                (args_ptrs.len() - 1) as ::core::ffi::c_int,
                args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
            ) as i32,
        )
    }
}
