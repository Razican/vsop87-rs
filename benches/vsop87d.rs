//! VSOP87d benchmarks.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use vsop87::vsop87d;

fn vsop87d_mercury(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87d Mercury", move |b| {
        b.iter(|| vsop87d::mercury(black_box(jde)))
    });
}

fn vsop87d_venus(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87d Venus", move |b| {
        b.iter(|| vsop87d::venus(black_box(jde)))
    });
}

fn vsop87d_earth(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87d Earth", move |b| {
        b.iter(|| vsop87d::earth(black_box(jde)))
    });
}

fn vsop87d_mars(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87d Mars", move |b| {
        b.iter(|| vsop87d::mars(black_box(jde)))
    });
}

fn vsop87d_jupiter(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87d Jupiter", move |b| {
        b.iter(|| vsop87d::jupiter(black_box(jde)))
    });
}

fn vsop87d_saturn(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87d Saturn", move |b| {
        b.iter(|| vsop87d::saturn(black_box(jde)))
    });
}

fn vsop87d_uranus(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87d Uranus", move |b| {
        b.iter(|| vsop87d::uranus(black_box(jde)))
    });
}

fn vsop87d_neptune(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87d Neptune", move |b| {
        b.iter(|| vsop87d::venus(black_box(jde)))
    });
}

criterion_group!(
    vsop87d_benches,
    vsop87d_mercury,
    vsop87d_venus,
    vsop87d_earth,
    vsop87d_mars,
    vsop87d_jupiter,
    vsop87d_saturn,
    vsop87d_uranus,
    vsop87d_neptune
);
criterion_main!(vsop87d_benches);
