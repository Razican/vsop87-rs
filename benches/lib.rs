#![feature(test)]

extern crate rand;
extern crate test;
extern crate vsop87;

use rand::{IsaacRng, Rng};
use test::Bencher;

#[bench]
fn vsop87_mercury(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87::mercury(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87_venus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87::venus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87_earth_moon(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87::earth_moon(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87_mars(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87::mars(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87_jupiter(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87::jupiter(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87_saturn(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87::saturn(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87_uranus(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87::uranus(rng.gen_range(990930.5, 3912521.5)))
}

#[bench]
fn vsop87_neptune(b: &mut Bencher) {
    let mut rng = IsaacRng::new_unseeded();
    b.iter(|| vsop87::venus(rng.gen_range(990930.5, 3912521.5)))
}
