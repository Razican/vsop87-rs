//! VSOP87b benchmarks.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use vsop87::vsop87b;

fn vsop87b_mercury(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87b Mercury", move |b| {
        b.iter(|| vsop87b::mercury(black_box(jde)))
    });
}

fn vsop87b_venus(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87b Venus", move |b| {
        b.iter(|| vsop87b::venus(black_box(jde)))
    });
}

fn vsop87b_earth(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87b Earth", move |b| {
        b.iter(|| vsop87b::earth(black_box(jde)))
    });
}

fn vsop87b_mars(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87b Mars", move |b| {
        b.iter(|| vsop87b::mars(black_box(jde)))
    });
}

fn vsop87b_jupiter(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87b Jupiter", move |b| {
        b.iter(|| vsop87b::jupiter(black_box(jde)))
    });
}

fn vsop87b_saturn(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87b Saturn", move |b| {
        b.iter(|| vsop87b::saturn(black_box(jde)))
    });
}

fn vsop87b_uranus(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87b Uranus", move |b| {
        b.iter(|| vsop87b::uranus(black_box(jde)))
    });
}

fn vsop87b_neptune(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87b Neptune", move |b| {
        b.iter(|| vsop87b::venus(black_box(jde)))
    });
}

criterion_group!(
    vsop87b_benches,
    vsop87b_mercury,
    vsop87b_venus,
    vsop87b_earth,
    vsop87b_mars,
    vsop87b_jupiter,
    vsop87b_saturn,
    vsop87b_uranus,
    vsop87b_neptune
);
criterion_main!(vsop87b_benches);
