use crate::core::*;
use crate::lzcomp::lzcomp::*;
use crate::util::stream2::Stream as Stream2;

pub unsafe fn unpackMtx(buf: &mut Stream2, mut _size: u32) -> Result<[Vec<u8>; 3], Error> {
    let versionMagic = buf.be_read_u8()
        .map_err(|_| Error::MTX_ERROR)?;
    let _copyLimit = buf.be_read_u24()
        .map_err(|_| Error::MTX_ERROR)?;

    let mut offsets = [10u32, 0, 0];
    for i in 1 /* sic */ ..3 {
        offsets[i] = buf.be_read_u24()
            .map_err(|_| Error::MTX_ERROR)?;
    }

    let sizes: [u32; 3] = [
        offsets[1] - offsets[0],
        offsets[2] - offsets[1],
        buf.buf.len() as u32 - offsets[2],
    ];

    for i in 0..3 {
        if offsets[i] + sizes[i] > buf.buf.len() as u32 {
            return Err(Error::MTX_ERROR);
        }
    }

    let bufs = [
        MTX_LZCOMP_UnPackMemory(
            &raw mut buf.buf[offsets[0] as usize] as *mut core::ffi::c_void,
            sizes[0] as _,
            versionMagic as u8,
        )?,
        MTX_LZCOMP_UnPackMemory(
            &raw mut buf.buf[offsets[1] as usize] as *mut core::ffi::c_void,
            sizes[1] as _,
            versionMagic as u8,
        )?,
        MTX_LZCOMP_UnPackMemory(
            &raw mut buf.buf[offsets[2] as usize] as *mut core::ffi::c_void,
            sizes[2] as _,
            versionMagic as u8,
        )?,
    ];

    Ok(bufs)
}
