use std::marker::PhantomData;

pub enum U1 {}
pub enum U2 {}
pub enum U3 {}
pub enum U4 {}
pub enum U5 {}
pub enum U6 {}
pub enum U7 {}
pub enum U8 {}
pub enum U9 {}
pub enum U10 {}
pub enum U11 {}
pub enum U12 {}
pub enum U13 {}
pub enum U14 {}
pub enum U15 {}
pub enum U16 {}
pub enum U17 {}
pub enum U18 {}
pub enum U19 {}
pub enum U20 {}
pub enum U21 {}
pub enum U22 {}
pub enum U23 {}
pub enum U24 {}
pub enum U25 {}
pub enum U26 {}
pub enum U27 {}
pub enum U28 {}
pub enum U29 {}
pub enum U30 {}
pub enum U31 {}

pub trait PackedElement {
    const NUM_BITS: usize;
    const MAX: u32 = (1 << Self::NUM_BITS) - 1;
}

impl PackedElement for U1 {
    const NUM_BITS: usize = 1;
}
impl PackedElement for U2 {
    const NUM_BITS: usize = 2;
}
impl PackedElement for U3 {
    const NUM_BITS: usize = 3;
}
impl PackedElement for U4 {
    const NUM_BITS: usize = 4;
}
impl PackedElement for U5 {
    const NUM_BITS: usize = 5;
}
impl PackedElement for U6 {
    const NUM_BITS: usize = 6;
}
impl PackedElement for U7 {
    const NUM_BITS: usize = 7;
}
impl PackedElement for U8 {
    const NUM_BITS: usize = 8;
}
impl PackedElement for U9 {
    const NUM_BITS: usize = 9;
}
impl PackedElement for U10 {
    const NUM_BITS: usize = 10;
}
impl PackedElement for U11 {
    const NUM_BITS: usize = 11;
}
impl PackedElement for U12 {
    const NUM_BITS: usize = 12;
}
impl PackedElement for U13 {
    const NUM_BITS: usize = 13;
}
impl PackedElement for U14 {
    const NUM_BITS: usize = 14;
}
impl PackedElement for U15 {
    const NUM_BITS: usize = 15;
}
impl PackedElement for U16 {
    const NUM_BITS: usize = 16;
}
impl PackedElement for U17 {
    const NUM_BITS: usize = 17;
}
impl PackedElement for U18 {
    const NUM_BITS: usize = 18;
}
impl PackedElement for U19 {
    const NUM_BITS: usize = 19;
}
impl PackedElement for U20 {
    const NUM_BITS: usize = 20;
}
impl PackedElement for U21 {
    const NUM_BITS: usize = 21;
}
impl PackedElement for U22 {
    const NUM_BITS: usize = 22;
}
impl PackedElement for U23 {
    const NUM_BITS: usize = 23;
}
impl PackedElement for U24 {
    const NUM_BITS: usize = 24;
}
impl PackedElement for U25 {
    const NUM_BITS: usize = 25;
}
impl PackedElement for U26 {
    const NUM_BITS: usize = 26;
}
impl PackedElement for U27 {
    const NUM_BITS: usize = 27;
}
impl PackedElement for U28 {
    const NUM_BITS: usize = 28;
}
impl PackedElement for U29 {
    const NUM_BITS: usize = 29;
}
impl PackedElement for U30 {
    const NUM_BITS: usize = 30;
}
impl PackedElement for U31 {
    const NUM_BITS: usize = 31;
}

pub struct PackedVec<T: PackedElement> {
    buf: Vec<u32>,
    len: usize,
    phantom: PhantomData<T>,
}

impl<T: PackedElement> PackedVec<T> {
    const U32_NUM_BITS: usize = 32;

    pub fn new() -> PackedVec<T> {
        PackedVec {
            buf: Vec::new(),
            len: 0,
            phantom: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> PackedVec<T> {
        let capacity = (T::NUM_BITS * capacity + (Self::U32_NUM_BITS - 1)) / Self::U32_NUM_BITS;

        PackedVec {
            buf: Vec::with_capacity(capacity),
            len: 0,
            phantom: PhantomData,
        }
    }

    pub fn capacity(&self) -> usize {
        self.buf.capacity() * Self::U32_NUM_BITS / T::NUM_BITS
    }

    pub fn get(&self, index: usize) -> Option<u32> {
        if index >= self.len {
            return None;
        }

        let buf_index = index * T::NUM_BITS / Self::U32_NUM_BITS;
        let start_bit = index * T::NUM_BITS % Self::U32_NUM_BITS;
        let available_bits = Self::U32_NUM_BITS - start_bit;

        if available_bits >= T::NUM_BITS {
            Some((self.buf[buf_index] >> start_bit) & T::MAX)
        } else {
            // Value spans 2 buffer cells.
            let lo = self.buf[buf_index] >> start_bit;
            let hi = self.buf[buf_index + 1] << (Self::U32_NUM_BITS - start_bit);

            Some(lo ^ ((lo ^ hi) & (T::MAX >> available_bits << available_bits)))
        }
    }

    pub fn iter(&self) -> PackedVecIterator<'_, T> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: u32) {
        if value > T::MAX {
            panic!("value is outside the range 0..={}", T::MAX);
        }

        let buf_index = self.len * T::NUM_BITS / Self::U32_NUM_BITS;
        let start_bit = self.len * T::NUM_BITS % Self::U32_NUM_BITS;
        let available_bits = Self::U32_NUM_BITS - start_bit;

        if available_bits >= T::NUM_BITS {
            if buf_index == self.buf.len() {
                self.buf.push(0);
            }

            self.buf[buf_index] |= value << start_bit;
        } else {
            // Value spans 2 buffer cells.
            self.buf.push(0);

            self.buf[buf_index] |= value << start_bit;
            self.buf[buf_index + 1] |= value >> available_bits;
        }

        self.len += 1;
    }
}

pub struct PackedVecIntoIterator<T: PackedElement> {
    vec: PackedVec<T>,
    index: usize,
}

impl<T: PackedElement> IntoIterator for PackedVec<T> {
    type Item = u32;
    type IntoIter = PackedVecIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        PackedVecIntoIterator {
            vec: self,
            index: 0,
        }
    }
}

impl<T: PackedElement> Iterator for PackedVecIntoIterator<T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vec.get(self.index);
        self.index += 1;

        result
    }
}

pub struct PackedVecIterator<'a, T: PackedElement> {
    vec: &'a PackedVec<T>,
    index: usize,
}

impl<'a, T: PackedElement> IntoIterator for &'a PackedVec<T> {
    type Item = u32;
    type IntoIter = PackedVecIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        PackedVecIterator {
            vec: self,
            index: 0,
        }
    }
}

impl<'a, T: PackedElement> Iterator for PackedVecIterator<'a, T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vec.get(self.index);
        self.index += 1;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buflen_no_span() {
        let mut v = PackedVec::<U8>::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        assert_eq!(v.buf.len(), 1);

        v.push(5);
        assert_eq!(v.buf.len(), 2);
    }

    #[test]
    fn buflen_has_span() {
        let mut v = PackedVec::<U9>::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v.buf.len(), 1);

        v.push(4);
        assert_eq!(v.buf.len(), 2);
    }

    #[test]
    fn capacity() {
        let v1 = PackedVec::<U9>::with_capacity(7);
        assert_eq!(v1.buf.capacity(), 2);
        assert_eq!(v1.capacity(), 7);

        let v2 = PackedVec::<U9>::with_capacity(8);
        assert_eq!(v2.buf.capacity(), 3);
        assert_eq!(v2.capacity(), 10);
    }
}
