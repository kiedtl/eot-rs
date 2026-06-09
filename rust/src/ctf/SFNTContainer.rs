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
#[no_mangle]
pub unsafe extern "C" fn _writeTblCheckingSum(
    mut tbl: *mut SFNTTable,
    mut out: *mut Stream,
) -> StreamResult {
    (*tbl).checksum = 0 as ::core::ffi::c_uint;
    (*tbl).offset = (*out).pos;
    let mut tblStream: Stream = constructStream((*tbl).buf, (*tbl).bufSize);
    let mut sResult: StreamResult = EOT_STREAM_OK;
    let mut sResult2: StreamResult = EOT_STREAM_OK;
    loop {
        let mut chunk: uint32_t = 0;
        sResult = BEReadRestAsU32(&raw mut tblStream, &raw mut chunk);
        if !(sResult as ::core::ffi::c_uint
            == EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            break;
        }
        (*tbl).checksum = (*tbl).checksum.wrapping_add(chunk as ::core::ffi::c_uint);
        sResult2 = BEWriteU32(out, chunk);
        if sResult2 as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return sResult2;
        }
    }
    if sResult as ::core::ffi::c_uint
        == EOT_NOT_ENOUGH_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_STREAM_OK;
    }
    return sResult;
}
#[no_mangle]
pub unsafe extern "C" fn _writeTableDirectory(
    mut ctr: *mut SFNTContainer,
    mut out: *mut Stream,
) -> StreamResult {
    let mut sResult: StreamResult = EOT_STREAM_OK;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < (*ctr).numTables {
        let mut tbl: *mut SFNTTable = (*ctr).tables.offset(i as isize) as *mut SFNTTable;
        let mut iTag: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while iTag < 4 as ::core::ffi::c_uint {
            sResult = BEWriteU8(out, (*tbl).tag[iTag as usize] as uint8_t);
            if sResult as ::core::ffi::c_uint
                != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return sResult;
            }
            iTag = iTag.wrapping_add(1);
        }
        sResult = BEWriteU32(out, (*tbl).checksum as uint32_t);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return sResult;
        }
        sResult = BEWriteU32(out, (*tbl).offset as uint32_t);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return sResult;
        }
        sResult = BEWriteU32(out, (*tbl).bufSize as uint32_t);
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return sResult;
        }
        i = i.wrapping_add(1);
    }
    return EOT_STREAM_OK;
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
#[no_mangle]
pub unsafe extern "C" fn _writeOffsetTable(
    mut ctr: *mut SFNTContainer,
    mut out: *mut Stream,
) -> StreamResult {
    let mut sResult: StreamResult = EOT_STREAM_OK;
    let mut scalerType: uint32_t = 0x10000 as uint32_t;
    let mut numTables: uint16_t = (*ctr).numTables as uint16_t;
    let mut searchRange: uint16_t = _maxpw((*ctr).numTables)
        .wrapping_mul(16 as ::core::ffi::c_uint) as uint16_t;
    let mut entrySelector: uint16_t = _lgflr((*ctr).numTables) as uint16_t;
    let mut rangeShift: uint16_t = (numTables as ::core::ffi::c_int
        * 16 as ::core::ffi::c_int - searchRange as ::core::ffi::c_int) as uint16_t;
    sResult = BEWriteU32(out, scalerType);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return sResult;
    }
    sResult = BEWriteU16(out, numTables);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return sResult;
    }
    sResult = BEWriteU16(out, searchRange);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return sResult;
    }
    sResult = BEWriteU16(out, entrySelector);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return sResult;
    }
    sResult = BEWriteU16(out, rangeShift);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return sResult;
    }
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn _getTableDirectorySize(
    mut ctr: *mut SFNTContainer,
) -> ::core::ffi::c_uint {
    return (16 as ::core::ffi::c_uint).wrapping_mul((*ctr).numTables);
}
#[no_mangle]
pub unsafe extern "C" fn _getRequiredSize(
    mut ctr: *mut SFNTContainer,
) -> ::core::ffi::c_uint {
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
pub unsafe extern "C" fn dumpContainer(
    mut ctr: *mut SFNTContainer,
    mut outBuf: *mut *mut uint8_t,
    mut outSize: *mut ::core::ffi::c_uint,
) -> EOTError {
    let mut tableDirectoryOffset: ::core::ffi::c_uint = 0;
    let mut head: *mut SFNTTable = ::core::ptr::null_mut::<SFNTTable>();
    let mut chk: ::core::ffi::c_uint = 0;
    let mut beginningChk: ::core::ffi::c_uint = 0;
    let mut finalChecksum: ::core::ffi::c_uint = 0;
    let mut sChkOut: Stream = Stream {
        buf: ::core::ptr::null_mut::<uint8_t>(),
        size: 0,
        reserved: 0,
        pos: 0,
        bitPos: 0,
    };
    let mut current_block: u64;
    let mut s: Stream = constructStream(
        ::core::ptr::null_mut::<uint8_t>(),
        0 as ::core::ffi::c_uint,
    );
    let mut requiredSize: ::core::ffi::c_uint = _getRequiredSize(ctr);
    let mut sResult: StreamResult = reserve(&raw mut s, requiredSize);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return EOT_CANT_ALLOCATE_MEMORY;
    }
    let mut returnedStatus: EOTError = EOT_SUCCESS;
    sResult = _writeOffsetTable(ctr, &raw mut s);
    if sResult as ::core::ffi::c_uint
        != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        returnedStatus = EOT_LOGIC_ERROR;
    } else {
        tableDirectoryOffset = s.pos;
        sResult = seekRelativeThroughReserve(
            &raw mut s,
            _getTableDirectorySize(ctr) as ::core::ffi::c_int,
        );
        if sResult as ::core::ffi::c_uint
            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            returnedStatus = EOT_LOGIC_ERROR;
        } else {
            head = ::core::ptr::null_mut::<SFNTTable>();
            chk = 0 as ::core::ffi::c_uint;
            let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            loop {
                if !(i < (*ctr).numTables) {
                    current_block = 11584701595673473500;
                    break;
                }
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
                (*tbl).offset = s.pos;
                sResult = _writeTblCheckingSum(tbl, &raw mut s);
                if sResult as ::core::ffi::c_uint
                    != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    returnedStatus = EOT_LOGIC_ERROR;
                    current_block = 7623316132418092903;
                    break;
                } else {
                    chk = chk.wrapping_add((*tbl).checksum);
                    i = i.wrapping_add(1);
                }
            }
            match current_block {
                7623316132418092903 => {}
                _ => {
                    if head.is_null() {
                        returnedStatus = EOT_LOGIC_ERROR;
                    } else {
                        seekAbsolute(&raw mut s, tableDirectoryOffset);
                        sResult = _writeTableDirectory(ctr, &raw mut s);
                        if sResult as ::core::ffi::c_uint
                            != EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            returnedStatus = EOT_LOGIC_ERROR;
                        } else {
                            beginningChk = 0;
                            sResult = BEcheckSum32(
                                &raw mut s,
                                &raw mut beginningChk,
                                0 as ::core::ffi::c_uint,
                                s.pos,
                            );
                            if sResult as ::core::ffi::c_uint
                                != EOT_STREAM_OK as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            {
                                returnedStatus = EOT_LOGIC_ERROR;
                            } else {
                                chk = chk.wrapping_add(beginningChk);
                                finalChecksum = (0xb1b0afba as ::core::ffi::c_uint)
                                    .wrapping_sub(chk);
                                sChkOut = constructStream((*head).buf, (*head).bufSize);
                                sResult = seekAbsolute(
                                    &raw mut sChkOut,
                                    8 as ::core::ffi::c_uint,
                                );
                                if sResult as ::core::ffi::c_uint
                                    != EOT_STREAM_OK as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                {
                                    returnedStatus = EOT_LOGIC_ERROR;
                                } else {
                                    sResult = BEWriteU32(
                                        &raw mut sChkOut,
                                        finalChecksum as uint32_t,
                                    );
                                    if sResult as ::core::ffi::c_uint
                                        != EOT_STREAM_OK as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        returnedStatus = EOT_LOGIC_ERROR;
                                    } else {
                                        returnedStatus = EOT_SUCCESS;
                                        *outBuf = s.buf;
                                        *outSize = s.size;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return returnedStatus;
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
