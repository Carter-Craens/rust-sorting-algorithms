//! Tim sort (based off of cpython implementation)
//!
//! - Not in-place | Dependent on merge-sort implementation
//! - O(n) -> O(nlogn) comparisons
//! - O(run_count()) -> O(n) extra space
//! - Stable

use std::{
    cmp::{Reverse, min},
    i8::MIN,
};
// TODO:
// Mergestuff
// - All the memory functions (memcpy, memmove)
//      - sortslice_memcpy/move
// - All the run incr/decr helpers
//      - sortslice_copy_incr/decr
//      - sorctslice_advance
// - Finalize merge_lo/hi once helper function worked out
//      - Assure that the mergestate returns to original-ish
// - Put effort into error handling once memory functions are in
//      - Logic errors to debug_asserts, panic on memory errors
// Tests:
// - Normal array tests
// - Unit tests for each function
//

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
    let mut ms = MergeState::<T>::new(data.len() / 2);
    let minrun = merge_compute_minrun(data.len());
    let n = data.len();

    let mut lo = 0;
    while lo < n {
        let (mut run_len, descending) = count_run_by(&data[lo..], less);

        if descending {
            let _ = &data[lo..lo + run_len].reverse(); // Stable for equal items
        }

        // If the run length is too short, extend it with binary sort
        let force = min(minrun, n - lo);
        if run_len < force {
            binarysort(&mut data[lo..lo + force], less, run_len);
            run_len = force;
        }
        ms.push_run(lo, run_len);
        merge_collapse(&mut ms, data, less);

        lo += run_len;
    }

    merge_force_collapse(&mut ms, data, less);
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

/// Binary sort implementation using 'less' ordering, knowing 0..cur_len is sorted.
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

// ----- MergeSort section -----

// Run and its functions
#[derive(Clone, Copy, Debug, Default)]
struct Run {
    start: usize,
    len: usize,
}

impl Run {
    #[inline]
    fn end(self) -> usize {
        self.start + self.len
    }
}

const EMPTY_RUN: Run = Run { start: 0, len: 0 };

// Mergestate and its functions
struct MergeState<T> {
    min_gallop: usize,
    pending: [Run; MAX_PENDING_SLICES],
    n: usize,
    temparray: Vec<T>,
}

