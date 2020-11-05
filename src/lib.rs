//! `packed_integers` provides a growable array for integer types in the range `u1` to `u31`.
//!
//! # Use case
//!
//! Assume you have a sequence of unsigned integers in the range [0, 100000] that you would like to
//! hold in memory. That range of values can be represented using 17 bits per integer, since
//! 2<sup>17</sup> - 1 = 131071. As Rust has no `u17` type, you would typically store these values
//! in a `u32` array, wasting 15 bits per integer.
//!
//! `packed_integers` helps alleviate this issue by packing these integers at the bit level,
//! essentially trading time for space.
//!
//! # API
//!
//! Where possible, `packed_integers` mimics the API for Rust's `Vec` in order to provide a set of
//! methods you're probably already familiar with.

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;

mod packed_int;
pub use crate::packed_int::*;

/// A growable array of packed integers, backed by a `Vec<u32>` buffer.
///
/// # Examples
///
/// ```
/// use packed_integers::{PackedIntegers, U9};
///
/// let mut is = PackedIntegers::<U9>::new();
/// is.push(510);
/// is.push(511);
///
/// assert_eq!(is.len(), 2);
/// assert_eq!(is.get(0), Some(510));
///
/// assert_eq!(is.pop(), Some(511));
/// assert_eq!(is.len(), 1);
///
/// is.set(0, 509);
/// assert_eq!(is.get(0), Some(509));
///
/// // This will panic, as 512 > U9::MAX.
/// // is.push(512);
///
/// for i in &is {
///     println!("{}", i);
/// }
/// ```
///
/// The `packed_ints!` macro makes initialisation more convenient:
///
/// ```
/// use packed_integers::{packed_ints, U7, U9};
///
/// let mut is_u7 = packed_ints![125, 126, 127; U7];
///
/// let mut is_u9 = packed_ints![509, 510, 511; U9];
/// ```
#[derive(Clone)]
pub struct PackedIntegers<T: PackedInt> {
    buf: Vec<u32>,
    len: usize,
    phantom: PhantomData<T>,
}

impl<T: PackedInt> PackedIntegers<T> {
    const U32_NUM_BITS: usize = 32;

    /// Constructs a new, empty `PackedIntegers<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{PackedIntegers, U9};
    ///
    /// let mut is = PackedIntegers::<U9>::new();
    /// ```
    pub fn new() -> PackedIntegers<T> {
        PackedIntegers {
            buf: Vec::new(),
            len: 0,
            phantom: PhantomData,
        }
    }

    /// Constructs a new, empty `PackedIntegers<T>` with _at least_ the specified capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{PackedIntegers, U8};
    ///
    /// let mut is = PackedIntegers::<U8>::with_capacity(1);
    ///
    /// // The specified capacity is 1, but because `PackedIntegers` is backed by a `Vec<u32>`
    /// // buffer, it will actually hold 4 `U8`s without reallocating.
    /// assert_eq!(is.capacity(), 4);
    /// ```
    pub fn with_capacity(capacity: usize) -> PackedIntegers<T> {
        let capacity = Self::to_buf_capacity(capacity);

        PackedIntegers {
            buf: Vec::with_capacity(capacity),
            len: 0,
            phantom: PhantomData,
        }
    }

    /// Moves all integers of `other` into `Self`, leaving `other` empty.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U8};
    ///
    /// let mut is1 = packed_ints![10, 20, 30; U8];
    /// let mut is2 = packed_ints![40, 50, 60; U8];
    /// is1.append(&mut is2);
    ///
    /// assert_eq!(is1, packed_ints![10, 20, 30, 40, 50, 60; U8]);
    /// assert!(is2.is_empty());
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        self.reserve(other.len);

