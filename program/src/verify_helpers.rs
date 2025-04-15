const MAX_LEN: usize = 8;

pub fn slice_position<T, F>(slice: &[T], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    kani::assert(slice.len() <= MAX_LEN, "Slice should be bounded to <= MAX_LEN");
    for i in 0..slice.len() {
        kani::assume(i < MAX_LEN);
        if predicate(&slice[i]) {
            return Some(i);
        }
    }
    None
}

pub fn slice_find<T, F, const LIM: usize>(slice: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    kani::assert(slice.len() <= LIM, "Slice should be bounded to <= MAX_LEN");
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < LIM);
        let entry = &slice[i];
        if predicate(entry) {
            return Some(entry);
        }
        i += 1;
    }
    None
}

pub fn slice_all<T, F>(slice: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool,
{
    kani::assert(slice.len() <= MAX_LEN, "Slice should be bounded to <= MAX_LEN");
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
    kani::assert(slice.len() <= MAX_LEN, "Slice should be bounded to <= MAX_LEN");
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        f(&slice[i]);
        i += 1;
    }
}

pub fn slice_for_each_mut<T, F>(slice: &mut [T], mut f: F)
where
    F: FnMut(&mut T),
{
    kani::assert(slice.len() <= MAX_LEN, "Slice should be bounded to <= MAX_LEN");
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < MAX_LEN);
        f(&mut slice[i]);
        i += 1;
    }
}

pub fn slice_filter_map<'a, T, U, F, const LIM: usize>(slice: &'a [T], predicate: F) -> Vec<U>
where
    F: Fn(&'a T) -> Option<U>,
    U: 'a,
{
    kani::assert(slice.len() <= LIM, "Slice should be bounded to <= MAX_LEN");
    let mut vec = Vec::new();
    let mut i = 0;
    while i < slice.len() {
        kani::assume(i < LIM);
        let entry = &slice[i];
        if let Some(mapped) = predicate(entry) {
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
    kani::assert(slice.len() <= MAX_LEN, "Slice should be bounded to <= MAX_LEN");
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
