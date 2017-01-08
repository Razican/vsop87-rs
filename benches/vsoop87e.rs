#![feature(test)]

extern crate vsop87;
extern crate test;
extern crate rand;

use test::Bencher;
use rand::{Rng, IsaacRng};
use vsop87::vsop87e;

#[bench]
fn vsop87e_sun(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87e::sun(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87e_mercury(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87e::mercury(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87e_venus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87e::venus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87e_earth(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87e::earth(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87e_mars(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87e::mars(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87e_jupiter(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87e::jupiter(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87e_saturn(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87e::saturn(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87e_uranus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87e::uranus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87e_neptune(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87e::venus(rng.gen_range(990930.5, 3912521.5)))
}
