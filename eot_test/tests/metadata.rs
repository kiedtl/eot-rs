/// C API. Must match exactly.
mod c {
    #[repr(C)]
    pub struct EudcInfo {
        pub exists: bool,
        pub code_page: u32,
        pub flags: u32,
        pub font_data_size: u32,
        pub font_data: *mut u8,
    }

    #[repr(C)]
    pub struct RootStringInfo {
        pub root_string_size: u16,
        pub root_string: *mut u16,
    }

    #[repr(C)]
    pub struct EotMetadata {
        pub total_size: u32,
        pub version: u32, // enum EOTVersion
        pub flags: u32,
        pub panose: [u8; 10],
        pub charset: u32, // enum EOTCharset
        pub italic: bool,
        pub weight: u32,
        pub permissions: u16,
        pub unicode_range: [u32; 4],
        pub code_page_range: [u32; 2],
        pub check_sum_adjustment: u32,
        pub family_name_size: u16,
        pub family_name: *mut u16,
        pub style_name_size: u16,
        pub style_name: *mut u16,
        pub version_name_size: u16,
        pub version_name: *mut u16,
        pub full_name_size: u16,
        pub full_name: *mut u16,
        pub num_root_strings: u32,
        pub root_strings: *mut RootStringInfo,
        pub font_data_size: u32,
        pub font_data_offset: u32,
        pub eudc_info: EudcInfo,
        pub do_not_use_size: u16,
        pub do_not_use: *mut u16,
    }

    unsafe extern "C" {
        pub fn EOTfillMetadata(bytes: *const u8, bytes_length: u32, out: *mut EotMetadata) -> u32;
        pub fn EOTfreeMetadata(out: *mut EotMetadata);
    }
}

/// Convert the uint16_t* into a Vec so we can compare it easily.
unsafe fn c_u16s(ptr: *const u16, size_bytes: u16) -> Vec<u16> {
    let n = (size_bytes / 2) as usize;
    if ptr.is_null() || n == 0 {
        Vec::new()
    } else {
        unsafe {
            std::slice::from_raw_parts(ptr, n).to_vec()
        }
    }
}

/// Convert a uint8_t* into a Vec<u8> so we can compare it easily.
unsafe fn c_bytes(ptr: *const u8, size: u32) -> Vec<u8> {
    if ptr.is_null() || size == 0 {
        Vec::new()
    } else {
        unsafe {
            std::slice::from_raw_parts(ptr, size as usize).to_vec()
        }
    }
}

#[test]
fn metadata_matches_legacy_api() {
    let files = [
        "../testdata/font1.fntdata",
        "../testdata/font2.fntdata",
        "../testdata/font3.fntdata",
        "../testdata/font4.fntdata",
        "../testdata/Lato-bold.fntdata",
        "../testdata/Lato-boldItalic.fntdata",
        "../testdata/Lato-italic.fntdata",
        "../testdata/Lato-regular.fntdata",
        "../testdata/Raleway-bold.fntdata",
        "../testdata/Raleway-boldItalic.fntdata",
        "../testdata/Raleway-italic.fntdata",
        "../testdata/Raleway-regular.fntdata",
        "../testdata/7/compressed.eot",
        "../testdata/7/uncompressed.eot",
        "../testdata/4/font1.eot",
    ];

    for inp in files {
        eprintln!("*** Testing {inp}");

        let data = std::fs::read(inp).unwrap();

        let mut c: c::EotMetadata = unsafe { std::mem::zeroed() };
        let c_ret = unsafe { c::EOTfillMetadata(data.as_ptr(), data.len() as u32, &mut c) };
        assert_eq!(c_ret, 0, "EOTfillMetadata: ret = {c_ret}");

        let r = eot_parse::metadata::read_metadata(&data).unwrap();

        assert_eq!(r.total_size, c.total_size, "total_size");
        assert_eq!(r.version, c.version, "version");
        assert_eq!(r.flags, c.flags, "flags");
        assert_eq!(r.panose, c.panose, "panose");
        assert_eq!(r.charset, c.charset, "charset");
        assert_eq!(r.italic, c.italic, "italic");
        assert_eq!(r.weight, c.weight, "weight");
        assert_eq!(r.permissions, c.permissions, "permissions");
        assert_eq!(r.unicode_range, c.unicode_range, "unicode_range");
        assert_eq!(r.code_page_range, c.code_page_range, "code_page_range");
        assert_eq!(r.check_sum_adjustment, c.check_sum_adjustment, "check_sum_adjustment");
        assert_eq!(r.font_data_size, c.font_data_size, "font_data_size");
        assert_eq!(r.font_data_offset, c.font_data_offset, "font_data_offset");
        assert_eq!(r.num_root_strings, c.num_root_strings, "num_root_strings");

        unsafe {
            assert_eq!(r.family_name, c_u16s(c.family_name, c.family_name_size), "family_name");
            assert_eq!(r.style_name, c_u16s(c.style_name, c.style_name_size), "style_name");
            assert_eq!(r.version_name, c_u16s(c.version_name, c.version_name_size), "version_name");
            assert_eq!(r.full_name, c_u16s(c.full_name, c.full_name_size), "full_name");
            assert_eq!(r.do_not_use, c_u16s(c.do_not_use, c.do_not_use_size), "do_not_use");
        }

        assert_eq!(r.eudc_info.exists, c.eudc_info.exists, "eudc.exists");
        assert_eq!(r.eudc_info.code_page, c.eudc_info.code_page, "eudc.code_page");
        assert_eq!(r.eudc_info.flags, c.eudc_info.flags, "eudc.flags");
        unsafe {
            assert_eq!(
                r.eudc_info.font_data,
                c_bytes(c.eudc_info.font_data, c.eudc_info.font_data_size),
                "eudc.font_data"
            );
        }

        unsafe { c::EOTfreeMetadata(&mut c) };
    }
}
