//! Tim sort (based off of cpython implementation)
//!
//! - Not in-place | Dependent on merge-sort implementation
//! - O(n) -> O(nlogn) comparisons
//! - O(run_count()) -> O(n) extra space
//! - Stable

// TODO:
// Mergestuff (Mergestate?)
// finish tim_sort logic
// Reversing
// Binary sort
// Galloping

/// Sorts `data` in ascending order using tim sort.
/// Makes use of 'runs' of minimum length to merge more efficiently. Insertion sort until minimum
/// run length is met.
pub fn tim_sort<T: Ord>(data: &mut [T]) {
    let mut less = |a: &T, b: &T| a < b;
    tim_sort_by(data, &mut less);
}

pub fn tim_sort_by<T, F>(data: &mut [T], less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    // PYList_Check
    //
    // merge_init
    // merge_compute_min_run
    // merge_collapse
    // merge_force_collapse
    //
    // reverse_slice
    // reverse_sortslice
    //
    // sortslice_advance
    //
    // binarysort
    let mut lo = 0;
    let mut hi = data.len();

    let (run_len, descending) = count_run_by(&data[lo..hi], less);
    if descending {
        reverse_sortslice(&data[lo..lo + run_len]);
    }
}

// ----- helpers -----

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
