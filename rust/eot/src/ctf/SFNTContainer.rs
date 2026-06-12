use std::io::{Read, Cursor};

use crate::util::stream::*;
use crate::core::Error;
use crate::util::stream2::{Error as StreamError, Stream as Stream2};

extern "C" {
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

#[derive(Clone)]
pub struct SFNTTable {
    pub tag: [u8; 4],
    pub buf: Box<[u8]>,
    pub offset: usize,
    pub checksum: u32,
}

impl SFNTTable {
    pub fn new(tag: &[u8; 4]) -> Self {
        Self {
            tag: *tag,
            buf: Box::new([]),
            offset: 0,
            checksum: 0,
        }
    }
}

#[derive(Clone)]
pub struct SFNTContainer {
    pub tables: Vec<SFNTTable>,
}

impl SFNTContainer {
    pub fn new(cap: usize) -> Self {
        Self {
            tables: Vec::with_capacity(cap)
        }
    }

    pub fn add_table(&mut self, tag: &[u8; 4]) -> &mut SFNTTable {
        self.tables.push(SFNTTable::new(tag));
        let l = self.tables.len() - 1;
        &mut self.tables[l]
    }
}

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();

fn be_read_rest_as_u32(cursor: &mut Cursor<&Box<[u8]>>) -> Result<u32, StreamError> {
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
    (*tbl).checksum = 0;
    (*tbl).offset = (*out).pos;

    let mut c = Cursor::new(&(*tbl).buf);

    loop {
        match be_read_rest_as_u32(&mut c) {
            Ok(chunk) => {
                (*tbl).checksum = (*tbl).checksum.wrapping_add(chunk);
                out.be_write_u32(chunk)?;
            },
            Err(StreamError::NOT_ENOUGH_DATA) => break,
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

unsafe fn _writeTableDirectory(ctr: *mut SFNTContainer, out: &mut Stream2) -> Result<(), Error> {
    for i in 0..(*ctr).tables.len() {
        let mut tbl: *mut SFNTTable = &mut (*ctr).tables[i] as *mut SFNTTable;

        for iTag in 0..4 {
            out.be_write_u8((*tbl).tag[iTag as usize] as u8)?;
        }

        out.be_write_u32((*tbl).checksum)?;
        out.be_write_u32((*tbl).offset as u32)?;
        out.be_write_u32((*tbl).buf.len() as u32)?;
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
    let mut numTables: uint16_t = (*ctr).tables.len() as uint16_t;
    let mut searchRange: uint16_t = _maxpw((*ctr).tables.len() as u32)
        .wrapping_mul(16 as ::core::ffi::c_uint) as uint16_t;
    let mut entrySelector: uint16_t = _lgflr((*ctr).tables.len() as u32) as uint16_t;
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
    return (16 as ::core::ffi::c_uint).wrapping_mul((*ctr).tables.len() as u32);
}

unsafe fn _getRequiredSize(ctr: *mut SFNTContainer) -> ::core::ffi::c_uint {
    let mut ret: ::core::ffi::c_uint = 12;
    ret = ret.wrapping_add(_getTableDirectorySize(ctr));
    for tbl in &(*ctr).tables {
        ret = ret
            .wrapping_add(
                (tbl.buf.len() as core::ffi::c_uint)
                    .wrapping_add(3)
                    .wrapping_div(4)
                    .wrapping_mul(4)
            );
    }
    ret
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

    let mut head = None;
    let mut chk: core::ffi::c_uint = 0;
    for i in 0..(*ctr).tables.len() {
        let tbl = &mut (*ctr).tables[i];
        if &tbl.tag == b"head" {
            head = Some(i);
        }
        tbl.offset = s.pos;
        _writeTblCheckingSum(tbl, &mut s)?;
        chk = chk.wrapping_add((*tbl).checksum);
    }
    let Some(head) = head else {
        /* should have already caught the lack of a head table! */
        return Err(Error::LOGIC_ERROR);
    };

    s.seek_absolute(tableDirectoryOffset as usize)?;
    _writeTableDirectory(ctr, &mut s)?;

    beginningChk = s.be_checksum32(0, s.pos)?;
    chk = chk.wrapping_add(beginningChk);

    // now put in the global checksum. It's OK that this will make the head checksum incorrect!
    // this mystical number 0xB1B0AFBA is defined by the TTF standard, dunno why they picked this
    // value.
    finalChecksum = (0xb1b0afba as ::core::ffi::c_uint).wrapping_sub(chk);
    s.seek_absolute(((*ctr).tables[head].offset + 8) as usize)?;
    s.be_write_u32(finalChecksum)?;

    Ok(s.buf)
}

pub unsafe fn loadTableFromStream(
    tbl: &mut SFNTTable,
    mut s: *mut Stream,
) -> Result<(), Error> {
    let mut sResult: StreamResult = seekAbsolute(s, (*tbl).offset as _);
    if sResult != EOT_STREAM_OK {
        return Err(Error::CORRUPT_FILE);
    }
    let slice = std::slice::from_raw_parts((*s).buf, (*s).size as _);
    let end = ((*s).pos as usize) + tbl.buf.len();
    if end > slice.len() {
        return Err(Error::CORRUPT_FILE);
    }
    tbl.buf[..].copy_from_slice(&slice[(*s).pos as _..end]);
    Ok(())
}
