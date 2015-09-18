//! VSOP87 library
//!
//! This library implements the VSOP87 solutions to calculate the positions of the planets in the
//! solar system. To use you must include the following in your crate:
//!
//! ```
//! extern crate vsop87;
//! ```
//!
//! There is still only one module in the crate, that solves the basic VSOP87 algorithm.
//! Nevertheless, in the future, VSOP87A, VSOP87B, VSOP87C, VSOP87D and VSOP87E will be
//! implemented.

pub mod vsop87;
// pub mod vsop87a;
// pub mod vsop87b;
// pub mod vsop87c;
// pub mod vsop87d;
// pub mod vsop87e;

fn calculate_t(jde: f64) -> f64{
    return (jde - 2451545_f64)/365250_f64
}

fn calculate_var(t: f64, var: &[(f64, f64, f64)]) -> f64 {
    var.iter().fold(0_f64, |term, &(a, b, c)| term + a*(b + c*t).cos())
}
