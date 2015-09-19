extern crate vsop87_rs;

mod vsop87_tests;
mod vsop87a_tests;
// mod vsop87b_tests;
// mod vsop87c_tests;
// mod vsop87d_tests;
// mod vsop87e_tests;

use vsop87_rs::*;

#[test]
fn it_kepler() {
    let (a, l, k, h, q, p) = vsop87::mercury(2451545.0);

    assert!(a > 0.3870982121 && a < 0.3870982123);
    assert!(l > 4.4026057778 && l < 4.4026057780);
    assert!(k > 0.0446647517 && k < 0.0446647519);
    assert!(h > 0.2007208957 && h < 0.2007208959);
    assert!(q > 0.0406161540 && q < 0.0406161542);
    assert!(p > 0.04563512 && p < 0.04563588);

    let (a, e, i, lan, lper, l) = keplerian_elements_from_vsop87(a, l, k, h, q, p);

    assert!(a > 0.387097 && a < 0.387099);
    assert!(e > 0.205629 && e < 0.205631);
    assert!(i > 0.122260 && i < 0.122261);
    assert!(lan > 0.843525 && lan < 0.843527);
    assert!(lper >  1.35183 && lper < 1.35185);
    assert!(l > 4.40259 && l < 4.40261);
}
