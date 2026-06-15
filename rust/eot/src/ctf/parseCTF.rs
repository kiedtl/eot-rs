use core::mem::size_of;

use crate::triplet_encodings::*;
use crate::core::*;
use crate::ctf::SFNTContainer::*;
use crate::stream::{Error as StreamError, Stream};
use crate::ctf::parseTTF::*;

#[derive(Copy, Clone)]
pub struct SFNTOffsetTable {
    pub scalarType: u32,
    pub numTables: u16,
    pub searchRange: u16,
    pub entrySelector: u16,
    pub rangeShift: u16,
}

pub type _dpi_TypeRead = ::core::ffi::c_uint;
pub const SHORT: _dpi_TypeRead = 1;
pub const BYTE: _dpi_TypeRead = 0;

fn parseOffsetTable(s: &mut Stream) -> Result<SFNTOffsetTable, StreamError> {
    let scalarType = s.be_read_u32()?;
    let numTables = s.be_read_u16()?;
    let searchRange = s.be_read_u16()?;
    let entrySelector = s.be_read_u16()?;
    let rangeShift = s.be_read_u16()?;
    Ok(SFNTOffsetTable { scalarType, numTables, searchRange, entrySelector, rangeShift })
}

fn _ucvt_rdVal(s_in: &mut Stream, lastValue: &mut i16) -> Result<(), StreamError> {
    let code = s_in.be_read_u8()?;
    let mut b2 = 0u8;
    let mut val = 0i16;

    if code >= 248 {
        b2 = s_in.be_read_u8()?;
        val = 238 * ((code as i32 - 247 as i32) as i16) + b2 as i16;
    } else if code >= 239 {
        b2 = s_in.be_read_u8()?;
        val = -1 * (238 * (code as i16 - 239) + b2 as i16) as i16;
    } else if code == 238 {
        val = s_in.be_read_i16()?;
    } else {
        val = code as _;
    }

    // The CVT table in CTF format is set up so that this does the right thing even if it
    // overflows.
    *lastValue = (*lastValue).wrapping_add(val);
    // Unless someone tries to run this code on some horrible system that doesn't use twos
    // complement...
    Ok(())
}

pub fn unpackCVT(mut out: &mut SFNTTable, s_in: &mut Stream) -> Result<(), Error> {
    s_in.seek_absolute(out.offset as _).map_err(|_| Error::CORRUPT_FILE)?;
    let tableLength = s_in.be_read_u16().map_err(|_| Error::CORRUPT_FILE)?;
    let mut s_out = Stream::new2(0, tableLength as usize * 2);
    let mut lastValue = 0i16;
    for _ in 0..tableLength {
        _ucvt_rdVal(s_in, &mut lastValue).map_err(|_| Error::CORRUPT_FILE)?;
        s_out.be_write_i16(lastValue).map_err(|_| Error::LOGIC_ERROR)?;
    }
    let Stream { buf: s_out_buf, .. } = s_out;
    out.buf = s_out_buf.into_boxed_slice();
    Ok(())
}

// http://www.w3.org/Submission/MTX/#id_255USHORT
fn read255UShort2(s_in: &mut Stream) -> Result<u16, StreamError> {
    Ok(match s_in.be_read_u8()? {
        253 => s_in.be_read_u16()?,
        255 => 253 + s_in.be_read_u8()? as u16,
        254 => 506 + s_in.be_read_u8()? as u16,
        val => val as _,
    })
}

// http://www.w3.org/Submission/MTX/#id_255SHORT
fn read255Short2(sIn: &mut Stream) -> Result<i16, StreamError> {
    let mut code: u8 = sIn.be_read_u8()?;
    if code == 253 {
        return sIn.be_read_i16();
    }

    let mut sign = 1i16;
    if code == 250 {
        sign = -1;
        code = sIn.be_read_u8()?;
    }

    let out = match code {
        255 => 250 + sIn.be_read_u8()? as i16,
        254 => (250 * 2) + sIn.be_read_u8()? as i16,
        _ => code as i16,
    };

    Ok(out * sign)
}

