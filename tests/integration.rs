use packed_integers::*;
use std::cmp::Ordering;

#[test]
fn append() {
    let mut v1 = packed_ints![1, 2; U9];
    let mut v2 = packed_ints![3, 4, 5; U9];
    v1.append(&mut v2);

    assert_eq!(v1, packed_ints![1, 2, 3, 4, 5; U9]);
}

#[test]
fn append_empty() {
    let mut v1 = packed_ints![1, 2; U8];
    let mut v2 = packed_ints![; U8];
    v1.append(&mut v2);

    assert_eq!(v1, packed_ints![1, 2; U8]);
}

#[test]
fn clear() {
    let mut v = packed_ints![251, 252, 253, 254, 255; U8];
    v.clear();

    assert_eq!(v.len(), 0);
    assert_eq!(v, packed_ints![; U8]);
}

#[test]
fn get_has_span() {
    let v = packed_ints![507, 508, 509, 510, 511; U9];

    assert_eq!(v.len(), 5);
    assert_eq!(v.get(0), Some(507));
    assert_eq!(v.get(1), Some(508));
    assert_eq!(v.get(2), Some(509));
    assert_eq!(v.get(3), Some(510));
    assert_eq!(v.get(4), Some(511));
    assert_eq!(v.get(5), None);
}

#[test]
fn get_no_span() {
    let v = packed_ints![251, 252, 253, 254, 255; U8];

    assert_eq!(v.len(), 5);
    assert_eq!(v.get(0), Some(251));
    assert_eq!(v.get(1), Some(252));
    assert_eq!(v.get(2), Some(253));
    assert_eq!(v.get(3), Some(254));
    assert_eq!(v.get(4), Some(255));
    assert_eq!(v.get(5), None);
}

#[test]
fn insert() {
    let mut v = packed_ints![1, 2, 3; U8];
    v.insert(0, 4);

    assert_eq!(v.len(), 4);
    assert_eq!(v, packed_ints![4, 1, 2, 3; U8]);

    v.insert(2, 5);
    assert_eq!(v.len(), 5);
    assert_eq!(v, packed_ints![4, 1, 5, 2, 3; U8]);
}

#[test]
fn insert_empty() {
    let mut v = packed_ints![; U8];
    v.insert(0, 4);

    assert_eq!(v.len(), 1);
    assert_eq!(v, packed_ints![4; U8]);
}

#[test]
fn insert_eq_len() {
    let mut v = packed_ints![1; U8];
    v.insert(1, 4);

    assert_eq!(v.len(), 2);
    assert_eq!(v, packed_ints![1, 4; U8]);
}

#[test]
#[should_panic]
fn insert_gt_len() {
    let mut v = packed_ints![1; U8];
    v.insert(2, 4);
}

#[test]
fn into_iter_move() {
    let v = packed_ints![251, 252, 253, 254, 255; U8];

    let mut iter = v.into_iter();
    assert_eq!(iter.next(), Some(251));
    assert_eq!(iter.next(), Some(252));
    assert_eq!(iter.next(), Some(253));
    assert_eq!(iter.next(), Some(254));
    assert_eq!(iter.next(), Some(255));
    assert_eq!(iter.next(), None);

    // Moved. Compile error:
    // v.push(250);
}

#[test]
fn into_iter_ref() {
    let v = packed_ints![507, 508, 509, 510, 511; U9];

    let mut iter = (&v).into_iter();
    assert_eq!(iter.next(), Some(507));
    assert_eq!(iter.next(), Some(508));
    assert_eq!(iter.next(), Some(509));
    assert_eq!(iter.next(), Some(510));
    assert_eq!(iter.next(), Some(511));
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
    assert_eq!(iter.next(), Some(507));
    assert_eq!(iter.next(), Some(508));
    assert_eq!(iter.next(), Some(509));
    assert_eq!(iter.next(), Some(510));
    assert_eq!(iter.next(), Some(511));
    assert_eq!(iter.next(), None);

    // Ok:
    // v.push(506);
}

