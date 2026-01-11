//! Intro sort
//!
//! - in-place
//! - # comparisons
//! - # extra space
//! - (un)stable

use crate::sorts::{heap_sort, insertion_sort};

const INSERTION_THRESHOLD: usize = 16;

// ----- Public API -----
/// Sorts `data` in ascending order using quick sort into heap sort when recursion gets too deep.
pub fn intro_sort<T: Ord>(data: &mut [T]) {
    debug_assert!(INSERTION_THRESHOLD >= 1);
    let n = data.len();
    if n <= 1 {
        return;
    }
    // Depth budget
    let depth_limit = 2 * floor_log2_usize(n);
    debug_assert!(depth_limit >= 1);

    // Quicksort with fallback to heap. Stop recursion on small partitions.
    intro_sort_impl(data, depth_limit);

    // Final insertion osrt of nearly-sorted array.
    insertion_sort(data);
}

// ----- Helpers -----
/// The implementation of intro sort.
fn intro_sort_impl<T: Ord>(data: &mut [T], depth_lim: usize) {
    let length = data.len();
    if length <= INSERTION_THRESHOLD {
        return;
    }
    if depth_lim == 0 {
        heap_sort(data);
        return;
    }

    // Assures previous behavior and pivot precon
    debug_assert!(data.len() > INSERTION_THRESHOLD);
    debug_assert!(data.len() >= 2);
    let pivot = middle_of_three(data);

    // Hoare pre and post conditions
    debug_assert!(pivot < data.len());
    let p = hoare(data, pivot);
    debug_assert!(p < data.len());
    debug_assert!(p + 1 + (data.len() - (p + 1)) == data.len());

    intro_sort_impl(&mut data[0..p], depth_lim - 1);
    intro_sort_impl(&mut data[p + 1..], depth_lim - 1);
}

/// Pre: n > 0 | Returns the floor(log_2(n))
fn floor_log2_usize(n: usize) -> usize {
    debug_assert!(n > 0);
    (usize::BITS as usize - 1) - (n.leading_zeros() as usize)
}

/// Picks the median of 3 elements (first, middle, last) to be the pivot.
fn middle_of_three<T: Ord>(data: &[T]) -> usize {
    if data.len() <= 2 {
        return 0;
    }
    let last = data.len() - 1;
    let mid = last / 2;

    let a = &data[0];
    let b = &data[mid];
    let c = &data[last];
    if a < b {
        if b < c {
            mid
        } else if a < c {
            last
        } else {
            0
        }
    } else {
        if a < c {
            0
        } else if b < c {
            last
        } else {
            mid
        }
    }
}

/// Pivot put at start. Swap gt and lt when both found. Eventual pivot index returned
fn hoare<T: Ord>(data: &mut [T], pivot: usize) -> usize {
    // preconditions
    debug_assert!(data.len() >= 2);
    debug_assert!(pivot < data.len());

    // Place pivot at start and initialize
    data.swap(0, pivot);
    let len = data.len();
    let mut i = 0;
    let mut j = data.len();

    loop {
        i += 1;
        while i < len && data[i] < data[0] {
            i += 1;
        }
        debug_assert!(i <= data.len());

        j -= 1;
        while j > 0 && data[j] > data[0] {
            j -= 1;
        }
        debug_assert!(j < data.len());

        if i >= j {
            data.swap(0, j);
            debug_assert!(j < data.len()); // Valid location for j
            // Properly partitioned
            debug_assert!(data[..j].iter().all(|x| x <= &data[j]));
            debug_assert!(data[j + 1..].iter().all(|x| x >= &data[j]));
            return j;
        }
        data.swap(i, j);
    }
}

// ----- Tests -----
#[cfg(test)]
mod tests {
    use super::{INSERTION_THRESHOLD, intro_sort, intro_sort_impl};

    // Helper function that checks for every pair next to another that left <= right side
    fn is_sorted<T: Ord>(v: &[T]) -> bool {
        v.windows(2).all(|w| w[0] <= w[1])
    }

    // Determinstic RNG so no rand crate needed
    fn lcg_next(state: &mut u64) -> u64 {
        // Constants from Numerical Recipes
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        *state
    }

    // Generates random i32 vectore depending on random seed
    fn generate_vec_i32(len: usize, seed: u64) -> Vec<i32> {
        let mut s = seed;
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            let x = lcg_next(&mut s);
            // Since LCG returns a u64, fold it into i32 range and mix
            out.push((x >> 33) as i32 ^ (x as i32));
        }
        out
    }

    #[test]
    fn small_vec_integers_are_sorted() {
        let mut v = vec![3, 1, 2];
        intro_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        intro_sort(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn depth_limit_zero_triggers_heap_sort() {
        let n = INSERTION_THRESHOLD + 1;
        // Reverse sorted vec i32
        let mut v = (0..n as i32).rev().collect::<Vec<_>>();
        assert!(!is_sorted(&v), "input should not be sorted");
        assert!(v.len() > INSERTION_THRESHOLD);

        // Call with depth_limit 0 to ensure heap branch is taken
        intro_sort_impl(&mut v, 0);

        assert!(is_sorted(&v));
    }

    #[test]
    fn sorts_big_enough_input() {
        let n = INSERTION_THRESHOLD * 4;
        let mut v = (0..n as i32).rev().collect::<Vec<_>>();

        intro_sort(&mut v);

        assert!(is_sorted(&v));
    }
}
