//! VSOP87c benchmarks.

#[macro_use]
extern crate criterion;
extern crate rand;
extern crate test;
extern crate vsop87;
extern crate vsop87;

use criterion::Criterion;
use rand::{thread_rng, Rng};
use vsop87::vsop87c;

fn vsop87c_mercury(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87c Mercury", move |b| {
        b.iter(|| vsop87c::mercury(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87c_venus(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87c Venus", move |b| {
        b.iter(|| vsop87c::venus(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87c_earth(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87c Earth", move |b| {
        b.iter(|| vsop87c::earth(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87c_mars(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87c Mars", move |b| {
        b.iter(|| vsop87c::mars(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87c_jupiter(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87c Jupiter", move |b| {
        b.iter(|| vsop87c::jupiter(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87c_saturn(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87c Saturn", move |b| {
        b.iter(|| vsop87c::saturn(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87c_uranus(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87c Uranus", move |b| {
        b.iter(|| vsop87c::uranus(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87c_neptune(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87c Neptune", move |b| {
        b.iter(|| vsop87c::venus(rng.gen_range(990930.5, 3912521.5)))
    });
}

criterion_group!(
    vsop87c_benches,
    vsop87c_mercury,
    vsop87c_venus,
    vsop87c_earth,
    vsop87c_mars,
    vsop87c_jupiter,
    vsop87c_saturn,
    vsop87c_uranus,
    vsop87c_neptune
);
criterion_main!(vsop87c_benches);
