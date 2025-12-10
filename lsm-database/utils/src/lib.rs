//! Collection of lightweight helper algorithms shared across the workspace.
//!
//! The crate is intentionally small and focuses on keeping frequently used
//! utilities in one place so the higher-level crates (CLI, core engine, etc.)
//! can import them without duplicating logic. The currently exposed helpers are:
//! - [`searcher`]: linear and binary search routines that work
//!   on ordered, copyable data.
//! - [`sorter`]: a reference selection-sort implementation that
//!   keeps the input immutable and returns a newly allocated vector.
//!
//! Additional utilities should follow the same pattern: small, well-documented,
//! and dependency-free, making them easy to audit and test.
pub mod searcher;
pub mod sorter;

pub mod linked_list;
