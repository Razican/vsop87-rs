//! VSOP87e benchmarks.

#[macro_use]
extern crate criterion;
extern crate rand;
extern crate vsop87;

use criterion::Criterion;
use rand::{thread_rng, Rng};
use vsop87::vsop87e;

fn vsop87e_sun(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87e Sun", move |b| {
        b.iter(|| vsop87e::sun(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87e_mercury(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87e Mercury", move |b| {
        b.iter(|| vsop87e::mercury(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87e_venus(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87e Venus", move |b| {
        b.iter(|| vsop87e::venus(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87e_earth(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87e Earth", move |b| {
        b.iter(|| vsop87e::earth(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87e_mars(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87e Mars", move |b| {
        b.iter(|| vsop87e::mars(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87e_jupiter(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87e Jupiter", move |b| {
        b.iter(|| vsop87e::jupiter(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87e_saturn(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87e Saturn", move |b| {
        b.iter(|| vsop87e::saturn(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87e_uranus(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87e Uranus", move |b| {
        b.iter(|| vsop87e::uranus(rng.gen_range(990930.5, 3912521.5)))
    });
}

fn vsop87e_neptune(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87e Neptune", move |b| {
        b.iter(|| vsop87e::venus(rng.gen_range(990930.5, 3912521.5)))
    });
}

criterion_group!(
    vsop87e_benches,
    vsop87e_sun,
    vsop87e_mercury,
    vsop87e_venus,
    vsop87e_earth,
    vsop87e_mars,
    vsop87e_jupiter,
    vsop87e_saturn,
    vsop87e_uranus,
    vsop87e_neptune
);
criterion_main!(vsop87e_benches);
