//! GAT equivalent of `std` iterator traits, often referred to as a lending iterator

mod adapters;

pub use adapters::*;

/// # Safety:
/// This is only safe to use if the item provided is sound to have a lifetime of `'b`.
///
/// This is true in cases such as the polonius borrow case and when the user is sure the value can
/// actually live for the desired time.
unsafe fn change_lifetime<'a, 'b, I: ?Sized + Iterator>(i: I::Item<'a>) -> I::Item<'b> {
    // SAFETY: This functions preconditions assure this is sound
    unsafe { core::mem::transmute::<I::Item<'a>, I::Item<'b>>(i) }
}

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

    /// Advance the iterator by `n` elements
    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        let mut idx = 0;
        while idx < n {
            if self.next().is_none() {
                return Err(idx);
            }
            idx += 1;
        }
        Ok(())
    }

    /// Return the `n`th element of the iterator
    ///
    /// This does not rewind the iterator after completion, so repeatedly calling `nth(0)` is
    /// equivalent to calling `next`
    fn nth(&mut self, mut n: usize) -> Option<Self::Item<'_>> {
        while n > 0 {
            self.next()?;
            n -= 1;
        }
        self.next()
    }

    // Lazy Adaptors

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

    /// Execute a closure on each item in the iterator, returning true if it should be included, or
    /// false to skip it
    fn filter<F>(self, f: F) -> Filter<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item<'_>) -> bool,
    {
        Filter::new(self, f)
    }

    /// Creates an iterator starting at the same point, but stepping by the given amount at each
    /// iteration
    fn step_by(self, step: usize) -> StepBy<Self>
    where
        Self: Sized,
    {
        assert_ne!(step, 0);
        StepBy::new(self, step)
    }

    /// Takes two iterators and creates a new iterator over both in sequence
    fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter>
    where
        Self: Sized,
        U: IntoIterator,
        U::IntoIter: for<'a> Iterator<Item<'a> = Self::Item<'a>>,
    {
        Chain::new(self, other.into_iter())
    }

    /// ‘Zips up’ two iterators into a single iterator of pairs
    fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
    where
        Self: Sized,
        U: IntoIterator,
    {
        Zip::new(self, other.into_iter())
    }

    /// Creates an iterator which gives the current iteration count as well as the next value
    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate::new(self)
    }

    /// Creates an iterator that skips elements based on a predicate
    fn skip_while<F>(self, f: F) -> SkipWhile<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item<'_>) -> bool,
    {
        SkipWhile::new(self, f)
    }

    /// Creates an iterator that yields elements based on a predicate
    fn take_while<F>(self, f: F) -> TakeWhile<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item<'_>) -> bool,
    {
        TakeWhile::new(self, f)
    }

    /// Creates an iterator that skips the first n elements
    fn skip(self, n: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        Skip::new(self, n)
    }

    /// Creates an iterator that yields the first n elements, or fewer if the underlying iterator
    /// ends sooner
    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, n)
    }

    // Consumers

    /// Tests if every element of the iterator matches a predicate
    fn all<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(Self::Item<'_>) -> bool,
    {
        while let Some(val) = self.next() {
            if !f(val) {
                return false;
            }
        }
        true
    }

    /// Tests if any element of the iterator matches a predicate
    fn any<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(Self::Item<'_>) -> bool,
    {
        while let Some(val) = self.next() {
            if f(val) {
                return true;
            }
        }
        false
    }

    /// Searches for an element of an iterator that satisfies a predicate
    fn find<F>(&mut self, mut f: F) -> Option<Self::Item<'_>>
    where
        F: FnMut(&Self::Item<'_>) -> bool,
    {
        while let Some(val) = self.next() {
            if f(&val) {
                // SAFETY: Polonius case
                return Some(unsafe { change_lifetime::<Self>(val) });
            }
        }
        None
    }

    /// Consume the iterator, counting the number of items and returning it
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.fold(0, |acc, _| acc + 1)
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
        F: FnMut(&mut T, Self::Item<'_>) -> Option<B>,
    {
        Scan::new(self, acc, f)
    }
}

/// Trait for values which can be converted into an [`Iterator`]
pub trait IntoIterator {
    /// The type of the returned iterator
    type IntoIter: Iterator;

    /// Convert this value into an [`Iterator`]
    fn into_iter(self) -> Self::IntoIter;
}

impl<T> IntoIterator for T
where
    T: Iterator,
{
    type IntoIter = T;

    fn into_iter(self) -> Self::IntoIter {
        self
    }
}

/// Trait for converting a normal, non-lending iterator into a lending iterator.
///
/// This is useful for methods such as [`Iterator::zip`], where you may want to combine a standard
/// iterator onto a lending one.
pub trait IntoLending: Sized {
    /// Convert this iterator into a lending one
    fn into_lending(self) -> FromCore<Self>;
}

impl<I> IntoLending for I
where
    I: core::iter::Iterator,
{
    fn into_lending(self) -> FromCore<Self> {
        FromCore(self)
    }
}

#[cfg(test)]
mod test;
