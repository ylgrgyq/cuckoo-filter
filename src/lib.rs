#![crate_name="rust_cuckoofilter"]
#![crate_type = "rlib"]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate siphasher;
extern crate rand;

mod fingerprint;
mod util;
mod bucket;

use self::fingerprint::{FingerPrint};
use self::bucket::{Bucket, ENTRIES_PER_BUCKET};
use self::util::get_hash;

use std::hash::{Hash};
use std::iter::repeat;
use siphasher::sip::SipHasher;
use rand::{Rng, ThreadRng};

const BUCKETS_SIZE: usize = 100000;
const MAX_NUM_KICKS: u32 = 500;

pub struct CuckooFilter {
    buckets: Box<[Bucket]>,
    hash_fn: SipHasher,
    size: usize,
    capacity: usize,
    max_num_kicks: u32
}

impl CuckooFilter {
    pub fn new() -> CuckooFilter {
        CuckooFilter {
            hash_fn: CuckooFilter::sip_new(),
            size: 0,
            capacity: BUCKETS_SIZE as usize * ENTRIES_PER_BUCKET,
            max_num_kicks: MAX_NUM_KICKS,
            buckets:
            repeat(Bucket::new())
                .take(BUCKETS_SIZE as usize)
                .collect::<Vec<_>>()
                .into_boxed_slice()
        }
    }

    pub fn with_bucket_size(size: usize) -> CuckooFilter {
        CuckooFilter {
            hash_fn: CuckooFilter::sip_new(),
            size: 0,
            capacity: size as usize * ENTRIES_PER_BUCKET,
            max_num_kicks: MAX_NUM_KICKS,
            buckets:
            repeat(Bucket::new())
                .take(size as usize)
                .collect::<Vec<_>>()
                .into_boxed_slice()
        }
    }

    pub fn insert<T>(&mut self, item: &T) -> Result<bool, &str>
        where T: Hash {
        let mut fp = FingerPrint::gen_finger_print(item, &self.hash_fn);
        let i1 = self.get_index(item);
        let i2 = self.get_alt_index(i1, &fp);
        if self.put(i1, fp) || self.put(i2, fp) {
            self.size += 1;
            return Ok(true);
        }

        let mut rng = rand::thread_rng();
        let mut i = self.choose_random_index(i1, i2, &mut rng);
        let len = self.buckets.len();

        for _ in 0..self.max_num_kicks {
            let next_fp;
            {
                let e = &mut (self.buckets[i % len].entries[rng.gen_range(0, ENTRIES_PER_BUCKET)]);
                next_fp = *e;
                *e = fp;
            }
            i = self.get_alt_index(i, &next_fp);
            if self.put(i, next_fp) {
                self.size += 1;
                return Ok(true)
            }
            fp = next_fp;
        }

        Err("CuckooFilter is Full")
    }

    pub fn lookup<T>(&self, item: &T) -> bool
        where T: Hash {
        let fp = FingerPrint::gen_finger_print(item, &self.hash_fn);
        let i1 = self.get_index(item);
        let i2 = self.get_alt_index(i1, &fp);
        if self.get(i1, &fp) || self.get(i2, &fp) {
            return true;
        }

        false
    }

    pub fn delete<T>(&mut self, item: &T) -> bool
        where T: Hash {
        let fp = FingerPrint::gen_finger_print(item, &self.hash_fn);
        let i1 = self.get_index(item);
        let i2 = self.get_alt_index(i1, &fp);
        if self.remove(i1, &fp) || self.remove(i2, &fp) {
            self.size -= 1;
            return true;
        }
        return false
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    fn get(&self, index: usize, fp: &FingerPrint) -> bool {
        let index = index % self.buckets.len();
        self.buckets[index].get(fp)
    }

    fn put(&mut self, index: usize, fp: FingerPrint) -> bool {
        let index = index % self.buckets.len();
        self.buckets[index].put(fp)
    }

    fn remove(&mut self, index: usize, fp: &FingerPrint) -> bool {
        let index = index % self.buckets.len();
        self.buckets[index].remove(fp)
    }

    fn get_index<T>(&self, item: &T) -> usize
        where T: Hash {
        get_hash(item, &self.hash_fn) as usize
    }

    fn get_alt_index(&self, index: usize, fp: &FingerPrint) -> usize {
        index ^ get_hash(fp, &self.hash_fn) as usize
    }

    fn choose_random_index(&self, index1: usize, index2: usize, rng: &mut ThreadRng) -> usize {
        if rng.gen() {
            return index1
        } else {
            return index2
        }
    }

    fn sip_new() -> SipHasher {
        let mut rng = rand::thread_rng();
        SipHasher::new_with_keys(rand::Rand::rand(&mut rng),
                                 rand::Rand::rand(&mut rng))
    }
}

#[test]
fn test_insert_lookup_delete() {
    let mut f = CuckooFilter::new();
    let test_str = &String::from("Haha");
    assert!(f.lookup(test_str) == false);
    assert!(f.insert(test_str).unwrap_or(false) == true);
    assert_eq!(f.get_size(), 1);
    assert!(f.lookup(test_str) == true);
    assert!(f.insert(test_str).unwrap_or(false) == true);
    assert!(f.insert(test_str).unwrap_or(false) == true);
    assert_eq!(f.get_size(), 3);
    assert!(f.lookup(test_str) == true);
    assert!(f.delete(test_str) == true);
    assert_eq!(f.get_size(), 2);
    assert!(f.lookup(test_str) == true);
    assert!(f.delete(test_str) == true);
    assert_eq!(f.get_size(), 1);
    assert!(f.lookup(test_str) == true);
    assert!(f.delete(test_str) == true);
    assert_eq!(f.get_size(), 0);
    assert!(f.lookup(test_str) == false);
}