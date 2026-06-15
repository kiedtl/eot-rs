#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Error {
    OFF_BYTE_BOUNDARY = 7,
    VALUE_OUT_OF_BOUNDS = 6,
    OUT_OF_RESERVED_SPACE = 5,
    CANT_ALLOCATE_MEMORY_FOR_STREAM = 4,
    SEEK_PAST_EOS = 3,
    NEGATIVE_SEEK = 2,
    NOT_ENOUGH_DATA = 1,
}

#[derive(Clone)]
pub struct Stream {
    pub buf: Vec<u8>,
    pub pos: usize,
    pub bit_pos: usize,
}

impl Stream {
    fn check_byte_boundary(&self) -> Result<(), Error> {
        if self.bit_pos != 0 {
            return Err(Error::OFF_BYTE_BOUNDARY);
        }
        Ok(())
    }

    pub fn new(size: usize) -> Self {
        Self::new2(size, size)
    }

    pub fn new2(size: usize, reserved: usize) -> Self {
        let mut ret = Stream {
            buf: Vec::with_capacity(reserved),
            pos: 0,
            bit_pos: 0,
        };
        for _ in 0..size {
            ret.buf.push(0);
        }
        ret
    }

    pub fn be_read_rest_as_u32(&mut self) -> Result<u32, Error> {
        if self.pos >= self.buf.len() {
            return Err(Error::NOT_ENOUGH_DATA);
        }

        Ok(match self.buf.len() - self.pos {
            1 => (self.be_read_u8()? as u32) << 24,
            2 => (self.be_read_u16()? as u32) << 16,
            3 => self.be_read_u24()? << 8,
            4 | _ => self.be_read_u32()?,
        })
    }

    pub fn be_read_u8(&mut self) -> Result<u8, Error> {
        let v = self.be_peek_u8()?;
        self.pos += 1;
        Ok(v)
    }

    pub fn be_peek_u8(&mut self) -> Result<u8, Error> {
        self.check_byte_boundary()?;

        if self.pos >= self.buf.len() {
            return Err(Error::NOT_ENOUGH_DATA);
        }
        Ok(self.buf[self.pos])
    }

    pub fn be_read_u16(&mut self) -> Result<u16, Error> {
        self.check_byte_boundary()?;

        if self.pos + 2 > self.buf.len() {
            return Err(Error::NOT_ENOUGH_DATA);
        }

        let opos = self.pos;
        self.pos += 2;
        Ok((self.buf[opos] as u16) << 8 | self.buf[opos + 1] as u16)
    }

    pub fn be_read_u24(&mut self) -> Result<u32, Error> {
        self.check_byte_boundary()?;

        if self.pos + 3 > self.buf.len() {
            return Err(Error::NOT_ENOUGH_DATA);
        }

        let opos = self.pos;
        self.pos += 3;
        Ok((self.buf[opos] as u32) << 16 | (self.buf[opos + 1] as u32) << 8 | self.buf[opos + 2] as u32)
    }

    pub fn be_read_u32(&mut self) -> Result<u32, Error> {
        self.check_byte_boundary()?;

        if self.pos + 4 > self.buf.len() {
            return Err(Error::NOT_ENOUGH_DATA);
        }

        let opos = self.pos;
        self.pos += 4;
        Ok((self.buf[opos] as u32) << 24
            | (self.buf[opos + 1] as u32) << 16
            | (self.buf[opos + 2] as u32) << 8
            | self.buf[opos + 3] as u32)
    }

    pub fn be_read_i8(&mut self) -> Result<i8, Error> {
        Ok(self.be_read_u8()? as i8)
    }

    pub fn be_read_i16(&mut self) -> Result<i16, Error> {
        Ok(self.be_read_u16()? as i16)
    }

    pub fn be_read_i24(&mut self) -> Result<i32, Error> {
        Ok(self.be_read_u24()? as i32)
    }

    pub fn be_read_i32(&mut self) -> Result<i32, Error> {
        Ok(self.be_read_u32()? as i32)
    }

    pub fn read_n_bits(&mut self, n: u32) -> Result<u32, Error> {
        let masks: [u8; 8] = [0x80, 0x40, 0x20, 0x10, 0x8, 0x4, 0x2, 0x1];
        if n > 32 {
            return Err(Error::VALUE_OUT_OF_BOUNDS);
        }

        let mut out = 0u32;
        for i in 0..n {
            if self.pos >= self.buf.len() {
                return Err(Error::NOT_ENOUGH_DATA);
            }
            let is_bit_set = self.buf[self.pos] & masks[self.bit_pos] > 0;
            out |= if is_bit_set { 1 } else { 0 } << (n - i - 1);
            self.bit_pos += 1;
            if self.bit_pos == 8 {
                self.bit_pos = 0;
                self.pos += 1;
            }
        }

        Ok(out)
    }