fn _dpi_dump2(
    out: &mut Stream,
    lastRead: &mut _dpi_TypeRead,
    typeLastReadCount: &mut u32,
    data: &mut Vec<i16>,
    dataIndex: &mut u32,
) -> Result<(), StreamError> {
    if *typeLastReadCount > 0 {
        if *typeLastReadCount < 8 {
            let op: u8 = (if *lastRead == BYTE { PUSHB } else { PUSHW }) as u8
                | (*typeLastReadCount).wrapping_sub(1) as u8;
            out.be_write_u8(op)?;
        } else {
            let op: u8 = if *lastRead == BYTE { NPUSHB } else { NPUSHW } as u8;
            out.be_write_u8(op)?;
            out.be_write_u8(*typeLastReadCount as u8)?;
        }

        for i in 0..*typeLastReadCount {
            if *lastRead == BYTE {
                out.be_write_u8(data[(*dataIndex - *typeLastReadCount + i) as usize] as _)?;
            } else {
                out.be_write_i16(data[(*dataIndex - *typeLastReadCount + i) as usize])?;
            }
        }
    }

    Ok(())
}

const NPUSHB: i32 = 0x40;
const NPUSHW: i32 = 0x41;
const PUSHB: i32 = 0xb0;
const PUSHW: i32 = 0xb8;

fn _dpi_put2(
    value: i16,
    out: &mut Stream,
    lastRead: &mut _dpi_TypeRead,
    typeLastReadCount: &mut u32,
    data: &mut Vec<i16>,
    dataIndex: &mut u32,
) -> Result<(), StreamError> {
    let newType = if value >= 0 && value < 256 { BYTE } else { SHORT };
    if newType != *lastRead || *typeLastReadCount == 255 {
        _dpi_dump2(out, lastRead, typeLastReadCount, data, dataIndex)?;
        *lastRead = newType;
        *typeLastReadCount = 0 as ::core::ffi::c_uint;
    }
    let fresh0 = *dataIndex;
    *dataIndex = (*dataIndex).wrapping_add(1);
    data[fresh0 as usize] = value;
    *typeLastReadCount = (*typeLastReadCount).wrapping_add(1);
    Ok(())
}

// http://www.w3.org/Submission/MTX/#HopCodes
fn decodePushInstructions(sIn: &mut Stream, sOut: &mut Stream, pushCount: u32) -> Result<(), Error> {
    let mut remaining = pushCount;
    let mut typeLastRead: _dpi_TypeRead = BYTE;
    let mut typeLastReadCount = 0u32;
    let mut dataIndex = 0u32;
    let mut data = vec![0i16; pushCount as _];

    while remaining > 0 {
        let mut code = sIn.be_peek_u8()
            .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
        let mut val = 0i16;
        let mut prev = 0i16;
        match code {
            0xFB => {
                /* A B 0xFB C -> A B A C A */
                if remaining < 3 || dataIndex < 2 {
                    return Err(Error::CORRUPT_HOPCODE_DATA);
                }
                remaining -= 3;
                prev = data[(dataIndex - 2) as usize];
                code = sIn.be_read_u8()?;
                _dpi_put2(prev, sOut, &mut typeLastRead, &mut typeLastReadCount, &mut data, &mut dataIndex)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                val = read255Short2(sIn)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(val, sOut, &mut typeLastRead, &mut typeLastReadCount, &mut data, &mut dataIndex)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(prev, sOut, &mut typeLastRead, &mut typeLastReadCount, &mut data, &mut dataIndex)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
            }
            0xFC => {
                if remaining < 5 || dataIndex < 2 {
                    return Err(Error::CORRUPT_HOPCODE_DATA);
                }
                remaining -= 5;
                prev = data[(dataIndex - 2) as usize];
                code = sIn.be_read_u8()?;
                _dpi_put2(prev, sOut, &mut typeLastRead, &mut typeLastReadCount, &mut data, &mut dataIndex)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                val = read255Short2(sIn)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(val, sOut, &mut typeLastRead, &mut typeLastReadCount, &mut data, &mut dataIndex)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(prev, sOut, &mut typeLastRead, &mut typeLastReadCount, &mut data, &mut dataIndex)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                val = read255Short2(sIn)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(val, sOut, &mut typeLastRead, &mut typeLastReadCount, &mut data, &mut dataIndex)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(prev, sOut, &mut typeLastRead, &mut typeLastReadCount, &mut data, &mut dataIndex)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
            }
            _ => {
                val = read255Short2(sIn)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(val, sOut, &mut typeLastRead, &mut typeLastReadCount, &mut data, &mut dataIndex)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                remaining -= 1;
            }
        }
    }

    _dpi_dump2(sOut, &mut typeLastRead, &mut typeLastReadCount, &mut data, &mut dataIndex)
        .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
    Ok(())
}