impl<T: Copy> MergeState<T> {
    fn new(capacity: usize) -> Self {
        Self {
            min_gallop: MIN_GALLOP,
            pending: [EMPTY_RUN; MAX_PENDING_SLICES],
            n: 0,
            temparray: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    fn push_run(&mut self, start: usize, len: usize) {
        self.pending[self.n] = Run { start, len };
        self.n += 1;
    }

    #[inline]
    fn run(&self, i: usize) -> Run {
        self.pending[i]
    }
}

// Mergesort itself
fn merge_force_collapse<T: Copy, F>(ms: &mut MergeState<T>, data: &mut [T], less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    while ms.n > 1 {
        let mut n = ms.n - 2;
        if n > 0 && ms.run(n - 1).len < ms.run(n + 1).len {
            n -= 1;
        }
        let _ = merge_at(ms, data, n, less);
    }
}

fn merge_collapse<T: Copy, F>(ms: &mut MergeState<T>, data: &mut [T], less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    while ms.n > 1 {
        let mut n = ms.n - 2;

        if (n > 0 && ms.run(n - 1).len <= ms.run(n).len + ms.run(n + 1).len)
            || (n > 1 && ms.run(n - 2).len <= ms.run(n - 1).len + ms.run(n).len)
        {
            if ms.run(n - 1).len < ms.run(n + 1).len {
                n -= 1;
            }
            merge_at(ms, data, n, less);
        } else if ms.run(n).len <= ms.run(n + 1).len {
            merge_at(ms, data, n, less);
        } else {
            break;
        }
    }
}

fn merge_at<T: Copy, F>(ms: &mut MergeState<T>, data: &mut [T], i: usize, less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    ms.pending[i].len += ms.run(i + 1).len;
    if i == ms.n - 3 {
        ms.pending[i + 1] = ms.pending[i + 2];
    }
    ms.n -= 1;
    let k = gallop_right(data, data[ms.run(i + 1).start], &mut ms.run(i), 0, less);

    assert!(k > 0);

    ms.run(i).start += k;
    ms.run(i).len -= k;
    if ms.run(i).len == 0 {
        return;
    }

    let l = gallop_left(
        data,
        data[ms.run(i).start],
        &mut ms.run(i + 1),
        ms.run(i + 1).start - 1,
        less,
    );

    ms.run(i + 1).len = l;
    if l == 0 {
        return;
    }

    if ms.run(i).len <= ms.run(i + 1).len {
        merge_lo(data, ms, &mut ms.run(i), &mut ms.run(i + 1), less);
    } else {
        merge_hi(data, ms, ms.run(i), ms.run(i + 1), less);
    }
}

fn merge_lo<T: Copy, F>(
    data: &mut [T],
    ms: &mut MergeState<T>,
    runa: &mut Run,
    runb: &mut Run,
    less: &mut F,
) where
    F: FnMut(&T, &T) -> bool,
{
    assert!(runa.len > 0 && runb.len > 0);
    ms.temparray.copy_from_slice(&data[runa.start..runa.end()]);
    let min_gallop_t = ms.min_gallop;
    // Dest -> runa start.
    let mut dest: Run = runa.clone();
    runa.start = 0; // Now will be used to index temparray

    sortslice_copy_incr(&dest, &runb);
    // --nb?
    enum Exit {
        Success,
        Fail(&'static str),
        CopyB,
    }
    let exit = 'outer: loop {
        // data[dest.start] = data[runb.start];
        // dest.start += 1;
        // dest.len -= 1;
        // runb.start += 1;
        // runb.len -= 1;
        if runb.len == 0 {
            break 'outer Exit::Success;
        }
        if runa.len == 1 {
            break 'outer Exit::CopyB;
        }

        let mut acount = 0;
        let mut bcount = 0;
        loop {
            let k: bool = less(&ms.temparray[runa.start], &data[runb.start]);
            if k {
                sortslice_copy_incr(&data, &dest, &runb);
                bcount += 1;
                acount = 0;
                if runb.len == 0 {
                    break 'outer Exit::Success;
                }
                if bcount >= ms.min_gallop {
                    break;
                }
            } else {
                sortslice_copy_incr(&data, &dest, &runa);
                acount += 1;
                bcount = 0;
                if runa.len == 1 {
                    break 'outer Exit::CopyB;
                }
                if acount >= min_gallop_t {
                    break;
                }
            }
        }

        // Galloping logic
        min_gallop_t += 1;
        while acount >= MIN_GALLOP || bcount >= MIN_GALLOP {
            assert!(runa.len > 1 && runb.len > 0);
            min_gallop_t -= if min_gallop_t > 1 { 1 } else { 0 };
            ms.min_gallop = min_gallop_t;
            let k = gallop_right(data, data[runb.start], runa, 0, less);
            acount = k;
            if k > 0 {
                sortslice_memcpy(&data, &dest, 0, &runa, 0, k);
                run_advance(&dest, k);
                run_advance(&runa, k);
                if runa.len == 1 {
                    break 'outer Exit::CopyB;
                }
                if runa.len == 0 {
                    break 'outer Exit::Success;
                }
            }
            sortslice_copy_incr(&data, &dest, &runb);
            if runb.len == 0 {
                break 'outer Exit::Success;
            }
            let k = gallop_left(data, data[runa.start], runb, 0, less);
            bcount = k;
            if k > 0 {
                sortslice_memmove(&data, &dest, 0, &runb, 0, k);
                sortslice_advance(&data, &dest, k);
                sortslice_advance(&data, &runb, k);
                if runb.len == 0 {
                    break 'outer Exit::Success;
                }
            }
            sortslice_copy_incr(&data, &dest, &runa);
            if runa.len == 1 {
                break 'outer Exit::CopyB;
            }
        }
        min_gallop_t += 1;

        ms.min_gallop = min_gallop_t;
    };

    match exit {
        Exit::Success => {
            if runa.len > 0 {
                sortslice_memcpy(&data, &dest, 0, &runa, 0);
            }
        }

        Exit::Fail(e) => {
            if runa.len > 0 {
                sortslice_memcpy(&data, &dest, 0, &runa, 0);
            }
        }

        Exit::CopyB => {
            assert!(runa.len == 1 && runb.len > 0);

            sortslice_memmove(&data, &dest, 0, &runb, 0);
            sortslice_copy(&data, &dest, &runa, 0);
        }
    }
}

fn merge_hi<T: Copy, F>(data: &mut [T], ms: &mut MergeState<T>, runa: Run, runb: Run, less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    assert!(runa.len > 0 && runb.len > 0);
    ms.temparray.copy_from_slice(&data[runb.start..runb.end()]);
    let min_gallop_t = ms.min_gallop;
    // Dest -> runb start.
    let mut dest: Run = runb.clone();
    runb.start = 0; // Now will be used to index temparray

    sortslice_copy_decr(&dest, &runa);
    // --na
    enum Exit {
        Success,
        Fail(&'static str),
        CopyA,
    }
    let exit = 'outer: loop {
        // data[dest.start] = data[runb.start];
        // dest.start += 1;
        // dest.len -= 1;
        // runb.start += 1;
        // runb.len -= 1;
        if runa.len == 0 {
            break 'outer Exit::Success;
        }
        if runb.len == 1 {
            break 'outer Exit::CopyA;
        }

        let mut acount = 0;
        let mut bcount = 0;
        loop {
            let k: bool = less(&ms.temparray[runb.start], &data[runa.start]);
            if k {
                sortslice_copy_decr(&data, &dest, &runa);
                acount += 1;
                bcount = 0;
                // na--?
                if runa.len == 0 {
                    break 'outer Exit::Success;
                }
                if acount >= ms.min_gallop {
                    break;
                }
            } else {
                sortslice_copy_decr(&data, &dest, &runb);
                bcount += 1;
                acount = 0;
                // --nb?
                if runb.len == 1 {
                    break 'outer Exit::CopyA;
                }
                if bcount >= min_gallop_t {
                    break;
                }
            }
        }

        // Galloping logic
        min_gallop_t += 1;
        while bcount >= MIN_GALLOP || acount >= MIN_GALLOP {
            assert!(runb.len > 1 && runa.len > 0);
            min_gallop_t -= if min_gallop_t > 1 { 1 } else { 0 };
            ms.min_gallop = min_gallop_t;
            // ??
            let mut k = gallop_right(data, data[runb.start], &mut runa, runa.len - 1, less);
            k = runa.len - k;
            acount = k;
            if k > 0 {
                run_advance(&dest, -k);
                run_advance(&runa, -k);
                sortslice_memmove(&dest, 1, &runa, 1, k);
                // runa.len -= k?
                if runa.len == 0 {
                    break 'outer Exit::Success;
                }
            }
            sortslice_copy_decr(&data, &dest, &runb);
            // --runb.len?
            if runb.len == 1 {
                break 'outer Exit::CopyA;
            }
            // ???
            let mut k = gallop_left(data, data[runa.start], &mut runb, runb.len - 1, less);
            k = runb.len - k;
            bcount = k;
            if k > 0 {
                sortslice_advance(&data, &dest, -k);
                sortslice_advance(&data, &runb, -k);
                sortslice_memcpy(&data, &dest, 1, &runb, k);
                // nb -= k?
                if runb.len == 1 {
                    break 'outer Exit::CopyA;
                }
                if runb.len == 0 {
                    break 'outer Exit::Success;
                }
            }
            sortslice_cop_decr(&data, &dest, &runa);
            // -- na?
            if runa.len == 0 {
                break 'outer Exit::Success;
            }
        }
        min_gallop_t += 1;

        ms.min_gallop = min_gallop_t;
    };

    match exit {
        Exit::Success => {
            if runb.len > 0 {
                sortslice_memcpy(&data, &dest, -runb.len - 1, &runb, runb.len);
            }
        }

        Exit::Fail(e) => {
            if runb.len > 0 {
                sortslice_memcpy(&data, &dest, -runb.len - 1, &runb, runb.len);
            }
        }

        Exit::CopyB => {
            assert!(runa.len == 1 && runb.len > 0);

            sortslice_memmove(&data, &dest, 1 - runa.len, &runa, 1 - runa.len, runa.len);
            sortslice_advance(&data, &dest, -runa.len);
            sortslice_advance(&data, &runa, -runa.len);
            sortslice_copy(&data, &dest, 0, &runb, 0);
        }
    }
}

fn gallop_right<T: Copy, F>(
    data: &mut [T],
    key: T,
    run: &mut Run,
    hint: usize,
    less: &mut F,
) -> usize
where
    F: FnMut(&T, &T) -> bool,
{
    let mut lastofs: usize = 0;
    let mut ofs: usize = 1;
    let mut a: usize = run.start + hint;

    if less(&key, &data[a]) {
        let maxofs = hint + 1;
        while ofs < maxofs {
            if less(&key, &data[a - ofs]) {
                lastofs = ofs;
                ofs = (ofs << 1) + 1;
                if ofs <= 0 {
                    ofs = maxofs;
                }
            } else {
                break;
            }
        }
        if ofs > maxofs {
            ofs = lastofs;
        }
        let k = lastofs;
        lastofs = hint - ofs;
        ofs = hint - k;
    } else {
        let maxofs = run.len - hint;
        while ofs < maxofs {
            if less(&key, &data[a + ofs]) {
                break;
            }
            lastofs = ofs;
            ofs = (ofs << 1) + 1;
            if ofs <= 0 {
                ofs = maxofs;
            }
        }
        if ofs > maxofs {
            ofs = maxofs;
        }

        lastofs += hint;
        ofs += hint;
    }
    a -= hint;

    lastofs += 1;
    while lastofs < ofs {
        let m = lastofs + ((ofs + lastofs) >> 1);

        if less(&key, &data[a + m]) {
            ofs = m;
        } else {
            lastofs = m + 1;
        }
    }

    ofs
}

fn gallop_left<T: Copy, F>(
    data: &mut [T],
    key: T,
    run: &mut Run,
    hint: usize,
    less: &mut F,
) -> usize
where
    F: FnMut(&T, &T) -> bool,
{
    let mut lastofs: usize = 0;
    let mut ofs: usize = 1;
    let mut a: usize = run.start + hint;

    if less(&data[a], &key) {
        let maxofs = run.len - hint;
        while ofs < maxofs {
            if less(&data[a + ofs], &key) {
                lastofs = ofs;
                ofs = (ofs << 1) + 1;
                if ofs <= 0 {
                    ofs = maxofs;
                }
            } else {
                break;
            }
        }
        if ofs > maxofs {
            ofs = lastofs;
        }
        lastofs += hint;
        ofs += hint;
    } else {
        let maxofs = hint + 1;
        while ofs < maxofs {
            if less(&data[a - ofs], &key) {
                break;
            }
            lastofs = ofs;
            ofs = (ofs << 1) + 1;
            if ofs <= 0 {
                ofs = maxofs;
            }
        }
        if ofs > maxofs {
            ofs = maxofs;
        }
        let k = lastofs;
        lastofs += hint - ofs;
        ofs += hint - k;
    }
    a -= hint;

    lastofs += 1;
    while lastofs < ofs {
        let m = lastofs + ((ofs - lastofs) >> 1);

        if less(&data[a + m], &key) {
            lastofs = m + 1;
        } else {
            ofs = m;
        }
    }

    ofs
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
