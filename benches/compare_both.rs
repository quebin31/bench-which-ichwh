use criterion::{black_box, criterion_group, criterion_main, Criterion};
use futures::executor::block_on;
use std::time::Duration;

fn which_for_existent(c: &mut Criterion) {
    c.bench_function("which git", |b| b.iter(|| which::which(black_box("git"))));
}

fn which_for_non_existent(c: &mut Criterion) {
    c.bench_function("which non_existent_exec", |b| {
        b.iter(|| which::which(black_box("non_existent_exec")))
    });
}

fn ichwh_for_existent(c: &mut Criterion) {
    c.bench_function("ichwh git", |b| {
        b.iter(|| block_on(ichwh::which(black_box("git"))))
    });
}

fn ichwh_for_non_existent(c: &mut Criterion) {
    c.bench_function("ichwh non_existent_exec", |b| {
        b.iter(|| block_on(ichwh::which(black_box("non_existent_exec"))))
    });
}

criterion_group! {
    name = which_benches;
    config = Criterion::default().sample_size(10);
    targets = which_for_existent, which_for_non_existent
}

criterion_group! {
    name = ichwh_benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(15));
    targets = ichwh_for_existent, ichwh_for_non_existent
}

criterion_main!(which_benches, ichwh_benches);
