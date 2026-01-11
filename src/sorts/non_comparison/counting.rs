//! counting sort
//! ?????
//! - Not in-place
//! - O(nlog(n)) comparisons
//! - O(n) extra space
//! - Unstable

// ----- Public API -----
/// Sorts `data` in ascending order using counting sort
pub fn counting_sort(data: &mut [u32]) {
    if data.is_empty() {
        return;
    }

    // Safe unwrap: Max will be defined for all non-empty T that have Ord trait
    let max: usize = *data.iter().max().unwrap() as usize;
    let mut counts = vec![0; max + 1];
    for &x in data.iter() {
        counts[x as usize] += 1;
    }

    let mut i = 0;
    for (value, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            data[i] = value as u32;
            i += 1;
        }
    }
}

// ----- Tests -----
#[cfg(test)]
mod tests {
    use super::counting_sort;

    #[test]
    fn sorts_integers() {
        let mut v = vec![3, 1, 2];
        counting_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn handles_empty() {
        let mut v: Vec<u32> = vec![];
        counting_sort(&mut v);
        assert_eq!(v, vec![]);
    }
}
