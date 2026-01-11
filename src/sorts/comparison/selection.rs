//! Selection sort.
//!
//! - In-place
//! - O(n^2) comparisons
//! - O(1) extra space
//! - Not stable

/// Sorts `data` in ascending order using selection sort.
pub fn selection_sort<T: Ord>(data: &mut [T]) {
    for i in 0..data.len() {
        let min: usize = i + get_min(&data[i..]);
        if min != i {
            data.swap(i, min);
        }
    }
}

// ----- helpers -----
/// Returns index of smallest element of a given slice.
fn get_min<T: Ord>(data: &[T]) -> usize {
    let mut min = 0;
    for i in 1..data.len() {
        if data[min] > data[i] {
            min = i;
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn sorts_integers() {
        let mut v = vec![3, 1, 2];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        selection_sort(&mut v);
        assert_eq!(v, vec![]);
    }
}
