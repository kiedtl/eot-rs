extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}

pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Error {
    WARN_NOT_ENOUGH_GLYPHS = 1002,
    WARN_BAD_VERSION = 1001,
    WARN_NOT_ENOUGH_SPACE_RESERVED = 1000,
    MALFORMED_HEAD_TABLE = 19,
    MTX_ERROR = 18,
    UNKNOWN_BUFFER_WRITE_ERROR = 17,
    CORRUPT_HOPCODE_DATA = 16,
    NO_HDMX_TABLE = 15,
    NO_HMTX_TABLE = 14,
    NO_HEAD_TABLE = 13,
    NO_MAXP_TABLE = 12,
    LOGIC_ERROR = 11,
    COMPRESSION_NOT_YET_IMPLEMENTED = 10,
    FWRITE_ERROR = 9,
    OTHER_STDLIB_ERROR = 8,
    CANT_ALLOCATE_MEMORY = 7,
    THIRD_STREAM_INCOMPLETE = 6,
    SECOND_STREAM_INCOMPLETE = 5,
    CORRUPT_FILE = 4,
    BOGUS_STRING_SIZE = 3,
    HEADER_TOO_BIG = 2,
    INSUFFICIENT_BYTES = 1,
}

#[derive(Clone)]
#[repr(C)]
pub struct EOTRootStringInfo {
    pub rootStringSize: uint16_t,
    pub rootString: *mut uint16_t,
}

#[derive(Clone)]
#[repr(C)]
pub struct EUDCInfo {
    pub exists: bool,
    pub codePage: uint32_t,
    pub flags: uint32_t,
    pub fontData: Vec<u8>,
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

#[derive(Clone)]
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
    pub familyName: Vec<uint16_t>,
    pub styleName: Vec<uint16_t>,
    pub versionName: Vec<uint16_t>,
    pub fullName: Vec<uint16_t>,
    pub numRootStrings: ::core::ffi::c_uint,
    pub rootStrings: *mut EOTRootStringInfo,
    pub fontDataSize: uint32_t,
    pub fontDataOffset: ::core::ffi::c_uint,
    pub eudcInfo: EUDCInfo,
    pub do_not_use: Vec<uint16_t>,
}

impl Drop for EOTMetadata {
    fn drop(&mut self) {
        unsafe {
            if !(*self).rootStrings.is_null() {
                let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                while i < (*self).numRootStrings {
                    free(
                        (*(*self).rootStrings.offset(i as isize)).rootString
                            as *mut ::core::ffi::c_void,
                    );
                    i = i.wrapping_add(1);
                }
                free((*self).rootStrings as *mut ::core::ffi::c_void);
            }
        }
    }
}

impl EOTMetadata {
    pub const ZERO: EOTMetadata = EOTMetadata {
        totalSize: 0 as uint32_t,
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
        familyName: Vec::new(),
        styleName: Vec::new(),
        versionName: Vec::new(),
        fullName: Vec::new(),
        do_not_use: Vec::new(),
        numRootStrings: 0,
        rootStrings: ::core::ptr::null_mut::<EOTRootStringInfo>(),
        fontDataSize: 0,
        fontDataOffset: 0,
        eudcInfo: EUDCInfo {
            exists: false,
            codePage: 0,
            flags: 0,
            fontData: Vec::new(),
        },
    };
}
