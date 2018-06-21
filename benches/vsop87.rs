//! VSOP87 benchmarks

#[macro_use]
extern crate criterion;
extern crate vsop87;
extern crate rand;

use criterion::Criterion;
use rand::{Rng, thread_rng};

fn vsop87_mercury(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87 Mercury", move |b| b.iter(|| vsop87::mercury(rng.gen_range(990930.5, 3912521.5))));
}

fn vsop87_venus(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87 Venus", move |b| b.iter(|| vsop87::venus(rng.gen_range(990930.5, 3912521.5))));
}

fn vsop87_earth_moon(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87 Earth/Moon barycenter", move |b| b.iter(|| vsop87::earth_moon(rng.gen_range(990930.5, 3912521.5))));
}

fn vsop87_mars(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87 Mars", move |b| b.iter(|| vsop87::mars(rng.gen_range(990930.5, 3912521.5))));
}

fn vsop87_jupiter(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87 Jupiter", move |b| b.iter(|| vsop87::jupiter(rng.gen_range(990930.5, 3912521.5))));
}

fn vsop87_saturn(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87 Saturn", move |b| b.iter(|| vsop87::saturn(rng.gen_range(990930.5, 3912521.5))));
}

fn vsop87_uranus(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87 Uranus", move |b| b.iter(|| vsop87::uranus(rng.gen_range(990930.5, 3912521.5))));
}

fn vsop87_neptune(c: &mut Criterion) {
    let mut rng = thread_rng();
    c.bench_function("VSOP87 Neptune", move |b| b.iter(|| vsop87::venus(rng.gen_range(990930.5, 3912521.5))));
}

criterion_group!(vsop87_benches, vsop87_mercury, vsop87_venus, vsop87_earth_moon, vsop87_mars, vsop87_jupiter, vsop87_saturn, vsop87_uranus, vsop87_neptune);
criterion_main!(vsop87_benches);