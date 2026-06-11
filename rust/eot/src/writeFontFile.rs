use crate::core::Error;
use crate::ctf::SFNTContainer::{SFNTContainer, dumpContainer};

#[repr(C)] pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)] pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)] pub struct _IO_marker { _opaque: [u8; 0] }
extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn constructStream(buf: *mut uint8_t, size: ::core::ffi::c_uint) -> Stream;
    fn freeContainer(ctr: *mut SFNTContainer);
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
pub type _IO_lock_t = ();
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();

pub const ENCRYPTION_KEY: uint8_t = 0x50 as uint8_t;

#[no_mangle]
pub unsafe fn writeFontBuffer(data: &[u8], compressed: bool, encrypted: bool) -> Result<Vec<u8>, Error> {
    let fontSize = data.len() as u32;

    let mut finalOutBuffer: Vec<u8>;
    let mut result: EOTError = EOT_SUCCESS;

    let mut buf = Vec::with_capacity(data.len());
    for i in 0..data.len() {
        buf.push(
            if encrypted {
                data[i] ^ ENCRYPTION_KEY
            } else {
                data[i]
            }
        );
    }

    let mut ctfs: [*mut uint8_t; 3] = [
        ::core::ptr::null_mut::<uint8_t>(),
        ::core::ptr::null_mut::<uint8_t>(),
        ::core::ptr::null_mut::<uint8_t>(),
    ];

    let mut ctr: *mut SFNTContainer = ::core::ptr::null_mut::<SFNTContainer>();

    if compressed {
        let mut sizes: [::core::ffi::c_uint; 3] = [0; 3];
        let mut sBuf: Stream = constructStream(buf.as_mut_ptr(), fontSize);
        result = unpackMtx(
            &raw mut sBuf,
            fontSize,
            &raw mut ctfs as *mut *mut uint8_t,
            &raw mut sizes as *mut ::core::ffi::c_uint,
        );

        if result != EOT_SUCCESS {
            panic!("error");
        }

        let mut streams: [Stream; 3] = [Stream {
            buf: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            reserved: 0,
            pos: 0,
            bitPos: 0,
        }; 3];
        for i in 0..3 {
            streams[i] = constructStream(ctfs[i], sizes[i]);
        }
        let mut streamPtrs: [*mut Stream; 3] = [
            &raw mut streams as *mut Stream,
            (&raw mut streams as *mut Stream)
                .offset(1 as ::core::ffi::c_int as isize),
            (&raw mut streams as *mut Stream)
                .offset(2 as ::core::ffi::c_int as isize),
        ];
        result = parseCTF(&raw mut streamPtrs as *mut *mut Stream, &raw mut ctr);
        if result != EOT_SUCCESS {
            panic!("error");
        }

        finalOutBuffer = dumpContainer(ctr)?;
    } else {
        finalOutBuffer = buf;
    }

    for i in 0..3 {
        free(ctfs[i] as *mut ::core::ffi::c_void);
    }

    if !ctr.is_null() {
        freeContainer(ctr);
    }

    Ok(finalOutBuffer)
}
