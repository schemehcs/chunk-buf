#[derive(Clone)]
pub struct ChunkBuf {
    buf: Vec<u8>,
    size: usize,
    cursor: usize,
}

impl ChunkBuf {
    pub fn new(size: usize) -> Self {
        Self {
            buf: vec![0; size],
            cursor: 0,
            size,
        }
    }

    pub fn update(&mut self, bytes: &[u8]) -> Option<(&[u8], usize)> {
        let cap = self.size - self.cursor;
        if cap > bytes.len() {
            let cursor_n = self.cursor + bytes.len();
            self.buf[self.cursor..cursor_n].copy_from_slice(bytes);
            self.cursor = cursor_n;
            None
        } else {
            self.buf[self.cursor..].copy_from_slice(&bytes[..cap]);
            self.cursor = 0;
            Some((&self.buf, cap))
        }
    }

    pub fn remainder(&self) -> &[u8] {
        &self.buf[..self.cursor]
    }
}

impl Drop for ChunkBuf {
    fn drop(&mut self) {
        self.buf.fill(0);
        self.cursor = 0;
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
            Some((&[0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8][..], 4))
        );
        assert_eq!(
            buf.update(&buf2[4..]),
            Some((&[0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x0][..], 8))
        );
        assert_eq!(buf.remainder(), &[][..]);
        assert_eq!(buf.update(&buf2[12..]), None);
        assert_eq!(buf.remainder(), &[0xa0, 0xa1, 0xa2, 0xa3][..]);
    }
}
