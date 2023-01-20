use anyhow::anyhow;
use std::error;

#[derive(Debug)]
pub struct ByteBuffer<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ByteBuffer<'a> {
    pub fn from(buf: &'a [u8]) -> ByteBuffer<'a> {
        ByteBuffer { buf, pos: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn get(&self, pos: usize) -> Option<u8> {
        self.check_bounds(pos, 1).ok()?;
        Some(self.buf[pos])
    }

    pub fn peek(&mut self) -> Option<u8> {
        self.check_bounds(self.pos, 1).ok()?;
        Some(self.buf[self.pos])
    }

    pub fn read(&mut self) -> Option<u8> {
        self.check_bounds(self.pos, 1).ok()?;
        let value = self.peek()?;
        self.pos += 1;
        Some(value)
    }

    pub fn jump(&mut self, pos: usize) -> Result<(), Box<dyn error::Error>> {
        self.check_bounds(pos, 1)?;
        self.pos = pos;
        Ok(())
    }

    pub fn read_range(&mut self, len: usize) -> Result<&'a [u8], Box<dyn error::Error>> {
        self.check_bounds(self.pos, self.pos + len)?;
        let slice = &self.buf[self.pos..self.pos + len];
        self.pos += len;
        Ok(slice)
    }

    pub fn read_u16(&mut self) -> Result<u16, Box<dyn error::Error>> {
        let bytes = self.read_range(2)?;
        let bytes: [u8; 2] = bytes.try_into()?;
        let value = u16::from_be_bytes(bytes);
        Ok(value)
    }

    pub fn read_i32(&mut self) -> Result<i32, Box<dyn error::Error>> {
        let bytes = self.read_range(4)?;
        let bytes: [u8; 4] = bytes.try_into()?;
        let value = i32::from_be_bytes(bytes);
        Ok(value)
    }

    fn check_bounds(&self, from: usize, to: usize) -> Result<(), Box<dyn error::Error>> {
        if self.buf.is_empty() && from + to > 0 {
            Err(anyhow!(
                "from:{} to:{} len:{} out of bounds",
                from,
                to,
                self.buf.len()
            )
            .into())
        } else if from > self.buf.len() - 1 {
            Err(anyhow!(
                "from:{} to:{} len:{} out of bounds",
                from,
                to,
                self.buf.len()
            )
            .into())
        } else if to > self.buf.len() {
            Err(anyhow!(
                "from:{} to:{} len:{} out of bounds",
                from,
                to,
                self.buf.len()
            )
            .into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------
    // pos()
    // --------------------------------------------------

    #[test]
    fn test_pos_returns_expected_position() {
        let buf = ByteBuffer {
            buf: &vec![1, 2, 3, 4, 5],
            pos: 3,
        };
        assert_eq!(buf.pos(), 3);
    }

    // --------------------------------------------------
    // peek()
    // --------------------------------------------------

    #[test]
    fn test_peek_returns_none_on_empty_buffer() {
        let bytes = vec![];
        let mut buf = ByteBuffer::from(&bytes);
        assert_eq!(buf.peek(), None);
    }

    #[test]
    fn test_peek_returns_expected_value_for_buffer_len_1() {
        let bytes = vec![1];
        let mut buf = ByteBuffer::from(&bytes);
        assert_eq!(buf.peek(), Some(1));
    }

    #[test]
    fn test_peek_returns_expected_values_for_buffer_len_3() {
        let bytes = vec![1, 2, 3];
        let mut buf = ByteBuffer::from(&bytes);
        assert_eq!(buf.peek(), Some(1));
        let _ = buf.read();
        assert_eq!(buf.peek(), Some(2));
        let _ = buf.read();
        assert_eq!(buf.peek(), Some(3));
    }

    #[test]
    fn test_peek_returns_none_at_end_of_buffer_len_1() {
        let bytes = vec![1];
        let mut buf = ByteBuffer::from(&bytes);
        assert_eq!(buf.peek(), Some(1));
        let _ = buf.read();
        assert_eq!(buf.peek(), None);
    }

    #[test]
    fn test_peek_does_not_update_pos() {
        let bytes = vec![1, 2];
        let mut buf = ByteBuffer::from(&bytes);
        let _ = buf.peek();
        assert_eq!(buf.pos(), 0);
    }

    // --------------------------------------------------
    // read()
    // --------------------------------------------------

    #[test]
    fn test_read_returns_none_on_empty_buffer() {
        let bytes = vec![];
        let mut buf = ByteBuffer::from(&bytes);
        assert_eq!(buf.read(), None);
    }

    #[test]
    fn test_read_returns_expected_value_for_buffer_len_1() {
        let bytes = vec![1];
        let mut buf = ByteBuffer::from(&bytes);
        assert_eq!(buf.read(), Some(1));
    }

    #[test]
    fn test_read_returns_expected_values_for_buffer_len_3() {
        let bytes = vec![1, 2, 3];
        let mut buf = ByteBuffer::from(&bytes);
        assert_eq!(buf.read(), Some(1));
        assert_eq!(buf.read(), Some(2));
        assert_eq!(buf.read(), Some(3));
    }

    #[test]
    fn test_read_returns_none_at_end_of_buffer_len_1() {
        let bytes = vec![1];
        let mut buf = ByteBuffer::from(&bytes);
        assert_eq!(buf.read(), Some(1));
        assert_eq!(buf.read(), None);
    }

    #[test]
    fn test_read_does_not_update_pos_for_empty_buffer() {
        let bytes = vec![];
        let mut buf = ByteBuffer::from(&bytes);
        let _ = buf.read();
        assert_eq!(buf.pos(), 0);
    }

    #[test]
    fn test_read_updates_pos_for_buffer_len_1() {
        let bytes = vec![1];
        let mut buf = ByteBuffer::from(&bytes);
        let _ = buf.read();
        assert_eq!(buf.pos(), 1);
    }

    #[test]
    fn test_read_updates_pos_for_buffer_len_3() {
        let bytes = vec![1, 2, 3];
        let mut buf = ByteBuffer::from(&bytes);
        assert_eq!(buf.pos(), 0);
        let _ = buf.read();
        assert_eq!(buf.pos(), 1);
        let _ = buf.read();
        assert_eq!(buf.pos(), 2);
        let _ = buf.read();
        assert_eq!(buf.pos(), 3);
        let _ = buf.read();
        assert_eq!(buf.pos(), 3);
    }

    // --------------------------------------------------
    // read_range()
    // --------------------------------------------------

    #[test]
    fn test_read_range_returns_error_on_empty_buf() {
        let bytes = vec![];
        let mut buf = ByteBuffer::from(&bytes);
        assert!(buf.read_range(1).is_err());
    }

    #[test]
    fn test_read_range_returns_error_on_out_of_bounds() {
        let bytes = vec![1, 2, 3, 4, 5];
        let mut buf = ByteBuffer::from(&bytes);
        assert!(buf.read_range(10).is_err());
    }

    #[test]
    fn test_read_range_returns_expected_value_on_buf_len_1() -> Result<(), Box<dyn error::Error>> {
        let bytes = vec![1];
        let mut buf = ByteBuffer::from(&bytes);
        assert_eq!(buf.read_range(1)?, &bytes[0..1]);
        Ok(())
    }

    #[test]
    fn test_read_range_returns_expected_value_on_buf_len_3() -> Result<(), Box<dyn error::Error>> {
        let bytes = vec![1, 2, 3];
        let mut buf = ByteBuffer::from(&bytes);
        assert_eq!(buf.read_range(3)?, &bytes[0..3]);
        Ok(())
    }

    // --------------------------------------------------
    // read_u16()
    // --------------------------------------------------
}
