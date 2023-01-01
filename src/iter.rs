//! GAT equivalent of `std` iterator traits, often referred to as a lending iterator

mod adapters;

pub use adapters::*;

/// A lending iterator, whose items may have their lifetimes tied to the individual borrow of the
/// iterator. This allows for things like yielding mutable references that overlap, with the
/// trade-off that there's no generic `collect` interface - the items of this iterator cannot
/// co-exist.
pub trait Iterator {
    /// The value yielded by each call to `next` on this iterator
    type Item<'a>
    where
        Self: 'a;

    /// Get the next value of this iterator, or return `None`
    fn next(&mut self) -> Option<Self::Item<'_>>;

    /// Get a hint as to the size of this iterator - the first value is a lower bound, second
    /// is an optional upper bound.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Take a closure which will take each value from the iterator, and yield a new value computed
    /// from it.
    ///
    /// The result cannot reference the provided data, as such, this returns an iterator which also
    /// implements the non-lending core iterator
    fn map<O, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item<'_>) -> O,
    {
        Map::new(self, f)
    }

    /// Gain mutable access to each value in this iterator, then yield it to the next step.
    /// This allows altering each item without consuming it, preserving the lending nature
    /// or the iterator
    fn touch<F>(self, f: F) -> Touch<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut Self::Item<'_>),
    {
        Touch::new(self, f)
    }

    /// Execute a closure on each value of this iterator
    fn for_each<F>(mut self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item<'_>),
    {
        while let Some(next) = self.next() {
            f(next)
        }
    }

    /// Execute a closure on each value of this iterator, with an additional 'accumulator' value
    /// passed to each call. The closure is expected to return the new value of the accumulator.
    fn fold<T, F>(mut self, acc: T, mut f: F) -> T
    where
        Self: Sized,
        F: FnMut(T, Self::Item<'_>) -> T,
    {
        let mut acc = acc;
        while let Some(x) = self.next() {
            acc = f(acc, x);
        }
        acc
    }

    /// Execute a closure on each value of this iterator, with an additional state value passed
    /// via mutable reference to each call. The closure is expected to return the new value
    /// for each step of the iterator, if the returned value is `None` the iterator stops early.
    ///
    /// The result cannot reference the provided data, as such, this returns an iterator which also
    /// implements the non-lending core iterator
    fn scan<T, B, F>(self, acc: T, f: F) -> Scan<Self, T, F>
    where
        Self: Sized,
        F: FnMut(&mut T, Self::Item<'_>) -> Option<B>
    {
        Scan::new(self, acc, f)
    }
}

impl<T> Iterator for T
where
    T: core::iter::Iterator,
{
    type Item<'a> = T::Item
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        <Self as core::iter::Iterator>::next(self)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        <Self as core::iter::Iterator>::size_hint(self)
    }
}

/// Trait for values which can be converted into an [`Iterator`]
pub trait IntoIterator {
    /// The item yielded by the returned iterator
    type Item<'a>
    where
        Self: 'a;

    /// The type of the returned iterator
    type IntoIter<'a>: Iterator<Item<'a> = Self::Item<'a>>
    where
        Self: 'a;

    /// Convert this value into an [`Iterator`]
    fn into_iter<'a>(self) -> Self::IntoIter<'a>
    where
        Self: 'a;
}

#[cfg(feature = "alloc")]
impl<T> IntoIterator for alloc::vec::Vec<T> {
    type Item<'a> = T
    where
        Self: 'a;
    type IntoIter<'a> = alloc::vec::IntoIter<T>
    where
        Self: 'a;

    fn into_iter<'a>(self) -> Self::IntoIter<'a>
    where
        Self: 'a,
    {
        <Self as core::iter::IntoIterator>::into_iter(self)
    }
}

impl IntoIterator for core::ops::Range<usize> {
    type Item<'a> = usize;

    type IntoIter<'a> = core::ops::Range<usize> where Self: 'a ;

    fn into_iter<'a>(self) -> Self::IntoIter<'a> where Self: 'a {
        self
    }
}

// impl<T> IntoIterator for T
// where
//     T: Iterator,
// {
//     type Item<'a> = T::Item<'a>
//     where
//         Self: 'a;
//     type IntoIter<'a> = T
//     where
//         Self: 'a;
//
//     fn into_iter<'a>(self) -> Self::IntoIter<'a>
//     where
//         Self: 'a
//     {
//         self
//     }
// }
