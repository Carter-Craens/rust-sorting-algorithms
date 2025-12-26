//! Insertion sort
//!
//! - In-place
//! - O(n^2) comparisons (best: ~n, average: ~n^2/4, worst: ~n^2/2)
//! - O(1) extra space
//! - Stable

/// Sorts `data` in ascending order using insertion sort
pub fn insertion_sort<T: Ord>(data: &mut [T]) {
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j] < data[j - 1] {
            data.swap(j, j - 1);
            j -= 1;
        }
    }
}

// ----- helpers -----

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    #[test]
    fn sorts_integers() {
        let mut v = vec![3, 1, 2];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        insertion_sort(&mut v);
        assert_eq!(v, vec![]);
    }
}
