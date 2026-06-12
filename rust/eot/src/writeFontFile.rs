use crate::core::Error;
use crate::util::stream::*;
use crate::ctf::SFNTContainer::dumpContainer;
use crate::ctf::parseCTF::parseCTF;

extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn unpackMtx(
        buf: *mut Stream,
        size: ::core::ffi::c_uint,
        bufsOut: *mut *mut uint8_t,
        bufSizesOut: *mut ::core::ffi::c_uint,
    ) -> EOTError;
}
pub type uint8_t = u8;
pub type EOTError = ::core::ffi::c_uint;

pub const ENCRYPTION_KEY: uint8_t = 0x50 as uint8_t;

#[no_mangle]
pub unsafe fn writeFontBuffer(data: &[u8], compressed: bool, encrypted: bool) -> Result<Vec<u8>, Error> {
    let fontSize = data.len() as u32;

    let mut finalOutBuffer: Vec<u8>;

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

    if compressed {
        let mut sizes: [::core::ffi::c_uint; 3] = [0; 3];
        let mut sBuf: Stream = constructStream(buf.as_mut_ptr(), fontSize);
        let result = unpackMtx(
            &raw mut sBuf,
            fontSize,
            &raw mut ctfs as *mut *mut uint8_t,
            &raw mut sizes as *mut ::core::ffi::c_uint,
        );

        if result != 0 {
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
        let mut ctr = parseCTF(&raw mut streamPtrs as *mut *mut Stream)?;
        finalOutBuffer = dumpContainer(&raw mut ctr)?;
    } else {
        finalOutBuffer = buf;
    }

    for i in 0..3 {
        free(ctfs[i] as *mut ::core::ffi::c_void);
    }

    Ok(finalOutBuffer)
}
