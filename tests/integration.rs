use packed_vec::*;

#[test]
fn push_eq_max() {
    let mut v = PackedVec::<U10>::new();
    v.push(1023);
}

#[test]
#[should_panic]
fn push_gt_max() {
    let mut v = PackedVec::<U10>::new();
    v.push(1024);
}

#[test]
fn get_no_span() {
    let v1 = vec![251, 252, 253, 254, 255];

    let mut v2 = PackedVec::<U8>::new();
    for x in &v1 {
        v2.push(*x);
    }

    assert_eq!(v2.len(), v1.len());
    assert_eq!(v2.get(0).unwrap(), v1[0]);
    assert_eq!(v2.get(1).unwrap(), v1[1]);
    assert_eq!(v2.get(2).unwrap(), v1[2]);
    assert_eq!(v2.get(3).unwrap(), v1[3]);
    assert_eq!(v2.get(4).unwrap(), v1[4]);
}

#[test]
fn get_has_span() {
    let v1 = vec![507, 508, 509, 510, 511];

    let mut v2 = PackedVec::<U9>::new();
    for x in &v1 {
        v2.push(*x);
    }

    assert_eq!(v2.len(), v1.len());
    assert_eq!(v2.get(0).unwrap(), v1[0]);
    assert_eq!(v2.get(1).unwrap(), v1[1]);
    assert_eq!(v2.get(2).unwrap(), v1[2]);
    assert_eq!(v2.get(3).unwrap(), v1[3]);
    assert_eq!(v2.get(4).unwrap(), v1[4]);
}

#[test]
fn into_iter_move() {
    let v1 = vec![251, 252, 253, 254, 255];

    let mut v2 = PackedVec::<U8>::new();
    for x in &v1 {
        v2.push(*x);
    }

    let mut iter = v2.into_iter();
    assert_eq!(iter.next().unwrap(), v1[0]);
    assert_eq!(iter.next().unwrap(), v1[1]);
    assert_eq!(iter.next().unwrap(), v1[2]);
    assert_eq!(iter.next().unwrap(), v1[3]);
    assert_eq!(iter.next().unwrap(), v1[4]);
    assert_eq!(iter.next(), None);

    // Moved. Compile error:
    // v2.push(250);
}

#[test]
fn into_iter_ref() {
    let v1 = vec![507, 508, 509, 510, 511];

    let mut v2 = PackedVec::<U9>::new();
    for x in &v1 {
        v2.push(*x);
    }

    let mut iter = (&v2).into_iter();
    assert_eq!(iter.next().unwrap(), v1[0]);
    assert_eq!(iter.next().unwrap(), v1[1]);
    assert_eq!(iter.next().unwrap(), v1[2]);
    assert_eq!(iter.next().unwrap(), v1[3]);
    assert_eq!(iter.next().unwrap(), v1[4]);
    assert_eq!(iter.next(), None);

    // Ok:
    // v2.push(506);
}

#[test]
fn iter() {
    let v1 = vec![507, 508, 509, 510, 511];

    let mut v2 = PackedVec::<U9>::new();
    for x in &v1 {
        v2.push(*x);
    }

    let mut iter = v2.iter();
    assert_eq!(iter.next().unwrap(), v1[0]);
    assert_eq!(iter.next().unwrap(), v1[1]);
    assert_eq!(iter.next().unwrap(), v1[2]);
    assert_eq!(iter.next().unwrap(), v1[3]);
    assert_eq!(iter.next().unwrap(), v1[4]);
    assert_eq!(iter.next(), None);

    // Ok:
    // v2.push(506);
}
