// Copyright (c) 2013-2026 Brennan T. Vincent <brennanv@email.arizona.edu>
// Copyright (c) 2026 Kiëd Llaentenn <kiedtl@tilde.team>
//
// This file is part of eot-rs, which is based on libeot, which is licensed under the MPL license,
// version 2.0. For full details, read the LICENSE and PATENTS file.

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Error {
    WARN_NOT_ENOUGH_GLYPHS = 1002,
    WARN_BAD_VERSION = 1001,
    WARN_NOT_ENOUGH_SPACE_RESERVED = 1000,
    BITIO_END_OF_FILE = 22,
    LZCOMP_ERROR = 21,
    CORRUPT_FILE_PADDING_NOT_ZERO = 20,
    MALFORMED_HEAD_TABLE = 19,
    MTX_ERROR = 18,
    UNKNOWN_BUFFER_WRITE_ERROR = 17,
    CORRUPT_HOPCODE_DATA = 16,
    NO_HDMX_TABLE = 15,
    NO_HMTX_TABLE = 14,
    NO_HEAD_TABLE = 13,
    NO_MAXP_TABLE = 12,
    LOGIC_ERROR = 11,
    THIRD_STREAM_INCOMPLETE = 6,
    SECOND_STREAM_INCOMPLETE = 5,
    CORRUPT_FILE = 4,
    BOGUS_STRING_SIZE = 3,
    HEADER_TOO_BIG = 2,
    INSUFFICIENT_BYTES = 1,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::WARN_NOT_ENOUGH_GLYPHS => "Not enough glyphs",
            Error::WARN_BAD_VERSION => "Incorrect version in EOT header",
            Error::WARN_NOT_ENOUGH_SPACE_RESERVED => "Not enough space reserved",
            Error::BITIO_END_OF_FILE => "Unexpected end of file",
            Error::LZCOMP_ERROR => "LZCOMP error",
            Error::CORRUPT_FILE_PADDING_NOT_ZERO => "Corrupt file: padding is not zeroed",
            Error::MALFORMED_HEAD_TABLE => "Malformed HEAD table",
            Error::MTX_ERROR => "Couldn't decode MTX data",
            Error::UNKNOWN_BUFFER_WRITE_ERROR => "Unknown buffer write error. This may be a bug in libeot.",
            Error::CORRUPT_HOPCODE_DATA => "Corrupt hopcode data",
            Error::NO_HDMX_TABLE => "No HDMX table",
            Error::NO_HMTX_TABLE => "No HMTX table",
            Error::NO_HEAD_TABLE => "No HEAD table",
            Error::NO_MAXP_TABLE => "No MAXP table",
            Error::LOGIC_ERROR => "Logic error. This may be a bug in libeot.",
            Error::THIRD_STREAM_INCOMPLETE => "Third data stream incomplete",
            Error::SECOND_STREAM_INCOMPLETE => "Second data stream incomplete",
            Error::CORRUPT_FILE => "Corrupt data",
            Error::BOGUS_STRING_SIZE => "Corrupt data: bogus string size",
            Error::HEADER_TOO_BIG => "Corrupt data: header too big",
            Error::INSUFFICIENT_BYTES => "Font file is truncated",
        })
    }
}

impl From<crate::stream::Error> for Error {
    fn from(_: crate::stream::Error) -> Error {
        Error::LOGIC_ERROR
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct EOTRootStringInfo {
    pub root_string_size: u16,
    pub root_string: *mut u16,
}

#[derive(Clone)]
#[repr(C)]
pub struct EUDCInfo {
    pub exists: bool,
    pub code_page: u32,
    pub flags: u32,
    pub font_data: Vec<u8>,
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
