#[macro_use]
extern crate criterion;
use std::iter::{zip};
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;

use kvs::{KvStore, SledStore, KvsEngine, Result};
use tempfile::TempDir;
use walkdir::WalkDir;


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

fn set_benchmark<S: KvsEngine>(storage: &mut S, keys: Vec<String>, values: Vec<String>) {
    for (key, value) in zip(keys, values) {
        &storage.set(key, value);
    }
}

fn bench_bench(store_type: &str,  keys: Vec<String>, values: Vec<String>) {
    let temp_dir = TempDir::new().unwrap();
    match store_type{
        "kvs" => {
            let mut store = KvStore::open(temp_dir.path()).expect("cant create store");
            set_benchmark(&mut store, keys, values);
        }
        "sled" => {
            let mut store = SledStore::open(temp_dir.path()).expect("cant create store");
            set_benchmark(&mut store, keys, values);
        }
        _ => ()
    }
}

fn from_elem(c: &mut Criterion) {

    // let temp_dir_kvs = TempDir::new().unwrap();
    // let mut kvs_store = KvStore::open(temp_dir_kvs.path()).expect("cant create KVS store");

    // let temp_dir_sled = TempDir::new().unwrap();
    // let mut sled_store = SledStore::open(temp_dir_sled.path()).expect("cant create Sled store");


    let mut group = c.benchmark_group("set_benchmark");

    for store_type in ["kvs", "sled"].iter() {
        group.bench_with_input(BenchmarkId::new("input_example", store_type), &store_type, |b, &s| {
            let keys = || generate_strings(100, 1, 100000).expect("Cant create strings");
            let values = || generate_strings(100, 1, 100000).expect("Cant create strings");
            b.iter(|| bench_bench(s, keys(), values())); 
        });
    }


    
    // for store in [kvs_store].iter() {
    //     group.bench_with_input(BenchmarkId::from_parameter("kvs"), store, |b, &store| {
    //         b.iter(|| zip(keys, values).map(|item| store.set(item.0, item.1)));
    //     });
    // }
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
