//! VSOP87a benchmarks.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use vsop87::vsop87a;

fn vsop87a_mercury(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87a Mercury", move |b| {
        b.iter(|| vsop87a::mercury(black_box(jde)))
    });
}

fn vsop87a_venus(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87a Venus", move |b| {
        b.iter(|| vsop87a::venus(black_box(jde)))
    });
}

fn vsop87a_earth(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87a Earth", move |b| {
        b.iter(|| vsop87a::earth(black_box(jde)))
    });
}

fn vsop87a_earth_moon(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87a Earth/Moon barycenter", move |b| {
        b.iter(|| vsop87a::earth_moon(black_box(jde)))
    });
}

fn vsop87a_mars(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87a Mars", move |b| {
        b.iter(|| vsop87a::mars(black_box(jde)))
    });
}

fn vsop87a_jupiter(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87a Jupiter", move |b| {
        b.iter(|| vsop87a::jupiter(black_box(jde)))
    });
}

fn vsop87a_saturn(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87a Saturn", move |b| {
        b.iter(|| vsop87a::saturn(black_box(jde)))
    });
}

fn vsop87a_uranus(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87a Uranus", move |b| {
        b.iter(|| vsop87a::uranus(black_box(jde)))
    });
}

fn vsop87a_neptune(c: &mut Criterion) {
    let mut rng = thread_rng();
    let jde = rng.gen_range(990930.5..3912521.5);
    c.bench_function("VSOP87a Neptune", move |b| {
        b.iter(|| vsop87a::venus(black_box(jde)))
    });
}

criterion_group!(
    vsop87a_benches,
    vsop87a_mercury,
    vsop87a_venus,
    vsop87a_earth,
    vsop87a_earth_moon,
    vsop87a_mars,
    vsop87a_jupiter,
    vsop87a_saturn,
    vsop87a_uranus,
    vsop87a_neptune
);
criterion_main!(vsop87a_benches);
