extern "C" {
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn BEReadRestAsU32(
    mut s: *mut Stream,
    mut out: *mut uint32_t,
) -> StreamResult {
    if (*s).pos >= (*s).size {
        return EOT_NOT_ENOUGH_DATA;
    }
    let mut o8: uint8_t = 0;
    let mut o16: uint16_t = 0;
    match (*s).size.wrapping_sub((*s).pos) {
        1 => {
            BEReadU8(s, &raw mut o8);
            *out = (o8 as uint32_t) << 24 as ::core::ffi::c_int;
        }
        2 => {
            BEReadU16(s, &raw mut o16);
            *out = (o16 as uint32_t) << 16 as ::core::ffi::c_int;
        }
        3 => {
            BEReadU24(s, out);
            *out <<= 8 as ::core::ffi::c_int;
        }
        4 | _ => {
            BEReadU32(s, out);
        }
    }
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn constructStream(
    mut buf: *mut uint8_t,
    mut size: ::core::ffi::c_uint,
) -> Stream {
    return constructStream2(buf, size, size);
}
#[no_mangle]
pub unsafe extern "C" fn constructStream2(
    mut buf: *mut uint8_t,
    mut size: ::core::ffi::c_uint,
    mut reserved: ::core::ffi::c_uint,
) -> Stream {
    let mut ret: Stream = Stream {
        buf: ::core::ptr::null_mut::<uint8_t>(),
        size: 0,
        reserved: 0,
        pos: 0,
        bitPos: 0,
    };
    ret.buf = buf;
    ret.size = size;
    ret.pos = 0 as ::core::ffi::c_uint;
    ret.reserved = reserved;
    ret.bitPos = 0 as ::core::ffi::c_uint;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn BEReadU8(
    mut s: *mut Stream,
    mut out: *mut uint8_t,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    if (*s).pos >= (*s).size {
        return EOT_NOT_ENOUGH_DATA;
    }
    let fresh0 = (*s).pos;
    (*s).pos = (*s).pos.wrapping_add(1);
    *out = *(*s).buf.offset(fresh0 as isize);
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn BEReadChar(
    mut s: *mut Stream,
    mut out: *mut ::core::ffi::c_char,
) -> StreamResult {
    return BEReadU8(s, out as *mut uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn BEReadU16(
    mut s: *mut Stream,
    mut out: *mut uint16_t,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    if (*s).pos.wrapping_add(2 as ::core::ffi::c_uint) > (*s).size {
        return EOT_NOT_ENOUGH_DATA;
    }
    *out = ((*(*s).buf.offset((*s).pos as isize) as uint16_t as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | *(*s).buf.offset((*s).pos.wrapping_add(1 as ::core::ffi::c_uint) as isize)
            as uint16_t as ::core::ffi::c_int) as uint16_t;
    (*s).pos = (*s).pos.wrapping_add(2 as ::core::ffi::c_uint);
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn BEReadU24(
    mut s: *mut Stream,
    mut out: *mut uint32_t,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    if (*s).pos.wrapping_add(3 as ::core::ffi::c_uint) > (*s).size {
        return EOT_NOT_ENOUGH_DATA;
    }
    *out = (*(*s).buf.offset((*s).pos as isize) as uint32_t) << 16 as ::core::ffi::c_int
        | (*(*s).buf.offset((*s).pos.wrapping_add(1 as ::core::ffi::c_uint) as isize)
            as uint32_t) << 8 as ::core::ffi::c_int
        | *(*s).buf.offset((*s).pos.wrapping_add(2 as ::core::ffi::c_uint) as isize)
            as uint32_t;
    (*s).pos = (*s).pos.wrapping_add(3 as ::core::ffi::c_uint);
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn BEReadU32(
    mut s: *mut Stream,
    mut out: *mut uint32_t,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    if (*s).pos.wrapping_add(4 as ::core::ffi::c_uint) > (*s).size {
        return EOT_NOT_ENOUGH_DATA;
    }
    *out = (*(*s).buf.offset((*s).pos as isize) as uint32_t) << 24 as ::core::ffi::c_int
        | (*(*s).buf.offset((*s).pos.wrapping_add(1 as ::core::ffi::c_uint) as isize)
            as uint32_t) << 16 as ::core::ffi::c_int
        | (*(*s).buf.offset((*s).pos.wrapping_add(2 as ::core::ffi::c_uint) as isize)
            as uint32_t) << 8 as ::core::ffi::c_int
        | *(*s).buf.offset((*s).pos.wrapping_add(3 as ::core::ffi::c_uint) as isize)
            as uint32_t;
    (*s).pos = (*s).pos.wrapping_add(4 as ::core::ffi::c_uint);
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn BEReadS8(
    mut s: *mut Stream,
    mut out: *mut int8_t,
) -> StreamResult {
    return BEReadU8(s, out as *mut uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn BEReadS16(
    mut s: *mut Stream,
    mut out: *mut int16_t,
) -> StreamResult {
    return BEReadU16(s, out as *mut uint16_t);
}
#[no_mangle]
pub unsafe extern "C" fn BEReadS24(
    mut s: *mut Stream,
    mut out: *mut int32_t,
) -> StreamResult {
    return BEReadU24(s, out as *mut uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn BEReadS32(
    mut s: *mut Stream,
    mut out: *mut int32_t,
) -> StreamResult {
    return BEReadU32(s, out as *mut uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn BEPeekU8(
    mut s: *mut Stream,
    mut out: *mut uint8_t,
) -> StreamResult {
    let mut ret1: StreamResult = BEReadU8(s, out);
    (*s).pos = (*s).pos.wrapping_sub(1);
    return ret1;
}
#[no_mangle]
pub unsafe extern "C" fn seekRelative(
    mut s: *mut Stream,
    mut offset: ::core::ffi::c_int,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    let mut newPos: ::core::ffi::c_int = (*s)
        .pos
        .wrapping_add(offset as ::core::ffi::c_uint) as ::core::ffi::c_int;
    if newPos < 0 as ::core::ffi::c_int {
        return EOT_NEGATIVE_SEEK;
    }
    if newPos as ::core::ffi::c_uint > (*s).size {
        return EOT_SEEK_PAST_EOS;
    }
    (*s).pos = newPos as ::core::ffi::c_uint;
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn seekRelativeThroughReserve(
    mut s: *mut Stream,
    mut offset: ::core::ffi::c_int,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    let mut newPos: ::core::ffi::c_int = (*s)
        .pos
        .wrapping_add(offset as ::core::ffi::c_uint) as ::core::ffi::c_int;
    if newPos < 0 as ::core::ffi::c_int {
        return EOT_NEGATIVE_SEEK;
    }
    if newPos as ::core::ffi::c_uint > (*s).reserved {
        return EOT_SEEK_PAST_EOS;
    }
    (*s).pos = newPos as ::core::ffi::c_uint;
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn seekAbsolute(
    mut s: *mut Stream,
    mut pos: ::core::ffi::c_uint,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    if pos > (*s).size {
        return EOT_SEEK_PAST_EOS;
    }
    (*s).pos = pos;
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn seekAbsoluteThroughReserve(
    mut s: *mut Stream,
    mut pos: ::core::ffi::c_uint,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    if pos > (*s).reserved {
        return EOT_SEEK_PAST_EOS;
    }
    (*s).pos = pos;
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn reserve(
    mut s: *mut Stream,
    mut toReserve: ::core::ffi::c_uint,
) -> StreamResult {
    if (*s).reserved >= toReserve {
        return EOT_STREAM_OK;
    }
    let mut newBuf: *mut uint8_t = realloc(
        (*s).buf as *mut ::core::ffi::c_void,
        toReserve as size_t,
    ) as *mut uint8_t;
    if newBuf.is_null() {
        return EOT_CANT_ALLOCATE_MEMORY_FOR_STREAM;
    }
    (*s).buf = newBuf;
    (*s).reserved = toReserve;
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn BEWriteU8(
    mut s: *mut Stream,
    mut in_0: uint8_t,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    if (*s).pos.wrapping_add(1 as ::core::ffi::c_uint) > (*s).reserved {
        return EOT_OUT_OF_RESERVED_SPACE;
    }
    let fresh1 = (*s).pos;
    (*s).pos = (*s).pos.wrapping_add(1);
    *(*s).buf.offset(fresh1 as isize) = in_0;
    if (*s).pos > (*s).size {
        (*s).size = (*s).pos;
    }
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn BEWriteU16(
    mut s: *mut Stream,
    mut in_0: uint16_t,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    if (*s).pos.wrapping_add(2 as ::core::ffi::c_uint) > (*s).reserved {
        return EOT_OUT_OF_RESERVED_SPACE;
    }
    let fresh2 = (*s).pos;
    (*s).pos = (*s).pos.wrapping_add(1);
    *(*s).buf.offset(fresh2 as isize) = (in_0 as ::core::ffi::c_int
        >> 8 as ::core::ffi::c_int) as uint8_t;
    let fresh3 = (*s).pos;
    (*s).pos = (*s).pos.wrapping_add(1);
    *(*s).buf.offset(fresh3 as isize) = (in_0 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int) as uint8_t;
    if (*s).pos > (*s).size {
        (*s).size = (*s).pos;
    }
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn BEWriteU24(
    mut s: *mut Stream,
    mut in_0: uint32_t,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    if in_0 & 0xff000000 as uint32_t != 0 {
        return EOT_VALUE_OUT_OF_BOUNDS;
    }
    if (*s).pos.wrapping_add(3 as ::core::ffi::c_uint) > (*s).reserved {
        return EOT_OUT_OF_RESERVED_SPACE;
    }
    let fresh4 = (*s).pos;
    (*s).pos = (*s).pos.wrapping_add(1);
    *(*s).buf.offset(fresh4 as isize) = (in_0 >> 16 as ::core::ffi::c_int) as uint8_t;
    let fresh5 = (*s).pos;
    (*s).pos = (*s).pos.wrapping_add(1);
    *(*s).buf.offset(fresh5 as isize) = (in_0 >> 8 as ::core::ffi::c_int
        & 0xff as uint32_t) as uint8_t;
    let fresh6 = (*s).pos;
    (*s).pos = (*s).pos.wrapping_add(1);
    *(*s).buf.offset(fresh6 as isize) = (in_0 & 0xff as uint32_t) as uint8_t;
    if (*s).pos > (*s).size {
        (*s).size = (*s).pos;
    }
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn BEWriteU32(
    mut s: *mut Stream,
    mut in_0: uint32_t,
) -> StreamResult {
    if (*s).bitPos != 0 as ::core::ffi::c_uint {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    if (*s).pos.wrapping_add(4 as ::core::ffi::c_uint) > (*s).reserved {
        return EOT_OUT_OF_RESERVED_SPACE;
    }
    let fresh7 = (*s).pos;
    (*s).pos = (*s).pos.wrapping_add(1);
    *(*s).buf.offset(fresh7 as isize) = (in_0 >> 24 as ::core::ffi::c_int) as uint8_t;
    let fresh8 = (*s).pos;
    (*s).pos = (*s).pos.wrapping_add(1);
    *(*s).buf.offset(fresh8 as isize) = (in_0 >> 16 as ::core::ffi::c_int
        & 0xff as uint32_t) as uint8_t;
    let fresh9 = (*s).pos;
    (*s).pos = (*s).pos.wrapping_add(1);
    *(*s).buf.offset(fresh9 as isize) = (in_0 >> 8 as ::core::ffi::c_int
        & 0xff as uint32_t) as uint8_t;
    let fresh10 = (*s).pos;
    (*s).pos = (*s).pos.wrapping_add(1);
    *(*s).buf.offset(fresh10 as isize) = (in_0 & 0xff as uint32_t) as uint8_t;
    if (*s).pos > (*s).size {
        (*s).size = (*s).pos;
    }
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn BEWriteS16(
    mut s: *mut Stream,
    mut in_0: int16_t,
) -> StreamResult {
    return BEWriteU16(s, in_0 as uint16_t);
}
#[no_mangle]
pub unsafe extern "C" fn streamCopy(
    mut sIn: *mut Stream,
    mut sOut: *mut Stream,
    mut length: ::core::ffi::c_uint,
) -> StreamResult {
    if (*sIn).bitPos != 0 as ::core::ffi::c_uint
        || (*sOut).bitPos != 0 as ::core::ffi::c_uint
    {
        return EOT_OFF_BYTE_BOUNDARY;
    }
    if (*sIn).pos.wrapping_add(length) > (*sIn).size {
        return EOT_NOT_ENOUGH_DATA;
    }
    if (*sOut).pos.wrapping_add(length) > (*sOut).reserved {
        return EOT_OUT_OF_RESERVED_SPACE;
    }
    memcpy(
        (*sOut).buf.offset((*sOut).pos as isize) as *mut ::core::ffi::c_void,
        (*sIn).buf.offset((*sIn).pos as isize) as *const ::core::ffi::c_void,
        length as size_t,
    );
    (*sOut).pos = (*sOut).pos.wrapping_add(length);
    (*sIn).pos = (*sIn).pos.wrapping_add(length);
    if (*sOut).pos > (*sOut).size {
        (*sOut).size = (*sOut).pos;
    }
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn readNBits(
    mut s: *mut Stream,
    mut out: *mut uint32_t,
    mut n: ::core::ffi::c_uint,
) -> StreamResult {
    let masks: [uint8_t; 8] = [
        0x80 as ::core::ffi::c_int as uint8_t,
        0x40 as ::core::ffi::c_int as uint8_t,
        0x20 as ::core::ffi::c_int as uint8_t,
        0x10 as ::core::ffi::c_int as uint8_t,
        0x8 as ::core::ffi::c_int as uint8_t,
        0x4 as ::core::ffi::c_int as uint8_t,
        0x2 as ::core::ffi::c_int as uint8_t,
        0x1 as ::core::ffi::c_int as uint8_t,
    ];
    if n > 32 as ::core::ffi::c_uint {
        return EOT_VALUE_OUT_OF_BOUNDS;
    }
    *out = 0 as uint32_t;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < n {
        if (*s).pos >= (*s).size {
            return EOT_NOT_ENOUGH_DATA;
        }
        let mut bitSet: bool = *(*s).buf.offset((*s).pos as isize) as ::core::ffi::c_int
            & masks[(*s).bitPos as usize] as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int;
        *out
            |= ((if bitSet as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) << n.wrapping_sub(i).wrapping_sub(1 as ::core::ffi::c_uint)) as uint32_t;
        (*s).bitPos = (*s).bitPos.wrapping_add(1);
        if (*s).bitPos == 8 as ::core::ffi::c_uint {
            (*s).bitPos = 0 as ::core::ffi::c_uint;
            (*s).pos = (*s).pos.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return EOT_STREAM_OK;
}
#[no_mangle]
pub unsafe extern "C" fn BEcheckSum32(
    mut s: *mut Stream,
    mut out: *mut uint32_t,
    mut beginPos: ::core::ffi::c_uint,
    mut endPos: ::core::ffi::c_uint,
) -> StreamResult {
    if beginPos > endPos {
        return EOT_VALUE_OUT_OF_BOUNDS;
    }
    if endPos > (*s).size {
        return EOT_NOT_ENOUGH_DATA;
    }
    let mut slice: Stream = constructStream(
        (*s).buf.offset(beginPos as isize),
        endPos.wrapping_sub(beginPos),
    );
    let mut sResult: StreamResult = EOT_STREAM_OK;
    *out = 0 as uint32_t;
    while sResult as ::core::ffi::c_uint
        == EOT_STREAM_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut chunk: uint32_t = 0;
        sResult = BEReadRestAsU32(&raw mut slice, &raw mut chunk);
        *out = (*out).wrapping_add(chunk);
    }
    if sResult as ::core::ffi::c_uint
        == EOT_NOT_ENOUGH_DATA as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        sResult = EOT_STREAM_OK;
    }
    return sResult;
}
