use crate::GptError;
use alloc::vec::Vec;
use core::ops::Deref;

pub(crate) trait Serialize {
    fn serialize(&self, size: usize) -> Vec<u8>;
}

pub(crate) trait Deserialize: Sized {
    fn deserialize(data: &[u8]) -> Result<Self, GptError>;
}

pub(crate) struct LittleEndianBytes<'a> {
    data: &'a [u8],
    cursor: usize,
}

impl<'a> From<&'a [u8]> for LittleEndianBytes<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self {
            data: value,
            cursor: 0,
        }
    }
}

impl LittleEndianBytes<'_> {
    pub fn parse_u8(&mut self) -> Option<u8> {
        if self.cursor < self.data.len() {
            let res = self.data[self.cursor];
            self.skip(1);
            return Some(res);
        }
        None
    }

    pub fn parse_u16(&mut self) -> Option<u16> {
        if self.cursor + 1 < self.data.len() {
            let (a0, a1) = (self.data[self.cursor], self.data[self.cursor + 1]);
            self.skip(2);
            let res = (a1 as u16) << 8 | a0 as u16;
            return Some(res);
        }
        None
    }

    pub fn parse_u32(&mut self) -> Option<u32> {
        if self.cursor + 3 < self.data.len() {
            let (a0, a1, a2, a3) = (
                self.data[self.cursor],
                self.data[self.cursor + 1],
                self.data[self.cursor + 2],
                self.data[self.cursor + 3],
            );
            self.skip(4);
            let res = (a3 as u32) << 24 | (a2 as u32) << 16 | (a1 as u32) << 8 | a0 as u32;
            return Some(res);
        }
        None
    }

    pub fn parse_u64(&mut self) -> Option<u64> {
        if self.cursor + 7 < self.data.len() {
            let (a0, a1, a2, a3, a4, a5, a6, a7) = (
                self.data[self.cursor],
                self.data[self.cursor + 1],
                self.data[self.cursor + 2],
                self.data[self.cursor + 3],
                self.data[self.cursor + 4],
                self.data[self.cursor + 5],
                self.data[self.cursor + 6],
                self.data[self.cursor + 7],
            );
            self.skip(8);
            let res = (a7 as u64) << 56
                | (a6 as u64) << 48
                | (a5 as u64) << 40
                | (a4 as u64) << 32
                | (a3 as u64) << 24
                | (a2 as u64) << 16
                | (a1 as u64) << 8
                | a0 as u64;
            return Some(res);
        }
        None
    }

    #[inline]
    pub fn skip(&mut self, size: usize) {
        self.cursor += size;
    }

    pub fn copy_from<const N: usize>(&mut self, offset: usize) -> [u8; N] {
        let mut res = [0; N];
        res.copy_from_slice(&self.data[offset..offset + N]);
        self.skip(N);
        res
    }
}

impl<'a> Deref for LittleEndianBytes<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[cfg(test)]
mod test {}
