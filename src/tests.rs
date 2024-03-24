#![cfg(test)]

use crate::*;

#[test]
fn basic_equality() {
    let v1 = ValueUnion::from([1, 2, 3]);
    assert!(v1 == 1);
    assert!(v1 == 2);
    assert!(v1 == 3);
}

#[test]
fn basic_inequality() {
    let v1 = ValueUnion::from([1, 2, 3]);
    assert!(v1 != 4);
    assert!(v1 != 5);
    assert!(v1 != 6);
}

#[test]
fn self_equality() {
    let v1 = ValueUnion::from([1, 2, 3]);
    let v2 = ValueUnion::from([1, 2, 3]);
    assert_eq!(v1, v2);
}

#[test]
fn self_inequality() {
    let v1 = ValueUnion::from([1, 2, 3]);
    let v2 = ValueUnion::from([4, 5, 6]);
    assert_ne!(v1, v2);
}

#[test]
fn unite_macro() {
    let v1 = ValueUnion::from([1, 2, 3]);
    let v2 = unite!(1, 2, 3);
    assert_eq!(v1, v2);
}
