use crate::core::Error;

pub struct BITIO {
    mem_bytes: *mut ::core::ffi::c_uchar,
    mem_index: i64,
    mem_size: i64,
    input_bit_count: u16,
    input_bit_buffer: u16,
    bytes_in: i64,
}

impl BITIO {
    pub fn new(memPtr: *mut ::core::ffi::c_void, memSize: i64) -> BITIO {
        BITIO {
            mem_bytes: memPtr as *mut ::core::ffi::c_uchar,
            mem_index: 0,
            mem_size: memSize,
            input_bit_count: 0,
            input_bit_buffer: 0,
            bytes_in: 0,
        }
    }

    pub unsafe fn read_value(&mut self, numberOfBits: i64) -> Result<::core::ffi::c_ulong, Error> {
        let mut value = 0u64;
        for _ in 0..numberOfBits {
            value <<= 1;
            if self.input_bit()? != 0 {
                value |= 1;
            }
        }
        Ok(value)
    }

    pub unsafe fn input_bit(&mut self) -> Result<i16, Error> {
        let input_bit_count = self.input_bit_count;
        self.input_bit_count = self.input_bit_count.wrapping_sub(1);
        if input_bit_count == 0 {
            let fresh2 = self.mem_index;
            self.mem_index = self.mem_index + 1;
            self.input_bit_buffer = *self.mem_bytes.offset(fresh2 as isize) as u16;
            if self.mem_index > self.mem_size {
                return Err(Error::BITIO_END_OF_FILE);
            }
            self.bytes_in += 1;
            self.input_bit_count = 7;
        }
        self.input_bit_buffer = ((self.input_bit_buffer as i32) << 1) as u16;
        Ok((self.input_bit_buffer as i32 & 0x100) as i16)
    }
}
