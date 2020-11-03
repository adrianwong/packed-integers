use packed_integers::*;

#[test]
fn get_has_span() {
    let v = packed_ints![507, 508, 509, 510, 511; U9];

    assert_eq!(v.len(), 5);
    assert_eq!(v.get(0).unwrap(), 507);
    assert_eq!(v.get(1).unwrap(), 508);
    assert_eq!(v.get(2).unwrap(), 509);
    assert_eq!(v.get(3).unwrap(), 510);
    assert_eq!(v.get(4).unwrap(), 511);
    assert_eq!(v.get(5), None);
}

#[test]
fn get_no_span() {
    let v = packed_ints![251, 252, 253, 254, 255; U8];

    assert_eq!(v.len(), 5);
    assert_eq!(v.get(0).unwrap(), 251);
    assert_eq!(v.get(1).unwrap(), 252);
    assert_eq!(v.get(2).unwrap(), 253);
    assert_eq!(v.get(3).unwrap(), 254);
    assert_eq!(v.get(4).unwrap(), 255);
    assert_eq!(v.get(5), None);
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
fn into_iter_move() {
    let v = packed_ints![251, 252, 253, 254, 255; U8];

    let mut iter = v.into_iter();
    assert_eq!(iter.next().unwrap(), 251);
    assert_eq!(iter.next().unwrap(), 252);
    assert_eq!(iter.next().unwrap(), 253);
    assert_eq!(iter.next().unwrap(), 254);
    assert_eq!(iter.next().unwrap(), 255);
    assert_eq!(iter.next(), None);

    // Moved. Compile error:
    // v.push(250);
}

#[test]
fn into_iter_ref() {
    let v = packed_ints![507, 508, 509, 510, 511; U9];

    let mut iter = (&v).into_iter();
    assert_eq!(iter.next().unwrap(), 507);
    assert_eq!(iter.next().unwrap(), 508);
    assert_eq!(iter.next().unwrap(), 509);
    assert_eq!(iter.next().unwrap(), 510);
    assert_eq!(iter.next().unwrap(), 511);
    assert_eq!(iter.next(), None);

    // Ok:
    // v.push(506);
}

#[test]
fn is_empty() {
    let mut v = packed_ints![; U31];
    assert!(v.is_empty());

    v.push(12345);
    assert!(!v.is_empty());
}

#[test]
fn iter() {
    let v = packed_ints![507, 508, 509, 510, 511; U9];

    let mut iter = v.iter();
    assert_eq!(iter.next().unwrap(), 507);
    assert_eq!(iter.next().unwrap(), 508);
    assert_eq!(iter.next().unwrap(), 509);
    assert_eq!(iter.next().unwrap(), 510);
    assert_eq!(iter.next().unwrap(), 511);
    assert_eq!(iter.next(), None);

    // Ok:
    // v.push(506);
}

#[test]
fn pop() {
    let mut v = packed_ints![100, 200, 300, 400, 500; U10];

    assert_eq!(v.pop().unwrap(), 500);
    assert_eq!(v.pop().unwrap(), 400);
    assert_eq!(v.get(0).unwrap(), 100);
    assert_eq!(v.get(1).unwrap(), 200);
    assert_eq!(v.get(2).unwrap(), 300);
    assert_eq!(v.get(3), None);

    v.push(600);
    assert_eq!(v.get(0).unwrap(), 100);
    assert_eq!(v.get(1).unwrap(), 200);
    assert_eq!(v.get(2).unwrap(), 300);
    assert_eq!(v.get(3).unwrap(), 600);
    assert_eq!(v.get(4), None);
}

#[test]
fn pop_none() {
    let mut v = packed_ints![; U10];

    assert_eq!(v.pop(), None);
}

#[test]
fn pop_one() {
    let mut v = packed_ints![100; U10];

    assert_eq!(v.pop().unwrap(), 100);
    assert_eq!(v.pop(), None);
}

#[test]
fn push_eq_max() {
    let mut v = PackedIntegers::<U10>::new();
    v.push(1023);
}

#[test]
#[should_panic]
fn push_gt_max() {
    let mut v = PackedIntegers::<U10>::new();
    v.push(1024);
}

#[test]
fn set() {
    let mut v = packed_ints![251, 252, 253, 254, 255; U8];
    v.set(0, 100);
    v.set(2, 150);
    v.set(4, 200);

    assert_eq!(v.get(0).unwrap(), 100);
    assert_eq!(v.get(1).unwrap(), 252);
    assert_eq!(v.get(2).unwrap(), 150);
    assert_eq!(v.get(3).unwrap(), 254);
    assert_eq!(v.get(4).unwrap(), 200);
}

#[test]
#[should_panic]
fn set_oob() {
    let mut v = packed_ints![100; U8];
    v.set(1, 200);
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
