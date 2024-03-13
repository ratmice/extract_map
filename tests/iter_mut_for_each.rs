#![cfg(feature = "iter_mut")]
use extract_map::{ExtractKey, ExtractMap, Lender};

struct User {
    id: u64,
    name: String,
}

impl ExtractKey<u64> for User {
    fn extract_key(&self) -> &u64 {
        &self.id
    }
}

#[test]
pub fn test() {
    let mut map = ExtractMap::<u64, User>::new();
    let iter_mut = map.iter_mut();
    // Should compile_fail.
    // drop(map);
    iter_mut.for_each(|_| {});
}
