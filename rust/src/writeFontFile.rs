use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn constructStream(buf: *mut uint8_t, size: ::core::ffi::c_uint) -> Stream;
    fn freeContainer(ctr: *mut SFNTContainer);
    fn dumpContainer(
        ctr: *mut SFNTContainer,
        outBuf: *mut *mut uint8_t,
        outSize: *mut ::core::ffi::c_uint,
    ) -> EOTError;
    fn parseCTF(streams: *mut *mut Stream, out: *mut *mut SFNTContainer) -> EOTError;
    fn unpackMtx(
        buf: *mut Stream,
        size: ::core::ffi::c_uint,
        bufsOut: *mut *mut uint8_t,
        bufSizesOut: *mut ::core::ffi::c_uint,
    ) -> EOTError;
}
pub type __uint8_t = u8;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint8_t = __uint8_t;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub static mut ENCRYPTION_KEY: uint8_t = 0x50 as uint8_t;
#[no_mangle]
pub unsafe extern "C" fn writeFontBuffer(
    mut font: *const uint8_t,
    mut fontSize: ::core::ffi::c_uint,
    mut compressed: bool,
    mut encrypted: bool,
    mut finalOutBuffer: *mut *mut uint8_t,
    mut finalFontSize: *mut ::core::ffi::c_uint,
) -> EOTError {
    let mut current_block: u64;
    let mut result: EOTError = EOT_SUCCESS;
    let mut buf: *mut uint8_t = malloc(fontSize as size_t) as *mut uint8_t;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < fontSize {
        if encrypted {
            *buf.offset(i as isize) = (*font.offset(i as isize) as ::core::ffi::c_int
                ^ ENCRYPTION_KEY as ::core::ffi::c_int) as uint8_t;
        } else {
            *buf.offset(i as isize) = *font.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    let mut ctfs: [*mut uint8_t; 3] = [
        ::core::ptr::null_mut::<uint8_t>(),
        ::core::ptr::null_mut::<uint8_t>(),
        ::core::ptr::null_mut::<uint8_t>(),
    ];
    let mut ctr: *mut SFNTContainer = ::core::ptr::null_mut::<SFNTContainer>();
    if compressed {
        let mut sizes: [::core::ffi::c_uint; 3] = [0; 3];
        let mut sBuf: Stream = constructStream(buf, fontSize);
        result = unpackMtx(
            &raw mut sBuf,
            fontSize,
            &raw mut ctfs as *mut *mut uint8_t,
            &raw mut sizes as *mut ::core::ffi::c_uint,
        );
        if result as ::core::ffi::c_uint
            != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            current_block = 1974090257903522222;
        } else {
            let mut streams: [Stream; 3] = [Stream {
                buf: ::core::ptr::null_mut::<uint8_t>(),
                size: 0,
                reserved: 0,
                pos: 0,
                bitPos: 0,
            }; 3];
            let mut i_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while i_0 < 3 as ::core::ffi::c_uint {
                streams[i_0 as usize] = constructStream(
                    ctfs[i_0 as usize],
                    sizes[i_0 as usize],
                );
                i_0 = i_0.wrapping_add(1);
            }
            let mut streamPtrs: [*mut Stream; 3] = [
                &raw mut streams as *mut Stream,
                (&raw mut streams as *mut Stream)
                    .offset(1 as ::core::ffi::c_int as isize),
                (&raw mut streams as *mut Stream)
                    .offset(2 as ::core::ffi::c_int as isize),
            ];
            result = parseCTF(&raw mut streamPtrs as *mut *mut Stream, &raw mut ctr);
            if result as ::core::ffi::c_uint
                != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                current_block = 1974090257903522222;
            } else {
                result = dumpContainer(ctr, finalOutBuffer, finalFontSize);
                if result as ::core::ffi::c_uint
                    != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    current_block = 1974090257903522222;
                } else {
                    current_block = 1109700713171191020;
                }
            }
        }
    } else {
        *finalOutBuffer = buf;
        *finalFontSize = fontSize;
        current_block = 1109700713171191020;
    }
    match current_block {
        1109700713171191020 => {
            result = EOT_SUCCESS;
        }
        _ => {}
    }
    if *finalOutBuffer != buf {
        free(buf as *mut ::core::ffi::c_void);
    }
    let mut i_1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i_1 < 3 as ::core::ffi::c_uint {
        free(ctfs[i_1 as usize] as *mut ::core::ffi::c_void);
        i_1 = i_1.wrapping_add(1);
    }
    if !ctr.is_null() {
        freeContainer(ctr);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn writeFontFile(
    mut font: *const uint8_t,
    mut fontSize: ::core::ffi::c_uint,
    mut compressed: bool,
    mut encrypted: bool,
    mut outFile: *mut FILE,
) -> EOTError {
    let mut itemsWritten: ::core::ffi::c_int = 0;
    let mut result: EOTError = EOT_SUCCESS;
    let mut finalBuf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut finalFontSize: ::core::ffi::c_uint = 0;
    result = writeFontBuffer(
        font,
        fontSize,
        compressed,
        encrypted,
        &raw mut finalBuf,
        &raw mut finalFontSize,
    );
    if !(result as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        itemsWritten = fwrite(
            finalBuf as *const ::core::ffi::c_void,
            1 as size_t,
            finalFontSize as ::core::ffi::c_long as size_t,
            outFile,
        ) as ::core::ffi::c_int;
        if itemsWritten as ::core::ffi::c_uint == finalFontSize {
            result = EOT_SUCCESS;
        } else {
            result = EOT_FWRITE_ERROR;
        }
    }
    if !finalBuf.is_null() {
        free(finalBuf as *mut ::core::ffi::c_void);
    }
    return result;
}
