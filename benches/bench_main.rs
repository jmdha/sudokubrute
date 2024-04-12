mod benchmarks;

use criterion::criterion_main;

criterion_main! {
    benchmarks::solve::benches,
}
