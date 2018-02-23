#![feature(test)]

extern crate rand;
extern crate test;
extern crate vsop87;

use test::Bencher;
use rand::{IsaacRng, Rng};
use vsop87::vsop87d;

#[bench]
fn vsop87d_mercury(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87d::mercury(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87d_venus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87d::venus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87d_earth(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87d::earth(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87d_mars(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87d::mars(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87d_jupiter(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87d::jupiter(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87d_saturn(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87d::saturn(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87d_uranus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87d::uranus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87d_neptune(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87d::venus(rng.gen_range(990930.5, 3912521.5)))
}
