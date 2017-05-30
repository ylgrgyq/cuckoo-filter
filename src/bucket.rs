use fingerprint::{FingerPrint, EMPTY_FINGER_PRINT};

pub const ENTRIES_PER_BUCKET: usize = 4;

#[derive(Clone)]
pub struct Bucket {
    pub entries: [FingerPrint; ENTRIES_PER_BUCKET]
}

impl Bucket {
    pub fn new() -> Bucket {
        Bucket {
            entries: [EMPTY_FINGER_PRINT; ENTRIES_PER_BUCKET]
        }
    }

    pub fn put(&mut self, fp: FingerPrint) -> bool {
        for i in 0..ENTRIES_PER_BUCKET {
            if self.entries[i].is_empty() {
                self.entries[i] = fp;
                return true;
            }
        }
        false
    }

    pub fn get(&self, fp: &FingerPrint) -> bool {
        for i in 0..ENTRIES_PER_BUCKET {
            if self.entries[i] == *fp {
                return true;
            }
        }

        false
    }

    pub fn remove(&mut self, fp: &FingerPrint) -> bool {
        for i in 0..ENTRIES_PER_BUCKET {
            if self.entries[i] == *fp {
                self.entries[i] = EMPTY_FINGER_PRINT;
                return true
            }
        }

        return false
    }
}