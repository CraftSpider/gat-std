//! GAT equivalents of `std` operators

/// Index operator for immutable contexts. As opposed to `std`, the returned value can be a
/// non-reference. This allows custom reference types for things like multi-dimensional matrices.
pub trait Index<T> {
    /// The output type of indexing this value
    type Output<'a>
    where
        Self: 'a;

    /// Get the value at this index immutably
    fn index(&self, idx: T) -> Self::Output<'_>;
}

/// Index operator for mutable contexts. As opposed to `std`, the returned value can be a
/// non-reference. This allows custom reference types for things like multi-dimensional matrices.
pub trait IndexMut<T>: Index<T> {
    /// The output type of indexing this value
    type OutputMut<'a>
    where
        Self: 'a;

    /// Get the value at this index mutably
    fn index_mut(&mut self, idx: T) -> Self::OutputMut<'_>;
}

impl<T: ?Sized, I> Index<I> for T
where
    T: core::ops::Index<I>,
    T::Output: 'static,
{
    type Output<'a> = &'a <T as core::ops::Index<I>>::Output
    where
        Self: 'a;

    fn index(&self, idx: I) -> Self::Output<'_> {
        <Self as core::ops::Index<I>>::index(self, idx)
    }
}

impl<T: ?Sized, I> IndexMut<I> for T
where
    T: core::ops::IndexMut<I>,
    T::Output: 'static,
{
    type OutputMut<'a> = &'a mut <T as core::ops::Index<I>>::Output
    where
        Self: 'a;

    fn index_mut(&mut self, idx: I) -> Self::OutputMut<'_> {
        <Self as core::ops::IndexMut<I>>::index_mut(self, idx)
    }
}
