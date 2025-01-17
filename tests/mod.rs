use iter_index::IndexerIterator;

#[test]
fn tests() {
    let items = vec!["a", "b", "c"];
    let result = items.iter().index::<i32>().collect::<Vec<_>>();
    assert_eq!(result, vec![(0_i32, &"a"), (1_i32, &"b"), (2_i32, &"c")]);

    let result = items.iter().index_start::<u8>(97).collect::<Vec<_>>();
    assert_eq!(result, vec![(97_u8, &"a"), (98_u8, &"b"), (99_u8, &"c")]);

    let result = items.into_iter().index_step::<i16>(100, 10).collect::<Vec<_>>();
    assert_eq!(result, vec![(100_i16, "a"), (110_i16, "b"), (120_i16, "c")]);

    let items = 'a'..='z';
    let mut result = items.index_step(100, 10);
    assert_eq!(result.next(), Some((100, 'a')));
    assert_eq!(result.nth(5), Some((160, 'g')));
}

