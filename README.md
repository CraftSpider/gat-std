
# GAT std

A variant of Rust `std` traits that use GATs, as well as a macro to allow rewriting code
to use these traits instead of the `std` equivalents.

## Why?

1) These traits provide a common base so crates can all use the same definitions, like with `num-traits`
2) `std` likely won't be able to change to use these traits for quite a while, if ever. This allows
   users to take advantage of GATs in their code smoothly despite that.
