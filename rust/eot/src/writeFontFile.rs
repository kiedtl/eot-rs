use crate::core::Error;
use crate::util::stream::*;
use crate::util::stream2::Stream as Stream2;
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

        let mut streams: [Stream2; 3] = [
            Stream2::new2(0, 0),
            Stream2::new2(0, 0),
            Stream2::new2(0, 0),
        ];
        for (i, stream) in streams.iter_mut().enumerate() {
            let slice = std::slice::from_raw_parts(ctfs[i], sizes[i] as _);
            stream.buf = slice.into();
        }
        let mut ctr = parseCTF(&mut streams)?;
        finalOutBuffer = dumpContainer(&mut ctr)?;
    } else {
        finalOutBuffer = buf;
    }

    for i in 0..3 {
        free(ctfs[i] as *mut ::core::ffi::c_void);
    }

    Ok(finalOutBuffer)
}
