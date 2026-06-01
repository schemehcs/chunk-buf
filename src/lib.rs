#[derive(PartialEq, Debug)]
pub struct Chunk<'a> {
    pub bytes: &'a [u8],
    pub consumed: usize,
}

impl<'a> Chunk<'a> {
    pub fn new(bytes: &'a [u8], consumed: usize) -> Self {
        Chunk { bytes, consumed }
    }
}

#[derive(Clone, Debug)]
pub struct ChunkBuf {
    buf: Vec<u8>,
    cursor: usize,
    acc_consumed: usize,
}

impl ChunkBuf {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![0; capacity],
            cursor: 0,
            acc_consumed: 0,
        }
    }

    pub fn update(&mut self, bytes: &[u8]) -> Option<Chunk<'_>> {
        let cap = self.buf.capacity() - self.cursor;
        if cap > bytes.len() {
            let cursor_n = self.cursor + bytes.len();
            self.buf[self.cursor..cursor_n].copy_from_slice(bytes);
            self.acc_consumed += bytes.len();
            self.cursor = cursor_n;
            None
        } else {
            self.buf[self.cursor..].copy_from_slice(&bytes[..cap]);
            self.acc_consumed += cap;
            self.cursor = 0;
            Some(Chunk::new(&self.buf, cap))
        }
    }

    pub fn remainder(&self) -> &[u8] {
        &self.buf[..self.cursor]
    }

    pub fn acc_consumed(&self) -> usize {
        self.acc_consumed
    }
}

impl Drop for ChunkBuf {
    fn drop(&mut self) {
        // zero buffer to avoid leaving sensitive data on the heap
        self.buf.fill(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let buf1 = [0x1u8, 0x2, 0x3, 0x4];
        let buf2 = [
            0x5u8, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x0, 0xa0, 0xa1, 0xa2, 0xa3,
        ];
        let mut buf = ChunkBuf::new(8);
        assert_eq!(buf.update(&buf1), None);
        assert_eq!(
            buf.update(&buf2),
            Some(Chunk::new(&[0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8], 4))
        );
        assert_eq!(
            buf.update(&buf2[4..]),
            Some(Chunk::new(&[0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x0], 8))
        );
        assert_eq!(buf.remainder(), &[][..]);
        assert_eq!(buf.update(&buf2[12..]), None);
        assert_eq!(buf.remainder(), &[0xa0, 0xa1, 0xa2, 0xa3][..]);
        assert_eq!(buf.acc_consumed(), 20)
    }
}
