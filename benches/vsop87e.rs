//! VSOP87e benchmarks.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use vsop87::vsop87e;

fn vsop87e_sun(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87e Sun", move |b| {
        b.iter(|| vsop87e::sun(black_box(jde)))
    });
}

fn vsop87e_mercury(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87e Mercury", move |b| {
        b.iter(|| vsop87e::mercury(black_box(jde)))
    });
}

fn vsop87e_venus(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87e Venus", move |b| {
        b.iter(|| vsop87e::venus(black_box(jde)))
    });
}

fn vsop87e_earth(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87e Earth", move |b| {
        b.iter(|| vsop87e::earth(black_box(jde)))
    });
}

fn vsop87e_mars(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87e Mars", move |b| {
        b.iter(|| vsop87e::mars(black_box(jde)))
    });
}

fn vsop87e_jupiter(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87e Jupiter", move |b| {
        b.iter(|| vsop87e::jupiter(black_box(jde)))
    });
}

fn vsop87e_saturn(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87e Saturn", move |b| {
        b.iter(|| vsop87e::saturn(black_box(jde)))
    });
}

fn vsop87e_uranus(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87e Uranus", move |b| {
        b.iter(|| vsop87e::uranus(black_box(jde)))
    });
}

fn vsop87e_neptune(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87e Neptune", move |b| {
        b.iter(|| vsop87e::venus(black_box(jde)))
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
