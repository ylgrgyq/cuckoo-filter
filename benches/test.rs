extern crate rust_cuckoofilter;

use rust_cuckoofilter::CuckooFilter;
use std::time::SystemTime;

#[test]
fn test() {
    let test_size = 1000000;
    println!("starting to test cuckoo filter, {} keys", test_size);

    let mut f1 = CuckooFilter::new();
    let now = SystemTime::now();
    let result = (0..test_size)
        .into_iter()
        .map(|n| f1.insert(&n.to_string()))
        .filter(|b: &Result<bool, &str> | b.unwrap_or(false))
        .count();
    let n2 = SystemTime::now();
    let d = n2.duration_since(now).unwrap();
    println!("test size {}, true {} in {}.{} secs",
             test_size,
             result,
             d.as_secs(),
             d.subsec_nanos());

    let test_size = 100000;
    println!("starting to test cuckoo filter, {} keys", test_size);

    let mut f1 = CuckooFilter::new();
    let now = SystemTime::now();
    let result = (0..test_size)
        .into_iter()
        .map(|n| f1.insert(&n.to_string()))
        .filter(|b: &Result<bool, &str> | b.unwrap_or(false))
        .count();
    let n2 = SystemTime::now();
    let d = n2.duration_since(now).unwrap();
    println!("test size {}, true {} in {}.{} secs",
             test_size,
             result,
             d.as_secs(),
             d.subsec_nanos());
}
