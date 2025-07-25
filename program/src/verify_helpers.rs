use std::ops::Add;
use super::vec::fast::Vec;

/// Default maximum length for slices used in verification helpers.
/// If we're working with an array, use an `array_` variant instead.
const MAX_LEN: usize = 2;

pub fn array_map_sum<const N: usize, T, A, F>(slice: &[T; N], predicate: F) -> A
where
    F: Fn(&T) -> A,
    A: Add<Output = A> + Default
{
    let mut total = A::default();
    let mut i = 0;
    while i < N {
        kani::assume(i < N);
        total = total + predicate(&slice[i]);
        i += 1;
    }
    total
}

pub fn slice_position<T, F>(slice: &[T], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    kani::assert(
        slice.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        if predicate(&slice[i]) {
            return Some(i);
        }
        i += 1;
    }
    None
}

pub fn array_position<const N: usize, T, F>(slice: &[T; N], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    let mut i = 0;
    while i < N {
        kani::assume(i < N);
        if predicate(&slice[i]) {
            return Some(i);
        }
        i += 1;
    }
    None
}

pub fn slice_find<T, F>(slice: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    kani::assert(
        slice.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        let entry = &slice[i];
        if predicate(entry) {
            return Some(entry);
        }
        i += 1;
    }
    None
}

pub fn slice_enumerate_find_mut<T, F>(slice: &mut [T], predicate: F) -> Option<(usize, &mut T)>
where
    F: Fn((usize, &mut T)) -> bool,
{
    kani::assert(
        slice.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        if predicate((i, &mut slice[i])) {
            return Some((i, &mut slice[i]));
        }
        i += 1;
    }
    None
}

pub fn array_enumerate_find_mut<const N: usize, T, F>(
    slice: &mut [T; N],
    predicate: F,
) -> Option<(usize, &mut T)>
where
    F: Fn((usize, &mut T)) -> bool,
{
    let mut i = 0;
    while i < N {
        kani::assume(i < N);
        if predicate((i, &mut slice[i])) {
            return Some((i, &mut slice[i]));
        }
        i += 1;
    }
    None
}

pub fn array_find<const N: usize, T, F>(slice: &[T; N], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    let mut i = 0;
    while i < N {
        kani::assume(i < N);
        let entry = &slice[i];
        if predicate(entry) {
            return Some(entry);
        }
        i += 1;
    }
    None
}

pub fn array_all<const N: usize, T, F>(slice: &[T; N], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    let mut i = 0;
    let mut res = true;
    while i < N {
        kani::assume(i < N);
        res &= predicate(&slice[i]);
        i += 1;
    }
    res
}

pub fn slice_all<T, F>(slice: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    kani::assert(
        slice.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut i = 0;
    let mut res = true;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        res &= predicate(&slice[i]);
        i += 1;
    }
    res
}

pub fn slice_for_each<T, F>(slice: &[T], mut f: F)
where
    F: FnMut(&T),
{
    kani::assert(
        slice.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        f(&slice[i]);
        i += 1;
    }
}

pub fn array_for_each<const N: usize, T, F>(slice: &[T; N], mut f: F)
where
    F: FnMut(&T),
{
    let mut i = 0;
    while i < N {
        kani::assume(i < N);
        f(&slice[i]);
        i += 1;
    }
}

