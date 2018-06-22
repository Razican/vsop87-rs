//! VSOP87b benchmarks.

#[macro_use]
extern crate criterion;
extern crate rand;
extern crate vsop87;

use criterion::Criterion;
use rand::{thread_rng, Rng};
use vsop87::vsop87b;

fn vsop87b_mercury(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87b Mercury", move |b| {
        b.iter(|| vsop87b::mercury(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87b_venus(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87b Venus", move |b| {
        b.iter(|| vsop87b::venus(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87b_earth(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87b Earth", move |b| {
        b.iter(|| vsop87b::earth(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87b_mars(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87b Mars", move |b| {
        b.iter(|| vsop87b::mars(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87b_jupiter(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87b Jupiter", move |b| {
        b.iter(|| vsop87b::jupiter(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87b_saturn(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87b Saturn", move |b| {
        b.iter(|| vsop87b::saturn(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87b_uranus(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87b Uranus", move |b| {
        b.iter(|| vsop87b::uranus(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87b_neptune(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87b Neptune", move |b| {
        b.iter(|| vsop87b::venus(rng.gen_range(990930.5, 3912521.5)))
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
