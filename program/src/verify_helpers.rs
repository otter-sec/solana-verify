pub fn slice_position<T, F>(slice: &[T], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool
{
    for i in 0..slice.len() {
        if (predicate(&slice[i])) { return Some(i); }
    }
    None
}

pub fn slice_find<T, F>(slice: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool
{
    for i in 0..slice.len() {
        if (predicate(&slice[i])) { return Some(&slice[i]); }
    }
    None
}

pub fn slice_all<T, F>(slice: &[T], predicate: F) -> bool
where
    F: Fn(&T) -> bool
{
    let mut res = true;
    for i in 0..slice.len() {
        res &= predicate(&slice[i])
    }
    res
}

pub fn slice_for_each_mut<T, F>(slice: &mut [T], f: F)
where
    F: Fn(&mut T),
{
    for i in 0..slice.len() {
        f(&mut slice[i]);
    }
}
