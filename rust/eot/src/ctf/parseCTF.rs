use core::mem::size_of;

use crate::{
    core::*,
    ctf::{SFNTContainer::*, parseTTF::*},
    stream::{Error as StreamError, Stream},
    triplet_encodings::*,
};

#[derive(Copy, Clone)]
pub struct SFNTOffsetTable {
    pub scalar_type: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
enum DpiTypeRead {
    Short = 1,
    Byte = 0,
}

fn parse_offset_table(s: &mut Stream) -> Result<SFNTOffsetTable, StreamError> {
    let scalar_type = s.be_read_u32()?;
    let num_tables = s.be_read_u16()?;
    let search_range = s.be_read_u16()?;
    let entry_selector = s.be_read_u16()?;
    let range_shift = s.be_read_u16()?;
    Ok(SFNTOffsetTable {
        scalar_type,
        num_tables,
        search_range,
        entry_selector,
        range_shift,
    })
}

fn _ucvt_rd_val(s_in: &mut Stream, last_value: &mut i16) -> Result<(), StreamError> {
    let code = s_in.be_read_u8()?;
    let val: i16;

    if code >= 248 {
        let b2 = s_in.be_read_u8()?;
        val = 238 * ((code as i32 - 247_i32) as i16) + b2 as i16;
    } else if code >= 239 {
        let b2 = s_in.be_read_u8()?;
        val = -(238 * (code as i16 - 239) + b2 as i16);
    } else if code == 238 {
        val = s_in.be_read_i16()?;
    } else {
        val = code as _;
    }

    // The CVT table in CTF format is set up so that this does the right thing even if it
    // overflows.
    *last_value = (*last_value).wrapping_add(val);
    // Unless someone tries to run this code on some horrible system that doesn't use twos
    // complement...
    Ok(())
}

pub fn unpack_cvt(out: &mut SFNTTable, s_in: &mut Stream) -> Result<(), Error> {
    s_in.seek_absolute(out.offset as _).map_err(|_| Error::CORRUPT_FILE)?;
    let table_length = s_in.be_read_u16().map_err(|_| Error::CORRUPT_FILE)?;
    let mut s_out = Stream::new2(0, table_length as usize * 2);
    let mut last_value = 0i16;
    for _ in 0..table_length {
        _ucvt_rd_val(s_in, &mut last_value).map_err(|_| Error::CORRUPT_FILE)?;
        s_out.be_write_i16(last_value).map_err(|_| Error::LOGIC_ERROR)?;
    }
    let Stream { buf: s_out_buf, .. } = s_out;
    out.buf = s_out_buf.into_boxed_slice();
    Ok(())
}

// http://www.w3.org/Submission/MTX/#id_255USHORT
fn read_255_ushort(s_in: &mut Stream) -> Result<u16, StreamError> {
    Ok(match s_in.be_read_u8()? {
        253 => s_in.be_read_u16()?,
        255 => 253 + s_in.be_read_u8()? as u16,
        254 => 506 + s_in.be_read_u8()? as u16,
        val => val as _,
    })
}

// http://www.w3.org/Submission/MTX/#id_255SHORT
fn read_255_short(s_in: &mut Stream) -> Result<i16, StreamError> {
    let mut code: u8 = s_in.be_read_u8()?;
    if code == 253 {
        return s_in.be_read_i16();
    }

    let mut sign = 1i16;
    if code == 250 {
        sign = -1;
        code = s_in.be_read_u8()?;
    }

    let out = match code {
        255 => 250 + s_in.be_read_u8()? as i16,
        254 => (250 * 2) + s_in.be_read_u8()? as i16,
        _ => code as i16,
    };

    Ok(out * sign)
}

fn _dpi_dump2(
    out: &mut Stream, last_read: &mut DpiTypeRead, type_last_read_count: &mut u32, data: &mut Vec<i16>,
    data_index: &mut u32,
) -> Result<(), StreamError> {
    if *type_last_read_count > 0 {
        if *type_last_read_count < 8 {
            let op: u8 = (if *last_read == DpiTypeRead::Byte { PUSHB } else { PUSHW }) as u8
                | (*type_last_read_count).wrapping_sub(1) as u8;
            out.be_write_u8(op)?;
        } else {
            let op: u8 = if *last_read == DpiTypeRead::Byte { NPUSHB } else { NPUSHW } as u8;
            out.be_write_u8(op)?;
            out.be_write_u8(*type_last_read_count as u8)?;
        }

        for i in 0..*type_last_read_count {
            if *last_read == DpiTypeRead::Byte {
                out.be_write_u8(data[(*data_index - *type_last_read_count + i) as usize] as _)?;
            } else {
                out.be_write_i16(data[(*data_index - *type_last_read_count + i) as usize])?;
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
    value: i16, out: &mut Stream, last_read: &mut DpiTypeRead, type_last_read_count: &mut u32,
    data: &mut Vec<i16>, data_index: &mut u32,
) -> Result<(), StreamError> {
    let new_type = if (0..256).contains(&value) { DpiTypeRead::Byte } else { DpiTypeRead::Short };
    if new_type != *last_read || *type_last_read_count == 255 {
        _dpi_dump2(out, last_read, type_last_read_count, data, data_index)?;
        *last_read = new_type;
        *type_last_read_count = 0 as ::core::ffi::c_uint;
    }
    let fresh0 = *data_index;
    *data_index = (*data_index).wrapping_add(1);
    data[fresh0 as usize] = value;
    *type_last_read_count = (*type_last_read_count).wrapping_add(1);
    Ok(())
}

// http://www.w3.org/Submission/MTX/#HopCodes
fn decode_push_instructions(s_in: &mut Stream, s_out: &mut Stream, push_count: u32) -> Result<(), Error> {
    let mut remaining = push_count;
    let mut type_last_read = DpiTypeRead::Byte;
    let mut type_last_read_count = 0u32;
    let mut data_index = 0u32;
    let mut data = vec![0i16; push_count as _];

    while remaining > 0 {
        let code = s_in.be_peek_u8().map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
        match code {
            0xFB => {
                // A B 0xFB C -> A B A C A
                if remaining < 3 || data_index < 2 {
                    return Err(Error::CORRUPT_HOPCODE_DATA);
                }
                remaining -= 3;
                let prev = data[(data_index - 2) as usize];
                // code = s_in.be_read_u8()?;
                _ = s_in.be_read_u8()?;
                _dpi_put2(prev, s_out, &mut type_last_read, &mut type_last_read_count, &mut data, &mut data_index)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                let val = read_255_short(s_in).map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(val, s_out, &mut type_last_read, &mut type_last_read_count, &mut data, &mut data_index)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(prev, s_out, &mut type_last_read, &mut type_last_read_count, &mut data, &mut data_index)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
            }
            0xFC => {
                if remaining < 5 || data_index < 2 {
                    return Err(Error::CORRUPT_HOPCODE_DATA);
                }
                remaining -= 5;
                let prev = data[(data_index - 2) as usize];
                // code = s_in.be_read_u8()?;
                _ = s_in.be_read_u8()?;
                _dpi_put2(prev, s_out, &mut type_last_read, &mut type_last_read_count, &mut data, &mut data_index)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                let mut val = read_255_short(s_in).map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(val, s_out, &mut type_last_read, &mut type_last_read_count, &mut data, &mut data_index)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(prev, s_out, &mut type_last_read, &mut type_last_read_count, &mut data, &mut data_index)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                val = read_255_short(s_in).map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(val, s_out, &mut type_last_read, &mut type_last_read_count, &mut data, &mut data_index)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(prev, s_out, &mut type_last_read, &mut type_last_read_count, &mut data, &mut data_index)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
            }
            _ => {
                let val = read_255_short(s_in).map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                _dpi_put2(val, s_out, &mut type_last_read, &mut type_last_read_count, &mut data, &mut data_index)
                    .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
                remaining -= 1;
            }
        }
    }

    _dpi_dump2(s_out, &mut type_last_read, &mut type_last_read_count, &mut data, &mut data_index)
        .map_err(|_| Error::SECOND_STREAM_INCOMPLETE)?;
    Ok(())
}

fn _dsg_make_flags(x: i16, y: i16, on_curve: bool, first_time: bool) -> u8 {
    const FLG_ON_CURVE: u8 = 0x1;
    const FLG_X_SHORT: u8 = 0x2;
    const FLG_Y_SHORT: u8 = 0x4;
    const FLG_X_SAME: u8 = 0x10;
    const FLG_Y_SAME: u8 = 0x20;

    let mut ret: u8 = 0_u8;
    if on_curve {
        ret |= FLG_ON_CURVE;
    }

    if !first_time && x == 0 {
        ret |= FLG_X_SAME;
    } else if -256 < x && x < 0 {
        ret |= FLG_X_SHORT;
    } else if (0..256).contains(&x) {
        ret |= FLG_X_SHORT | FLG_X_SAME;
    }

    if !first_time && y == 0 {
        ret |= FLG_Y_SAME;
    } else if -256 < y && y < 0 {
        ret |= FLG_Y_SHORT;
    } else if (0..256).contains(&y) {
        ret |= FLG_Y_SHORT | FLG_Y_SAME;
    }

    ret
}

fn decode_simple_glyph(
    num_contours: i16, streams: &mut [Stream], out: &mut Stream, calculate_bounding_box: bool, mut min_x: i16,
    mut min_y: i16, mut max_x: i16, mut max_y: i16,
) -> Result<(), Error> {
    if num_contours == 0 {
        return Ok(());
    }

    let mut bounding_box_location = None;

    out.be_write_i16(num_contours).map_err(|_| Error::CORRUPT_FILE)?;

    if calculate_bounding_box {
        bounding_box_location = Some(out.pos);
        out.seek_relative_through_reserve(4 * size_of::<i16>() as isize)
            .map_err(|_| Error::CORRUPT_FILE)?;
        min_x = i16::MAX;
        min_y = i16::MAX;
        max_x = i16::MIN;
        max_y = i16::MIN;
    } else {
        // FIXME: why are we returning CORRUPT_FILE and not LOGIC_ERROR here?
        out.be_write_i16(min_x).map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(min_y).map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(max_x).map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(max_y).map_err(|_| Error::CORRUPT_FILE)?;
    }

    let mut total_points: usize = 0;
    for i in 0..num_contours {
        if i == 0 {
            total_points = 1;
        }
        let points_in_contour = read_255_ushort(&mut streams[0]).map_err(|_| Error::CORRUPT_FILE)?;
        total_points += points_in_contour as usize;
        out.be_write_i16((total_points - 1) as _).map_err(|_| Error::CORRUPT_FILE)?;
    }

    let mut flags = vec![0u8; total_points as _];
    let mut x_coords = vec![0i16; total_points as _];
    let mut y_coords = vec![0i16; total_points as _];

    // Read X-Y coordinates in shitty format described here:
    // http://www.w3.org/Submission/MTX/#TripletEncoding
    // First flags and then actual coordinates.

    for i in 0..total_points {
        flags[i] = streams[0].be_read_u8().map_err(|_| Error::CORRUPT_FILE)?;
    }

    let mut curr_x = 0u32;
    let mut curr_y = 0u32;

    for i in 0..total_points {
        let enc = tripletEncodings[(flags[i] & 0x7F) as usize];
        let more_bytes = (enc.byte_count - 1) as usize;

        if streams[0].pos + more_bytes > streams[0].buf.len() {
            return Err(Error::CORRUPT_FILE);
        }

        // FIXME: stupid copying, probably don't even need a stream for this.
        let mut coords = Stream::new(0);
        coords.buf = (&streams[0].buf[streams[0].pos..streams[0].pos + more_bytes]).into();

        let dx: u32 = coords.read_n_bits(enc.x_bits)?; // logic error
        let dy: u32 = coords.read_n_bits(enc.y_bits)?; // logic error
        if coords.pos != coords.buf.len() || coords.bit_pos != 0 {
            return Err(Error::LOGIC_ERROR);
        }
        streams[0].seek_relative(coords.buf.len() as _)?; // logic error

        x_coords[i] = (enc.x_sign * (dx + enc.delta_x) as i32) as _;
        y_coords[i] = (enc.y_sign * (dy + enc.delta_y) as i32) as _;

        curr_x = curr_x.wrapping_add(x_coords[i] as i32 as u32);
        curr_y = curr_y.wrapping_add(y_coords[i] as i32 as u32);

        min_x = min_x.min(curr_x as i16);
        max_x = max_x.max(curr_x as i16);
        min_y = min_y.min(curr_y as i16);
        max_y = max_y.max(curr_y as i16);
    }

    // Coordinates are known now, but we need to handle instructions before they can be output.

    // advance past the code size output
    let code_size_location = out.pos as u32;
    out.seek_relative_through_reserve(size_of::<u16>() as _)
        .map_err(|_| Error::CORRUPT_FILE)?;

    // decode the push instructions for the glyph
    let push_count = read_255_ushort(&mut streams[0]).map_err(|_| Error::CORRUPT_FILE)? as u16;
    decode_push_instructions(&mut streams[1], out, push_count as _)?;
    let code_size = read_255_ushort(&mut streams[0]).map_err(|_| Error::CORRUPT_FILE)?;

    // copy over the rest of the instructions for the glyph
    for _ in 0..code_size {
        out.be_write_u8(streams[2].be_read_u8().map_err(|_| Error::CORRUPT_FILE)?)
            .map_err(|_| Error::CORRUPT_FILE)?;
    }

    // the below will be zero if we didn't go through the if (num_contours > 0) block.
    let unpacked_code_size = out.pos as u32 - (code_size_location + size_of::<u16>() as u32);
    // FIXME: Figure out if there is a huge savings from using the 'repeat' flag
    // and if so, use it. (but I kinda doubt there is.)
    for i in 0..total_points {
        let out_flags = _dsg_make_flags(x_coords[i], y_coords[i], flags[i] & 0x80 == 0, i == 0);
        out.be_write_u8(out_flags).map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
    }

    for i in 0..total_points {
        let mut x = x_coords[i];
        if i == 0 || x != 0 {
            if -256 < x && x < 0 {
                x *= -1;
            }
            if (0..256).contains(&x) {
                out.be_write_u8(x as _).map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
            } else {
                out.be_write_i16(x).map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
            }
        }
    }

    for i in 0..total_points {
        let mut y = y_coords[i];
        if i == 0 || y != 0 {
            if -256 < y && y < 0 {
                y *= -1;
            }
            if (0..256).contains(&y) {
                out.be_write_u8(y as _).map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
            } else {
                out.be_write_i16(y).map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
            }
        }
    }

    let curr_pos = out.pos;
    out.seek_absolute_through_reserve(code_size_location as _)
        .map_err(|_| Error::CORRUPT_FILE)?;
    out.be_write_u16(unpacked_code_size as _).map_err(|_| Error::CORRUPT_FILE)?;
    out.seek_absolute_through_reserve(curr_pos).map_err(|_| Error::CORRUPT_FILE)?;

    if calculate_bounding_box {
        let end_pos = out.pos;
        out.seek_absolute_through_reserve(bounding_box_location.unwrap())
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(min_x).map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(min_y).map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(max_x).map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_i16(max_y).map_err(|_| Error::CORRUPT_FILE)?;
        out.seek_absolute_through_reserve(end_pos as _).map_err(|_| Error::CORRUPT_FILE)?;
    }

    Ok(())
}

fn decode_composite_glyph(streams: &mut [Stream], out: &mut Stream) -> Result<(), Error> {
    // we don't need to interpret very much here, just the flags to know how much to pass along
    // into the output.
    const FLG_ARGS_WORDS: u16 = 0x1;
    const FLG_HAVE_SCALE: u16 = 0x8;
    const FLG_MORE_COMPONENTS: u16 = 0x20;
    const FLG_HAVE_XY_SCALE: u16 = 0x40;
    const FLG_HAVE_2_BY_2: u16 = 0x80;
    const FLG_HAVE_INSTR: u16 = 0x100;

    out.be_write_i16(-1).map_err(|_| Error::CORRUPT_FILE)?;
    let min_x = streams[0].be_read_i16().map_err(|_| Error::CORRUPT_FILE)?;
    let min_y = streams[0].be_read_i16().map_err(|_| Error::CORRUPT_FILE)?;
    let max_x = streams[0].be_read_i16().map_err(|_| Error::CORRUPT_FILE)?;
    let max_y = streams[0].be_read_i16().map_err(|_| Error::CORRUPT_FILE)?;
    out.be_write_i16(min_x).map_err(|_| Error::CORRUPT_FILE)?;
    out.be_write_i16(min_y).map_err(|_| Error::CORRUPT_FILE)?;
    out.be_write_i16(max_x).map_err(|_| Error::CORRUPT_FILE)?;
    out.be_write_i16(max_y).map_err(|_| Error::CORRUPT_FILE)?;

    let mut flags: u16;
    loop {
        flags = streams[0].be_read_u16().map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_u16(flags).map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_u8(streams[0].be_read_u8().map_err(|_| Error::CORRUPT_FILE)?)
            .map_err(|_| Error::CORRUPT_FILE)?;
        out.be_write_u8(streams[0].be_read_u8().map_err(|_| Error::CORRUPT_FILE)?)
            .map_err(|_| Error::CORRUPT_FILE)?;

        let args_length = if flags & FLG_ARGS_WORDS != 0 { 4 } else { 2 };
        for _ in 0..args_length {
            out.be_write_u8(streams[0].be_read_u8().map_err(|_| Error::CORRUPT_FILE)?)
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
            out.be_write_u8(streams[0].be_read_u8().map_err(|_| Error::CORRUPT_FILE)?)
                .map_err(|_| Error::CORRUPT_FILE)?;
        }

        if flags & FLG_MORE_COMPONENTS == 0 {
            break;
        }
    }

    if flags & FLG_HAVE_INSTR != 0 {
        // https://learn.microsoft.com/en-us/typography/opentype/spec/glyf
        // uint16 numInstr
        let num_instr_location = out.pos;
        out.seek_relative_through_reserve(2).map_err(|_| Error::CORRUPT_FILE)?;

        // decode the push instructions for the glyph
        let push_count = read_255_ushort(&mut streams[0]).map_err(|_| Error::CORRUPT_FILE)? as u16;
        decode_push_instructions(&mut streams[1], out, push_count as _)?;

        // copy over the rest of the instructions for the glyph
        let code_size = read_255_ushort(&mut streams[0]).map_err(|_| Error::CORRUPT_FILE)?;
        for _ in 0..code_size {
            out.be_write_u8(streams[2].be_read_u8().map_err(|_| Error::CORRUPT_FILE)?)
                .map_err(|_| Error::CORRUPT_FILE)?;
        }

        let num_instr: u16 = ((out.pos as i32) - (num_instr_location + 2) as i32) as _;
        if num_instr > 0 {
            let curr_pos = out.pos;
            out.seek_absolute_through_reserve(num_instr_location as _)
                .map_err(|_| Error::CORRUPT_FILE)?;
            out.be_write_u16(num_instr).map_err(|_| Error::CORRUPT_FILE)?;
            out.seek_absolute_through_reserve(curr_pos).map_err(|_| Error::CORRUPT_FILE)?;
        }
    }

    Ok(())
}

fn decode_glyph(streams: &mut [Stream], out: &mut Stream) -> Result<(), Error> {
    let in_0 = &mut streams[0];
    let mut calculate_bounding_box: bool = false;

    let num_contours = in_0.be_read_i16().map_err(|_| Error::CORRUPT_FILE)?;
    if num_contours < 0 {
        decode_composite_glyph(streams, out)?;
    } else {
        let (x_min, y_min, x_max, y_max);
        let actual_num_contours;
        if num_contours == 0x7fff {
            actual_num_contours = in_0.be_read_i16().map_err(|_| Error::CORRUPT_FILE)?;
            x_min = in_0.be_read_i16()?;
            y_min = in_0.be_read_i16()?;
            x_max = in_0.be_read_i16()?;
            y_max = in_0.be_read_i16()?;
        } else {
            calculate_bounding_box = true;
            (x_min, y_min, x_max, y_max) = (0, 0, 0, 0);
            actual_num_contours = num_contours;
        }
        decode_simple_glyph(actual_num_contours, streams, out, calculate_bounding_box, x_min, y_min, x_max, y_max)?;
    }

    Ok(())
}

// https://developer.apple.com/fonts/TTRefMan/RM06/Chap6glyf.html
// http://www.w3.org/Submission/MTX/#CTFGlyph
fn populate_glyf_and_loca(
    tables: &mut [SFNTTable], glyf: usize, loca: usize, head_data: &mut TtfHeadData,
    maxp_data: &mut TtfMaxpData, streams: &mut [Stream],
) -> Result<(), Error> {
    let sctf = &mut streams[0];
    sctf.seek_absolute(tables[glyf].offset as _)?;

    let overran_allocated_space: bool = false;
    let not_enough_glyphs: bool = false;

    streams[1].seek_absolute(0)?;
    streams[2].seek_absolute(0)?;

    let max_simple_glyph_size = 10
        + 2 * (maxp_data.max_contours as u32)
        + 2
        + (maxp_data.max_size_of_instructions as u32)
        + (maxp_data.max_points as u32 * 5);
    let max_compound_glyph_size = 26 + (maxp_data.max_size_of_instructions as u32);
    let max_glyph_size = max_simple_glyph_size.max(max_compound_glyph_size);
    let max_table_size = (maxp_data.num_glyphs as u32) * max_glyph_size;
    let is_short_loca = head_data.index_to_loc_format == 0;

    let mut s_out = Stream::new2(0, max_table_size as _);
    let mut s_loca_out = Stream::new2(0, 0);

    if is_short_loca {
        s_loca_out.buf.reserve(2 * (maxp_data.num_glyphs + 1) as usize);
        s_loca_out.be_write_u16(0).map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
    } else {
        s_loca_out.buf.reserve(4 * (maxp_data.num_glyphs + 1) as usize);
        s_loca_out.be_write_u32(0).map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
    }

    for _ in 0..maxp_data.num_glyphs {
        // decode a glyph outline
        decode_glyph(streams, &mut s_out)?;

        // do padding
        if !s_out.pos.is_multiple_of(2) {
            s_out.be_write_u8(0)?;
        }

        // add an entry to the location table
        if is_short_loca {
            s_loca_out
                .be_write_u16((s_out.pos / 2) as _)
                .map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
        } else {
            s_loca_out
                .be_write_u32(s_out.pos as _)
                .map_err(|_| Error::UNKNOWN_BUFFER_WRITE_ERROR)?;
        }
    }

    tables[glyf].buf = s_out.buf.into_boxed_slice();
    tables[loca].buf = s_loca_out.buf.into_boxed_slice();

    if not_enough_glyphs {
        return Err(Error::WARN_NOT_ENOUGH_GLYPHS);
    }

    if overran_allocated_space {
        return Err(Error::WARN_NOT_ENOUGH_SPACE_RESERVED);
    }

    Ok(())
}

pub fn parse_ctf(streams: &mut [Stream]) -> Result<SFNTContainer, Error> {
    let offset_table = parse_offset_table(&mut streams[0]).map_err(|_| Error::CORRUPT_FILE)?;
    let mut out = SFNTContainer::new(offset_table.num_tables as usize);

    for _ in 0..offset_table.num_tables {
        let mut tag = [0u8; 4];
        for j in 0..4 {
            tag[j] = streams[0].be_read_u8().map_err(|_| Error::CORRUPT_FILE)?;
        }

        if &tag == b"hdmx" || &tag == b"VDMX" {
            streams[0].seek_relative(12).map_err(|_| Error::CORRUPT_FILE)?;
            // eprintln!("Ignoring hdmx/VDMX table -- will be fixed in a future release.\n");
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
        let mut load_table = true;
        match &tbl_0.tag {
            b"loca" => {
                loca = Some(i);
                load_table = false;
            }
            b"glyf" => {
                glyf = Some(i);
                load_table = false;
            }
            b"maxp" => maxp = Some(i),
            b"head" => head = Some(i),
            b"hmtx" => hmtx = Some(i),
            b"hdmx" | b"VDMX" => unreachable!(),
            b"cvt " => {
                unpack_cvt(tbl_0, &mut streams[0])?;
                load_table = false;
            }
            _ => (),
        }

        if load_table {
            load_table_from_stream(tbl_0, &mut streams[0])?;
            if &tbl_0.tag == b"head" {
                // kill global checksum; we will be recalcultaing it later.
                if tbl_0.buf.len() < 12 {
                    return Err(Error::MALFORMED_HEAD_TABLE);
                }
                for i in 8..12 {
                    tbl_0.buf[i] = 0;
                }
            }
        }
    }

    let glyf_loca = if glyf.is_some() && loca.is_none() {
        // TODO: fix with a let chain
        out.add_table(b"loca");
        Some((glyf.unwrap(), out.tables.len() - 1))
    } else if glyf.is_some() && loca.is_some() {
        Some((glyf.unwrap(), loca.unwrap()))
    } else {
        None
    };

    let Some(maxp) = maxp else {
        return Err(Error::NO_MAXP_TABLE);
    };
    let Some(head) = head else {
        return Err(Error::NO_HEAD_TABLE);
    };
    let Some(____) = hmtx else {
        return Err(Error::NO_HMTX_TABLE);
    };

    let mut head_data = ttf_parse_head(&out.tables[head])?;
    let mut maxp_data = ttf_parse_maxp(&out.tables[maxp])?;

    if let Some((glyf, loca)) = glyf_loca {
        populate_glyf_and_loca(&mut out.tables, glyf, loca, &mut head_data, &mut maxp_data, streams)?;
    }

    Ok(out)
}
