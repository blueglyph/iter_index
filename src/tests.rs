#![cfg(test)]

use crate::IndexerIterator;

#[test]
fn index() {
    let items = vec!["a", "b", "c"];
    let result = items.into_iter().index::<i32>().collect::<Vec<_>>();
    assert_eq!(result, vec![(0_i32, "a"), (1_i32, "b"), (2_i32, "c")]);
}

#[test]
fn index_start() {
    let items = vec!["a", "b", "c"];
    let result = items.into_iter().index_start::<u8>(97).collect::<Vec<_>>();
    assert_eq!(result, vec![(97_u8, "a"), (98_u8, "b"), (99_u8, "c")]);
}

#[test]
fn index_step() {
    let items = vec!["a", "b", "c"];
    let result = items.into_iter().index_step::<i16>(100, 10).collect::<Vec<_>>();
    assert_eq!(result, vec![(100_i16, "a"), (110_i16, "b"), (120_i16, "c")]);
}

#[test]
fn index_nth() {
    let items = 'a' ..= 'z';
    let mut result = items.index_step(100, 10);
    assert_eq!(result.next(), Some((100, 'a')));
    assert_eq!(result.nth(5), Some((160, 'g')));
}

#[should_panic(expected = "Cannot convert n into u8")]
#[test]
fn index_nth_error() {
    let items = 1_u32 ..= 1000;
    let mut result = items.index_step::<u8>(10, 2);
    assert_eq!(result.nth(500), Some(((1010 & 255) as u8, 501_u32)));
}

#[test]
fn index_len() {
    let items = vec!["a", "b", "c"];
    assert_eq!(items.into_iter().index::<u8>().len(), 3);
}
