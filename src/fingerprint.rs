use util::get_hash;

use std::hash::{Hash, Hasher};
use std::cmp::PartialEq;

pub const FINGERPRINT_SIZE: usize = 1;
pub const EMPTY_FINGER_PRINT: FingerPrint = FingerPrint { fp: [0; FINGERPRINT_SIZE] };

#[derive(Clone, Copy)]
pub struct FingerPrint {
    fp: [u8; FINGERPRINT_SIZE]
}

impl FingerPrint {
    pub fn gen_finger_print<T, H>(item: &T, hash_fn: &H) -> FingerPrint
        where T: Hash, H: Hasher + Clone {
        let h = get_hash(item, hash_fn);
        let mut fp = [0; FINGERPRINT_SIZE];

        for i in 0..FINGERPRINT_SIZE {
            let (f, _) = h.overflowing_shr((8 - i - 1) as u32 * 8u32);
            fp[i] = f as u8;
        }

        FingerPrint {
            fp: fp
        }
    }

    pub fn is_empty(&self) -> bool {
        self.fp[0] == 0
    }
}

impl Hash for FingerPrint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fp.hash(state)
    }
}

impl PartialEq for FingerPrint {
    fn eq(&self, other: &FingerPrint) -> bool {
        for i in 0..FINGERPRINT_SIZE {
            if self.fp[i] != other.fp[i] {
                return false;
            }
        }
        true
    }
}
