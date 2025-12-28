//! heap sort
//!
//! - Not in-place
//! - O(nlog(n)) comparisons
//! - O(n) extra space
//! - Unstable

// ----- Public API -----
/// Sorts `data` in ascending order using heap sort
pub fn heap_sort<T: Ord>(data: &mut [T]) {
    let n = data.len();

    // Make array a valid heap
    for i in (0..n / 2).rev() {
        heapify(data, n, i);
    }
    // Extract element one by one from heap
    for i in (1..n).rev() {
        // Move current root (max) to end
        data.swap(0, i);

        // Max heapifies reduced heap
        heapify(data, i, 0);
    }
}

// ----- Helpers -----
/// Heapifies subtree rooted at i
fn heapify<T: Ord>(data: &mut [T], length: usize, i: usize) {
    let mut largest: usize = i;
    let left: usize = 2 * i + 1;
    let right: usize = 2 * i + 2;

    // left child larger than root
    if left < length && data[left] > data[largest] {
        largest = left;
    }

    // right child larger than root
    if right < length && data[right] > data[largest] {
        largest = right;
    }

    // If root is not largest
    if largest != i {
        data.swap(i, largest);

        // Recursively heapify correct subtree
        heapify(data, length, largest);
    }
}

// ----- Tests -----
#[cfg(test)]
mod tests {
    use super::heap_sort;

    #[test]
    fn sorts_integers() {
        let mut v = vec![3, 1, 2];
        heap_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        heap_sort(&mut v);
        assert_eq!(v, vec![]);
    }
}
