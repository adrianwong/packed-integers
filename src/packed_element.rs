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
