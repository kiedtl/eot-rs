extern "C" {
    fn constructStream(buf: *mut uint8_t, size: ::core::ffi::c_uint) -> Stream;
    fn BEReadU16(s: *mut Stream, out: *mut uint16_t) -> StreamResult;
    fn BEReadU32(s: *mut Stream, out: *mut uint32_t) -> StreamResult;
    fn BEReadS16(s: *mut Stream, out: *mut int16_t) -> StreamResult;
    fn seekAbsolute(s: *mut Stream, pos: ::core::ffi::c_uint) -> StreamResult;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFNTTable {
    pub tag: [::core::ffi::c_char; 4],
    pub buf: *mut uint8_t,
    pub bufSize: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_uint,
    pub checksum: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TTFheadData {
    pub indexToLocFormat: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TTFmaxpData {
    pub numGlyphs: uint16_t,
    pub maxPoints: uint16_t,
    pub maxContours: uint16_t,
    pub maxComponentPoints: uint16_t,
    pub maxComponentContours: uint16_t,
    pub maxZones: uint16_t,
    pub maxTwilightPoints: uint16_t,
    pub maxStorage: uint16_t,
    pub maxFunctionDefs: uint16_t,
    pub maxInstructionDefs: uint16_t,
    pub maxStackElements: uint16_t,
    pub maxSizeOfInstructions: uint16_t,
    pub maxComponentElements: uint16_t,
    pub maxComponentDepth: uint16_t,
}
#[no_mangle]
pub unsafe extern "C" fn TTFParseHead(
    mut tbl: *mut SFNTTable,
    mut out: *mut TTFheadData,
) -> EOTError {
    if (*tbl).bufSize < 52 as ::core::ffi::c_uint {
        return EOT_CORRUPT_FILE;
    }
    *out = TTFheadData {
        indexToLocFormat: 0 as int16_t,
    };
    let mut s: Stream = constructStream((*tbl).buf, (*tbl).bufSize);
    seekAbsolute(&raw mut s, 50 as ::core::ffi::c_uint);
    BEReadS16(&raw mut s, &raw mut (*out).indexToLocFormat);
    return EOT_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn TTFParseMaxp(
    mut tbl: *mut SFNTTable,
    mut out: *mut TTFmaxpData,
) -> EOTError {
    let mut s: Stream = constructStream((*tbl).buf, (*tbl).bufSize);
    let mut sResult: StreamResult = EOT_STREAM_OK;
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<TTFmaxpData>() as size_t,
    );
    let mut version: uint32_t = 0;
    sResult = BEReadU32(&raw mut s, &raw mut version);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    sResult = BEReadU16(&raw mut s, &raw mut (*out).numGlyphs);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    if version == 0x10000 as uint32_t {
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxPoints);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxContours);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxComponentPoints);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxComponentContours);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxZones);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxTwilightPoints);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxStorage);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxFunctionDefs);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxInstructionDefs);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxStackElements);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxSizeOfInstructions);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxComponentElements);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEReadU16(&raw mut s, &raw mut (*out).maxComponentDepth);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
    }
    return EOT_SUCCESS;
}