fn _dsg_makeFlags(x: i16, y: i16, onCurve: bool, firstTime: bool) -> u8 {
    const FLG_ON_CURVE: u8 = 0x1;
    const FLG_X_SHORT: u8 = 0x2;
    const FLG_Y_SHORT: u8 = 0x4;
    const FLG_X_SAME: u8 = 0x10;
    const FLG_Y_SAME: u8 = 0x20;

    let mut ret: u8 = 0 as u8;
    if onCurve {
        ret |= FLG_ON_CURVE;
    }

    if !firstTime && x == 0 {
        ret |= FLG_X_SAME;
    } else if -256 < x && x < 0 {
        ret |= FLG_X_SHORT;
    } else if 0 <= x && x < 256 {
        ret |= FLG_X_SHORT | FLG_X_SAME;
    }

    if !firstTime && y == 0 {
        ret |= FLG_Y_SAME;
    } else if -256 < y && y < 0 {
        ret |= FLG_Y_SHORT;
    } else if 0 <= y && y < 256 {
        ret |= FLG_Y_SHORT | FLG_Y_SAME;
    }

    ret
}

fn decodeSimpleGlyph(
    numContours: i16,
    streams: &mut [Stream],
    out: &mut Stream,
    calculateBoundingBox: bool,
    mut minX: i16,
    mut minY: i16,
    mut maxX: i16,
    mut maxY: i16,
) -> Result<(), Error> {
    if numContours == 0 {
        return Ok(());
    }

    let mut boundingBoxLocation = None;

    out.be_write_i16(numContours)
        .map_err(|_| Error::CORRUPT_FILE)?;

    if calculateBoundingBox {
        boundingBoxLocation = Some(out.pos);
        out.seek_relative_through_reserve(4 * size_of::<i16>() as isize)
            .map_err(|_| Error::CORRUPT_FILE)?;
        minX = i16::MAX;
        minY = i16::MAX;
        maxX = i16::MIN;
        maxY = i16::MIN;
    } else {
        // FIXME: why are we returning CORRUPT_FILE and not LOGIC_ERROR here?
        out.be_write_i16(minX)
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(minY)
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(maxX)
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(maxY)
            .map_err(|_| Error::CORRUPT_FILE)?;
    }

    let mut totalPoints: usize = 0;
    for i in 0..numContours {
        if i == 0 {
            totalPoints = 1;
        }
        let pointsInContour = read255UShort2(&mut streams[0])
            .map_err(|_| Error::CORRUPT_FILE)?;
        totalPoints += pointsInContour as usize;
        out.be_write_i16((totalPoints - 1) as _)
            .map_err(|_| Error::CORRUPT_FILE)?;
    }

    let mut flags = vec![0u8; totalPoints as _];
    let mut xCoords = vec![0i16; totalPoints as _];
    let mut yCoords = vec![0i16; totalPoints as _];

    // Read X-Y coordinates in shitty format described here:
    // http://www.w3.org/Submission/MTX/#TripletEncoding
    // First flags and then actual coordinates.

    for i in 0..totalPoints {
        flags[i] = streams[0].be_read_u8()
            .map_err(|_| Error::CORRUPT_FILE)?;
    }

    let mut currX = 0u32;
    let mut currY = 0u32;

    for i in 0..totalPoints {
        let enc = tripletEncodings[(flags[i] & 0x7F) as usize];
        let moreBytes = (enc.byteCount - 1) as usize;

        if streams[0].pos + moreBytes > streams[0].buf.len() {
            return Err(Error::CORRUPT_FILE);
        }

        // FIXME: stupid copying, probably don't even need a stream for this.
        let mut coords = Stream::new(0);
        coords.buf = (&streams[0].buf[streams[0].pos..streams[0].pos + moreBytes]).into();

        let dx: u32 = coords.read_n_bits(enc.xBits)?; // logic error
        let dy: u32 = coords.read_n_bits(enc.yBits)?; // logic error
        if coords.pos != coords.buf.len() || coords.bit_pos != 0 {
            return Err(Error::LOGIC_ERROR);
        }
        streams[0].seek_relative(coords.buf.len() as _)?; // logic error

        xCoords[i] = (enc.xSign * (dx + enc.deltaX) as i32) as _;
        yCoords[i] = (enc.ySign * (dy + enc.deltaY) as i32) as _;

        currX = currX.wrapping_add(xCoords[i] as i32 as u32);
        currY = currY.wrapping_add(yCoords[i] as i32 as u32);

        minX = minX.min(currX as i16);
        maxX = maxX.max(currX as i16);
        minY = minY.min(currY as i16);
        maxY = maxY.max(currY as i16);
    }

    // Coordinates are known now, but we need to handle instructions before they can be output.

    // advance past the code size output
    let codeSizeLocation = out.pos as u32;
    out.seek_relative_through_reserve(size_of::<u16>() as _)
        .map_err(|_| Error::CORRUPT_FILE)?;

    // decode the push instructions for the glyph
    let pushCount = read255UShort2(&mut streams[0])
        .map_err(|_| Error::CORRUPT_FILE)? as u16;
    decodePushInstructions(&mut streams[1], out, pushCount as _)?;
    let codeSize = read255UShort2(&mut streams[0])
        .map_err(|_| Error::CORRUPT_FILE)?;

    // copy over the rest of the instructions for the glyph
    for _ in 0..codeSize {
        out.be_write_u8(
            streams[2].be_read_u8()
                .map_err(|_| Error::CORRUPT_FILE)?
        )
            .map_err(|_| Error::CORRUPT_FILE)?;
    }

    // the below will be zero if we didn't go through the if (numContours > 0) block.
    let unpackedCodeSize = out.pos as u32 - (codeSizeLocation + size_of::<u16>() as u32);
    // FIXME: Figure out if there is a huge savings from using the 'repeat' flag
    // and if so, use it. (but I kinda doubt there is.)
    for i in 0..totalPoints {
        let outFlags = _dsg_makeFlags(xCoords[i], yCoords[i], flags[i] & 0x80 == 0, i == 0);
        out.be_write_u8(outFlags)
            .map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
    }

    for i in 0..totalPoints {
        let mut x = xCoords[i];
        if i == 0 || x != 0 {
            if -256 < x && x < 0 {
                x *= -1;
            }
            if 0 <= x && x < 256 {
                out.be_write_u8(x as _)
                    .map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
            } else {
                out.be_write_i16(x)
                    .map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
            }
        }
    }

    for i in 0..totalPoints {
        let mut y = yCoords[i];
        if i == 0 || y != 0 {
            if -256 < y && y < 0 {
                y *= -1;
            }
            if 0 <= y && y < 256 {
                out.be_write_u8(y as _)
                    .map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
            } else {
                out.be_write_i16(y)
                    .map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
            }
        }
    }

    let currPos = out.pos;
    out.seek_absolute_through_reserve(codeSizeLocation as _)
        .map_err(|_| Error::CORRUPT_FILE)?;
    out.be_write_u16(unpackedCodeSize as _)
        .map_err(|_| Error::CORRUPT_FILE)?;
    out.seek_absolute_through_reserve(currPos)
        .map_err(|_| Error::CORRUPT_FILE)?;

    if calculateBoundingBox {
        let endPos = out.pos;
        out.seek_absolute_through_reserve(boundingBoxLocation.unwrap())
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(minX)
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(minY)
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(maxX)
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(maxY)
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.seek_absolute_through_reserve(endPos as _)
            .map_err(|_| Error::CORRUPT_FILE)?;
    }

    Ok(())
}

