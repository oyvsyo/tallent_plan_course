#[macro_use]
extern crate criterion;
use criterion::BenchmarkId;
use criterion::Criterion;
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;
use std::iter::zip;

use kvs::{KvStore, KvsEngine, Result, SledStore};
use tempfile::TempDir;

fn generate_strings(num: usize, min: usize, max: usize) -> Result<Vec<String>> {
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

fn set_values<S: KvsEngine>(storage: &mut S, keys: Vec<String>, values: Vec<String>) {
    for (key, value) in zip(keys, values) {
        let _ = &storage.set(key, value);
    }
}

fn get_values<S: KvsEngine>(storage: &mut S, keys: Vec<String>, values: Vec<String>) {
    for (key, value) in zip(keys, values) {
        let old_value = storage.get(key).expect("cant get");
        assert_eq!(Some(value), old_value);
    }
}

fn set_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_benchmark");

    for store_type in ["kvs", "sled"].iter() {
        group.bench_with_input(
            BenchmarkId::new("input_example", store_type),
            &store_type,
            |b, &s| {
                let keys = generate_strings(100, 1, 100000).expect("Cant create strings");
                let values = generate_strings(100, 1, 100000).expect("Cant create strings");
                let temp_dir = TempDir::new().unwrap();
                match *s {
                    "kvs" => {
                        let mut store = KvStore::open(temp_dir.path()).expect("cant create store");
                        b.iter(|| {
                            set_values(&mut store, copy_strings(&keys), copy_strings(&values))
                        });
                    }
                    "sled" => {
                        let mut store =
                            SledStore::open(temp_dir.path()).expect("cant create store");
                        b.iter(|| {
                            set_values(&mut store, copy_strings(&keys), copy_strings(&values))
                        })
                    }
                    _ => (),
                }
            },
        );
    }
    group.finish();
}

fn get_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_benchmark");

    for store_type in ["kvs", "sled"].iter() {
        group.bench_with_input(
            BenchmarkId::new("input_example", store_type),
            &store_type,
            |b, &s| {
                let keys = generate_strings(100, 1, 100000).expect("Cant create strings");
                let values = generate_strings(100, 1, 100000).expect("Cant create strings");
                let temp_dir = TempDir::new().unwrap();
                match *s {
                    "kvs" => {
                        let mut store = KvStore::open(temp_dir.path()).expect("cant create store");
                        set_values(&mut store, copy_strings(&keys), copy_strings(&values));
                        b.iter(|| {
                            get_values(&mut store, copy_strings(&keys), copy_strings(&values))
                        });
                    }
                    "sled" => {
                        let mut store =
                            SledStore::open(temp_dir.path()).expect("cant create store");
                        set_values(&mut store, copy_strings(&keys), copy_strings(&values));
                        b.iter(|| {
                            get_values(&mut store, copy_strings(&keys), copy_strings(&values))
                        })
                    }
                    _ => (),
                }
            },
        );
    }
    group.finish();
}

criterion_group!(benches, set_benchmark, get_benchmark);
criterion_main!(benches);
