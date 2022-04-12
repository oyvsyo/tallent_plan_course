#[macro_use]
extern crate criterion;
use std::iter::{zip};
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;
use criterion::BenchmarkId;
use criterion::Criterion;

use kvs::{KvStore, SledStore, KvsEngine, Result};
use tempfile::TempDir;


fn generate_strings(num: usize, min: usize, max: usize) -> Result<Vec<String>>{
    
    let mut strings = vec![];
    let mut rng = rand::thread_rng();
    
    for _ in 0..num {
        let len: usize = rng.gen_range(min..max);
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), len);
        strings.push(string);
    } 
    Ok(strings)
}

fn copy_strings(strings: &Vec<String>) -> Vec<String> {
    strings.iter().map(|s| s.clone()).collect()
}

fn set_benchmark<S: KvsEngine>(storage: &mut S, keys: Vec<String>, values: Vec<String>) {
    for (key, value) in zip(keys, values) {
        let _ = &storage.set(key, value);
    }
}

fn from_elem(c: &mut Criterion) {

    let mut group = c.benchmark_group("set_benchmark");

    for store_type in ["kvs", "sled"].iter() {
        group.bench_with_input(BenchmarkId::new("input_example", store_type), &store_type, |b, &s| {
            let keys = generate_strings(100, 1, 100000).expect("Cant create strings");
            let values = generate_strings(100, 1, 100000).expect("Cant create strings");
            let temp_dir = TempDir::new().unwrap();
            match *s{
                "kvs" => {
                    let mut store = KvStore::open(temp_dir.path()).expect("cant create store");
                    b.iter(|| set_benchmark(&mut store, copy_strings(&keys), copy_strings(&values)));
                }
                "sled" => {
                    let mut store = SledStore::open(temp_dir.path()).expect("cant create store");
                    b.iter(|| set_benchmark(&mut store, copy_strings(&keys), copy_strings(&values)))
                }
                _ => ()
            }
        });
    }
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
