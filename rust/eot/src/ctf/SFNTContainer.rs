use std::io::{Read, Write, Seek, SeekFrom, Cursor};

use crate::core::Error;
use crate::util::stream2::{Error as StreamError, Stream as Stream2};

extern "C" {
    fn constructStream(buf: *mut uint8_t, size: ::core::ffi::c_uint) -> Stream;
    fn constructStream2(
        buf: *mut uint8_t,
        size: ::core::ffi::c_uint,
        reserved: ::core::ffi::c_uint,
    ) -> Stream;
    fn seekAbsolute(s: *mut Stream, pos: ::core::ffi::c_uint) -> StreamResult;
    fn seekRelativeThroughReserve(
        s: *mut Stream,
        offset: ::core::ffi::c_int,
    ) -> StreamResult;
    fn reserve(s: *mut Stream, toReserve: ::core::ffi::c_uint) -> StreamResult;
    fn BEWriteU8(s: *mut Stream, in_0: uint8_t) -> StreamResult;
    fn BEWriteU16(s: *mut Stream, in_0: uint16_t) -> StreamResult;
    fn BEWriteU32(s: *mut Stream, in_0: uint32_t) -> StreamResult;
    fn BEReadRestAsU32(s: *mut Stream, out: *mut uint32_t) -> StreamResult;
    fn streamCopy(
        sIn: *mut Stream,
        sOut: *mut Stream,
        length: ::core::ffi::c_uint,
    ) -> StreamResult;
    fn BEcheckSum32(
        s: *mut Stream,
        out: *mut uint32_t,
        beginPos: ::core::ffi::c_uint,
        endPos: ::core::ffi::c_uint,
    ) -> StreamResult;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
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
pub struct SFNTContainer {
    pub numTables: ::core::ffi::c_uint,
    pub _numTablesReserved: ::core::ffi::c_uint,
    pub tables: *mut SFNTTable,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub unsafe extern "C" fn reserveTables(
    mut ctr: *mut SFNTContainer,
    mut num: ::core::ffi::c_uint,
) -> EOTError {
    if (*ctr)._numTablesReserved >= num {
        return EOT_SUCCESS;
    }
    let mut allocated: *mut ::core::ffi::c_void = realloc(
        (*ctr).tables as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<SFNTTable>() as size_t).wrapping_mul(num as size_t),
    );
    if allocated.is_null() {
        return EOT_CANT_ALLOCATE_MEMORY;
    }
    (*ctr).tables = allocated as *mut SFNTTable;
    (*ctr)._numTablesReserved = num;
    return EOT_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn constructContainer(
    mut out: *mut *mut SFNTContainer,
) -> EOTError {
    *out = malloc(::core::mem::size_of::<SFNTContainer>() as size_t)
        as *mut SFNTContainer;
    if out.is_null() {
        return EOT_CANT_ALLOCATE_MEMORY;
    }
    (**out).numTables = 0 as ::core::ffi::c_uint;
    (**out)._numTablesReserved = 0 as ::core::ffi::c_uint;
    (**out).tables = ::core::ptr::null_mut::<SFNTTable>();
    return EOT_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn _freeTable(mut tbl: *mut SFNTTable) {
    free((*tbl).buf as *mut ::core::ffi::c_void);
    (*tbl).buf = ::core::ptr::null_mut::<uint8_t>();
}
#[no_mangle]
pub unsafe extern "C" fn freeContainer(mut ctr: *mut SFNTContainer) {
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < (*ctr).numTables {
        _freeTable((*ctr).tables.offset(i as isize));
        i = i.wrapping_add(1);
    }
    free((*ctr).tables as *mut ::core::ffi::c_void);
    free(ctr as *mut ::core::ffi::c_void);
}

fn be_read_rest_as_u32(cursor: &mut Cursor<&mut [u8]>) -> Result<u32, StreamError> {
    let pos = cursor.position() as usize;
    let len = cursor.get_ref().len();

    if pos >= len {
        return Err(StreamError::NOT_ENOUGH_DATA);
    }

    let remaining = len - pos;
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf[..remaining.min(4)])
        .map_err(|_| StreamError::NOT_ENOUGH_DATA)?;

    Ok(match remaining {
        1 => (buf[0] as u32) << 24,
        2 => (u16::from_be_bytes([buf[0], buf[1]]) as u32) << 16,
        3 => ((buf[0] as u32) << 16 | (buf[1] as u32) << 8 | buf[2] as u32) << 8,
        _ => u32::from_be_bytes(buf),
    })
}

unsafe fn _writeTblCheckingSum(tbl: *mut SFNTTable, out: &mut Stream2) -> Result<(), Error> {
    (*tbl).checksum = 0 as ::core::ffi::c_uint;
    (*tbl).offset = (*out).pos as u32;

    let tableSlice = std::slice::from_raw_parts_mut((*tbl).buf, (*tbl).bufSize as usize);
    let mut c = Cursor::new(tableSlice);

    loop {
        match be_read_rest_as_u32(&mut c) {
            Ok(chunk) => {
                (*tbl).checksum = (*tbl).checksum.wrapping_add(chunk as ::core::ffi::c_uint);
                out.be_write_u32(chunk)?;
            },
            Err(StreamError::NOT_ENOUGH_DATA) => break,
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

unsafe fn _writeTableDirectory(ctr: *mut SFNTContainer, out: &mut Stream2) -> Result<(), Error> {
    for i in 0..(*ctr).numTables {
        let mut tbl: *mut SFNTTable = (*ctr).tables.offset(i as isize) as *mut SFNTTable;

        for iTag in 0..4 {
            out.be_write_u8((*tbl).tag[iTag as usize] as u8)?;
        }

        out.be_write_u32((*tbl).checksum as u32)?;
        out.be_write_u32((*tbl).offset as u32)?;
        out.be_write_u32((*tbl).bufSize as u32)?;
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn _lgflr(mut n: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let mut ret: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while n > 1 as ::core::ffi::c_uint {
        n = n.wrapping_div(2 as ::core::ffi::c_uint);
        ret = ret.wrapping_add(1);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _maxpw(mut n: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let mut ret: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
    while n > 1 as ::core::ffi::c_uint {
        ret = ret.wrapping_mul(2 as ::core::ffi::c_uint);
        n = n.wrapping_div(2 as ::core::ffi::c_uint);
    }
    return ret;
}

unsafe fn _writeOffsetTable(ctr: *mut SFNTContainer, s: &mut Stream2) -> Result<(), Error> {
    let mut scalerType: uint32_t = 0x10000 as uint32_t;
    let mut numTables: uint16_t = (*ctr).numTables as uint16_t;
    let mut searchRange: uint16_t = _maxpw((*ctr).numTables)
        .wrapping_mul(16 as ::core::ffi::c_uint) as uint16_t;
    let mut entrySelector: uint16_t = _lgflr((*ctr).numTables) as uint16_t;
    let mut rangeShift: uint16_t = (numTables as ::core::ffi::c_int
        * 16 as ::core::ffi::c_int - searchRange as ::core::ffi::c_int) as uint16_t;

    s.be_write_u32(scalerType)?;
    s.be_write_u16(numTables)?;
    s.be_write_u16(searchRange)?;
    s.be_write_u16(entrySelector)?;
    s.be_write_u16(rangeShift)?;

    Ok(())
}

unsafe fn _getTableDirectorySize(ctr: *mut SFNTContainer) -> ::core::ffi::c_uint {
    return (16 as ::core::ffi::c_uint).wrapping_mul((*ctr).numTables);
}

unsafe fn _getRequiredSize(ctr: *mut SFNTContainer) -> ::core::ffi::c_uint {
    let mut ret: ::core::ffi::c_uint = 12 as ::core::ffi::c_uint;
    ret = ret.wrapping_add(_getTableDirectorySize(ctr));
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < (*ctr).numTables {
        let mut tbl: *mut SFNTTable = (*ctr).tables.offset(i as isize) as *mut SFNTTable;
        ret = ret
            .wrapping_add(
                (*tbl)
                    .bufSize
                    .wrapping_add(3 as ::core::ffi::c_uint)
                    .wrapping_div(4 as ::core::ffi::c_uint)
                    .wrapping_mul(4 as ::core::ffi::c_uint),
            );
        i = i.wrapping_add(1);
    }
    return ret;
}

#[no_mangle]
pub unsafe fn dumpContainer(ctr: *mut SFNTContainer) -> Result<Vec<u8>, Error> {
    let mut beginningChk: ::core::ffi::c_uint = 0;
    let mut finalChecksum: ::core::ffi::c_uint = 0;

    let requiredSize = _getRequiredSize(ctr);
    let mut s = Stream2::new(requiredSize as usize);
    _writeOffsetTable(ctr, &mut s)?;

    let tableDirectoryOffset = s.pos;
    s.seek_relative_through_reserve(_getTableDirectorySize(ctr) as isize)?;

    let mut head: *mut SFNTTable = ::core::ptr::null_mut::<SFNTTable>();
    let mut chk: core::ffi::c_uint = 0;
    for i in 0..(*ctr).numTables {
        let mut tbl: *mut SFNTTable = (*ctr).tables.offset(i as isize)
            as *mut SFNTTable;
        if strncmp(
            &raw mut (*tbl).tag as *mut ::core::ffi::c_char,
            b"head\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            head = tbl;
        }
        (*tbl).offset = s.pos as u32;
        _writeTblCheckingSum(tbl, &mut s)?;
        chk = chk.wrapping_add((*tbl).checksum);
    }
    if head.is_null() {
        /* should have already caught the lack of a head table! */
        return Err(Error::LOGIC_ERROR);
    }

    s.seek_absolute(tableDirectoryOffset as usize)?;
    _writeTableDirectory(ctr, &mut s)?;

    beginningChk = s.be_checksum32(0, s.pos)?;
    chk = chk.wrapping_add(beginningChk);

    // now put in the global checksum. It's OK that this will make the head checksum incorrect!
    // this mystical number 0xB1B0AFBA is defined by the TTF standard, dunno why they picked this
    // value.
    finalChecksum = (0xb1b0afba as ::core::ffi::c_uint).wrapping_sub(chk);

    let sChkOut = std::slice::from_raw_parts_mut((*head).buf, (*head).bufSize as usize);
    let mut sChkOutC = Cursor::new(sChkOut);
    sChkOutC.seek(SeekFrom::Start(8)).map_err(|_| Error::LOGIC_ERROR)?;

    let (a, b, c, d) = (
        (finalChecksum >> 24) as u8,
        ((finalChecksum >> 16) & 0xFF) as u8,
        ((finalChecksum >> 8) & 0xFF) as u8,
        (finalChecksum & 0xFF) as u8
    );
    sChkOutC.write_all(&[a, b, c, d]).map_err(|_| Error::LOGIC_ERROR)?;

    Ok(s.buf)
}

#[no_mangle]
pub unsafe extern "C" fn addTable(
    mut ctr: *mut SFNTContainer,
    mut tag: *const ::core::ffi::c_char,
    mut newTableOut: *mut *mut SFNTTable,
) -> EOTError {
    if (*ctr).numTables == (*ctr)._numTablesReserved {
        let mut err: EOTError = reserveTables(
            ctr,
            (*ctr).numTables.wrapping_mul(2 as ::core::ffi::c_uint),
        );
        if err as ::core::ffi::c_uint
            != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return err;
        }
    }
    let fresh0 = (*ctr).numTables;
    (*ctr).numTables = (*ctr).numTables.wrapping_add(1);
    let mut tbl: *mut SFNTTable = (*ctr).tables.offset(fresh0 as isize)
        as *mut SFNTTable;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < 4 as ::core::ffi::c_uint {
        (*tbl).tag[i as usize] = *tag.offset(i as isize);
        i = i.wrapping_add(1);
    }
    (*tbl).buf = ::core::ptr::null_mut::<uint8_t>();
    (*tbl).bufSize = 0 as ::core::ffi::c_uint;
    (*tbl).offset = 0 as ::core::ffi::c_uint;
    *newTableOut = tbl;
    return EOT_SUCCESS;
}

#[no_mangle]
pub unsafe extern "C" fn loadTableFromStream(
    mut tbl: *mut SFNTTable,
    mut s: *mut Stream,
) -> EOTError {
    let mut sResult: StreamResult = seekAbsolute(s, (*tbl).offset);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    (*tbl).buf = malloc((*tbl).bufSize as size_t) as *mut uint8_t;
    let mut sOut: Stream = constructStream2(
        (*tbl).buf,
        0 as ::core::ffi::c_uint,
        (*tbl).bufSize,
    );
    sResult = streamCopy(s, &raw mut sOut, (*tbl).bufSize);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CORRUPT_FILE;
    }
    return EOT_SUCCESS;
}
