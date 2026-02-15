//! Tim sort (based off of cpython implementation)
//!
//! - Not in-place | Dependent on merge-sort implementation
//! - O(n) -> O(nlogn) comparisons
//! - O(run_count()) -> O(n) extra space
//! - Stable

use std::cmp::Reverse;

// TODO:
// Mergestuff (Mergestate?)
// finish tim_sort logic
// Reversing
// Binary sort
// Galloping
static MIN_GALLOP: usize = 7;
static MAX_PENDING_SLICES: usize = 85;

/// Sorts `data` in ascending order using tim sort.
/// Makes use of 'runs' of minimum length to merge more efficiently. Insertion sort until minimum
/// run length is met.
pub fn tim_sort<T: Ord + Copy>(data: &mut [T]) {
    let mut less = |a: &T, b: &T| a < b;
    tim_sort_by(data, &mut less);
}

pub fn tim_sort_by<T: Copy, F>(data: &mut [T], less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    // merge_collapse
    // merge_force_collapse
    //
    // reverse_sortslice?
    //
    // binarysort

    let mut ms: MergeState<T>;
    let nremaining: usize;
    let minrun: usize;
    let sortslice: &mut [T];
    let reverse;
    let i: usize;

    // merge_init
    ms.min_gallop = MIN_GALLOP;
    ms.n = 0;

    nremaining = data.len();
    if nremaining < 2 {
        return;
    }

    minrun = merge_compute_minrun(nremaining);

    let mut lo = 0;
    let mut hi = data.len();

    while nremaining > 0 {
        let (mut run_len, descending) = count_run_by(&data[lo..lo + nremaining], less);

        if descending {
            &data[lo..lo + run_len].reverse(); // Unstable?
        }
        // If the run length is too short, extend it with binary sort
        if run_len < minrun {
            let force: usize;
            if nremaining <= minrun {
                force = nremaining;
            } else {
                force = minrun;
            }

            binarysort(&mut data[lo..lo + force], less, run_len);
            run_len = force;
        }
        ms.pending[ms.n] = &mut data[lo..lo + run_len];
        ms.n += 1;
        merge_collapse(ms);
        lo += run_len;
        nremaining -= run_len;
    }

    merge_force_collapse(ms);
}

// ----- helpers -----

struct MergeState<'a, T> {
    min_gallop: usize,
    temparray: &'a [T],
    n: usize,
    pending: [&'a mut [T]; MAX_PENDING_SLICES],
}

/// Returns the tuple (run_len, descending) where run_len is the run starting at slice[0]
/// slice.len() > 0 is required.
///
/// A run is the largest ascending subsequence where:
///     slice[0] <= slice[1] <= slice[2] <= ...
/// or longest descending sequence where:
///     slice[0] > slice[1] > slice[2] > ...
///
/// Used for totally ordered types.
fn count_run<T: Ord>(slice: &[T]) -> (usize, bool) {
    let mut less = |a: &T, b: &T| a < b;
    count_run_by(slice, &mut less)
}

/// Returns the tuple (run_len, descending) where run_len is the run starting at slice[0]
/// slice.len() > 0 is required.
///
/// A run is the largest ascending subsequence where:
///     slice[0] <= slice[1] <= slice[2] <= ...
/// or longest descending sequence where:
///     slice[0] > slice[1] > slice[2] > ...
///
/// Used for types given an ordering funtion 'less' .
fn count_run_by<T, F>(slice: &[T], less: &mut F) -> (usize, bool)
where
    F: FnMut(&T, &T) -> bool,
{
    assert!(slice.len() > 0);
    let len = slice.len();
    if len < 2 {
        return (len, false);
    }

    let mut i = 1;
    let descending = less(&slice[1], &slice[0]);

    while i + 1 < len {
        if descending {
            if less(&slice[i + 1], &slice[i]) {
                i += 1;
            } else {
                break;
            }
        } else {
            if less(&slice[i + 1], &slice[i]) {
                break;
            } else {
                i += 1
            }
        }
    }

    (i + 1, descending)
}

fn merge_compute_minrun(size: usize) -> usize {
    let mut r = 0;
    let mut n = size;
    assert!(n >= 0);

    while n >= 64 {
        r = r | (n & 1);
        n >>= 1;
    }

    r + size
}

/// Binary sort implementation using 'less' ordering, know 0..cur_len is sorted.
/// Must sort whole given slice, where slice will be of length nremaining or minrun.
fn binarysort<T: Copy, F>(slice: &mut [T], less: &mut F, cur_len: usize)
where
    F: FnMut(&T, &T) -> bool,
{
    assert!(cur_len <= slice.len());

    if slice.len() < 2 {
        return;
    }

    // 0..Start should be sorted
    let start = cur_len.max(1);

    for i in start..slice.len() {
        let pivot = slice[i];

        // Find insert l for pivot in [0..i)
        let mut l = 0usize;
        let mut r = i;

        while l < r {
            let p = l + ((r - l) >> 1);
            if less(&pivot, &slice[p]) {
                r = p;
            } else {
                l = p + 1; // stability when equal
            }
        }
        assert!(l == r);

        // Shift l..i right by 1 to place pivot
        for j in (l + 1..i).rev() {
            slice[j] = slice[j - 1];
        }
        slice[l] = pivot;
    }
}

#[cfg(test)]
mod tests {
    use super::tim_sort;

    #[test]
    fn sorts_integers() {
        let mut v = vec![3, 1, 2];
        tim_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        tim_sort(&mut v);
        assert_eq!(v, vec![]);
    }
}
