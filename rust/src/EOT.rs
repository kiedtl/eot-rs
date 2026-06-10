use crate::src::core::*;

extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut EDITING_MASK: uint16_t = 0x8 as uint16_t;
#[no_mangle]
pub unsafe extern "C" fn EOTreadU32LE(mut bytes: *const uint8_t) -> uint32_t {
    return *bytes.offset(0 as ::core::ffi::c_int as isize) as uint32_t
        | (*bytes.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*bytes.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*bytes.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
            << 24 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EOTreadU16LE(mut bytes: *const uint8_t) -> uint16_t {
    return (*bytes.offset(0 as ::core::ffi::c_int as isize) as uint16_t
        as ::core::ffi::c_int
        | (*bytes.offset(1 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn EOTgetMetadataLength(
    mut bytes: *const uint8_t,
) -> ::core::ffi::c_uint {
    let mut totalLength: uint32_t = EOTreadU32LE(bytes);
    let mut fontLength: uint32_t = EOTreadU32LE(
        bytes.offset(4 as ::core::ffi::c_int as isize),
    );
    return (totalLength as ::core::ffi::c_uint)
        .wrapping_sub(fontLength as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn EOTgetString(
    mut scanner: *mut *const uint8_t,
    mut begin: *const uint8_t,
    mut bytesLength: ::core::ffi::c_uint,
    mut size: *mut uint16_t,
    mut string: *mut *mut uint16_t,
) -> EOTError {
    if !(*string).is_null() {
        free(*string as *mut ::core::ffi::c_void);
    }
    *string = ::core::ptr::null_mut::<uint16_t>();
    if (*scanner).offset_from(begin) as ::core::ffi::c_long + 2 as ::core::ffi::c_long
        > bytesLength as ::core::ffi::c_long
    {
        return EOT_INSUFFICIENT_BYTES;
    }
    *size = EOTreadU16LE(*scanner);
    *scanner = (*scanner).offset(2 as ::core::ffi::c_int as isize);
    if *size as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        return EOT_BOGUS_STRING_SIZE;
    }
    if (*scanner).offset_from(begin) as ::core::ffi::c_long
        + *size as ::core::ffi::c_long > bytesLength as ::core::ffi::c_long
    {
        return EOT_INSUFFICIENT_BYTES;
    }
    if *size as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        *string = malloc(*size as size_t) as *mut uint16_t;
        if (*string).is_null() {
            return EOT_CANT_ALLOCATE_MEMORY;
        }
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while i
            < (*size as ::core::ffi::c_int / 2 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
        {
            *(*string).offset(i as isize) = EOTreadU16LE(*scanner);
            *scanner = (*scanner).offset(2 as ::core::ffi::c_int as isize);
            i = i.wrapping_add(1);
        }
    }
    return EOT_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn EOTgetByteArray(
    mut scanner: *mut *const uint8_t,
    mut begin: *const uint8_t,
    mut bytesLength: ::core::ffi::c_uint,
    mut size: *mut uint32_t,
    mut array: *mut *mut uint8_t,
) -> EOTError {
    if !(*array).is_null() {
        free(*array as *mut ::core::ffi::c_void);
    }
    *array = ::core::ptr::null_mut::<uint8_t>();
    if (*scanner).offset_from(begin) as ::core::ffi::c_long + 4 as ::core::ffi::c_long
        > bytesLength as ::core::ffi::c_long
    {
        return EOT_INSUFFICIENT_BYTES;
    }
    *size = EOTreadU32LE(*scanner);
    *scanner = (*scanner).offset(4 as ::core::ffi::c_int as isize);
    if (*scanner).offset_from(begin) as ::core::ffi::c_long
        + *size as ::core::ffi::c_long > bytesLength as ::core::ffi::c_long
    {
        return EOT_INSUFFICIENT_BYTES;
    }
    if *size != 0 as uint32_t {
        *array = malloc(*size as size_t) as *mut uint8_t;
        if (*array).is_null() {
            return EOT_CANT_ALLOCATE_MEMORY;
        }
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while (i as uint32_t) < *size {
            *(*array).offset(i as isize) = **scanner;
            *scanner = (*scanner).offset(1);
            i = i.wrapping_add(1);
        }
    }
    return EOT_SUCCESS;
}

#[no_mangle]
pub unsafe extern "C" fn EOTfreeMetadata(mut d: *mut EOTMetadata) {
    if true {
        return;
    }

    if !(*d).familyName.is_null() {
        free((*d).familyName as *mut ::core::ffi::c_void);
    }
    if !(*d).styleName.is_null() {
        free((*d).styleName as *mut ::core::ffi::c_void);
    }
    if !(*d).versionName.is_null() {
        free((*d).versionName as *mut ::core::ffi::c_void);
    }
    if !(*d).fullName.is_null() {
        free((*d).fullName as *mut ::core::ffi::c_void);
    }
    if !(*d).do_not_use.is_null() {
        free((*d).do_not_use as *mut ::core::ffi::c_void);
    }
    if !(*d).rootStrings.is_null() {
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while i < (*d).numRootStrings {
            free(
                (*(*d).rootStrings.offset(i as isize)).rootString
                    as *mut ::core::ffi::c_void,
            );
            i = i.wrapping_add(1);
        }
        free((*d).rootStrings as *mut ::core::ffi::c_void);
    }
    if !(*d).eudcInfo.fontData.is_null() {
        free((*d).eudcInfo.fontData as *mut ::core::ffi::c_void);
    }
    let mut zero: EOTMetadata = EOTMetadata {
        totalSize: 0 as uint32_t,
        version: 0 as EOTVersion,
        flags: 0,
        panose: [0; 10],
        charset: ANSI_CHARSET,
        italic: false,
        weight: 0,
        permissions: 0,
        unicodeRange: [0; 4],
        codePageRange: [0; 2],
        checkSumAdjustment: 0,
        familyNameSize: 0,
        familyName: ::core::ptr::null_mut::<uint16_t>(),
        styleNameSize: 0,
        styleName: ::core::ptr::null_mut::<uint16_t>(),
        versionNameSize: 0,
        versionName: ::core::ptr::null_mut::<uint16_t>(),
        fullNameSize: 0,
        fullName: ::core::ptr::null_mut::<uint16_t>(),
        numRootStrings: 0,
        rootStrings: ::core::ptr::null_mut::<EOTRootStringInfo>(),
        fontDataSize: 0,
        fontDataOffset: 0,
        eudcInfo: EUDCInfo {
            exists: false,
            codePage: 0,
            flags: 0,
            fontDataSize: 0,
            fontData: ::core::ptr::null_mut::<uint8_t>(),
        },
        do_not_use_size: 0,
        do_not_use: ::core::ptr::null_mut::<uint16_t>(),
    };
    *d = zero;
}
#[no_mangle]
pub unsafe extern "C" fn EOTfillMetadataSpecifyingVersion(
    mut bytes: *const uint8_t,
    mut bytesLength: ::core::ffi::c_uint,
    mut out: *mut EOTMetadata,
    mut version: EOTVersion,
    mut currIndex: ::core::ffi::c_int,
) -> EOTError {
    (*out).version = version;
    let mut scanner: *const uint8_t = bytes;
    if scanner.offset_from(bytes) as ::core::ffi::c_long + 4 as ::core::ffi::c_long
        >= bytesLength as ::core::ffi::c_long
    {
        EOTfreeMetadata(out);
        return EOT_INSUFFICIENT_BYTES;
    }
    (*out).flags = EOTreadU32LE(scanner);
    scanner = scanner.offset(4 as ::core::ffi::c_int as isize);
    if scanner.offset_from(bytes) as ::core::ffi::c_long + 10 as ::core::ffi::c_long
        >= bytesLength as ::core::ffi::c_long
    {
        EOTfreeMetadata(out);
        return EOT_INSUFFICIENT_BYTES;
    }
    memcpy(
        &raw mut (*out).panose as *mut ::core::ffi::c_void,
        scanner as *const ::core::ffi::c_void,
        10 as size_t,
    );
    scanner = scanner.offset(10 as ::core::ffi::c_int as isize);
    if scanner.offset_from(bytes) as ::core::ffi::c_long + 1 as ::core::ffi::c_long
        >= bytesLength as ::core::ffi::c_long
    {
        EOTfreeMetadata(out);
        return EOT_INSUFFICIENT_BYTES;
    }
    (*out).charset = *scanner as EOTCharset;
    scanner = scanner.offset(1);
    if scanner.offset_from(bytes) as ::core::ffi::c_long + 1 as ::core::ffi::c_long
        >= bytesLength as ::core::ffi::c_long
    {
        EOTfreeMetadata(out);
        return EOT_INSUFFICIENT_BYTES;
    }
    (*out).italic = *scanner != 0;
    scanner = scanner.offset(1);
    if scanner.offset_from(bytes) as ::core::ffi::c_long + 4 as ::core::ffi::c_long
        >= bytesLength as ::core::ffi::c_long
    {
        EOTfreeMetadata(out);
        return EOT_INSUFFICIENT_BYTES;
    }
    (*out).weight = EOTreadU32LE(scanner);
    scanner = scanner.offset(4 as ::core::ffi::c_int as isize);
    if scanner.offset_from(bytes) as ::core::ffi::c_long + 2 as ::core::ffi::c_long
        >= bytesLength as ::core::ffi::c_long
    {
        EOTfreeMetadata(out);
        return EOT_INSUFFICIENT_BYTES;
    }
    (*out).permissions = EOTreadU16LE(scanner);
    scanner = scanner.offset(2 as ::core::ffi::c_int as isize);
    if scanner.offset_from(bytes) as ::core::ffi::c_long + 2 as ::core::ffi::c_long
        >= bytesLength as ::core::ffi::c_long
    {
        EOTfreeMetadata(out);
        return EOT_INSUFFICIENT_BYTES;
    }
    if EOTreadU16LE(scanner) as ::core::ffi::c_int != 0x504c as ::core::ffi::c_int {
        return EOT_CORRUPT_FILE;
    }
    scanner = scanner.offset(2 as ::core::ffi::c_int as isize);
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < 4 as ::core::ffi::c_uint {
        if scanner.offset_from(bytes) as ::core::ffi::c_long + 4 as ::core::ffi::c_long
            >= bytesLength as ::core::ffi::c_long
        {
            EOTfreeMetadata(out);
            return EOT_INSUFFICIENT_BYTES;
        }
        (*out).unicodeRange[i as usize] = EOTreadU32LE(scanner);
        scanner = scanner.offset(4 as ::core::ffi::c_int as isize);
        i = i.wrapping_add(1);
    }
    let mut i_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i_0 < 2 as ::core::ffi::c_uint {
        if scanner.offset_from(bytes) as ::core::ffi::c_long + 4 as ::core::ffi::c_long
            >= bytesLength as ::core::ffi::c_long
        {
            EOTfreeMetadata(out);
            return EOT_INSUFFICIENT_BYTES;
        }
        (*out).codePageRange[i_0 as usize] = EOTreadU32LE(scanner);
        scanner = scanner.offset(4 as ::core::ffi::c_int as isize);
        i_0 = i_0.wrapping_add(1);
    }
    if scanner.offset_from(bytes) as ::core::ffi::c_long + 4 as ::core::ffi::c_long
        >= bytesLength as ::core::ffi::c_long
    {
        EOTfreeMetadata(out);
        return EOT_INSUFFICIENT_BYTES;
    }
    (*out).checkSumAdjustment = EOTreadU32LE(scanner);
    scanner = scanner.offset(22 as ::core::ffi::c_int as isize);
    let mut macro_defined_var_E: EOTError = EOTgetString(
        &raw mut scanner,
        bytes,
        bytesLength,
        &raw mut (*out).familyNameSize,
        &raw mut (*out).familyName,
    );
    if macro_defined_var_E as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        EOTfreeMetadata(out);
        return macro_defined_var_E;
    }
    scanner = scanner.offset(2 as ::core::ffi::c_int as isize);
    let mut macro_defined_var_E_0: EOTError = EOTgetString(
        &raw mut scanner,
        bytes,
        bytesLength,
        &raw mut (*out).styleNameSize,
        &raw mut (*out).styleName,
    );
    if macro_defined_var_E_0 as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        EOTfreeMetadata(out);
        return macro_defined_var_E_0;
    }
    scanner = scanner.offset(2 as ::core::ffi::c_int as isize);
    let mut macro_defined_var_E_1: EOTError = EOTgetString(
        &raw mut scanner,
        bytes,
        bytesLength,
        &raw mut (*out).versionNameSize,
        &raw mut (*out).versionName,
    );
    if macro_defined_var_E_1 as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        EOTfreeMetadata(out);
        return macro_defined_var_E_1;
    }
    scanner = scanner.offset(2 as ::core::ffi::c_int as isize);
    let mut macro_defined_var_E_2: EOTError = EOTgetString(
        &raw mut scanner,
        bytes,
        bytesLength,
        &raw mut (*out).fullNameSize,
        &raw mut (*out).fullName,
    );
    if macro_defined_var_E_2 as ::core::ffi::c_uint
        != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        EOTfreeMetadata(out);
        return macro_defined_var_E_2;
    }
    if (*out).version as ::core::ffi::c_uint
        > VERSION_1 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        scanner = scanner.offset(2 as ::core::ffi::c_int as isize);
        let mut macro_defined_var_E_3: EOTError = EOTgetString(
            &raw mut scanner,
            bytes,
            bytesLength,
            &raw mut (*out).do_not_use_size,
            &raw mut (*out).do_not_use,
        );
        if macro_defined_var_E_3 as ::core::ffi::c_uint
            != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            EOTfreeMetadata(out);
            return macro_defined_var_E_3;
        }
        if (*out).version as ::core::ffi::c_uint
            == VERSION_3 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if scanner.offset_from(bytes) as ::core::ffi::c_long
                + 4 as ::core::ffi::c_long >= bytesLength as ::core::ffi::c_long
            {
                EOTfreeMetadata(out);
                return EOT_INSUFFICIENT_BYTES;
            }
            EOTreadU32LE(scanner);
            scanner = scanner.offset(4 as ::core::ffi::c_int as isize);
            if scanner.offset_from(bytes) as ::core::ffi::c_long
                + 4 as ::core::ffi::c_long >= bytesLength as ::core::ffi::c_long
            {
                EOTfreeMetadata(out);
                return EOT_INSUFFICIENT_BYTES;
            }
            (*out).eudcInfo.codePage = EOTreadU32LE(scanner);
            scanner = scanner.offset(6 as ::core::ffi::c_int as isize);
            if scanner.offset_from(bytes) as ::core::ffi::c_long
                + 2 as ::core::ffi::c_long >= bytesLength as ::core::ffi::c_long
            {
                EOTfreeMetadata(out);
                return EOT_INSUFFICIENT_BYTES;
            }
            let mut signatureSize: uint16_t = EOTreadU16LE(scanner);
            scanner = scanner.offset(2 as ::core::ffi::c_int as isize);
            if scanner.offset_from(bytes) as ::core::ffi::c_long
                + signatureSize as ::core::ffi::c_long
                >= bytesLength as ::core::ffi::c_long
            {
                EOTfreeMetadata(out);
                return EOT_INSUFFICIENT_BYTES;
            }
            scanner = scanner.offset(signatureSize as ::core::ffi::c_int as isize);
            if scanner.offset_from(bytes) as ::core::ffi::c_long
                + 4 as ::core::ffi::c_long >= bytesLength as ::core::ffi::c_long
            {
                EOTfreeMetadata(out);
                return EOT_INSUFFICIENT_BYTES;
            }
            (*out).eudcInfo.flags = EOTreadU32LE(scanner);
            scanner = scanner.offset(4 as ::core::ffi::c_int as isize);
            let mut macro_defined_var_E_4: EOTError = EOTgetByteArray(
                &raw mut scanner,
                bytes,
                bytesLength,
                &raw mut (*out).eudcInfo.fontDataSize,
                &raw mut (*out).eudcInfo.fontData,
            );
            if macro_defined_var_E_4 as ::core::ffi::c_uint
                != EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                EOTfreeMetadata(out);
                return macro_defined_var_E_4;
            }
            if (*out).eudcInfo.fontDataSize > 0 as uint32_t {
                (*out).eudcInfo.exists = true_0 != 0;
            }
        }
    }
    (*out).fontDataOffset = (scanner.offset_from(bytes) as ::core::ffi::c_long
        + currIndex as ::core::ffi::c_long) as ::core::ffi::c_uint;
    let mut expectedHeaderSize: ::core::ffi::c_int = (*out).totalSize
        as ::core::ffi::c_int - (*out).fontDataSize as ::core::ffi::c_int;
    if (*out).fontDataOffset < expectedHeaderSize as ::core::ffi::c_uint {
        return EOT_HEADER_TOO_BIG;
    }
    return EOT_SUCCESS;
}