fn decodeCompositeGlyph(streams: &mut [Stream], out: &mut Stream) -> Result<(), Error> {
    // we don't need to interpret very much here, just the flags to know how much to pass along
    // into the output.
    const FLG_ARGS_WORDS: u16 = 0x1;
    const FLG_HAVE_SCALE: u16 = 0x8;
    const FLG_MORE_COMPONENTS: u16 = 0x20;
    const FLG_HAVE_XY_SCALE: u16 = 0x40;
    const FLG_HAVE_2_BY_2: u16 = 0x80;
    const FLG_HAVE_INSTR: u16 = 0x100;

    out.be_write_i16(-1)
        .map_err(|_| Error::CORRUPT_FILE)?;
    let minX = streams[0].be_read_i16()
        .map_err(|_| Error::CORRUPT_FILE)?;
    let minY = streams[0].be_read_i16()
        .map_err(|_| Error::CORRUPT_FILE)?;
    let maxX = streams[0].be_read_i16()
        .map_err(|_| Error::CORRUPT_FILE)?;
    let maxY = streams[0].be_read_i16()
        .map_err(|_| Error::CORRUPT_FILE)?;
    out.be_write_i16(minX)
        .map_err(|_| Error::CORRUPT_FILE)?;
    out.be_write_i16(minY)
        .map_err(|_| Error::CORRUPT_FILE)?;
    out.be_write_i16(maxX)
        .map_err(|_| Error::CORRUPT_FILE)?;
    out.be_write_i16(maxY)
        .map_err(|_| Error::CORRUPT_FILE)?;

    let mut flags = 0u16;
    loop {
        flags = streams[0].be_read_u16()
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_u16(flags)
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_u8(
            streams[0].be_read_u8()
                .map_err(|_| Error::CORRUPT_FILE)?
        )
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_u8(
            streams[0].be_read_u8()
                .map_err(|_| Error::CORRUPT_FILE)?
        )
            .map_err(|_| Error::CORRUPT_FILE)?;

        let args_length = if flags & FLG_ARGS_WORDS != 0 { 4 } else { 2 };
        for _ in 0..args_length {
            out.be_write_u8(
                streams[0].be_read_u8()
                    .map_err(|_| Error::CORRUPT_FILE)?
            )
                .map_err(|_| Error::CORRUPT_FILE)?;
        }

        let transform_bytes = if flags & FLG_HAVE_2_BY_2 != 0 {
            8
        } else if flags & FLG_HAVE_XY_SCALE != 0 {
            4
        } else if flags & FLG_HAVE_SCALE != 0 {
            2
        } else {
            0
        };
        for _ in 0..transform_bytes {
            out.be_write_u8(
                streams[0].be_read_u8()
                    .map_err(|_| Error::CORRUPT_FILE)?
            )
                .map_err(|_| Error::CORRUPT_FILE)?;
        }

        if flags & FLG_MORE_COMPONENTS == 0 {
            break;
        }
    }

    if flags & FLG_HAVE_INSTR != 0 {
        // https://learn.microsoft.com/en-us/typography/opentype/spec/glyf 
        // uint16 numInstr
        let mut numInstrLocation = out.pos;
        out.seek_relative_through_reserve(2)
            .map_err(|_| Error::CORRUPT_FILE)?;

        // decode the push instructions for the glyph
        let pushCount = read255UShort2(&mut streams[0])
            .map_err(|_| Error::CORRUPT_FILE)? as u16;
        decodePushInstructions(&mut streams[1], out, pushCount as _)?;

        // copy over the rest of the instructions for the glyph
        let code_size = read255UShort2(&mut streams[0])
            .map_err(|_| Error::CORRUPT_FILE)?;
        for _ in 0..code_size {
            out.be_write_u8(
                streams[2].be_read_u8()
                    .map_err(|_| Error::CORRUPT_FILE)?
            )
                .map_err(|_| Error::CORRUPT_FILE)?;
        }

        let numInstr: u16 = ((out.pos as i32) - ((numInstrLocation as usize) + 2) as i32) as _;
        if numInstr > 0 {
            let currPos = out.pos;
            out.seek_absolute_through_reserve(numInstrLocation as _)
                .map_err(|_| Error::CORRUPT_FILE)?;
            out.be_write_u16(numInstr)
                .map_err(|_| Error::CORRUPT_FILE)?;
            out.seek_absolute_through_reserve(currPos)
                .map_err(|_| Error::CORRUPT_FILE)?;
        }
    }

    Ok(())
}

