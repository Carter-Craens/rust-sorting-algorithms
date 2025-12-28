//! quick sort
//!
//! - In-place
//! - O(nlog(n)) comparisons
//! - O(1) extra space
//! - Stable

/// Sorts `data` in ascending order using quick sort
pub fn quick_sort<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }
    let pivot = middle_of_three(&data);
    let p = lomuto(data, pivot);

    quick_sort(&mut data[..p]);
    quick_sort(&mut data[p + 1..]);
}

// ----- helpers -----
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
/// Pivot put at end. Eventual pivot index returned.
fn lomuto<T: Ord>(data: &mut [T], pivot: usize) -> usize {
    // Guard. Should(!) never be true.
    if data.len() <= 1 {
        return 0;
    }

    // Place pivot at the end of the array
    let last = data.len() - 1;
    data.swap(pivot, last);

    let mut i: usize = 0;
    for j in 0..last {
        if data[j] <= data[last] {
            data.swap(i, j);
            i += 1;
        }
    }

    // Place pivot in its proper place
    data.swap(i, last);
    i
}
// fn hoare<T: Ord>(data: &mut [T]) -> usize {}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn sorts_integers() {
        let mut v = vec![3, 1, 2];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        quick_sort(&mut v);
        assert_eq!(v, vec![]);
    }
}
