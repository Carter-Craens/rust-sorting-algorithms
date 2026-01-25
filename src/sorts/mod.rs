#![allow(dead_code)]
#![allow(unused_imports)]
//! Sorting algorithms library.
//!
//! Organization:
//! - `comparison`: comparison-based sorts (Ord-based)
//! - `non_comparison`: counting/radix/bucket etc. (key-based)
//! - `string`: specialized / network / string-oriented / misc sorts

pub mod comparison;
pub mod non_comparison;
pub mod string;

pub use comparison::{
    heap::heap_sort,
    insertion::insertion_sort,
    intro::intro_sort,
    merge::merge_sort,
    quick::{Partition, quick_sort, quick_sort_with},
    selection::selection_sort,
    tim::tim_sort,
};

pub use non_comparison::counting::counting_sort;
