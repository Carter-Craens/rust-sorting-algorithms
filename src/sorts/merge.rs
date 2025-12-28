//! merge sort
//!
//! - Not in-place
//! - O(nlog(n)) comparisons
//! - O(n) extra space
//! - Unstable

/// Sorts `data` in ascending order using merge sort
pub fn merge_sort<T: Ord + Copy>(data: &mut [T]) {
    let hi = data.len();
    if hi <= 1 {
        return;
    }

    let mid = hi / 2;
    merge_sort(&mut data[..mid]);
    merge_sort(&mut data[mid..]);

    merge(data, mid);
}

// ----- helpers -----
/// Merges both sides into slice around mid. Uses 'mid' extra memory;
fn merge<T: Ord + Copy>(data: &mut [T], mid: usize) {
    let left: Vec<T> = data[..mid].to_vec();
    let right: Vec<T> = data[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            data[k] = left[i].clone();
            i += 1;
        } else {
            data[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        data[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        data[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn sorts_integers() {
        let mut v = vec![3, 1, 2];
        merge_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        merge_sort(&mut v);
        assert_eq!(v, vec![]);
    }
}
