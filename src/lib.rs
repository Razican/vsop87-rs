//! VSOP87 library
//!
//! This library implements the VSOP87 solutions to calculate the positions of the planets in the
//! solar system. To use you must include the following in your crate:
//!
//! ```
//! extern crate vsop87;
//! ```
//!
//! There is still one module per VSOP87 implementation. Currently only basic VSOP87 algorithm and
//! the VSOP87A versions are implemented. Nevertheless, in the future, VSOP87B, VSOP87C, VSOP87D
//! and VSOP87E will be implemented.

pub mod vsop87;
pub mod vsop87a;
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

/// Calculates the keplerian orbital elements from VSOP87
///
/// This function calculates the keplerian orbital elements from the VSOP87 solution (the
/// heliocentric orbital elements). The parameters needed are the 6 variables returned by the
/// VSOP87 function for a given planet. It returns, in order, a tuple with the keplerian orbital
/// elements of the planet:
///
/// - Eccentricity (*e*)
/// - Semimajor axis (*a*), in *AU*
/// - Inclination (*i*), in radians
/// - Longitude of the ascending node (*Ω*), in radians
/// - Longitude of the periapsis (*ϖ*), in radians
/// - Mean longitude (L₀), in radians
///
/// # Examples
///
/// ```
/// use vsop87::*;
///
/// let (a, l, k, h, q, p) = vsop87::mercury(2451545.0);
/// #
/// # assert!(a > 0.3870982121 && a < 0.3870982123);
/// # assert!(l > 4.4026057778 && l < 4.4026057780);
/// # assert!(k > 0.0446647517 && k < 0.0446647519);
/// # assert!(h > 0.2007208957 && h < 0.2007208959);
/// # assert!(q > 0.0406161540 && q < 0.0406161542);
/// # assert!(p > 0.04563512 && p < 0.04563588);
/// #
/// let (a, e, i, lan, lper, l) = keplerian_elements_from_vsop87(a, l, k, h, q, p);
///
/// assert!(a > 0.387097 && a < 0.387099);
/// assert!(e > 0.205629 && e < 0.205631);
/// assert!(i > 0.122260 && i < 0.122261);
/// assert!(lan > 0.843525 && lan < 0.843527);
/// assert!(lper >  1.35183 && lper < 1.35185);
/// assert!(l > 4.40259 && l < 4.40261);
/// ```
pub fn keplerian_elements_from_vsop87(a: f64, l: f64, k: f64, h: f64, q: f64, p: f64)
    -> (f64, f64, f64, f64, f64, f64) {

    let e = (h*h + k*k).sqrt();
    let i = (1_f64 - 2_f64*(p*p + q*q)).acos();
    let lan = (p/q).atan();
    let lper = (h/e).asin();

    (a, e, i, lan, lper, l)
}
