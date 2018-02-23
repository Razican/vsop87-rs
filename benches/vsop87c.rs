#![feature(test)]

extern crate rand;
extern crate test;
extern crate vsop87;

use test::Bencher;
use rand::{IsaacRng, Rng};
use vsop87::vsop87c;

#[bench]
fn vsop87c_mercury(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87c::mercury(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87c_venus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87c::venus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87c_earth(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87c::earth(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87c_mars(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87c::mars(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87c_jupiter(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87c::jupiter(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87c_saturn(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87c::saturn(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87c_uranus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87c::uranus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87c_neptune(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87c::venus(rng.gen_range(990930.5, 3912521.5)))
}
