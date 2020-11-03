use std::marker::PhantomData;

pub mod packed_int;
pub use crate::packed_int::*;

pub struct PackedIntegers<T: PackedInt> {
    buf: Vec<u32>,
    len: usize,
    phantom: PhantomData<T>,
}

impl<T: PackedInt> PackedIntegers<T> {
    const U32_NUM_BITS: usize = 32;

    pub fn new() -> PackedIntegers<T> {
        PackedIntegers {
            buf: Vec::new(),
            len: 0,
            phantom: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> PackedIntegers<T> {
        let capacity = (T::NUM_BITS * capacity + (Self::U32_NUM_BITS - 1)) / Self::U32_NUM_BITS;

        PackedIntegers {
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
            None
        } else {
            Some(self.get_unchecked(index))
        }
    }

    pub fn get_unchecked(&self, index: usize) -> u32 {
        let buf_index = Self::buf_index(index);
        let start_bit = Self::start_bit(index);
        let available_bits = Self::available_bits(start_bit);

        if available_bits >= T::NUM_BITS {
            (self.buf[buf_index] >> start_bit) & T::MAX
        } else {
            // Value spans 2 buffer cells.
            let lo = self.buf[buf_index] >> start_bit;
            let hi = self.buf[buf_index + 1] << (Self::U32_NUM_BITS - start_bit);

            lo ^ ((lo ^ hi) & (T::MAX >> available_bits << available_bits))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> PackedIntegersIterator<'_, T> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn pop(&mut self) -> Option<u32> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.get_unchecked(self.len))
        }
    }

    pub fn push(&mut self, value: u32) {
        if value > T::MAX {
            panic!("value is outside the range 0..={}", T::MAX);
        }

        let buf_index = Self::buf_index(self.len);
        let start_bit = Self::start_bit(self.len);
        let available_bits = Self::available_bits(start_bit);

        if available_bits >= T::NUM_BITS {
            if buf_index == self.buf.len() {
                self.buf.push(0);
            }

            self.buf[buf_index] &= !(T::MAX << start_bit);
            self.buf[buf_index] |= value << start_bit;
        } else {
            // Value spans 2 buffer cells.
            self.buf.push(0);

            self.buf[buf_index] &= !(T::MAX << start_bit);
            self.buf[buf_index] |= value << start_bit;

            self.buf[buf_index + 1] = !(T::MAX >> (Self::U32_NUM_BITS - start_bit));
            self.buf[buf_index + 1] |= value >> available_bits;
        }

        self.len += 1;
    }

    pub fn set(&mut self, index: usize, value: u32) {
        if index >= self.len {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.len, index
            );
        } else {
            self.set_unchecked(index, value);
        }
    }

    pub fn set_unchecked(&mut self, index: usize, value: u32) {
        if value > T::MAX {
            panic!("value is outside the range 0..={}", T::MAX);
        }

        let buf_index = Self::buf_index(index);
        let start_bit = Self::start_bit(index);
        let available_bits = Self::available_bits(start_bit);

        if available_bits >= T::NUM_BITS {
            self.buf[buf_index] &= !(T::MAX << start_bit);
            self.buf[buf_index] |= value << start_bit;
        } else {
            // Value spans 2 buffer cells.
            self.buf[buf_index] &= !(T::MAX << start_bit);
            self.buf[buf_index] |= value << start_bit;

            self.buf[buf_index + 1] = !(T::MAX >> (Self::U32_NUM_BITS - start_bit));
            self.buf[buf_index + 1] |= value >> available_bits;
        }
    }

    #[inline]
    fn available_bits(start_bit: usize) -> usize {
        Self::U32_NUM_BITS - start_bit
    }

    #[inline]
    fn buf_index(index: usize) -> usize {
        index * T::NUM_BITS / Self::U32_NUM_BITS
    }

    #[inline]
    fn start_bit(index: usize) -> usize {
        index * T::NUM_BITS % Self::U32_NUM_BITS
    }
}

pub struct PackedIntegersIntoIterator<T: PackedInt> {
    vec: PackedIntegers<T>,
    index: usize,
}

impl<T: PackedInt> IntoIterator for PackedIntegers<T> {
    type Item = u32;
    type IntoIter = PackedIntegersIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        PackedIntegersIntoIterator {
            vec: self,
            index: 0,
        }
    }
}

impl<T: PackedInt> Iterator for PackedIntegersIntoIterator<T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vec.get(self.index);
        self.index += 1;

        result
    }
}

pub struct PackedIntegersIterator<'a, T: PackedInt> {
    vec: &'a PackedIntegers<T>,
    index: usize,
}

impl<'a, T: PackedInt> IntoIterator for &'a PackedIntegers<T> {
    type Item = u32;
    type IntoIter = PackedIntegersIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        PackedIntegersIterator {
            vec: self,
            index: 0,
        }
    }
}

impl<'a, T: PackedInt> Iterator for PackedIntegersIterator<'a, T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vec.get(self.index);
        self.index += 1;

        result
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! count_integers {
    (; $type:ident) => { 0 };
    ($head:expr; $type:ident) => { 1 };
    ($head:expr, $($tail:expr),*; $type:ident) => {
        1 + count_integers!($($tail),*; $type)
    };
}

#[macro_export]
macro_rules! packed_ints {
    (; $type:ident) => {
        PackedIntegers::<$type>::new()
    };
    ($($ints:expr),+; $type:ident) => {
        {
            let capacity = count_integers!($($ints),+; $type);
            let mut is = PackedIntegers::<$type>::with_capacity(capacity);
            $(
                is.push($ints);
            )*
            is
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buflen_has_span() {
        let mut v = PackedIntegers::<U9>::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v.buf.len(), 1);

        v.push(4);
        assert_eq!(v.buf.len(), 2);
    }

    #[test]
    fn buflen_no_span() {
        let mut v = PackedIntegers::<U8>::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        assert_eq!(v.buf.len(), 1);

        v.push(5);
        assert_eq!(v.buf.len(), 2);
    }

    #[test]
    fn capacity() {
        let v1 = PackedIntegers::<U9>::with_capacity(7);
        assert_eq!(v1.buf.capacity(), 2);
        assert_eq!(v1.capacity(), 7);

        let v2 = PackedIntegers::<U9>::with_capacity(8);
        assert_eq!(v2.buf.capacity(), 3);
        assert_eq!(v2.capacity(), 10);
    }
}
