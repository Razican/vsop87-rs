#![feature(test)]

extern crate vsop87;
extern crate test;
extern crate rand;

use test::Bencher;
use rand::{Rng, IsaacRng};
use vsop87::vsop87b;

#[bench]
fn vsop87b_mercury(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87b::mercury(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87b_venus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87b::venus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87b_earth(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87b::earth(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87b_mars(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87b::mars(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87b_jupiter(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87b::jupiter(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87b_saturn(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87b::saturn(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87b_uranus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87b::uranus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87b_neptune(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87b::venus(rng.gen_range(990930.5, 3912521.5)))
}
