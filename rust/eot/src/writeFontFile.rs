use crate::core::Error;
use crate::lzcomp::liblzcomp::unpackMtx;
use crate::util::stream2::Stream as Stream2;
use crate::ctf::SFNTContainer::dumpContainer;
use crate::ctf::parseCTF::parseCTF;

const ENCRYPTION_KEY: u8 = 0x50;

pub unsafe fn writeFontBuffer(data: &[u8], compressed: bool, encrypted: bool) -> Result<Vec<u8>, Error> {
    let mut finalOutBuffer: Vec<u8>;

    let mut buf = Vec::from(data);
    if encrypted {
        for byte in &mut buf {
            *byte ^= ENCRYPTION_KEY;
        }
    }

    if compressed {
        let len = buf.len();
        let mut sBuf = Stream2::new(0);
        sBuf.buf = buf;
        let ctfs = unpackMtx(&mut sBuf, len as _)?;

        let mut streams: [Stream2; 3] = [
            Stream2::new2(0, 0),
            Stream2::new2(0, 0),
            Stream2::new2(0, 0),
        ];
        for (stream, ctf) in streams.iter_mut().zip(ctfs.into_iter()) {
            stream.buf = ctf;
        }
        let mut ctr = parseCTF(&mut streams)?;
        finalOutBuffer = dumpContainer(&mut ctr)?;
    } else {
        finalOutBuffer = buf;
    }

    Ok(finalOutBuffer)
}