#[test]
fn ord_eq() {
    let v1 = packed_ints![; U8];
    let v2 = packed_ints![; U8];

    assert_eq!(v1.cmp(&v2), Ordering::Equal);

    let v3 = packed_ints![1, 2; U8];
    let v4 = packed_ints![1, 2; U8];

    assert_eq!(v3.cmp(&v4), Ordering::Equal);
}

#[test]
fn ord_gt() {
    let v1 = packed_ints![1, 4, 3; U8];
    let v2 = packed_ints![1, 2, 3; U8];

    assert_eq!(v1.cmp(&v2), Ordering::Greater);

    let v3 = packed_ints![1, 4; U8];
    let v4 = packed_ints![1, 2, 3; U8];

    assert_eq!(v3.cmp(&v4), Ordering::Greater);

    let v5 = packed_ints![1, 2, 3, 4; U8];
    let v6 = packed_ints![1, 2, 3; U8];

    assert_eq!(v5.cmp(&v6), Ordering::Greater);
}

#[test]
fn ord_lt() {
    let v1 = packed_ints![1, 2, 3; U8];
    let v2 = packed_ints![1, 4, 3; U8];

    assert_eq!(v1.cmp(&v2), Ordering::Less);

    let v3 = packed_ints![1, 2, 3; U8];
    let v4 = packed_ints![1, 4; U8];

    assert_eq!(v3.cmp(&v4), Ordering::Less);

    let v5 = packed_ints![1, 2, 3; U8];
    let v6 = packed_ints![1, 2, 3, 4; U8];

    assert_eq!(v5.cmp(&v6), Ordering::Less);
}

#[test]
fn pop() {
    let mut v = packed_ints![100, 200, 300, 400, 500; U10];

    assert_eq!(v.pop(), Some(500));
    assert_eq!(v.pop(), Some(400));
    assert_eq!(v, packed_ints![100, 200, 300; U10]);

    v.push(600);
    assert_eq!(v, packed_ints![100, 200, 300, 600; U10]);
}

#[test]
fn pop_none() {
    let mut v = packed_ints![; U10];

    assert_eq!(v.pop(), None);
}

#[test]
fn pop_one() {
    let mut v = packed_ints![100; U10];

    assert_eq!(v.pop(), Some(100));
    assert_eq!(v.pop(), None);
}

#[test]
fn push_eq_max() {
    let mut v = PackedIntegers::<U10>::new();
    v.push(1023);

    assert_eq!(v, packed_ints![1023; U10]);
}

#[test]
#[should_panic]
fn push_gt_max() {
    let mut v = PackedIntegers::<U10>::new();
    v.push(1024);
}

#[test]
fn remove() {
    let mut v = packed_ints![251, 252, 253, 254, 255; U8];
    v.remove(0);

    assert_eq!(v.len(), 4);
    assert_eq!(v, packed_ints![252, 253, 254, 255; U8]);

    v.remove(2);
    assert_eq!(v.len(), 3);
    assert_eq!(v, packed_ints![252, 253, 255; U8]);
}

#[test]
#[should_panic]
fn remove_eq_len() {
    let mut v = packed_ints![251, 252; U8];
    v.remove(2);
}

#[test]
fn set() {
    let mut v = packed_ints![251, 252, 253, 254, 255; U8];
    v.set(0, 100);
    v.set(2, 150);
    v.set(4, 200);

    assert_eq!(v, packed_ints![100, 252, 150, 254, 200; U8]);
}

#[test]
#[should_panic]
fn set_oob() {
    let mut v = packed_ints![100; U8];
    v.set(1, 200);
}

#[test]
fn truncate() {
    let mut v = packed_ints![251, 252, 253, 254, 255; U8];
    v.truncate(2);

    assert_eq!(v.len(), 2);
    assert_eq!(v, packed_ints![251, 252; U8]);

    v.push(200);
    assert_eq!(v.len(), 3);
    assert_eq!(v, packed_ints![251, 252, 200; U8]);
}

#[test]
fn truncate_gt() {
    let mut v = packed_ints![251, 252, 253, 254, 255; U8];
    v.truncate(10);

    assert_eq!(v.len(), 5);
    assert_eq!(v, packed_ints![251, 252, 253, 254, 255; U8]);
}
