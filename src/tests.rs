#![cfg(test)]

use std::collections::HashMap;
use crate::IndexerIterator;

#[test]
fn index() {
    let items = vec!["a", "b", "c"];
    let result = items.into_iter().index::<i32>().collect::<Vec<_>>();
    assert_eq!(result, vec![(0_i32, "a"), (1_i32, "b"), (2_i32, "c")]);
}

#[test]
fn index_type_induced() {
    let mut map = HashMap::<u8, &str>::new();
    for (k, v) in vec!["a", "b", "c"].into_iter().index() {
        map.insert(k, v);
    }
    assert_eq!(map, HashMap::from([(0_u8, "a"), (1_u8, "b"), (2_u8, "c")]));
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

#[test]
fn index_double_ended_iterator() {
    let items = vec!["a", "b", "c"];
    let result = items.into_iter().index::<i32>().rev().collect::<Vec<_>>();
    assert_eq!(result, vec![(2_i32, "c"), (1_i32, "b"), (0_i32, "a")]);

    let items = vec!["a", "b", "c"];
    let result = items.into_iter().index_start::<u8>(97).rev().collect::<Vec<_>>();
    assert_eq!(result, vec![(99_u8, "c"), (98_u8, "b"), (97_u8, "a")]);

    let items = vec!["a", "b", "c"];
    let result = items.into_iter().index_step::<i16>(100, 10).rev().collect::<Vec<_>>();
    assert_eq!(result, vec![(120_i16, "c"), (110_i16, "b"), (100_i16, "a")]);

    let items = vec!["a", "b", "c", "d"];
    let mut result = items.into_iter().index_step::<i16>(100, 10);
    assert_eq!(result.nth_back(2), Some((110, "b")));
}