        for i in other.iter() {
            self.push(i);
        }
        other.clear();
    }

    /// Returns the number of integers the vector can hold without reallocating.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{PackedIntegers, U8};
    ///
    /// let mut is = PackedIntegers::<U8>::with_capacity(1);
    ///
    /// // The specified capacity is 1, but because `PackedIntegers` is backed by a `Vec<u32>`
    /// // buffer, it will actually hold 4 `U8`s without reallocating.
    /// assert_eq!(is.capacity(), 4);
    /// ```
    pub fn capacity(&self) -> usize {
        self.buf.capacity() * Self::U32_NUM_BITS / T::NUM_BITS
    }

    /// Clears the vector. This method does not affect the vector's allocated capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U9};
    ///
    /// let mut is = packed_ints![100, 200, 300; U9];
    ///
    /// is.clear();
    ///
    /// assert!(is.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.truncate(0)
    }

    /// Returns the value of the integer at position `index`, or `None` if out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U9};
    ///
    /// let mut is = packed_ints![100, 200, 300; U9];
    ///
    /// assert_eq!(is.get(1), Some(200));
    /// assert_eq!(is.get(3), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<u32> {
        if index >= self.len {
            None
        } else {
            Some(self.get_unchecked(index))
        }
    }

    fn get_unchecked(&self, index: usize) -> u32 {
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

    /// Inserts an integer at position `index`, shifting all integers after it to the right.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U8};
    ///
    /// let mut is = packed_ints![10, 20, 30; U8];
    ///
    /// is.insert(1, 40);
    /// assert_eq!(is, packed_ints![10, 40, 20, 30; U8]);
    ///
    /// is.insert(4, 50);
    /// assert_eq!(is, packed_ints![10, 40, 20, 30, 50; U8]);
    /// ```
    pub fn insert(&mut self, index: usize, value: u32) {
        if index > self.len {
            panic!(
                "insertion index (is {}) should be <= len (is {})",
                index, self.len
            );
        }

        self.push(value);
        for i in ((index + 1)..self.len).rev() {
            self.set_unchecked(i, self.get_unchecked(i - 1))
        }
        self.set_unchecked(index, value);
    }

    /// Returns `true` if the vector contains no integers.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{PackedIntegers, U8};
    ///
    /// let mut is = PackedIntegers::<U8>::new();
    /// assert!(is.is_empty());
    ///
    /// is.push(255);
    /// assert!(!is.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns an iterator over the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U9};
    ///
    /// let is = packed_ints![509, 510, 511; U9];
    /// let mut iter = is.iter();
    ///
    /// assert_eq!(iter.next(), Some(509));
    /// assert_eq!(iter.next(), Some(510));
    /// assert_eq!(iter.next(), Some(511));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> PackedIntegersIterator<'_, T> {
        self.into_iter()
    }

    /// Returns the number of integers in the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U9};
    ///
    /// let is = packed_ints![507, 508, 509, 510, 511; U9];
    ///
    /// assert_eq!(is.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Removes the last integer from the vector and returns it, or `None` if empty.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U10};
    ///
    /// let mut is = packed_ints![100, 200, 300; U10];
    ///
    /// assert_eq!(is.pop(), Some(300));
    /// assert_eq!(is, packed_ints![100, 200; U10]);
    /// ```
    pub fn pop(&mut self) -> Option<u32> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.get_unchecked(self.len))
        }
    }

    /// Appends an integer to the back of the vector.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U10};
    ///
    /// let mut is = packed_ints![100, 200; U10];
    /// is.push(300);
    ///
    /// assert_eq!(is, packed_ints![100, 200, 300; U10]);
    /// ```
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

    /// Removes and returns the integer at position `index`, shifting all integers after it to the
    /// left.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U8};
    ///
    /// let mut is = packed_ints![10, 20, 30; U8];
    ///
    /// assert_eq!(is.remove(1), 20);
    /// assert_eq!(is, packed_ints![10, 30; U8]);
    /// ```
    pub fn remove(&mut self, index: usize) -> u32 {
        if index >= self.len {
            panic!(
                "removal index (is {}) should be < len (is {})",
                index, self.len
            );
        }

        let result = self.get_unchecked(index);

        for i in (index + 1)..self.len {
            self.set_unchecked(i - 1, self.get_unchecked(i));
        }
        self.len -= 1;

        result
    }

    /// Reserves capacity for _at least_ `additional` more integers to be inserted.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U8};
    ///
    /// let mut is = packed_ints![100; U8];
    /// is.reserve(4);
    ///
    /// assert!(is.capacity() >= 5);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        if self.capacity() >= self.len + additional {
            return;
        }
        let additional = Self::to_buf_capacity(additional);
        self.buf.reserve(additional);
    }

    /// Sets the integer value at `index` to `value`.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U9};
    ///
    /// let mut is = packed_ints![100, 200, 300; U9];
    /// is.set(1, 400);
    ///
    /// assert_eq!(is, packed_ints![100, 400, 300; U9]);
    /// ```
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

    fn set_unchecked(&mut self, index: usize, value: u32) {
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

    /// Keeps the first `len` integers, and drops the rest.
    ///
    /// # Example
    ///
    /// ```
    /// use packed_integers::{packed_ints, U9};
    ///
    /// let mut is = packed_ints![100, 200, 300, 400, 500; U9];
    /// is.truncate(2);
    ///
    /// assert_eq!(is, packed_ints![100, 200; U9]);
    /// ```
    pub fn truncate(&mut self, len: usize) {
        if len > self.len {
            return;
        }
        self.len = len;
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

    #[inline]
    fn to_buf_capacity(capacity: usize) -> usize {
        (T::NUM_BITS * capacity + (Self::U32_NUM_BITS - 1)) / Self::U32_NUM_BITS
    }
}

/// A consuming iterator for `PackedIntegers`.
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

/// An iterator for `PackedIntegers`.
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

impl<T: PackedInt> Eq for PackedIntegers<T> {}

impl<T: PackedInt> PartialEq for PackedIntegers<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.cmp(other) == Ordering::Equal
    }
}

