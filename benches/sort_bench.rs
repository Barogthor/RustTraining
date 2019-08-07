extern crate rand;
extern crate rust_training;
extern crate bencher;

use rand::Rng;
use rust_training::sort::select_sort;
use bencher::Bencher;
use bencher::benchmark_group;
use bencher::benchmark_main;

fn bench_select_sort(b: &mut Bencher) {
    let mut _rand = rand::thread_rng();
    let values: Vec<i64> = (0..1000).map(|_| _rand.gen_range(0, 1000)).collect();
    b.iter(|| select_sort(&values, None));
}

benchmark_group!(benches, bench_select_sort);
benchmark_main!(benches);
