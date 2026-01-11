//! quick sort
//!
//! - In-place
//! - O(nlog(n)) comparisons
//! - O(1) extra space
//! - Stable

// ----- Types / Internal data structures -----
/// Enum to select partitioning style for benchmarking purposes
pub enum Partition {
    Lomuto,
    Hoare,
    Dutch3Way,
}

// ----- Public API -----
pub fn quick_sort<T: Ord>(data: &mut [T]) {
    quick_sort_with(data, Partition::Hoare);
}

/// Sorts `data` in ascending order using quick sort
pub fn quick_sort_with<T: Ord>(data: &mut [T], scheme: Partition) {
    if data.len() <= 1 {
        return;
    }
    let pivot = middle_of_three(&data);
    match scheme {
        Partition::Hoare => {
            let p = hoare(data, pivot);
            quick_sort_with(&mut data[..p], Partition::Hoare);
            quick_sort_with(&mut data[p + 1..], Partition::Hoare);
        }
        Partition::Lomuto => {
            let p = lomuto(data, pivot);
            quick_sort_with(&mut data[..p], Partition::Lomuto);
            quick_sort_with(&mut data[p + 1..], Partition::Lomuto);
        }
        Partition::Dutch3Way => {
            let (lt, gt) = dutch_3_way(data, pivot);
            quick_sort_with(&mut data[..lt], Partition::Dutch3Way);
            quick_sort_with(&mut data[gt + 1..], Partition::Dutch3Way);
        }
    }
}

// ----- Partitioning Strategies -----
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

/// Pivot put at start. Swap gt and lt when both found. Eventual pivot index returned
fn hoare<T: Ord>(data: &mut [T], pivot: usize) -> usize {
    data.swap(0, pivot);
    let len = data.len();
    let mut i = 0;
    let mut j = data.len();

    loop {
        i += 1;
        while i < len && data[i] < data[0] {
            i += 1;
        }

        j -= 1;
        while j > 0 && data[j] > data[0] {
            j -= 1;
        }

        if i >= j {
            data.swap(0, j);
            return j;
        }
        data.swap(i, j);
    }
}

/// 3-way quicksort. ..lt: Less than pivot, lt..gt+1: equal to pivot, gt+1..: larger than pivot
fn dutch_3_way<T: Ord>(data: &mut [T], pivot: usize) -> (usize, usize) {
    // Guard for length <= 2
    if data.len() <= 2 {
        if data[0] > data[data.len() - 1] {
            data.swap(0, data.len() - 1);
        }
        return (0, data.len() - 1);
    }

    data.swap(pivot, data.len() - 1);
    let p = data.len() - 1;

    let mut lt = 0;
    let mut i = 0;
    let mut gt = data.len() - 1;

    while i < gt {
        if data[i] < data[p] {
            data.swap(lt, i);
            lt += 1;
            i += 1;
        } else if data[i] == data[p] {
            i += 1;
        } else if data[i] > data[p] {
            gt -= 1;
            data.swap(i, gt);
        }
    }
    data.swap(gt, p);
    return (lt, gt);
}
// ----- Helpers -----
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

// ----- Tests -----
#[cfg(test)]
mod tests {
    use super::Partition;
    use super::quick_sort;
    use super::quick_sort_with;

    #[test]
    fn sorts_integers_lomuto() {
        let mut v = vec![3, 1, 2, 4, 0, 13, 12];
        quick_sort_with(&mut v, Partition::Lomuto);
        assert_eq!(v, vec![0, 1, 2, 3, 4, 12, 13]);
    }

    #[test]
    fn sorts_integers_hoare() {
        let mut v = vec![3, 1, 2, 4, 0, 13, 12];
        quick_sort_with(&mut v, Partition::Hoare);
        assert_eq!(v, vec![0, 1, 2, 3, 4, 12, 13]);
    }

    #[test]
    fn sorts_integers_3_way() {
        let mut v = vec![3, 1, 2, 4, 0, 13, 12];
        quick_sort_with(&mut v, Partition::Dutch3Way);
        assert_eq!(v, vec![0, 1, 2, 3, 4, 12, 13]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<i32> = vec![];
        quick_sort(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn three_way_optimal() {
        let mut v = vec![3, 6, 1, 4, 1, 1, 4, 3, 2, 7, 6, 6, 6, 7, 5, 5, 5];
        quick_sort_with(&mut v, Partition::Dutch3Way);
        assert_eq!(v, vec![1, 1, 1, 2, 3, 3, 4, 4, 5, 5, 5, 6, 6, 6, 6, 7, 7]);
    }
}
