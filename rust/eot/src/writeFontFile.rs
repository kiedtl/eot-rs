use crate::core::Error;
use crate::lzcomp::lzcomp::unpackMtx;
use crate::stream::Stream;
use crate::ctf::parseCTF::parseCTF;

const ENCRYPTION_KEY: u8 = 0x50;

pub fn writeFontBuffer(data: &[u8], compressed: bool, encrypted: bool) -> Result<Vec<u8>, Error> {
    let finalOutBuffer: Vec<u8>;

    let mut buf = Vec::from(data);
    if encrypted {
        for byte in &mut buf {
            *byte ^= ENCRYPTION_KEY;
        }
    }

    if compressed {
        let len = buf.len();
        let mut sBuf = Stream::new(0);
        sBuf.buf = buf;
        let ctfs = unpackMtx(&mut sBuf, len as _)?;

        let mut streams: [Stream; 3] = [
            Stream::new2(0, 0),
            Stream::new2(0, 0),
            Stream::new2(0, 0),
        ];
        for (stream, ctf) in streams.iter_mut().zip(ctfs.into_iter()) {
            stream.buf = ctf;
        }
        let mut ctr = parseCTF(&mut streams)?;
        finalOutBuffer = ctr.dumpToVec()?;
    } else {
        finalOutBuffer = buf;
    }

    Ok(finalOutBuffer)
}
