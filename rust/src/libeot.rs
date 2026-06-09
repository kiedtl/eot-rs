use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn EOTfillMetadata(
        bytes: *const uint8_t,
        bytesLength: ::core::ffi::c_uint,
        out: *mut EOTMetadata,
    ) -> EOTError;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn writeFontBuffer(
        font: *const uint8_t,
        fontSize: ::core::ffi::c_uint,
        compressed: bool,
        encrypted: bool,
        finalOutBuffer: *mut *mut uint8_t,
        finalFontSize: *mut ::core::ffi::c_uint,
    ) -> EOTError;
    fn writeFontFile(
        font: *const uint8_t,
        fontSize: ::core::ffi::c_uint,
        compressed: bool,
        encrypted: bool,
        outFile: *mut FILE,
    ) -> EOTError;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub const EOT_WARN: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const TTEMBED_TTCOMPRESSED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const TTEMBED_XORENCRYPTDATA: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn EOTprintError(mut error: EOTError, mut out: *mut FILE) {
    match error as ::core::ffi::c_uint {
        0 => {}
        1 => {
            fputs(
                b"The font file appears truncated.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                out,
            );
        }
        3 | 4 => {
            fputs(
                b"The font file appears corrupt.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                out,
            );
        }
        7 => {
            fputs(
                b"Couldn't allocate sufficient memory.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                out,
            );
        }
        8 => {
            fputs(
                b"There was an unknown system error.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                out,
            );
        }
        10 => {
            fputs(
                b"MTX Compression has not yet been implemented in this version of libeot. The font could therefore not be converted.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                out,
            );
        }
        _ => {
            fputs(
                b"Unknown error: this is a bug in libeot; it does not *necessarily* indicate a corrupted font file.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                out,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EOT2ttf_file(
    mut font: *const uint8_t,
    mut fontSize: ::core::ffi::c_uint,
    mut metadataOut: *mut EOTMetadata,
    mut out: *mut FILE,
) -> EOTError {
    let mut result: EOTError = EOTfillMetadata(font, fontSize, metadataOut);
    if result as ::core::ffi::c_uint >= EOT_WARN as ::core::ffi::c_uint {
        EOTprintError(result, stderr);
    } else if result as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result
    }
    let mut writeResult: EOTError = writeFontFile(
        font.offset((*metadataOut).fontDataOffset as isize),
        (*metadataOut).fontDataSize as ::core::ffi::c_uint,
        (*metadataOut).flags & TTEMBED_TTCOMPRESSED as uint32_t != 0,
        (*metadataOut).flags & TTEMBED_XORENCRYPTDATA as uint32_t != 0,
        out,
    );
    if writeResult as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return writeResult;
    }
    return EOT_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn EOT2ttf_buffer(
    mut font: *const uint8_t,
    mut fontSize: ::core::ffi::c_uint,
    mut metadataOut: *mut EOTMetadata,
    mut fontOut: *mut *mut uint8_t,
    mut fontSizeOut: *mut ::core::ffi::c_uint,
) -> EOTError {
    let mut result: EOTError = EOTfillMetadata(font, fontSize, metadataOut);
    if result as ::core::ffi::c_uint >= EOT_WARN as ::core::ffi::c_uint {
        EOTprintError(result, stderr);
    } else if result as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result
    }
    let mut writeResult: EOTError = writeFontBuffer(
        font.offset((*metadataOut).fontDataOffset as isize),
        (*metadataOut).fontDataSize as ::core::ffi::c_uint,
        (*metadataOut).flags & TTEMBED_TTCOMPRESSED as uint32_t != 0,
        (*metadataOut).flags & TTEMBED_XORENCRYPTDATA as uint32_t != 0,
        fontOut,
        fontSizeOut,
    );
    if result as ::core::ffi::c_uint >= EOT_WARN as ::core::ffi::c_uint {
        EOTprintError(result, stderr);
    } else if writeResult as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return writeResult
    }
    return EOT_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn EOTfreeBuffer(mut buffer: *const uint8_t) {
    free(buffer as *mut ::core::ffi::c_void);
}