impl<T: PackedInt> Ord for PackedIntegers<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut o_iter = other.iter();
        for s in self {
            let o = match o_iter.next() {
                Some(o) => o,
                None => return Ordering::Greater,
            };

            match s.cmp(&o) {
                Ordering::Equal => continue,
                cmp => return cmp,
            }
        }

        match o_iter.next() {
            Some(_) => Ordering::Less,
            None => Ordering::Equal,
        }
    }
}

impl<T: PackedInt> PartialOrd for PackedIntegers<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PackedInt> Debug for PackedIntegers<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "(U{}) ", T::NUM_BITS)?;
        formatter.debug_list().entries(self.iter()).finish()
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! count_integers {
    (; $type:ident) => { 0 };
    ($head:expr; $type:ident) => { 1 };
    ($head:expr, $($tail:expr),*; $type:ident) => {
        1 + $crate::count_integers!($($tail),*; $type)
    };
}

/// A macro for a more convenient initialisation of `PackedIntegers`.
#[macro_export]
macro_rules! packed_ints {
    (; $type:ident) => {
        PackedIntegers::<$type>::new()
    };
    ($($ints:expr),+; $type:ident) => {
        {
            let capacity = $crate::count_integers!($($ints),+; $type);
            let mut is = $crate::PackedIntegers::<$type>::with_capacity(capacity);
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

    #[test]
    #[should_panic]
    fn get_unchecked() {
        let v = packed_ints![251, 252, 253, 254, 255; U8];

        assert_eq!(v.get_unchecked(0), 251);
        assert_eq!(v.get_unchecked(1), 252);
        assert_eq!(v.get_unchecked(2), 253);
        assert_eq!(v.get_unchecked(3), 254);
        assert_eq!(v.get_unchecked(4), 255);

        // UB if index >= len.
        assert_eq!(v.get_unchecked(5), 0); // Fine.
        assert_eq!(v.get_unchecked(6), 0); // Fine.
        assert_eq!(v.get_unchecked(7), 0); // Fine.
        v.get_unchecked(8); // Panics.
    }

    #[test]
    fn reserve() {
        let mut v = packed_ints![100; U8];
        v.reserve(4);

        assert!(v.buf.capacity() >= 2);
    }

    #[test]
    fn reserve_sufficient_capacity() {
        let mut v = packed_ints![100; U8];
        v.reserve(3);

        assert_eq!(v.buf.capacity(), 1);
    }

    #[test]
    #[should_panic]
    fn set_unchecked() {
        let mut v = packed_ints![100; U8];
        // UB if index >= len.
        v.set_unchecked(1, 101); // Fine.
        v.set_unchecked(2, 102); // Fine.
        v.set_unchecked(3, 103); // Fine.
        v.set_unchecked(4, 104); // Panics.
    }
}
