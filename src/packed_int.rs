#[derive(Clone)]
pub enum U1 {}
#[derive(Clone)]
pub enum U2 {}
#[derive(Clone)]
pub enum U3 {}
#[derive(Clone)]
pub enum U4 {}
#[derive(Clone)]
pub enum U5 {}
#[derive(Clone)]
pub enum U6 {}
#[derive(Clone)]
pub enum U7 {}
#[derive(Clone)]
pub enum U8 {}
#[derive(Clone)]
pub enum U9 {}
#[derive(Clone)]
pub enum U10 {}
#[derive(Clone)]
pub enum U11 {}
#[derive(Clone)]
pub enum U12 {}
#[derive(Clone)]
pub enum U13 {}
#[derive(Clone)]
pub enum U14 {}
#[derive(Clone)]
pub enum U15 {}
#[derive(Clone)]
pub enum U16 {}
#[derive(Clone)]
pub enum U17 {}
#[derive(Clone)]
pub enum U18 {}
#[derive(Clone)]
pub enum U19 {}
#[derive(Clone)]
pub enum U20 {}
#[derive(Clone)]
pub enum U21 {}
#[derive(Clone)]
pub enum U22 {}
#[derive(Clone)]
pub enum U23 {}
#[derive(Clone)]
pub enum U24 {}
#[derive(Clone)]
pub enum U25 {}
#[derive(Clone)]
pub enum U26 {}
#[derive(Clone)]
pub enum U27 {}
#[derive(Clone)]
pub enum U28 {}
#[derive(Clone)]
pub enum U29 {}
#[derive(Clone)]
pub enum U30 {}
#[derive(Clone)]
pub enum U31 {}

/// A trait for packed integer types.
pub trait PackedInt {
    /// The number of bits in this integer type.
    const NUM_BITS: usize;

    /// The largest value this integer type can hold.
    const MAX: u32 = (1 << Self::NUM_BITS) - 1;
}

impl PackedInt for U1 {
    const NUM_BITS: usize = 1;
}
impl PackedInt for U2 {
    const NUM_BITS: usize = 2;
}
impl PackedInt for U3 {
    const NUM_BITS: usize = 3;
}
impl PackedInt for U4 {
    const NUM_BITS: usize = 4;
}
impl PackedInt for U5 {
    const NUM_BITS: usize = 5;
}
impl PackedInt for U6 {
    const NUM_BITS: usize = 6;
}
impl PackedInt for U7 {
    const NUM_BITS: usize = 7;
}
impl PackedInt for U8 {
    const NUM_BITS: usize = 8;
}
impl PackedInt for U9 {
    const NUM_BITS: usize = 9;
}
impl PackedInt for U10 {
    const NUM_BITS: usize = 10;
}
impl PackedInt for U11 {
    const NUM_BITS: usize = 11;
}
impl PackedInt for U12 {
    const NUM_BITS: usize = 12;
}
impl PackedInt for U13 {
    const NUM_BITS: usize = 13;
}
impl PackedInt for U14 {
    const NUM_BITS: usize = 14;
}
impl PackedInt for U15 {
    const NUM_BITS: usize = 15;
}
impl PackedInt for U16 {
    const NUM_BITS: usize = 16;
}
impl PackedInt for U17 {
    const NUM_BITS: usize = 17;
}
impl PackedInt for U18 {
    const NUM_BITS: usize = 18;
}
impl PackedInt for U19 {
    const NUM_BITS: usize = 19;
}
impl PackedInt for U20 {
    const NUM_BITS: usize = 20;
}
impl PackedInt for U21 {
    const NUM_BITS: usize = 21;
}
impl PackedInt for U22 {
    const NUM_BITS: usize = 22;
}
impl PackedInt for U23 {
    const NUM_BITS: usize = 23;
}
impl PackedInt for U24 {
    const NUM_BITS: usize = 24;
}
impl PackedInt for U25 {
    const NUM_BITS: usize = 25;
}
impl PackedInt for U26 {
    const NUM_BITS: usize = 26;
}
impl PackedInt for U27 {
    const NUM_BITS: usize = 27;
}
impl PackedInt for U28 {
    const NUM_BITS: usize = 28;
}
impl PackedInt for U29 {
    const NUM_BITS: usize = 29;
}
impl PackedInt for U30 {
    const NUM_BITS: usize = 30;
}
impl PackedInt for U31 {
    const NUM_BITS: usize = 31;
}
