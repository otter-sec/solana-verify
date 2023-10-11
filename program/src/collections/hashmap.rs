use crate::vec::fast::Vec;

const CAPACITY: usize = 10;

#[derive(Debug)]
pub struct HashMap<K, V> {
    buckets: Vec<(K, V)>,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Copy + Default,
    V: Copy + Default,
{
    pub fn new() -> Self {
        let buckets = Vec::with_capacity(CAPACITY);
        HashMap { buckets }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.buckets.push((key, value));
    }

    pub fn get(&self, key: K) -> Option<&V> {
        for (k, v) in &self.buckets {
            if *k == key {
                return Some(v);
            }
        }
        None
    }
}

impl<K, V> Default for HashMap<K, V>
where
    K: Eq + Copy + Default,
    V: Copy + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Eq + Copy + Default,
    V: Copy + Default,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut map = HashMap::new();

        for (key, value) in iter {
            map.insert(key, value);
        }

        map
    }
}
