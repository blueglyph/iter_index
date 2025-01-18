// Copyright 2025 Redglyph
//

//! This crate provides a simple extension trait that provides a more flexible alternative to the iterator's method `enumerate()`.
//!
//! It allows to:
//!  * use a custom type for the index with `index::<T>()`
//!  * define a custom start value with `index_start::<T>(start: T)`
//!  * define a custom step value with `index_step::<T>(start: T, step: T)`.
//!
//! ```rust
//! use iter_index::IndexerIterator;
//!
//! let items = vec!["a", "b", "c"];
//! let result = items.iter().index::<i32>().collect::<Vec<_>>();
//! assert_eq!(result, vec![(0_i32, &"a"), (1_i32, &"b"), (2_i32, &"c")]);
//!
//! let result = items.iter().index_start::<u8>(97).collect::<Vec<_>>();
//! assert_eq!(result, vec![(97_u8, &"a"), (98_u8, &"b"), (99_u8, &"c")]);
//!
//! let result = items.into_iter().index_step::<i16>(100, 10).collect::<Vec<_>>();
//! assert_eq!(result, vec![(100_i16, "a"), (110_i16, "b"), (120_i16, "c")]);
//!
//! let items = 'a'..='z';
//! let mut result = items.index_step(100, 10);
//! assert_eq!(result.next(), Some((100, 'a')));
//! assert_eq!(result.nth(5), Some((160, 'g')));
//! ```

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul};

mod tests;

//------------------------------------------------------------------------------

/// An iterator that yields the current count, with the generic type, and the iteration item.
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Indexer<I, T> {
    iter: I,
    counter: T,
    step: T
}

impl<I, T> Indexer<I, T> {
    pub fn new(iter: I, start: T, step: T) -> Indexer<I, T> {
        Indexer { iter, counter: start, step  }
    }
}

pub trait IndexerIterator {
    /// Creates an iterator which gives an index of the source iterator value as well as the value itself.
    ///
    /// The iterator yields pairs `(i, val)`, where `i` is of type `T` and contains the current
    /// index of iteration, and `val` is the value returned by the source iterator.
    ///
    /// `index::<T>()` starts counting at 0 and increments by 1. If you need another start value, use
    /// `index_start::<T>(start: T)` instead. If you need different steps than 1, use
    /// `index_step::<T>(start: T, step: T)`.
    ///
    /// # Overflow Behavior
    ///
    /// The method does no guarding against overflows, so you may have to prevent it, depending on the type `T`
    /// and the number of items generated by the source iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_index::IndexerIterator;
    ///
    /// let items = vec!["a", "b", "c"];
    /// let mut result = items.into_iter().index::<i32>();
    ///
    /// assert_eq!(result.next(), Some((0_i32, "a")));
    /// assert_eq!(result.next(), Some((1_i32, "b")));
    /// assert_eq!(result.next(), Some((2_i32, "c")));
    /// assert_eq!(result.next(), None);
    /// ```
    fn index<T>(self) -> Indexer<Self, T> where Self: Sized, u8: Into<T> {
        Indexer::new(self, 0.into(), 1.into())
    }

    /// Creates an iterator which gives an index of the source iterator value as well as the value itself.
    ///
    /// The iterator yields pairs `(i, val)`, where `i` is of type `T` and contains the current
    /// index of iteration, and `val` is the value returned by the source iterator.
    ///
    /// `index_start::<T>(start: T)` starts counting at `start` and increments by 1. If you need different
    /// steps than 1, use `index_step::<T>(start: T, step: T)`.
    ///
    /// # Overflow Behavior
    ///
    /// The method does no guarding against overflows, so you may have to prevent it, depending on the type `T`
    /// and the number of items generated by the source iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_index::IndexerIterator;
    ///
    /// let items = vec!["a", "b", "c"];
    /// let mut result = items.into_iter().index_start::<u8>(97);
    ///
    /// assert_eq!(result.next(), Some((97_u8, "a")));
    /// assert_eq!(result.next(), Some((98_u8, "b")));
    /// assert_eq!(result.next(), Some((99_u8, "c")));
    /// assert_eq!(result.next(), None);
    /// ```
    fn index_start<T>(self, start: T) -> Indexer<Self, T> where Self: Sized, u8: Into<T> {
        Indexer::new(self, start, 1.into())
    }

    /// Creates an iterator which gives an index of the source iterator value as well as the value itself.
    ///
    /// The iterator yields pairs `(i, val)`, where `i` is of type `T` and contains the current
    /// index of iteration, and `val` is the value returned by the source iterator.
    ///
    /// `index_step::<T>(start: T, step: T)` starts counting at `start` and increments by `step`.
    ///
    /// # Overflow Behavior
    ///
    /// The method does no guarding against overflows, so you may have to prevent it, depending on the type `T`
    /// and the number of items generated by the source iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_index::IndexerIterator;
    ///
    /// let items = vec!["a", "b", "c"];
    /// let mut result = items.into_iter().index_step::<u32>(100, 10);
    ///
    /// assert_eq!(result.next(), Some((100_u32, "a")));
    /// assert_eq!(result.next(), Some((110_u32, "b")));
    /// assert_eq!(result.next(), Some((120_u32, "c")));
    /// assert_eq!(result.next(), None);
    /// ```
    fn index_step<T>(self, start: T, step: T) -> Indexer<Self, T> where Self: Sized {
        Indexer::new(self, start, step)
    }
}

//------------------------------------------------------------------------------
// Iterator methods

impl<I, T> Iterator for Indexer<I, T>
where
    I: Iterator,
    T: Clone + for<'a> AddAssign<&'a T> + From<u8> + TryFrom<usize>,
    for<'a> &'a T: Add<Output=T> + Mul<Output=T>,
    <T as TryFrom<usize>>::Error: Debug,
{
    type Item = (T, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => {
                let result = Some((self.counter.clone(), v));
                self.counter += &self.step;
                result
            }
            None => None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let a = self.iter.nth(n)?;
        let nn: T = n.try_into().expect(&format!("Cannot convert n into {}", std::any::type_name::<T>()));
        let i = &self.counter + &(&nn * &self.step);
        self.counter = &i + &self.step;
        Some((i.clone(), a))
    }
}

//------------------------------------------------------------------------------

impl<I, T> ExactSizeIterator for Indexer<I, T>
where
    I: ExactSizeIterator,
    T: Clone + for<'a> AddAssign<&'a T> + From<u8> + TryFrom<usize>,
    for<'a> &'a T: Add<Output=T> + Mul<Output=T>,
    <T as TryFrom<usize>>::Error: Debug,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

//------------------------------------------------------------------------------
// Blanket implementation

impl<I: Iterator> IndexerIterator for I {}

