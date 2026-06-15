use crate::core::Error;

pub struct BITIO<'a> {
    data: &'a [u8],
    index: usize,
    input_bit_count: u16,
    input_bit_buffer: u16,
}

impl BITIO<'_> {
    pub fn new<'a>(data: &'a [u8]) -> BITIO<'a> {
        BITIO {
            data,
            index: 0,
            input_bit_count: 0,
            input_bit_buffer: 0,
        }
    }

    pub fn read_value(&mut self, num_bits: i64) -> Result<u64, Error> {
        let mut value = 0u64;
        for _ in 0..num_bits {
            value <<= 1;
            if self.input_bit()? != 0 {
                value |= 1;
            }
        }
        Ok(value)
    }

    // Read one bit from the input memory
    pub fn input_bit(&mut self) -> Result<u16, Error> {
        let input_bit_count = self.input_bit_count;
        self.input_bit_count = self.input_bit_count.wrapping_sub(1);
        if input_bit_count == 0 {
            if self.index >= self.data.len() {
                return Err(Error::BITIO_END_OF_FILE);
            }
            self.input_bit_buffer = self.data[self.index] as u16;
            self.index += 1;
            self.input_bit_count = 7;
        }
        self.input_bit_buffer <<= 1;
        Ok(self.input_bit_buffer & 0x100)
    }
}