#[no_mangle]
pub unsafe fn EOTfillMetadata(
    mut bytes: *const uint8_t,
    mut bytesLength: ::core::ffi::c_uint,
) -> Result<EOTMetadata, Error> {
    let mut met = EOTMetadata::ZERO;
    let mut scanner: *const uint8_t = bytes;
    if bytesLength < 8 as ::core::ffi::c_uint
        || bytesLength < EOTgetMetadataLength(bytes)
    {
        return Err(Error::INSUFFICIENT_BYTES);
    }
    if scanner.offset_from(bytes) as ::core::ffi::c_long + 4 as ::core::ffi::c_long
        >= bytesLength as ::core::ffi::c_long
    {
        return Err(Error::INSUFFICIENT_BYTES);
    }
    let mut totalSize: ::core::ffi::c_uint = EOTreadU32LE(scanner)
        as ::core::ffi::c_uint;
    scanner = scanner.offset(4 as ::core::ffi::c_int as isize);
    if scanner.offset_from(bytes) as ::core::ffi::c_long + 4 as ::core::ffi::c_long
        >= bytesLength as ::core::ffi::c_long
    {
        return Err(Error::INSUFFICIENT_BYTES);
    }
    let mut fontDataSize: ::core::ffi::c_uint = EOTreadU32LE(scanner)
        as ::core::ffi::c_uint;
    scanner = scanner.offset(4 as ::core::ffi::c_int as isize);
    if scanner.offset_from(bytes) as ::core::ffi::c_long + 4 as ::core::ffi::c_long
        >= bytesLength as ::core::ffi::c_long
    {
        return Err(Error::INSUFFICIENT_BYTES);
    }
    let mut versionMagic: uint32_t = EOTreadU32LE(scanner);
    scanner = scanner.offset(4 as ::core::ffi::c_int as isize);
    let mut codedVersion: EOTVersion = 0 as EOTVersion;
    match versionMagic {
        65536 => {
            codedVersion = VERSION_1;
        }
        131073 => {
            codedVersion = VERSION_2;
        }
        131074 => {
            codedVersion = VERSION_3;
        }
        _ => return Err(Error::CORRUPT_FILE),
    }
    let mut tryVersion: EOTVersion = codedVersion;
    let mut bumpedUp: bool = false_0 != 0;
    let mut knockedDown: bool = false_0 != 0;
    loop {
        met.totalSize = totalSize as uint32_t;
        met.fontDataSize = fontDataSize as uint32_t;
        if bytes.offset(bytesLength as isize)
            < scanner.offset(met.fontDataSize as isize)
        {
            return Err(Error::CORRUPT_FILE);
        }
        let mut result: EOTError = EOTfillMetadataSpecifyingVersion(
            scanner,
            ((bytesLength as uint32_t).wrapping_sub(met.fontDataSize)
                as ::core::ffi::c_long
                - scanner.offset_from(bytes) as ::core::ffi::c_long)
                as ::core::ffi::c_uint,
            &mut met,
            tryVersion,
            scanner.offset_from(bytes) as ::core::ffi::c_long as ::core::ffi::c_int,
        );
        if result as ::core::ffi::c_uint
            == EOT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if tryVersion == codedVersion {
                return Ok(met);
            } else {
                return Err(Error::WARN_BAD_VERSION);
            }
        }
        if result as ::core::ffi::c_uint
            == EOT_HEADER_TOO_BIG as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if knockedDown as ::core::ffi::c_int != 0
                || tryVersion as ::core::ffi::c_uint
                    == VERSION_3 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return Err(Error::CORRUPT_FILE);
            }
            knockedDown = false_0 != 0;
            bumpedUp = true_0 != 0;
            tryVersion += 1;
        } else if result as ::core::ffi::c_uint
            == EOT_INSUFFICIENT_BYTES as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if bumpedUp as ::core::ffi::c_int != 0
                || tryVersion as ::core::ffi::c_uint
                    == VERSION_1 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return Err(Error::CORRUPT_FILE);
            }
            knockedDown = true_0 != 0;
            bumpedUp = false_0 != 0;
            tryVersion -= 1;
        } else {
            return Ok(met);
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn EOTcanLegallyEdit(mut metadata: *const EOTMetadata) -> bool {
    return (*metadata).permissions as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*metadata).permissions as ::core::ffi::c_int
            & EDITING_MASK as ::core::ffi::c_int != 0 as ::core::ffi::c_int;
}