    pub fn seek_relative(&mut self, offset: isize) -> Result<(), Error> {
        self.check_byte_boundary()?;

        let newpos = self.pos as isize + offset;
        if newpos < 0 {
            return Err(Error::NEGATIVE_SEEK);
        } else if newpos as usize > self.buf.len() {
            return Err(Error::SEEK_PAST_EOS);
        }

        self.pos = newpos as usize;
        Ok(())
    }

    pub fn seek_relative_through_reserve(&mut self, offset: isize) -> Result<(), Error> {
        self.check_byte_boundary()?;

        let newpos = self.pos as isize + offset;
        if newpos < 0 {
            return Err(Error::NEGATIVE_SEEK);
        } else if newpos as usize > self.buf.capacity() {
            return Err(Error::SEEK_PAST_EOS);
        }

        while self.buf.len() < newpos as usize {
            self.buf.push(0);
        }

        self.pos = newpos as usize;
        Ok(())
    }

    pub fn seek_absolute(&mut self, pos: usize) -> Result<(), Error> {
        self.check_byte_boundary()?;

        if pos > self.buf.len() {
            return Err(Error::SEEK_PAST_EOS);
        }

        self.pos = pos;
        Ok(())
    }

    pub fn seek_absolute_through_reserve(&mut self, pos: usize) -> Result<(), Error> {
        self.check_byte_boundary()?;
        if pos > self.buf.capacity() {
            return Err(Error::SEEK_PAST_EOS);
        }
        self.pos = pos;
        while self.buf.len() < self.pos {
            self.buf.push(0);
        }
        Ok(())
    }

    pub fn be_write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.check_byte_boundary()?;

        if self.pos + 1 > self.buf.capacity() {
            return Err(Error::OUT_OF_RESERVED_SPACE);
        }

        if self.pos == self.buf.len() {
            self.buf.push(val);
        } else {
            self.buf[self.pos] = val;
        }

        self.pos += 1;
        Ok(())
    }

    pub fn be_write_i16(&mut self, val: i16) -> Result<(), Error> {
        self.be_write_u16(val as u16)
    }

    pub fn be_write_u16(&mut self, val: u16) -> Result<(), Error> {
        self.check_byte_boundary()?;

        if self.pos + 2 > self.buf.capacity() {
            return Err(Error::OUT_OF_RESERVED_SPACE);
        }

        let bytes = [((val >> 8) & 0xFF) as u8, (val & 0xFF) as u8];
        for byte in bytes {
            if self.pos == self.buf.len() {
                self.buf.push(byte);
            } else {
                self.buf[self.pos] = byte;
            }
            self.pos += 1;
        }

        Ok(())
    }

    pub fn be_write_u24(&mut self, val: u32) -> Result<(), Error> {
        self.check_byte_boundary()?;

        if val & 0xff000000 != 0 {
            return Err(Error::VALUE_OUT_OF_BOUNDS);
        }

        if self.pos + 3 > self.buf.capacity() {
            return Err(Error::OUT_OF_RESERVED_SPACE);
        }

        let bytes = [((val >> 16) & 0xFF) as u8, ((val >> 8) & 0xFF) as u8, (val & 0xFF) as u8];

        for byte in bytes {
            if self.pos == self.buf.len() {
                self.buf.push(byte);
            } else {
                self.buf[self.pos] = byte;
            }
            self.pos += 1;
        }

        self.pos += 3;
        Ok(())
    }

    pub fn be_write_u32(&mut self, val: u32) -> Result<(), Error> {
        self.check_byte_boundary()?;

        if self.pos + 4 > self.buf.capacity() {
            return Err(Error::OUT_OF_RESERVED_SPACE);
        }

        let bytes =
            [(val >> 24) as u8, ((val >> 16) & 0xFF) as u8, ((val >> 8) & 0xFF) as u8, (val & 0xFF) as u8];

        for byte in bytes {
            if self.pos == self.buf.len() {
                self.buf.push(byte);
            } else {
                self.buf[self.pos] = byte;
            }
            self.pos += 1;
        }

        Ok(())
    }

    pub fn be_checksum32(&mut self, begin_pos: usize, end_pos: usize) -> Result<u32, Error> {
        if begin_pos > end_pos {
            return Err(Error::VALUE_OUT_OF_BOUNDS);
        }

        if end_pos > self.buf.len() {
            return Err(Error::NOT_ENOUGH_DATA);
        }

        let mut slice = Stream::new(end_pos - begin_pos);
        slice.buf[..].copy_from_slice(&self.buf[begin_pos..end_pos]);
        let mut out: u32 = 0;

        loop {
            match slice.be_read_rest_as_u32() {
                Ok(c) => out = out.wrapping_add(c),
                Err(Error::NOT_ENOUGH_DATA) => break,
                Err(e) => return Err(e),
            }
        }

        Ok(out)
    }
}
