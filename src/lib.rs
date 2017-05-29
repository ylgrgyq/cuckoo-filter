extern crate siphasher;

use std::hash::{Hash, Hasher};
use siphasher::sip::SipHasher;

const FINGERPRINT_SIZE: usize = 1;
const ENTRIES_PER_BUCKET: usize = 4;
const BUCKETS_SIZE: u64 = 100000;

struct FingerPrint {
    fp: [u8; FINGERPRINT_SIZE]
}

impl FingerPrint {
//    fn gen_finger_print<T>(item: &T) -> [u8] {
//
//    }
}

struct Bucket {
    entry: [FingerPrint; ENTRIES_PER_BUCKET]
}

struct CuckooFilter {
    buckets: Box<[Bucket]>,
    hash_fn: SipHasher
}

impl CuckooFilter {
    fn set<T>(&mut self, item: &T) -> bool
    where T: Hash {
//        let fp = FingerPrint::gen_finger_print(item);
        true
    }
}

fn get_hash<T, H>(item: &T, hash_fn: &H) -> u64
where T: Hash, H: Hasher + Clone {
    let hasher = &mut hash_fn.clone();
    item.hash(hasher);
    let ret = hash_fn.finish();
    println!("{}", ret);
    ret
}

#[test]
fn test_haha() {
    let s = String::from("sdsd");
    get_hash(&s, &SipHasher::new());
    assert!(true);
}



