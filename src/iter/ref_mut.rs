use std::{
    collections::VecDeque,
    hash::{BuildHasher, Hash},
};

use crate::{mut_guard::MutGuard, ExtractKey, ExtractMap};
use lender::prelude::*;

#[must_use = "Iterators do nothing if not consumed"]
pub struct IterMut<'a, K, V, S> {
    map: &'a mut ExtractMap<K, V, S>,
    keys: VecDeque<K>,
}

impl<'a, K, V, S> IterMut<'a, K, V, S>
where
    K: Hash + Eq + Clone,
    V: ExtractKey<K>,
    S: BuildHasher,
{
    pub fn new(map: &'a mut ExtractMap<K, V, S>) -> Self {
        let keys = map.iter().map(ExtractKey::extract_key).cloned().collect();
        Self { map, keys }
    }
}

impl<'lend, 'this, K, V, S> Lending<'lend> for IterMut<'this, K, V, S>
where
    K: Hash + Eq,
    V: ExtractKey<K>,
    S: BuildHasher,
{
    type Lend = MutGuard<'lend, K, V, S>;
}

impl<'lend, K, V, S> Lender for IterMut<'lend, K, V, S>
where
    K: Hash + Eq,
    V: ExtractKey<K>,
    S: BuildHasher,
{
    fn next(&mut self) -> Option<Lend<'_, Self>> {
        let key = self.keys.pop_front()?;
        self.map.get_mut(&key)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.keys.len(), Some(self.keys.len()))
    }
}