fn decodeGlyph(streams: &mut [Stream], out: &mut Stream) -> Result<(), Error> {
    let mut in_0 = &mut streams[0];
    let mut calculateBoundingBox: bool = false;

    let numContours = in_0.be_read_i16().map_err(|_| Error::CORRUPT_FILE)?;
    if numContours < 0 {
        decodeCompositeGlyph(streams, out)?;
    } else {
        let (xMin, yMin, xMax, yMax);
        let actual_num_contours;
        if numContours == 0x7fff {
            actual_num_contours = in_0.be_read_i16().map_err(|_| Error::CORRUPT_FILE)?;
            xMin = in_0.be_read_i16()?;
            yMin = in_0.be_read_i16()?;
            xMax = in_0.be_read_i16()?;
            yMax = in_0.be_read_i16()?;
        } else {
            calculateBoundingBox = true;
            (xMin, yMin, xMax, yMax) = (0, 0, 0, 0);
            actual_num_contours = numContours;
        }
        decodeSimpleGlyph(actual_num_contours, streams, out, calculateBoundingBox, xMin, yMin, xMax, yMax)?;
    }

    Ok(())
}

// https://developer.apple.com/fonts/TTRefMan/RM06/Chap6glyf.html
// http://www.w3.org/Submission/MTX/#CTFGlyph
fn populateGlyfAndLoca(
    tables: &mut [SFNTTable],
    glyf: usize,
    loca: usize,
    headData: &mut TTFheadData,
    maxpData: &mut TTFmaxpData,
    streams: &mut [Stream],
) -> Result<(), Error> {
    let mut sctf = &mut streams[0];
    sctf.seek_absolute(tables[glyf].offset as _)?;

    let mut overranAllocatedSpace: bool = false;
    let mut notEnoughGlyphs: bool = false;

    streams[1].seek_absolute(0)?;
    streams[2].seek_absolute(0)?;

    let maxSimpleGlyphSize = 10 + 2 * (maxpData.maxContours as u32) + 2
        + (maxpData.maxSizeOfInstructions as u32)
        + (maxpData.maxPoints as u32 * 5);
    let maxCompoundGlyphSize = 26 + (maxpData.maxSizeOfInstructions as u32);
    let maxGlyphSize = maxSimpleGlyphSize.max(maxCompoundGlyphSize);
    let maxTableSize = (maxpData.numGlyphs as u32) * maxGlyphSize;
    let is_short_loca = headData.indexToLocFormat == 0;

    let mut s_out = Stream::new2(0, maxTableSize as _);
    let mut s_loca_out = Stream::new2(0, 0);

    if is_short_loca {
        s_loca_out.buf.reserve(2 * (maxpData.numGlyphs + 1) as usize);
        s_loca_out.be_write_u16(0)
            .map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
    } else {
        s_loca_out.buf.reserve(4 * (maxpData.numGlyphs + 1) as usize);
        s_loca_out.be_write_u32(0)
            .map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
    }

    for _ in 0..maxpData.numGlyphs {
        // decode a glyph outline
        decodeGlyph(streams, &mut s_out)?;

        // do padding
        if s_out.pos % 2 != 0 {
            s_out.be_write_u8(0)?;
        }

        // add an entry to the location table
        if is_short_loca {
            s_loca_out.be_write_u16((s_out.pos / 2) as _)
                .map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
        } else {
            s_loca_out.be_write_u32(s_out.pos as _)
                .map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
        }
    }

    tables[glyf].buf = s_out.buf.into_boxed_slice();
    tables[loca].buf = s_loca_out.buf.into_boxed_slice();

    if notEnoughGlyphs {
        return Err(Error::WARN_NOT_ENOUGH_GLYPHS);
    }

    if overranAllocatedSpace {
        return Err(Error::WARN_NOT_ENOUGH_SPACE_RESERVED);
    }

    Ok(())
}

