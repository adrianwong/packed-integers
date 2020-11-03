use packed_integers::*;

#[test]
fn append() {
    let mut v1 = packed_ints![1, 2; U9];
    let mut v2 = packed_ints![3, 4, 5; U9];
    v1.append(&mut v2);

    assert_eq!(v1.get(0).unwrap(), 1);
    assert_eq!(v1.get(1).unwrap(), 2);
    assert_eq!(v1.get(2).unwrap(), 3);
    assert_eq!(v1.get(3).unwrap(), 4);
    assert_eq!(v1.get(4).unwrap(), 5);
    assert_eq!(v1.get(5), None);
    assert!(v2.is_empty());
}

#[test]
fn append_empty() {
    let mut v1 = packed_ints![1, 2; U8];
    let mut v2 = packed_ints![; U8];
    v1.append(&mut v2);

    assert_eq!(v1.get(0).unwrap(), 1);
    assert_eq!(v1.get(1).unwrap(), 2);
    assert_eq!(v1.get(2), None);
    assert!(v2.is_empty());
}

#[test]
fn clear() {
    let mut v = packed_ints![251, 252, 253, 254, 255; U8];
    v.clear();

    assert_eq!(v.len(), 0);
    assert_eq!(v.get(0), None);
}

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
fn insert() {
    let mut v = packed_ints![1, 2, 3; U8];
    v.insert(0, 4);

    assert_eq!(v.len(), 4);
    assert_eq!(v.get(0).unwrap(), 4);
    assert_eq!(v.get(1).unwrap(), 1);
    assert_eq!(v.get(2).unwrap(), 2);
    assert_eq!(v.get(3).unwrap(), 3);
    assert_eq!(v.get(4), None);

    v.insert(2, 5);
    assert_eq!(v.len(), 5);
    assert_eq!(v.get(0).unwrap(), 4);
    assert_eq!(v.get(1).unwrap(), 1);
    assert_eq!(v.get(2).unwrap(), 5);
    assert_eq!(v.get(3).unwrap(), 2);
    assert_eq!(v.get(4).unwrap(), 3);
    assert_eq!(v.get(5), None);
}

#[test]
fn insert_empty() {
    let mut v = packed_ints![; U8];
    v.insert(0, 4);

    assert_eq!(v.len(), 1);
    assert_eq!(v.get(0).unwrap(), 4);
}

#[test]
fn insert_eq_len() {
    let mut v = packed_ints![1; U8];
    v.insert(1, 4);

    assert_eq!(v.len(), 2);
    assert_eq!(v.get(0).unwrap(), 1);
    assert_eq!(v.get(1).unwrap(), 4);
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
fn truncate() {
    let mut v = packed_ints![251, 252, 253, 254, 255; U8];
    v.truncate(2);

    assert_eq!(v.len(), 2);
    assert_eq!(v.get(0).unwrap(), 251);
    assert_eq!(v.get(1).unwrap(), 252);
    assert_eq!(v.get(2), None);

    v.push(200);
    assert_eq!(v.len(), 3);
    assert_eq!(v.get(0).unwrap(), 251);
    assert_eq!(v.get(1).unwrap(), 252);
    assert_eq!(v.get(2).unwrap(), 200);
    assert_eq!(v.get(3), None);
}

#[test]
fn truncate_gt() {
    let mut v = packed_ints![251, 252, 253, 254, 255; U8];
    v.truncate(10);

    assert_eq!(v.len(), 5);
    assert_eq!(v.get(0).unwrap(), 251);
    assert_eq!(v.get(1).unwrap(), 252);
    assert_eq!(v.get(2).unwrap(), 253);
    assert_eq!(v.get(3).unwrap(), 254);
    assert_eq!(v.get(4).unwrap(), 255);
    assert_eq!(v.get(5), None);
}
