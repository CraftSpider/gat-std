//! GAT implementations of `std` traits
//!
//! Traits are in the same respective paths as their `std` variants. The `gatify`
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
    clippy::cast_possible_wrap
)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

/// Rewrites `std` operators to use their GAT equivalents. Can be applied to any item or statement.
///
/// ## Index
///
/// The `[]` operator is converted to use the [`ops::Index`] or [`ops::IndexMut`] trait.
/// This may not always be a perfect drop-in replacement, despite the blanket impl for
/// [`core::ops::Index`] - if the macro can't tell which impl is expected from context, it will
/// error out, pointing to the operator that caused the error.
///
/// ## For Loops
///
/// For loops are converted to use either [`core::iter::Iterator`] or [`iter::Iterator`], depending
/// on which is implemented. If both are implemented, priority is given to the lending iterator.
///
pub use gat_std_proc::gatify;

pub mod iter;
pub mod ops;

#[doc(hidden)]
pub mod __impl {
    pub struct IntoIter<T>(pub T);

    pub trait ViaLending {
        type Selector;

        fn select(&self) -> Self::Selector;
    }

    impl<T: crate::iter::IntoIterator> ViaLending for &IntoIter<T> {
        type Selector = Lending;

        fn select(&self) -> Self::Selector {
            Lending
        }
    }

    pub trait ViaCore {
        type Selector;

        fn select(&self) -> Self::Selector;
    }

    impl<T: core::iter::IntoIterator> ViaCore for IntoIter<T> {
        type Selector = Core;

        fn select(&self) -> Self::Selector {
            Core
        }
    }

    pub struct Lending;

    impl Lending {
        pub fn into_iter<T: crate::iter::IntoIterator>(self, iter: IntoIter<T>) -> T::IntoIter {
            iter.0.into_iter()
        }
    }

    pub struct Core;

    impl Core {
        pub fn into_iter<T: core::iter::IntoIterator>(self, iter: IntoIter<T>) -> T::IntoIter {
            iter.0.into_iter()
        }
    }
}