pub fn parseCTF(streams: &mut [Stream]) -> Result<SFNTContainer, Error> {
    let offsetTable = parseOffsetTable(&mut streams[0])
        .map_err(|_| Error::CORRUPT_FILE)?;
    let mut out = SFNTContainer::new(offsetTable.numTables as usize);

    for _ in 0..offsetTable.numTables {
        let mut tag = [0u8; 4];
        for j in 0..4 {
            tag[j] = streams[0].be_read_u8().map_err(|_| Error::CORRUPT_FILE)?;
        }

        if &tag == b"hdmx" || &tag == b"VDMX" {
            streams[0].seek_relative(12).map_err(|_| Error::CORRUPT_FILE)?;
            //eprintln!("Ignoring hdmx/VDMX table -- will be fixed in a future release.\n");
        } else {
            let tag = [tag[0] as _, tag[1] as _, tag[2] as _, tag[3] as _];
            let tbl = out.add_table(&tag);
            streams[0].seek_relative(4).map_err(|_| Error::CORRUPT_FILE)?;
            tbl.offset = streams[0].be_read_u32().map_err(|_| Error::CORRUPT_FILE)? as _;
            let buf_size = streams[0].be_read_u32().map_err(|_| Error::CORRUPT_FILE)?;
            tbl.buf = vec![0u8; buf_size as _].into_boxed_slice();
        }
    }

    let mut glyf: Option<usize> = None;
    let mut loca: Option<usize> = None;
    let mut maxp: Option<usize> = None;
    let mut head: Option<usize> = None;
    let mut hmtx: Option<usize> = None;

    for (i, tbl_0) in out.tables.iter_mut().enumerate() {
        let mut loadTable = true;
        match &tbl_0.tag {
            b"loca" => {
                loca = Some(i);
                loadTable = false;
            },
            b"glyf" => {
                glyf = Some(i);
                loadTable = false;
            },
            b"maxp" => maxp = Some(i),
            b"head" => head = Some(i),
            b"hmtx" => hmtx = Some(i),
            b"hdmx" | b"VDMX" => unreachable!(),
            b"cvt " => {
                unpackCVT(tbl_0, &mut streams[0])?;
                loadTable = false;
            },
            _ => (),
        }

        if loadTable {
            loadTableFromStream(tbl_0, &mut streams[0])?;
            if &tbl_0.tag == b"head" {
                /* kill global checksum; we will be recalcultaing it later. */
                if tbl_0.buf.len() < 12 {
                    return Err(Error::MALFORMED_HEAD_TABLE);
                }
                for i in 8..12 {
                    tbl_0.buf[i] = 0;
                }
            }
        }
    }

    let glyf_loca =
        if glyf.is_some() && loca.is_none() { // TODO: fix with a let chain
            out.add_table(b"loca");
            Some((glyf.unwrap(), out.tables.len() - 1))
        } else if glyf.is_some() && loca.is_some() {
            Some((glyf.unwrap(), loca.unwrap()))
        } else {
            None
        };

    let Some(maxp) = maxp else { return Err(Error::NO_MAXP_TABLE) };
    let Some(head) = head else { return Err(Error::NO_HEAD_TABLE) };
    let Some(____) = hmtx else { return Err(Error::NO_HMTX_TABLE) };

    let mut headData = TTFParseHead(&mut out.tables[head])?;
    let mut maxpData = TTFParseMaxp(&mut out.tables[maxp])?;

    if let Some((glyf, loca)) = glyf_loca {
        populateGlyfAndLoca(&mut out.tables, glyf, loca, &mut headData, &mut maxpData, streams)?;
    }

    Ok(out)
}
