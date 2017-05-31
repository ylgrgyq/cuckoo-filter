
use std::hash::{Hash, Hasher};

pub fn get_hash<T, H>(item: &T, hash_fn: &H) -> u64
    where T: Hash, H: Hasher + Clone {
    let hasher = &mut hash_fn.clone();
    item.hash(hasher);
    let ret = hasher.finish();
    ret
}