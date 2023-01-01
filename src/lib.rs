//! GAT implementations of `std` traits
//!
//! Traits are in the same respective paths as their `std` variants. The `gat_desugar`
//! macro changes operators to desugar to the traits in this crate instead of their `std`
//! equivalents.

#![warn(
    missing_docs,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    missing_abi,
    noop_method_call,
    pointer_structural_match,
    semicolon_in_expressions_from_macros,
    unused_import_braces,
    unused_lifetimes,
    unsafe_op_in_unsafe_fn,
    clippy::cargo,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::ptr_as_ptr,
    clippy::cloned_instead_of_copied,
    clippy::unreadable_literal,
    clippy::undocumented_unsafe_blocks,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use gat_std_proc::gat_desugar;

pub mod ops;
pub mod iter;

#[doc(hidden)]
pub mod __impl {
    pub trait IdxRef<I> {
        type Output<'a> where Self: 'a;
        fn __index(&self, idx: I) -> Self::Output<'_>;
    }
    pub trait IdxMut<I> {
        type Output<'a> where Self: 'a;
        fn __index(&mut self, idx: I) -> Self::Output<'_>;
    }

    impl<I, T> IdxRef<I> for T
    where
        T: crate::ops::Index<I>
    {
        type Output<'a> = T::Output<'a>
        where
            Self: 'a;

        fn __index(&self, idx: I) -> Self::Output<'_> {
            T::index(self, idx)
        }
    }

    impl<I, T> IdxMut<I> for T
    where
        T: crate::ops::IndexMut<I>
    {
        type Output<'a> = T::OutputMut<'a>
        where
            Self: 'a;

        fn __index(&mut self, idx: I) -> Self::Output<'_> {
            T::index_mut(self, idx)
        }
    }
}
