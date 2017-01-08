#![feature(test)]

extern crate vsop87;
extern crate test;
extern crate rand;

use test::Bencher;
use rand::{Rng, IsaacRng};
use vsop87::vsop87a;

#[bench]
fn vsop87a_mercury(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87a::mercury(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87a_venus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87a::venus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87a_earth(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87a::earth(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87a_earth_moon(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87a::earth_moon(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87a_mars(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87a::mars(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87a_jupiter(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87a::jupiter(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87a_saturn(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87a::saturn(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87a_uranus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87a::uranus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87a_neptune(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87a::venus(rng.gen_range(990930.5, 3912521.5)))
}
