use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    static tripletEncodings: [TripletEncoding; 0];
    fn constructStream(buf: *mut uint8_t, size: ::core::ffi::c_uint) -> Stream;
    fn BEReadU8(s: *mut Stream, out: *mut uint8_t) -> StreamResult;
    fn BEReadU16(s: *mut Stream, out: *mut uint16_t) -> StreamResult;
    fn BEReadU32(s: *mut Stream, out: *mut uint32_t) -> StreamResult;
    fn BEReadS16(s: *mut Stream, out: *mut int16_t) -> StreamResult;
    fn BEReadChar(s: *mut Stream, out: *mut ::core::ffi::c_char) -> StreamResult;
    fn BEPeekU8(s: *mut Stream, out: *mut uint8_t) -> StreamResult;
    fn seekRelative(s: *mut Stream, offset: ::core::ffi::c_int) -> StreamResult;
    fn seekAbsolute(s: *mut Stream, pos: ::core::ffi::c_uint) -> StreamResult;
    fn seekRelativeThroughReserve(
        s: *mut Stream,
        offset: ::core::ffi::c_int,
    ) -> StreamResult;
    fn seekAbsoluteThroughReserve(
        s: *mut Stream,
        pos: ::core::ffi::c_uint,
    ) -> StreamResult;
    fn reserve(s: *mut Stream, toReserve: ::core::ffi::c_uint) -> StreamResult;
    fn BEWriteU8(s: *mut Stream, in_0: uint8_t) -> StreamResult;
    fn BEWriteU16(s: *mut Stream, in_0: uint16_t) -> StreamResult;
    fn BEWriteU32(s: *mut Stream, in_0: uint32_t) -> StreamResult;
    fn BEWriteS16(s: *mut Stream, in_0: int16_t) -> StreamResult;
    fn streamCopy(
        sIn: *mut Stream,
        sOut: *mut Stream,
        length: ::core::ffi::c_uint,
    ) -> StreamResult;
    fn readNBits(
        s: *mut Stream,
        out: *mut uint32_t,
        n: ::core::ffi::c_uint,
    ) -> StreamResult;
    fn constructContainer(out: *mut *mut SFNTContainer) -> EOTError;
    fn reserveTables(ctr: *mut SFNTContainer, num: ::core::ffi::c_uint) -> EOTError;
    fn addTable(
        ctr: *mut SFNTContainer,
        tag: *const ::core::ffi::c_char,
        newTableOut: *mut *mut SFNTTable,
    ) -> EOTError;
    fn loadTableFromStream(tbl: *mut SFNTTable, s: *mut Stream) -> EOTError;
    fn TTFParseHead(tbl: *mut SFNTTable, out: *mut TTFheadData) -> EOTError;
    fn TTFParseMaxp(tbl: *mut SFNTTable, out: *mut TTFmaxpData) -> EOTError;
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int16_t = __int16_t;
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
pub struct TripletEncoding {
    pub byteCount: ::core::ffi::c_uint,
    pub xBits: ::core::ffi::c_uint,
    pub yBits: ::core::ffi::c_uint,
    pub deltaX: ::core::ffi::c_uint,
    pub deltaY: ::core::ffi::c_uint,
    pub xSign: ::core::ffi::c_int,
    pub ySign: ::core::ffi::c_int,
}
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
pub struct SFNTContainer {
    pub numTables: ::core::ffi::c_uint,
    pub _numTablesReserved: ::core::ffi::c_uint,
    pub tables: *mut SFNTTable,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFNTOffsetTable {
    pub scalarType: uint32_t,
    pub numTables: uint16_t,
    pub searchRange: uint16_t,
    pub entrySelector: uint16_t,
    pub rangeShift: uint16_t,
}
pub type _dpi_TypeRead = ::core::ffi::c_uint;
pub const SHORT: _dpi_TypeRead = 1;
pub const BYTE: _dpi_TypeRead = 0;
pub const INT16_MIN: ::core::ffi::c_int = -(32767 as ::core::ffi::c_int)
    - 1 as ::core::ffi::c_int;
pub const INT16_MAX: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EOT_WARN: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub unsafe extern "C" fn logWarning(mut out: *const ::core::ffi::c_char) {
    fputs(out, stderr);
}
#[no_mangle]
pub unsafe extern "C" fn umax(
    mut a: ::core::ffi::c_uint,
    mut b: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn imax(
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn imin(
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn i16max(mut a: int16_t, mut b: int16_t) -> int16_t {
    return imax(a as ::core::ffi::c_int, b as ::core::ffi::c_int) as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn i16min(mut a: int16_t, mut b: int16_t) -> int16_t {
    return imin(a as ::core::ffi::c_int, b as ::core::ffi::c_int) as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn parseOffsetTable(
    mut s: *mut Stream,
    mut tbl: *mut SFNTOffsetTable,
) -> StreamResult {
    let mut res: StreamResult = EOT_STREAM_OK;
    res = BEReadU32(s, &raw mut (*tbl).scalarType);
    if res as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return res;
    }
    res = BEReadU16(s, &raw mut (*tbl).numTables);
    if res as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return res;
    }
    res = BEReadU16(s, &raw mut (*tbl).searchRange);
    if res as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return res;
    }
    res = BEReadU16(s, &raw mut (*tbl).entrySelector);
    if res as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return res;
    }
    res = BEReadU16(s, &raw mut (*tbl).rangeShift);
    if res as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return res;
    }
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn _ucvt_rdVal(
    mut sIn: *mut Stream,
    mut lastValue: *mut int16_t,
) -> StreamResult {
    let mut code: uint8_t = 0;
    let mut b2: uint8_t = 0;
    let mut sResult: StreamResult = BEReadU8(sIn, &raw mut code);
    let mut val: int16_t = 0;
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return sResult;
    }
    if code as ::core::ffi::c_int >= 248 as ::core::ffi::c_int {
        sResult = BEReadU8(sIn, &raw mut b2);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return sResult;
        }
        val = (238 as ::core::ffi::c_int
            * (code as ::core::ffi::c_int - 247 as ::core::ffi::c_int)
            + b2 as ::core::ffi::c_int) as int16_t;
    } else if code as ::core::ffi::c_int >= 239 as ::core::ffi::c_int {
        sResult = BEReadU8(sIn, &raw mut b2);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return sResult;
        }
        val = (-(1 as ::core::ffi::c_int)
            * (238 as ::core::ffi::c_int
                * (code as ::core::ffi::c_int - 239 as ::core::ffi::c_int)
                + b2 as ::core::ffi::c_int)) as int16_t;
    } else if code as ::core::ffi::c_int == 238 as ::core::ffi::c_int {
        sResult = BEReadS16(sIn, &raw mut val);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return sResult;
        }
    } else {
        val = code as int16_t;
    }
    *lastValue = (*lastValue as ::core::ffi::c_int + val as ::core::ffi::c_int)
        as int16_t;
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn unpackCVT(
    mut out: *mut SFNTTable,
    mut sIn: *mut Stream,
) -> EOTError {
    let mut sResult: StreamResult = seekAbsolute(sIn, (*out).offset);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    let mut tableLength: uint16_t = 0;
    sResult = BEReadU16(sIn, &raw mut tableLength);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    let mut sOut: Stream = constructStream(
        ::core::ptr::null_mut::<uint8_t>(),
        0 as ::core::ffi::c_uint,
    );
    sResult = reserve(
        &raw mut sOut,
        (tableLength as usize).wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
            as ::core::ffi::c_uint,
    );
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    let mut lastValue: int16_t = 0 as int16_t;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < tableLength as ::core::ffi::c_uint {
        sResult = _ucvt_rdVal(sIn, &raw mut lastValue);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEWriteS16(&raw mut sOut, lastValue);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_LOGIC_ERROR;
        }
        i = i.wrapping_add(1);
    }
    (*out).buf = sOut.buf;
    (*out).bufSize = sOut.size;
    return EOT_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn read255UShort(
    mut sIn: *mut Stream,
    mut out: *mut uint16_t,
) -> StreamResult {
    let mut code: uint8_t = 0;
    let mut val1: uint8_t = 0;
    let mut sResult: StreamResult = EOT_STREAM_OK;
    sResult = BEReadU8(sIn, &raw mut code);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return sResult;
    }
    match code as ::core::ffi::c_int {
        253 => {
            sResult = BEReadU16(sIn, out);
            return sResult;
        }
        255 => {
            sResult = BEReadU8(sIn, &raw mut val1);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return sResult;
            }
            *out = (253 as ::core::ffi::c_int + val1 as ::core::ffi::c_int) as uint16_t;
            return EOT_STREAM_OK;
        }
        254 => {
            sResult = BEReadU8(sIn, &raw mut val1);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return sResult;
            }
            *out = (506 as ::core::ffi::c_int + val1 as ::core::ffi::c_int) as uint16_t;
            return EOT_STREAM_OK;
        }
        _ => {
            *out = code as uint16_t;
            return EOT_STREAM_OK;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn read255Short(
    mut sIn: *mut Stream,
    mut out: *mut int16_t,
) -> StreamResult {
    let mut code: uint8_t = 0;
    let mut sResult: StreamResult = BEReadU8(sIn, &raw mut code);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return sResult;
    }
    if code as ::core::ffi::c_int == 253 as ::core::ffi::c_int {
        sResult = BEReadS16(sIn, out);
        return sResult;
    }
    let mut sign: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if code as ::core::ffi::c_int == 250 as ::core::ffi::c_int {
        sign = -(1 as ::core::ffi::c_int);
        sResult = BEReadU8(sIn, &raw mut code);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return sResult;
        }
    }
    let mut out8: uint8_t = 0;
    match code as ::core::ffi::c_int {
        255 => {
            sResult = BEReadU8(sIn, &raw mut out8);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return sResult;
            }
            *out = (250 as ::core::ffi::c_int + out8 as ::core::ffi::c_int) as int16_t;
        }
        254 => {
            sResult = BEReadU8(sIn, &raw mut out8);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return sResult;
            }
            *out = (250 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                + out8 as ::core::ffi::c_int) as int16_t;
        }
        _ => {
            *out = code as int16_t;
        }
    }
    *out = (*out as ::core::ffi::c_int * sign) as int16_t;
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn _dpi_dump(
    mut out: *mut Stream,
    mut lastRead: *mut _dpi_TypeRead,
    mut typeLastReadCount: *mut ::core::ffi::c_uint,
    mut data: *mut int16_t,
    mut dataIndex: *mut ::core::ffi::c_uint,
) -> StreamResult {
    let mut sResult: StreamResult = EOT_STREAM_OK;
    if *typeLastReadCount > 0 as ::core::ffi::c_uint {
        if *typeLastReadCount < 8 as ::core::ffi::c_uint {
            let mut op: uint8_t = ((if *lastRead as ::core::ffi::c_uint
                == BYTE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                PUSHB
            } else {
                PUSHW
            })
                | (*typeLastReadCount).wrapping_sub(1 as ::core::ffi::c_uint) as uint8_t
                    as ::core::ffi::c_int) as uint8_t;
            sResult = BEWriteU8(out, op);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return sResult;
            }
        } else {
            let mut op_0: uint8_t = (if *lastRead as ::core::ffi::c_uint
                == BYTE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                NPUSHB
            } else {
                NPUSHW
            }) as uint8_t;
            sResult = BEWriteU8(out, op_0);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return sResult;
            }
            sResult = BEWriteU8(out, *typeLastReadCount as uint8_t);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return sResult;
            }
        }
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while i < *typeLastReadCount {
            if *lastRead as ::core::ffi::c_uint
                == BYTE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                sResult = BEWriteU8(
                    out,
                    *data
                        .offset(
                            (*dataIndex).wrapping_sub(*typeLastReadCount).wrapping_add(i)
                                as isize,
                        ) as uint8_t,
                );
                if sResult as ::core::ffi::c_uint
                    != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return sResult;
                }
            } else {
                sResult = BEWriteS16(
                    out,
                    *data
                        .offset(
                            (*dataIndex).wrapping_sub(*typeLastReadCount).wrapping_add(i)
                                as isize,
                        ),
                );
                if sResult as ::core::ffi::c_uint
                    != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return sResult;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    return EOT_STREAM_OK;
}
pub const NPUSHB: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const NPUSHW: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const PUSHB: ::core::ffi::c_int = 0xb0 as ::core::ffi::c_int;
pub const PUSHW: ::core::ffi::c_int = 0xb8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn _dpi_put(
    mut value: int16_t,
    mut out: *mut Stream,
    mut lastRead: *mut _dpi_TypeRead,
    mut typeLastReadCount: *mut ::core::ffi::c_uint,
    mut data: *mut int16_t,
    mut dataIndex: *mut ::core::ffi::c_uint,
) -> StreamResult {
    let mut sResult: StreamResult = EOT_STREAM_OK;
    let mut newType: _dpi_TypeRead = (if value as ::core::ffi::c_int
        >= 0 as ::core::ffi::c_int
        && (value as ::core::ffi::c_int) < 256 as ::core::ffi::c_int
    {
        BYTE as ::core::ffi::c_int
    } else {
        SHORT as ::core::ffi::c_int
    }) as _dpi_TypeRead;
    if newType as ::core::ffi::c_uint != *lastRead as ::core::ffi::c_uint
        || *typeLastReadCount == 255 as ::core::ffi::c_uint
    {
        sResult = _dpi_dump(out, lastRead, typeLastReadCount, data, dataIndex);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return sResult;
        }
        *lastRead = newType;
        *typeLastReadCount = 0 as ::core::ffi::c_uint;
    }
    let fresh0 = *dataIndex;
    *dataIndex = (*dataIndex).wrapping_add(1);
    *data.offset(fresh0 as isize) = value;
    *typeLastReadCount = (*typeLastReadCount).wrapping_add(1);
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn decodePushInstructions(
    mut sIn: *mut Stream,
    mut sOut: *mut Stream,
    mut pushCount: ::core::ffi::c_uint,
) -> EOTError {
    let mut current_block: u64;
    let mut sResult: StreamResult = EOT_STREAM_OK;
    let mut remaining: ::core::ffi::c_uint = pushCount;
    let mut typeLastRead: _dpi_TypeRead = BYTE;
    let mut typeLastReadCount: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut dataIndex: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut data: *mut int16_t = malloc(
        (::core::mem::size_of::<int16_t>() as size_t).wrapping_mul(pushCount as size_t),
    ) as *mut int16_t;
    let mut returnedStatus: EOTError = EOT_SUCCESS;
    if data.is_null() {
        return EOT_CANT_ALLOCATE_MEMORY;
    }
    loop {
        if !(remaining != 0) {
            current_block = 1134115459065347084;
            break;
        }
        let mut code: uint8_t = 0;
        sResult = BEPeekU8(sIn, &raw mut code);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
            current_block = 410770814531635848;
            break;
        } else {
            let mut val: int16_t = 0;
            let mut prev: int16_t = 0;
            match code as ::core::ffi::c_int {
                251 => {
                    if remaining < 3 as ::core::ffi::c_uint {
                        returnedStatus = EOT_CORRUPT_HOPCODE_DATA;
                        current_block = 410770814531635848;
                        break;
                    } else {
                        remaining = remaining.wrapping_sub(3 as ::core::ffi::c_uint);
                        if dataIndex < 2 as ::core::ffi::c_uint {
                            returnedStatus = EOT_CORRUPT_HOPCODE_DATA;
                            current_block = 410770814531635848;
                            break;
                        } else {
                            prev = *data
                                .offset(
                                    dataIndex.wrapping_sub(2 as ::core::ffi::c_uint) as isize,
                                );
                            BEReadU8(sIn, &raw mut code);
                            sResult = _dpi_put(
                                prev,
                                sOut,
                                &raw mut typeLastRead,
                                &raw mut typeLastReadCount,
                                data,
                                &raw mut dataIndex,
                            );
                            if sResult as ::core::ffi::c_uint
                                != EOT_STREAM_OK as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            {
                                returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                                current_block = 410770814531635848;
                                break;
                            } else {
                                sResult = read255Short(sIn, &raw mut val);
                                if sResult as ::core::ffi::c_uint
                                    != EOT_STREAM_OK as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                {
                                    returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                                    current_block = 410770814531635848;
                                    break;
                                } else {
                                    sResult = _dpi_put(
                                        val,
                                        sOut,
                                        &raw mut typeLastRead,
                                        &raw mut typeLastReadCount,
                                        data,
                                        &raw mut dataIndex,
                                    );
                                    if sResult as ::core::ffi::c_uint
                                        != EOT_STREAM_OK as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                                        current_block = 410770814531635848;
                                        break;
                                    } else {
                                        sResult = _dpi_put(
                                            prev,
                                            sOut,
                                            &raw mut typeLastRead,
                                            &raw mut typeLastReadCount,
                                            data,
                                            &raw mut dataIndex,
                                        );
                                        if !(sResult as ::core::ffi::c_uint
                                            != EOT_STREAM_OK as ::core::ffi::c_int
                                                as ::core::ffi::c_uint)
                                        {
                                            continue;
                                        }
                                        returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                                        current_block = 410770814531635848;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                252 => {
                    if remaining < 5 as ::core::ffi::c_uint {
                        returnedStatus = EOT_CORRUPT_HOPCODE_DATA;
                        current_block = 410770814531635848;
                        break;
                    } else {
                        remaining = remaining.wrapping_sub(5 as ::core::ffi::c_uint);
                        if dataIndex < 2 as ::core::ffi::c_uint {
                            returnedStatus = EOT_CORRUPT_HOPCODE_DATA;
                            current_block = 410770814531635848;
                            break;
                        } else {
                            prev = *data
                                .offset(
                                    dataIndex.wrapping_sub(2 as ::core::ffi::c_uint) as isize,
                                );
                            BEReadU8(sIn, &raw mut code);
                            sResult = _dpi_put(
                                prev,
                                sOut,
                                &raw mut typeLastRead,
                                &raw mut typeLastReadCount,
                                data,
                                &raw mut dataIndex,
                            );
                            if sResult as ::core::ffi::c_uint
                                != EOT_STREAM_OK as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            {
                                returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                                current_block = 410770814531635848;
                                break;
                            } else {
                                sResult = read255Short(sIn, &raw mut val);
                                if sResult as ::core::ffi::c_uint
                                    != EOT_STREAM_OK as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                {
                                    returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                                    current_block = 410770814531635848;
                                    break;
                                } else {
                                    sResult = _dpi_put(
                                        val,
                                        sOut,
                                        &raw mut typeLastRead,
                                        &raw mut typeLastReadCount,
                                        data,
                                        &raw mut dataIndex,
                                    );
                                    if sResult as ::core::ffi::c_uint
                                        != EOT_STREAM_OK as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                                        current_block = 410770814531635848;
                                        break;
                                    } else {
                                        sResult = _dpi_put(
                                            prev,
                                            sOut,
                                            &raw mut typeLastRead,
                                            &raw mut typeLastReadCount,
                                            data,
                                            &raw mut dataIndex,
                                        );
                                        if sResult as ::core::ffi::c_uint
                                            != EOT_STREAM_OK as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                        {
                                            returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                                            current_block = 410770814531635848;
                                            break;
                                        } else {
                                            sResult = read255Short(sIn, &raw mut val);
                                            if sResult as ::core::ffi::c_uint
                                                != EOT_STREAM_OK as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                                                current_block = 410770814531635848;
                                                break;
                                            } else {
                                                sResult = _dpi_put(
                                                    val,
                                                    sOut,
                                                    &raw mut typeLastRead,
                                                    &raw mut typeLastReadCount,
                                                    data,
                                                    &raw mut dataIndex,
                                                );
                                                if sResult as ::core::ffi::c_uint
                                                    != EOT_STREAM_OK as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                {
                                                    returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                                                    current_block = 410770814531635848;
                                                    break;
                                                } else {
                                                    sResult = _dpi_put(
                                                        prev,
                                                        sOut,
                                                        &raw mut typeLastRead,
                                                        &raw mut typeLastReadCount,
                                                        data,
                                                        &raw mut dataIndex,
                                                    );
                                                    if !(sResult as ::core::ffi::c_uint
                                                        != EOT_STREAM_OK as ::core::ffi::c_int
                                                            as ::core::ffi::c_uint)
                                                    {
                                                        continue;
                                                    }
                                                    returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                                                    current_block = 410770814531635848;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    sResult = read255Short(sIn, &raw mut val);
                    if sResult as ::core::ffi::c_uint
                        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
                        current_block = 410770814531635848;
                        break;
                    } else {
                        sResult = _dpi_put(
                            val,
                            sOut,
                            &raw mut typeLastRead,
                            &raw mut typeLastReadCount,
                            data,
                            &raw mut dataIndex,
                        );
                        remaining = remaining.wrapping_sub(1);
                    }
                }
            }
        }
    }
    match current_block {
        1134115459065347084 => {
            sResult = _dpi_dump(
                sOut,
                &raw mut typeLastRead,
                &raw mut typeLastReadCount,
                data,
                &raw mut dataIndex,
            );
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                returnedStatus = EOT_SECOND_STREAM_INCOMPLETE;
            } else {
                returnedStatus = EOT_SUCCESS;
            }
        }
        _ => {}
    }
    free(data as *mut ::core::ffi::c_void);
    return returnedStatus;
}
#[no_mangle]
pub unsafe extern "C" fn _dsg_makeFlags(
    mut x: int16_t,
    mut y: int16_t,
    mut onCurve: bool,
    mut firstTime: bool,
) -> uint8_t {
    let FLG_ON_CURVE: uint8_t = 0x1 as uint8_t;
    let FLG_X_SHORT: uint8_t = 0x2 as uint8_t;
    let FLG_Y_SHORT: uint8_t = 0x4 as uint8_t;
    let FLG_X_SAME: uint8_t = 0x10 as uint8_t;
    let FLG_Y_SAME: uint8_t = 0x20 as uint8_t;
    let mut ret: uint8_t = 0 as uint8_t;
    if onCurve {
        ret = (ret as ::core::ffi::c_int | FLG_ON_CURVE as ::core::ffi::c_int)
            as uint8_t;
    }
    if !firstTime && x as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        ret = (ret as ::core::ffi::c_int | FLG_X_SAME as ::core::ffi::c_int) as uint8_t;
    } else if -(256 as ::core::ffi::c_int) < x as ::core::ffi::c_int
        && (x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        ret = (ret as ::core::ffi::c_int | FLG_X_SHORT as ::core::ffi::c_int) as uint8_t;
    } else if 0 as ::core::ffi::c_int <= x as ::core::ffi::c_int
        && (x as ::core::ffi::c_int) < 256 as ::core::ffi::c_int
    {
        ret = (ret as ::core::ffi::c_int | FLG_X_SHORT as ::core::ffi::c_int) as uint8_t;
        ret = (ret as ::core::ffi::c_int | FLG_X_SAME as ::core::ffi::c_int) as uint8_t;
    }
    if !firstTime && y as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        ret = (ret as ::core::ffi::c_int | FLG_Y_SAME as ::core::ffi::c_int) as uint8_t;
    } else if -(256 as ::core::ffi::c_int) < y as ::core::ffi::c_int
        && (y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        ret = (ret as ::core::ffi::c_int | FLG_Y_SHORT as ::core::ffi::c_int) as uint8_t;
    } else if 0 as ::core::ffi::c_int <= y as ::core::ffi::c_int
        && (y as ::core::ffi::c_int) < 256 as ::core::ffi::c_int
    {
        ret = (ret as ::core::ffi::c_int | FLG_Y_SHORT as ::core::ffi::c_int) as uint8_t;
        ret = (ret as ::core::ffi::c_int | FLG_Y_SAME as ::core::ffi::c_int) as uint8_t;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn decodeSimpleGlyph(
    mut numContours: int16_t,
    mut streams: *mut *mut Stream,
    mut out: *mut Stream,
    mut calculateBoundingBox: bool,
    mut minX: int16_t,
    mut minY: int16_t,
    mut maxX: int16_t,
    mut maxY: int16_t,
) -> EOTError {
    let mut currX: ::core::ffi::c_uint = 0;
    let mut currY: ::core::ffi::c_uint = 0;
    let mut codeSizeLocation: ::core::ffi::c_uint = 0;
    let mut pushCount: uint16_t = 0;
    let mut result: EOTError = EOT_SUCCESS;
    let mut codeSize: uint16_t = 0;
    let mut unpackedCodeSize: ::core::ffi::c_uint = 0;
    let mut currPos: ::core::ffi::c_uint = 0;
    let mut current_block: u64;
    if numContours as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return EOT_SUCCESS;
    }
    let mut in_0: *mut Stream = *streams.offset(0 as ::core::ffi::c_int as isize);
    let mut sResult: StreamResult = EOT_STREAM_OK;
    let mut returnedStatus: EOTError = EOT_SUCCESS;
    let mut boundingBoxLocation: ::core::ffi::c_uint = 0;
    sResult = BEWriteS16(out, numContours);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    if calculateBoundingBox {
        boundingBoxLocation = (*out).pos;
        sResult = seekRelativeThroughReserve(
            out,
            (4 as usize).wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                as ::core::ffi::c_int,
        );
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        minX = INT16_MAX as int16_t;
        minY = INT16_MAX as int16_t;
        maxX = INT16_MIN as int16_t;
        maxY = INT16_MIN as int16_t;
    } else {
        sResult = BEWriteS16(out, minX);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEWriteS16(out, minY);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEWriteS16(out, maxX);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEWriteS16(out, maxY);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
    }
    let mut totalPoints: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < numContours as ::core::ffi::c_uint {
        if i == 0 as ::core::ffi::c_uint {
            totalPoints = 1 as ::core::ffi::c_uint;
        }
        let mut pointsInContour: uint16_t = 0;
        sResult = read255UShort(in_0, &raw mut pointsInContour);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        totalPoints = totalPoints.wrapping_add(pointsInContour as ::core::ffi::c_uint);
        sResult = BEWriteS16(
            out,
            totalPoints.wrapping_sub(1 as ::core::ffi::c_uint) as int16_t,
        );
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        i = i.wrapping_add(1);
    }
    
    
    
    
    
    let mut flags:  *mut uint8_t =
     malloc(
        (totalPoints as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;let mut xCoords:  *mut int16_t =
     malloc(
        (totalPoints as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;let mut yCoords:  *mut int16_t =
     malloc(
        (totalPoints as size_t).wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
    ) as *mut int16_t;
    if flags.is_null() || xCoords.is_null() || yCoords.is_null() {
        returnedStatus = EOT_CANT_ALLOCATE_MEMORY;
    } else {
        let mut i_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        loop {
            if !(i_0 < totalPoints) {
                current_block = 6450597802325118133;
                break;
            }
            sResult = BEReadU8(in_0, flags.offset(i_0 as isize) as *mut uint8_t);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                returnedStatus = EOT_CANT_ALLOCATE_MEMORY;
                current_block = 17468087227630049781;
                break;
            } else {
                i_0 = i_0.wrapping_add(1);
            }
        }
        match current_block {
            17468087227630049781 => {}
            _ => {
                currX = 0 as ::core::ffi::c_uint;
                currY = 0 as ::core::ffi::c_uint;
                let mut i_1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                loop {
                    if !(i_1 < totalPoints) {
                        current_block = 15594603006322722090;
                        break;
                    }
                    let enc: TripletEncoding = *(&raw const tripletEncodings
                        as *const TripletEncoding)
                        .offset(
                            (*flags.offset(i_1 as isize) as ::core::ffi::c_int
                                & 0x7f as ::core::ffi::c_int) as isize,
                        );
                    let mut moreBytes: ::core::ffi::c_uint = enc
                        .byteCount
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                    if (*in_0).pos.wrapping_add(moreBytes) > (*in_0).size {
                        returnedStatus = EOT_CORRUPT_FILE;
                        current_block = 17468087227630049781;
                        break;
                    } else {
                        let mut coords: Stream = constructStream(
                            (*in_0).buf.offset((*in_0).pos as isize),
                            moreBytes,
                        );
                        let mut dx: uint32_t = 0;
                        let mut dy: uint32_t = 0;
                        sResult = readNBits(&raw mut coords, &raw mut dx, enc.xBits);
                        if sResult as ::core::ffi::c_uint
                            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            returnedStatus = EOT_LOGIC_ERROR;
                            current_block = 17468087227630049781;
                            break;
                        } else {
                            sResult = readNBits(&raw mut coords, &raw mut dy, enc.yBits);
                            if sResult as ::core::ffi::c_uint
                                != EOT_STREAM_OK as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            {
                                returnedStatus = EOT_LOGIC_ERROR;
                                current_block = 17468087227630049781;
                                break;
                            } else if coords.pos != coords.size
                                || coords.bitPos != 0 as ::core::ffi::c_uint
                            {
                                returnedStatus = EOT_LOGIC_ERROR;
                                current_block = 17468087227630049781;
                                break;
                            } else {
                                sResult = seekRelative(
                                    in_0,
                                    coords.size as ::core::ffi::c_int,
                                );
                                if sResult as ::core::ffi::c_uint
                                    != EOT_STREAM_OK as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                {
                                    returnedStatus = EOT_LOGIC_ERROR;
                                    current_block = 17468087227630049781;
                                    break;
                                } else {
                                    *xCoords.offset(i_1 as isize) = (enc.xSign as uint32_t)
                                        .wrapping_mul(dx.wrapping_add(enc.deltaX as uint32_t))
                                        as int16_t;
                                    currX = currX
                                        .wrapping_add(
                                            *xCoords.offset(i_1 as isize) as ::core::ffi::c_uint,
                                        );
                                    *yCoords.offset(i_1 as isize) = (enc.ySign as uint32_t)
                                        .wrapping_mul(dy.wrapping_add(enc.deltaY as uint32_t))
                                        as int16_t;
                                    currY = currY
                                        .wrapping_add(
                                            *yCoords.offset(i_1 as isize) as ::core::ffi::c_uint,
                                        );
                                    minX = i16min(minX, currX as int16_t);
                                    maxX = i16max(maxX, currX as int16_t);
                                    minY = i16min(minY, currY as int16_t);
                                    maxY = i16max(maxY, currY as int16_t);
                                    i_1 = i_1.wrapping_add(1);
                                }
                            }
                        }
                    }
                }
                match current_block {
                    17468087227630049781 => {}
                    _ => {
                        codeSizeLocation = (*out).pos;
                        sResult = seekRelativeThroughReserve(
                            out,
                            ::core::mem::size_of::<uint16_t>() as ::core::ffi::c_int,
                        );
                        if sResult as ::core::ffi::c_uint
                            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            returnedStatus = EOT_CORRUPT_FILE;
                        } else {
                            pushCount = 0;
                            sResult = read255UShort(in_0, &raw mut pushCount);
                            if sResult as ::core::ffi::c_uint
                                != EOT_STREAM_OK as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            {
                                returnedStatus = EOT_CORRUPT_FILE;
                            } else {
                                result = decodePushInstructions(
                                    *streams.offset(1 as ::core::ffi::c_int as isize),
                                    out,
                                    pushCount as ::core::ffi::c_uint,
                                );
                                if result as ::core::ffi::c_uint
                                    != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
                                    && (result as ::core::ffi::c_uint)
                                        < EOT_WARN as ::core::ffi::c_uint
                                {
                                    returnedStatus = result;
                                } else {
                                    codeSize = 0;
                                    sResult = read255UShort(in_0, &raw mut codeSize);
                                    if sResult as ::core::ffi::c_uint
                                        != EOT_STREAM_OK as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        returnedStatus = EOT_CORRUPT_FILE;
                                    } else {
                                        sResult = streamCopy(
                                            *streams.offset(2 as ::core::ffi::c_int as isize),
                                            out,
                                            codeSize as ::core::ffi::c_uint,
                                        );
                                        if sResult as ::core::ffi::c_uint
                                            != EOT_STREAM_OK as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                        {
                                            returnedStatus = EOT_CORRUPT_FILE;
                                        } else {
                                            unpackedCodeSize = ((*out).pos as usize)
                                                .wrapping_sub(
                                                    (codeSizeLocation as usize)
                                                        .wrapping_add(::core::mem::size_of::<uint16_t>() as usize),
                                                ) as ::core::ffi::c_uint;
                                            let mut i_2: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                                            loop {
                                                if !(i_2 < totalPoints) {
                                                    current_block = 14329534724295951598;
                                                    break;
                                                }
                                                let mut outFlags: uint8_t = _dsg_makeFlags(
                                                    *xCoords.offset(i_2 as isize),
                                                    *yCoords.offset(i_2 as isize),
                                                    *flags.offset(i_2 as isize) as ::core::ffi::c_int
                                                        & 0x80 as ::core::ffi::c_int == 0,
                                                    i_2 == 0 as ::core::ffi::c_uint,
                                                );
                                                sResult = BEWriteU8(out, outFlags);
                                                if sResult as ::core::ffi::c_uint
                                                    != EOT_STREAM_OK as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                {
                                                    returnedStatus = EOT_UNKNOWN_BUFFER_WRITE_ERROR;
                                                    current_block = 17468087227630049781;
                                                    break;
                                                } else {
                                                    i_2 = i_2.wrapping_add(1);
                                                }
                                            }
                                            match current_block {
                                                17468087227630049781 => {}
                                                _ => {
                                                    let mut i_3: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                                                    loop {
                                                        if !(i_3 < totalPoints) {
                                                            current_block = 1428307939028130064;
                                                            break;
                                                        }
                                                        let mut x: int16_t = *xCoords.offset(i_3 as isize);
                                                        if i_3 == 0 as ::core::ffi::c_uint
                                                            || x as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                                        {
                                                            if -(256 as ::core::ffi::c_int) < x as ::core::ffi::c_int
                                                                && (x as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                                                            {
                                                                x = (x as ::core::ffi::c_int * -(1 as ::core::ffi::c_int))
                                                                    as int16_t;
                                                            }
                                                            if 0 as ::core::ffi::c_int <= x as ::core::ffi::c_int
                                                                && (x as ::core::ffi::c_int) < 256 as ::core::ffi::c_int
                                                            {
                                                                sResult = BEWriteU8(out, x as uint8_t);
                                                            } else {
                                                                sResult = BEWriteS16(out, x);
                                                            }
                                                            if sResult as ::core::ffi::c_uint
                                                                != EOT_STREAM_OK as ::core::ffi::c_int
                                                                    as ::core::ffi::c_uint
                                                            {
                                                                returnedStatus = EOT_UNKNOWN_BUFFER_WRITE_ERROR;
                                                                current_block = 17468087227630049781;
                                                                break;
                                                            }
                                                        }
                                                        i_3 = i_3.wrapping_add(1);
                                                    }
                                                    match current_block {
                                                        17468087227630049781 => {}
                                                        _ => {
                                                            let mut i_4: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                                                            loop {
                                                                if !(i_4 < totalPoints) {
                                                                    current_block = 14913924298693586572;
                                                                    break;
                                                                }
                                                                let mut y: int16_t = *yCoords.offset(i_4 as isize);
                                                                if i_4 == 0 as ::core::ffi::c_uint
                                                                    || y as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                                                {
                                                                    if -(256 as ::core::ffi::c_int) < y as ::core::ffi::c_int
                                                                        && (y as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                                                                    {
                                                                        y = (y as ::core::ffi::c_int * -(1 as ::core::ffi::c_int))
                                                                            as int16_t;
                                                                    }
                                                                    if 0 as ::core::ffi::c_int <= y as ::core::ffi::c_int
                                                                        && (y as ::core::ffi::c_int) < 256 as ::core::ffi::c_int
                                                                    {
                                                                        sResult = BEWriteU8(out, y as uint8_t);
                                                                    } else {
                                                                        sResult = BEWriteS16(out, y);
                                                                    }
                                                                    if sResult as ::core::ffi::c_uint
                                                                        != EOT_STREAM_OK as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint
                                                                    {
                                                                        returnedStatus = EOT_UNKNOWN_BUFFER_WRITE_ERROR;
                                                                        current_block = 17468087227630049781;
                                                                        break;
                                                                    }
                                                                }
                                                                i_4 = i_4.wrapping_add(1);
                                                            }
                                                            match current_block {
                                                                17468087227630049781 => {}
                                                                _ => {
                                                                    currPos = (*out).pos;
                                                                    sResult = seekAbsoluteThroughReserve(out, codeSizeLocation);
                                                                    if sResult as ::core::ffi::c_uint
                                                                        != EOT_STREAM_OK as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint
                                                                    {
                                                                        return EOT_CORRUPT_FILE;
                                                                    }
                                                                    sResult = BEWriteU16(out, unpackedCodeSize as uint16_t);
                                                                    if sResult as ::core::ffi::c_uint
                                                                        != EOT_STREAM_OK as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint
                                                                    {
                                                                        return EOT_CORRUPT_FILE;
                                                                    }
                                                                    if sResult as ::core::ffi::c_uint
                                                                        != EOT_STREAM_OK as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint
                                                                    {
                                                                        return EOT_CORRUPT_FILE;
                                                                    }
                                                                    sResult = seekAbsoluteThroughReserve(out, currPos);
                                                                    if sResult as ::core::ffi::c_uint
                                                                        != EOT_STREAM_OK as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint
                                                                    {
                                                                        return EOT_CORRUPT_FILE;
                                                                    }
                                                                    if calculateBoundingBox {
                                                                        let mut endPos: ::core::ffi::c_uint = (*out).pos;
                                                                        sResult = seekAbsoluteThroughReserve(
                                                                            out,
                                                                            boundingBoxLocation,
                                                                        );
                                                                        if sResult as ::core::ffi::c_uint
                                                                            != EOT_STREAM_OK as ::core::ffi::c_int
                                                                                as ::core::ffi::c_uint
                                                                        {
                                                                            return EOT_CORRUPT_FILE;
                                                                        }
                                                                        sResult = BEWriteS16(out, minX);
                                                                        if sResult as ::core::ffi::c_uint
                                                                            != EOT_STREAM_OK as ::core::ffi::c_int
                                                                                as ::core::ffi::c_uint
                                                                        {
                                                                            return EOT_CORRUPT_FILE;
                                                                        }
                                                                        sResult = BEWriteS16(out, minY);
                                                                        if sResult as ::core::ffi::c_uint
                                                                            != EOT_STREAM_OK as ::core::ffi::c_int
                                                                                as ::core::ffi::c_uint
                                                                        {
                                                                            return EOT_CORRUPT_FILE;
                                                                        }
                                                                        sResult = BEWriteS16(out, maxX);
                                                                        if sResult as ::core::ffi::c_uint
                                                                            != EOT_STREAM_OK as ::core::ffi::c_int
                                                                                as ::core::ffi::c_uint
                                                                        {
                                                                            return EOT_CORRUPT_FILE;
                                                                        }
                                                                        sResult = BEWriteS16(out, maxY);
                                                                        if sResult as ::core::ffi::c_uint
                                                                            != EOT_STREAM_OK as ::core::ffi::c_int
                                                                                as ::core::ffi::c_uint
                                                                        {
                                                                            return EOT_CORRUPT_FILE;
                                                                        }
                                                                        sResult = seekAbsoluteThroughReserve(out, endPos);
                                                                    }
                                                                    returnedStatus = EOT_SUCCESS;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(flags as *mut ::core::ffi::c_void);
    free(xCoords as *mut ::core::ffi::c_void);
    free(yCoords as *mut ::core::ffi::c_void);
    return returnedStatus;
}
#[no_mangle]
pub unsafe extern "C" fn decodeCompositeGlyph(
    mut streams: *mut *mut Stream,
    mut out: *mut Stream,
) -> EOTError {
    let FLG_ARGS_WORDS: uint16_t = 0x1 as uint16_t;
    let FLG_HAVE_SCALE: uint16_t = 0x8 as uint16_t;
    let FLG_MORE_COMPONENTS: uint16_t = 0x20 as uint16_t;
    let FLG_HAVE_XY_SCALE: uint16_t = 0x40 as uint16_t;
    let FLG_HAVE_2_BY_2: uint16_t = 0x80 as uint16_t;
    let FLG_HAVE_INSTR: uint16_t = 0x100 as uint16_t;
    let mut in_0: *mut Stream = *streams.offset(0 as ::core::ffi::c_int as isize);
    let mut minX: int16_t = 0;
    let mut minY: int16_t = 0;
    let mut maxX: int16_t = 0;
    let mut maxY: int16_t = 0;
    let mut sResult: StreamResult = EOT_STREAM_OK;
    sResult = BEWriteS16(out, -(1 as ::core::ffi::c_int) as int16_t);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    sResult = BEReadS16(in_0, &raw mut minX);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    sResult = BEReadS16(in_0, &raw mut minY);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    sResult = BEReadS16(in_0, &raw mut maxX);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    sResult = BEReadS16(in_0, &raw mut maxY);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    sResult = BEWriteS16(out, minX);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    sResult = BEWriteS16(out, minY);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    sResult = BEWriteS16(out, maxX);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    sResult = BEWriteS16(out, maxY);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    let mut flags: uint16_t = 0;
    loop {
        sResult = BEReadU16(in_0, &raw mut flags);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = BEWriteU16(out, flags);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = streamCopy(in_0, out, 2 as ::core::ffi::c_uint);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        let mut argsLength: ::core::ffi::c_uint = (if flags as ::core::ffi::c_int
            & FLG_ARGS_WORDS as ::core::ffi::c_int != 0
        {
            4 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint;
        sResult = streamCopy(in_0, out, argsLength);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        let mut transformBytes: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        if flags as ::core::ffi::c_int & FLG_HAVE_2_BY_2 as ::core::ffi::c_int != 0 {
            transformBytes = 8 as ::core::ffi::c_uint;
        } else if flags as ::core::ffi::c_int & FLG_HAVE_XY_SCALE as ::core::ffi::c_int
            != 0
        {
            transformBytes = 4 as ::core::ffi::c_uint;
        } else if flags as ::core::ffi::c_int & FLG_HAVE_SCALE as ::core::ffi::c_int != 0
        {
            transformBytes = 2 as ::core::ffi::c_uint;
        }
        sResult = streamCopy(in_0, out, transformBytes);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        if !(flags as ::core::ffi::c_int & FLG_MORE_COMPONENTS as ::core::ffi::c_int
            != 0)
        {
            break;
        }
    }
    if flags as ::core::ffi::c_int & FLG_HAVE_INSTR as ::core::ffi::c_int != 0 {
        
        let mut numInstrLocation: ::core::ffi::c_uint = (*out).pos;
        sResult = seekRelativeThroughReserve(
            out,
            ::core::mem::size_of::<uint16_t>() as ::core::ffi::c_int,
        );
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        let mut pushCount: uint16_t = 0;
        sResult = read255UShort(in_0, &raw mut pushCount);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        let mut result: EOTError = decodePushInstructions(
            *streams.offset(1 as ::core::ffi::c_int as isize),
            out,
            pushCount as ::core::ffi::c_uint,
        );
        if result as ::core::ffi::c_uint
            != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
            && (result as ::core::ffi::c_uint) < EOT_WARN as ::core::ffi::c_uint
        {
            return result;
        }
        let mut codeSize: uint16_t = 0;
        sResult = read255UShort(in_0, &raw mut codeSize);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        sResult = streamCopy(
            *streams.offset(2 as ::core::ffi::c_int as isize),
            out,
            codeSize as ::core::ffi::c_uint,
        );
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return EOT_CORRUPT_FILE;
        }
        let mut numInstr:  uint16_t =
     ((*out).pos as usize)
            .wrapping_sub(
                (numInstrLocation as usize)
                    .wrapping_add(::core::mem::size_of::<uint16_t>() as usize),
            ) as uint16_t;
        if numInstr as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            let mut currPos: ::core::ffi::c_uint = (*out).pos;
            sResult = seekAbsoluteThroughReserve(out, numInstrLocation);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
            sResult = BEWriteU16(out, numInstr);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
            sResult = seekAbsoluteThroughReserve(out, currPos);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
        }
    }
    return EOT_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn decodeGlyph(
    mut streams: *mut *mut Stream,
    mut out: *mut Stream,
) -> EOTError {
    let mut in_0: *mut Stream = *streams.offset(0 as ::core::ffi::c_int as isize);
    let mut numContours: int16_t = 0;
    let mut xMin: int16_t = 0 as int16_t;
    let mut yMin: int16_t = 0 as int16_t;
    let mut xMax: int16_t = 0 as int16_t;
    let mut yMax: int16_t = 0 as int16_t;
    let mut calculateBoundingBox: bool = false_0 != 0;
    let mut sResult: StreamResult = EOT_STREAM_OK;
    sResult = BEReadS16(in_0, &raw mut numContours);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    if (numContours as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        let mut result: EOTError = decodeCompositeGlyph(streams, out);
        if result as ::core::ffi::c_uint
            != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
            && (result as ::core::ffi::c_uint) < EOT_WARN as ::core::ffi::c_uint
        {
            return result;
        }
    } else {
        if numContours as ::core::ffi::c_int == 0x7fff as ::core::ffi::c_int {
            sResult = BEReadS16(in_0, &raw mut numContours);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
            sResult = BEReadS16(in_0, &raw mut xMin);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
            sResult = BEReadS16(in_0, &raw mut yMin);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
            sResult = BEReadS16(in_0, &raw mut xMax);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
            sResult = BEReadS16(in_0, &raw mut yMax);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
        } else {
            calculateBoundingBox = true_0 != 0;
        }
        let mut result_0: EOTError = decodeSimpleGlyph(
            numContours,
            streams,
            out,
            calculateBoundingBox,
            xMin,
            yMin,
            xMax,
            yMax,
        );
        if result_0 as ::core::ffi::c_uint
            != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
            && (result_0 as ::core::ffi::c_uint) < EOT_WARN as ::core::ffi::c_uint
        {
            return result_0;
        }
    }
    return EOT_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn populateGlyfAndLoca(
    mut glyf: *mut SFNTTable,
    mut loca: *mut SFNTTable,
    mut headData: *mut TTFheadData,
    mut maxpData: *mut TTFmaxpData,
    mut streams: *mut *mut Stream,
) -> EOTError {
    let mut sCTF: *mut Stream = *streams.offset(0 as ::core::ffi::c_int as isize);
    let mut sResult: StreamResult = seekAbsolute(sCTF, (*glyf).offset);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    let mut overranAllocatedSpace: bool = false_0 != 0;
    let mut notEnoughGlyphs: bool = false_0 != 0;
    seekAbsolute(
        *streams.offset(1 as ::core::ffi::c_int as isize),
        0 as ::core::ffi::c_uint,
    );
    seekAbsolute(
        *streams.offset(2 as ::core::ffi::c_int as isize),
        0 as ::core::ffi::c_uint,
    );
    let mut maxSimpleGlyphSize: ::core::ffi::c_uint = (10 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int * (*maxpData).maxContours as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + (*maxpData).maxSizeOfInstructions as ::core::ffi::c_int
        + (*maxpData).maxPoints as ::core::ffi::c_int * 5 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    let mut maxCompoundGlyphSize: ::core::ffi::c_uint = (26 as ::core::ffi::c_int
        + (*maxpData).maxSizeOfInstructions as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    let mut maxGlyphSize: ::core::ffi::c_uint = umax(
        maxSimpleGlyphSize,
        maxCompoundGlyphSize,
    );
    let mut maxTableSize: ::core::ffi::c_uint = ((*maxpData).numGlyphs
        as ::core::ffi::c_uint)
        .wrapping_mul(maxGlyphSize);
    let mut sOut: Stream = constructStream(
        ::core::ptr::null_mut::<uint8_t>(),
        0 as ::core::ffi::c_uint,
    );
    reserve(&raw mut sOut, maxTableSize);
    let mut sLocaOut: Stream = constructStream(
        ::core::ptr::null_mut::<uint8_t>(),
        0 as ::core::ffi::c_uint,
    );
    let mut shortLoca: bool = (*headData).indexToLocFormat == 0;
    if shortLoca {
        reserve(
            &raw mut sLocaOut,
            (2 as ::core::ffi::c_int
                * ((*maxpData).numGlyphs as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)) as ::core::ffi::c_uint,
        );
        BEWriteU16(&raw mut sLocaOut, 0 as uint16_t);
    } else {
        reserve(
            &raw mut sLocaOut,
            (4 as ::core::ffi::c_int
                * ((*maxpData).numGlyphs as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int)) as ::core::ffi::c_uint,
        );
        BEWriteU32(&raw mut sLocaOut, 0 as uint32_t);
    }
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < (*maxpData).numGlyphs as ::core::ffi::c_uint {
        let mut result: EOTError = decodeGlyph(streams, &raw mut sOut);
        if result as ::core::ffi::c_uint
            != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return result;
        }
        if sOut.pos.wrapping_rem(2 as ::core::ffi::c_uint) != 0 {
            BEWriteU8(&raw mut sOut, 0 as uint8_t);
        }
        if shortLoca {
            BEWriteU16(
                &raw mut sLocaOut,
                sOut.pos.wrapping_div(2 as ::core::ffi::c_uint) as uint16_t,
            );
        } else {
            BEWriteU32(&raw mut sLocaOut, sOut.pos as uint32_t);
        }
        i = i.wrapping_add(1);
    }
    (*glyf).buf = sOut.buf;
    (*glyf).bufSize = sOut.size;
    (*loca).buf = sLocaOut.buf;
    (*loca).bufSize = sLocaOut.size;
    if notEnoughGlyphs {
        return EOT_WARN_NOT_ENOUGH_GLYPHS;
    }
    if overranAllocatedSpace {
        return EOT_WARN_NOT_ENOUGH_SPACE_RESERVED;
    }
    return EOT_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn parseCTF(
    mut streams: *mut *mut Stream,
    mut out: *mut *mut SFNTContainer,
) -> EOTError {
    *out = ::core::ptr::null_mut::<SFNTContainer>();
    let mut result: EOTError = constructContainer(out);
    let mut offsetTable: SFNTOffsetTable = SFNTOffsetTable {
        scalarType: 0,
        numTables: 0,
        searchRange: 0,
        entrySelector: 0,
        rangeShift: 0,
    };
    let mut sResult: StreamResult = parseOffsetTable(
        *streams.offset(0 as ::core::ffi::c_int as isize),
        &raw mut offsetTable,
    );
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    result = reserveTables(*out, offsetTable.numTables as ::core::ffi::c_uint);
    if result as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < offsetTable.numTables as ::core::ffi::c_uint {
        let mut tag: [::core::ffi::c_char; 4] = [0; 4];
        let mut j: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while j < 4 as ::core::ffi::c_uint {
            sResult = BEReadChar(
                *streams.offset(0 as ::core::ffi::c_int as isize),
                (&raw mut tag as *mut ::core::ffi::c_char).offset(j as isize),
            );
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
            j = j.wrapping_add(1);
        }
        let mut tbl: *mut SFNTTable = ::core::ptr::null_mut::<SFNTTable>();
        if strncmp(
            &raw mut tag as *mut ::core::ffi::c_char,
            b"hdmx\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                &raw mut tag as *mut ::core::ffi::c_char,
                b"VDMX\0" as *const u8 as *const ::core::ffi::c_char,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            sResult = seekRelative(
                *streams.offset(0 as ::core::ffi::c_int as isize),
                12 as ::core::ffi::c_int,
            );
            logWarning(
                b"Ignoring hdmx/VDMX table -- will be fixed in a future release.\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            result = addTable(
                *out,
                &raw mut tag as *mut ::core::ffi::c_char,
                &raw mut tbl,
            );
            sResult = seekRelative(
                *streams.offset(0 as ::core::ffi::c_int as isize),
                4 as ::core::ffi::c_int,
            );
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
            sResult = BEReadU32(
                *streams.offset(0 as ::core::ffi::c_int as isize),
                &raw mut (*tbl).offset,
            );
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
            sResult = BEReadU32(
                *streams.offset(0 as ::core::ffi::c_int as isize),
                &raw mut (*tbl).bufSize,
            );
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return EOT_CORRUPT_FILE;
            }
        }
        i = i.wrapping_add(1);
    }
    let mut glyf: *mut SFNTTable = ::core::ptr::null_mut::<SFNTTable>();
    let mut loca: *mut SFNTTable = ::core::ptr::null_mut::<SFNTTable>();
    let mut maxp: *mut SFNTTable = ::core::ptr::null_mut::<SFNTTable>();
    let mut head: *mut SFNTTable = ::core::ptr::null_mut::<SFNTTable>();
    let mut hmtx: *mut SFNTTable = ::core::ptr::null_mut::<SFNTTable>();
    let mut i_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i_0 < (**out).numTables {
        let mut tbl_0: *mut SFNTTable = (**out).tables.offset(i_0 as isize)
            as *mut SFNTTable;
        let mut loadTable: bool = true_0 != 0;
        if strncmp(
            &raw mut (*tbl_0).tag as *mut ::core::ffi::c_char,
            b"loca\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            loca = tbl_0;
            loadTable = false_0 != 0;
        } else if strncmp(
            &raw mut (*tbl_0).tag as *mut ::core::ffi::c_char,
            b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            glyf = tbl_0;
            loadTable = false_0 != 0;
        } else if strncmp(
            &raw mut (*tbl_0).tag as *mut ::core::ffi::c_char,
            b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            maxp = tbl_0;
        } else if strncmp(
            &raw mut (*tbl_0).tag as *mut ::core::ffi::c_char,
            b"head\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            head = tbl_0;
        } else if strncmp(
            &raw mut (*tbl_0).tag as *mut ::core::ffi::c_char,
            b"hmtx\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            hmtx = tbl_0;
        } else if strncmp(
            &raw mut (*tbl_0).tag as *mut ::core::ffi::c_char,
            b"hdmx\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                &raw mut (*tbl_0).tag as *mut ::core::ffi::c_char,
                b"VDMX\0" as *const u8 as *const ::core::ffi::c_char,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            return EOT_LOGIC_ERROR
        } else if strncmp(
            &raw mut (*tbl_0).tag as *mut ::core::ffi::c_char,
            b"cvt \0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            result = unpackCVT(tbl_0, *streams.offset(0 as ::core::ffi::c_int as isize));
            if result as ::core::ffi::c_uint
                != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return result;
            }
            loadTable = false_0 != 0;
        } else if strncmp(
            &raw mut (*tbl_0).tag as *mut ::core::ffi::c_char,
            b"VDMX\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            loadTable = false_0 != 0;
        }
        if loadTable {
            result = loadTableFromStream(
                tbl_0,
                *streams.offset(0 as ::core::ffi::c_int as isize),
            );
            if result as ::core::ffi::c_uint
                != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return result;
            }
            if strncmp(
                &raw mut (*tbl_0).tag as *mut ::core::ffi::c_char,
                b"head\0" as *const u8 as *const ::core::ffi::c_char,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if (*tbl_0).bufSize < 12 as ::core::ffi::c_uint {
                    return EOT_MALFORMED_HEAD_TABLE;
                }
                let mut i_1: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
                while i_1 < 12 as ::core::ffi::c_uint {
                    *(*tbl_0).buf.offset(i_1 as isize) = 0 as uint8_t;
                    i_1 = i_1.wrapping_add(1);
                }
            }
        }
        i_0 = i_0.wrapping_add(1);
    }
    if !glyf.is_null() && loca.is_null() {
        result = addTable(
            *out,
            b"loca\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut loca,
        );
        if result as ::core::ffi::c_uint
            != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return result;
        }
        if loca.is_null() {
            return EOT_LOGIC_ERROR;
        }
    }
    if maxp.is_null() {
        return EOT_NO_MAXP_TABLE;
    }
    if head.is_null() {
        return EOT_NO_HEAD_TABLE;
    }
    if hmtx.is_null() {
        return EOT_NO_HMTX_TABLE;
    }
    let mut headData: TTFheadData = TTFheadData { indexToLocFormat: 0 };
    result = TTFParseHead(head, &raw mut headData);
    if result as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    let mut maxpData: TTFmaxpData = TTFmaxpData {
        numGlyphs: 0,
        maxPoints: 0,
        maxContours: 0,
        maxComponentPoints: 0,
        maxComponentContours: 0,
        maxZones: 0,
        maxTwilightPoints: 0,
        maxStorage: 0,
        maxFunctionDefs: 0,
        maxInstructionDefs: 0,
        maxStackElements: 0,
        maxSizeOfInstructions: 0,
        maxComponentElements: 0,
        maxComponentDepth: 0,
    };
    result = TTFParseMaxp(maxp, &raw mut maxpData);
    if result as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return result;
    }
    if !glyf.is_null() {
        result = populateGlyfAndLoca(
            glyf,
            loca,
            &raw mut headData,
            &raw mut maxpData,
            streams,
        );
        if result as ::core::ffi::c_uint
            != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return result;
        }
    }
    return EOT_SUCCESS;
}