pub fn slice_for_each_zip<T, U, F>(slice1: &[T], slice2: &[U], mut f: F)
where
    F: FnMut((&T, &U)),
{
    kani::assert(
        slice1.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    kani::assert(
        slice2.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut i = 0;
    while i < slice1.len() && i < slice2.len() {
        kani::assume(i < MAX_LEN);
        f((&slice1[i], &slice2[i]));
        i += 1;
    }
}

pub fn slice_for_each_mut<T, F>(slice: &mut [T], mut f: F)
where
    F: FnMut(&mut T),
{
    kani::assert(
        slice.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        f(&mut slice[i]);
        i += 1;
    }
}

pub fn slice_map<'a, T, U, F>(slice: &'a [T], predicate: F) -> Vec<U>
where
    F: Fn(&'a T) -> U,
    U: 'a,
{
    kani::assert(
        slice.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut vec = Vec::new();
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        let entry = &slice[i];
        vec.push(predicate(entry));
        i += 1;
    }
    vec
}

pub fn array_filter<'a, const N: usize, T, F>(slice: &'a [T; N], predicate: F) -> Vec<&'a T>
where
    F: Fn(&'a T) -> bool,
{
    let mut vec = Vec::new();
    let mut i = 0;
    while i < N {
        kani::assume(i < N);
        let entry = &slice[i];
        if predicate(entry) {
            vec.push(entry);
        }
        i += 1;
    }
    vec
}

pub fn array_filter_mut<'a, const N: usize, T, F>(
    slice: &'a mut [T; N],
    predicate: F,
) -> Vec<&'a mut T>
where
    F: for<'b> Fn(&'b mut T) -> bool,
{
    let mut vec = Vec::new();
    let mut i = 0;
    while i < N {
        kani::assume(i < N);
        // SAFETY: This is in bounds due to the loop condition, and the mutable borrows are disjoint
        let entry = unsafe { &mut *slice.as_mut_ptr().add(i) };
        if predicate(entry) {
            vec.push(entry);
        }
        i += 1;
    }
    vec
}

pub fn array_enumerate_filter_mut<'a, const N: usize, T, F>(
    slice: &'a mut [T; N],
    predicate: F,
) -> Vec<(usize, &'a mut T)>
where
    F: for<'b> Fn((usize, &'b mut T)) -> bool,
{
    let mut vec = Vec::new();
    let mut i = 0;
    while i < N {
        kani::assume(i < N);
        // SAFETY: This is in bounds due to the loop condition, and the mutable borrows are disjoint
        let entry = unsafe { &mut *slice.as_mut_ptr().add(i) };
        if predicate((i, entry)) {
            vec.push((i, entry));
        }
        i += 1;
    }
    vec
}

pub fn slice_filter_map<'a, T, U, F>(slice: &'a [T], predicate: F) -> Vec<U>
where
    F: Fn(&'a T) -> Option<U>,
    U: 'a,
{
    kani::assert(
        slice.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut vec = Vec::new();
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        let entry = &slice[i];
        if let Some(mapped) = predicate(entry) {
            vec.push(mapped);
        }
        i += 1;
    }
    vec
}

pub fn slice_enumerate_filter_map<'a, T, U, F>(slice: &'a [T], predicate: F) -> Vec<U>
where
    F: Fn((usize, &'a T)) -> Option<U>,
    U: 'a,
{
    kani::assert(
        slice.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut vec = Vec::new();
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        let entry = &slice[i];
        if let Some(mapped) = predicate((i, entry)) {
            vec.push(mapped);
        }
        i += 1;
    }
    vec
}

pub fn array_enumerate_filter_map<'a, const N: usize, T, U, F>(
    slice: &'a [T; N],
    predicate: F,
) -> Vec<U>
where
    F: Fn((usize, &'a T)) -> Option<U>,
    U: 'a,
{
    let mut vec = Vec::new();
    let mut i = 0;
    while i < N {
        kani::assume(i < N);
        let entry = &slice[i];
        if let Some(mapped) = predicate((i, entry)) {
            vec.push(mapped);
        }
        i += 1;
    }
    vec
}

pub fn slice_filter_count<'a, T, F>(slice: &'a [T], predicate: F) -> usize
where
    F: Fn(&T) -> bool,
{
    kani::assert(
        slice.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut count: usize = 0;
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        let entry = &slice[i];
        if predicate(entry) {
            count += 1;
        }
        i += 1;
    }
    count
}

pub fn slice_reverse<T>(slice: &mut [T]) {
    kani::assert(
        slice.len() <= MAX_LEN,
        "Slice should be bounded to <= MAX_LEN",
    );
    let mut count: usize = 0;
    let mut i = 0;
    let half_len = slice.len() / 2;
    while i < half_len {
        kani::assume(i < MAX_LEN);
        kani::assume(slice.len() - 1 - i < MAX_LEN);
        slice.swap(slice.len() - 1 - i, i);
    }
}
