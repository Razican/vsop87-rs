//! VSOP87 module
//!
//! This module calculates heliocentric orbital elements for the planets in the solar system.
mod mercury;
mod venus;
mod earth_moon;
mod mars;
mod jupiter;
mod saturn;
mod uranus;
mod neptune;

use std::f64::consts::PI;
use super::{calculate_t, calculate_var};

/// Calculates VSOP87 solution for Mercury
///
/// This function calculates the VSOP87 solution (the heliocentric orbital elements) for the planet
/// Mercury. The parameter needed is the Julian Day Efemeris (*JDE*) for the given date. It
/// returns, in order, a tuple with the values ```(a, l, k, h, q, p)``` of the VSOP87 solution.
///
/// # Examples
///
/// ```
/// extern crate vsop87;
/// use vsop87::vsop87::*;
///
/// let (a, l, k, h, q, p) = mercury(2415020.0);
///
/// assert!(a > 0.3870977205 && a < 0.3870977207);
/// assert!(l > 3.1341564064 && l < 3.1341564066);
/// assert!(k > 0.0452159417 && k < 0.0452159419);
/// assert!(h > 0.2005915793 && h < 0.2005915795);
/// assert!(q > 0.0405500077 && q < 0.0405500079);
/// assert!(p > 0.0457636621 && p < 0.0457636623);
/// ```
pub fn mercury(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t = calculate_t(jde);

    let a0 = calculate_var(t, &mercury::A0);
    let a1 = calculate_var(t, &mercury::A1);
    let a2 = calculate_var(t, &mercury::A2);

    let l0 = calculate_var(t, &mercury::L0);
    let l1 = calculate_var(t, &mercury::L1);
    let l2 = calculate_var(t, &mercury::L2);
    let l3 = calculate_var(t, &mercury::L3);

    let k0 = calculate_var(t, &mercury::K0);
    let k1 = calculate_var(t, &mercury::K1);
    let k2 = calculate_var(t, &mercury::K2);
    let k3 = calculate_var(t, &mercury::K3);
    let k4 = calculate_var(t, &mercury::K4);
    let k5 = calculate_var(t, &mercury::K5);

    let h0 = calculate_var(t, &mercury::H0);
    let h1 = calculate_var(t, &mercury::H1);
    let h2 = calculate_var(t, &mercury::H2);
    let h3 = calculate_var(t, &mercury::H3);
    let h4 = calculate_var(t, &mercury::H4);
    let h5 = calculate_var(t, &mercury::H5);

    let q0 = calculate_var(t, &mercury::Q0);
    let q1 = calculate_var(t, &mercury::Q1);
    let q2 = calculate_var(t, &mercury::Q2);
    let q3 = calculate_var(t, &mercury::Q3);
    let q4 = calculate_var(t, &mercury::Q4);
    let q5 = calculate_var(t, &mercury::Q5);

    let p0 = calculate_var(t, &mercury::P0);
    let p1 = calculate_var(t, &mercury::P1);
    let p2 = calculate_var(t, &mercury::P2);
    let p3 = calculate_var(t, &mercury::P3);
    let p4 = calculate_var(t, &mercury::P4);

    let a = a0 + a1*t +a2*t*t;
    let l = (l0 + l1*t +l2*t*t + l3*t.powi(3)) % (2_f64*PI);
    let k = (k0 + k1*t +k2*t*t + k3*t.powi(3) + k4*t.powi(4) + k5*t.powi(5)) % (2_f64*PI);
    let h = (h0 + h1*t +h2*t*t + h3*t.powi(3) + h4*t.powi(4) + h5*t.powi(5)) % (2_f64*PI);
    let q = (q0 + q1*t +q2*t*t + q3*t.powi(3) + q4*t.powi(4) + q5*t.powi(5)) % (2_f64*PI);
    let p = (p0 + p1*t +p2*t*t + p3*t.powi(3) + p4*t.powi(4)) % (2_f64*PI);

    (a, if l > 0_f64 {l} else {2_f64*PI+l}, k, h, q, p)
}

/// Calculates VSOP87 solution for Venus
///
/// This function calculates the VSOP87 solution (the heliocentric orbital elements) for the planet
/// Venus. The parameter needed is the Julian Day Efemeris (*JDE*) for the given date. It returns,
/// in order, a tuple with the values ```(a, l, k, h, q, p)``` of the VSOP87 solution.
///
/// # Examples
///
/// ```
/// extern crate vsop87;
/// use vsop87::vsop87::*;
///
/// let (a, l, k, h, q, p) = venus(2451545.0);
///
/// assert!(a > 0.7233269303 && a < 0.7233269305);
/// assert!(l > 3.1761350909 && l < 3.1761350911);
/// assert!(k > -0.0045086078 && k < -0.0045086076);
/// assert!(h > 0.0050312181 && h < 0.0050312183);
/// assert!(q > 0.0068248057 && q < 0.0068248059);
/// assert!(p > 0.0288221480 && p < 0.0288221482);
/// ```
pub fn venus(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    // TODO
    (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
}

/// Calculates VSOP87 solution for Earth - Moon barycenter
///
/// This function calculates the VSOP87 solution (the heliocentric orbital elements) for the Earth
/// - Moon barycenter. The parameter needed is the Julian Day Efemeris (*JDE*) for the given date.
/// It returns, in order, a tuple with the values ```(a, l, k, h, q, p)``` of the VSOP87 solution.
///
/// # Examples
///
/// ```
/// extern crate vsop87;
/// use vsop87::vsop87::*;
///
/// let (a, l, k, h, q, p) = earth_moon(2122820.0);
///
/// assert!(a > 1.0000134925 && a < 1.0000134927);
/// assert!(l > 1.8519621672 && l < 1.8519621674);
/// assert!(k > -0.0029638176 && k < -0.0029638174);
/// assert!(h > 0.0168402193 && h < 0.0168402195);
/// assert!(q > 0.0010301900 && q < 0.0010301902);
/// assert!(p > -0.0000530796 && p < -0.0000530794);
/// ```
pub fn earth_moon(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    // TODO
    (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
}

/// Calculates VSOP87 solution for Mars
///
/// This function calculates the VSOP87 solution (the heliocentric orbital elements) for the planet
/// Mars. The parameter needed is the Julian Day Efemeris (*JDE*) for the given date. It returns,
/// in order, a tuple with the values ```(a, l, k, h, q, p)``` of the VSOP87 solution.
///
/// # Examples
///
/// ```
/// extern crate vsop87;
/// use vsop87::vsop87::*;
///
/// let (a, l, k, h, q, p) = mars(2159345.0);
///
/// assert!(a > 1.5237578877 && a < 1.5237578879);
/// assert!(l > 4.0669853278 && l < 4.0669853280);
/// assert!(k > 0.0821906316 && k < 0.0821906318);
/// assert!(h > -0.0427917583 && h < -0.0427917581);
/// assert!(q > 0.0103081045 && q < 0.0103081047);
/// assert!(p > 0.0131364564 && p < 0.0131364566);
/// ```
pub fn mars(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    // TODO
    (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
}

/// Calculates VSOP87 solution for Jupiter
///
/// This function calculates the VSOP87 solution (the heliocentric orbital elements) for the planet
/// Jupiter. The parameter needed is the Julian Day Efemeris (*JDE*) for the given date. It
/// returns, in order, a tuple with the values ```(a, l, k, h, q, p)``` of the VSOP87 solution.
///
/// # Examples
///
/// ```
/// extern crate vsop87;
/// use vsop87::vsop87::*;
///
/// let (a, l, k, h, q, p) = jupiter(2378495.0);
///
/// assert!(a > 5.2027276672 && a < 5.2027276674);
/// assert!(l > 1.4820596291 && l < 1.4820596293);
/// assert!(k > 0.0464780412 && k < 0.0464780414);
/// assert!(h > 0.0116460263 && h < 0.0116460265);
/// assert!(q > -0.0019921307 && q < -0.0019921305);
/// assert!(p > 0.0112348477 && p < 0.0112348479);
/// ```
pub fn jupiter(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    // TODO
    (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
}

/// Calculates VSOP87 solution for Saturn
///
/// This function calculates the VSOP87 solution (the heliocentric orbital elements) for the planet
/// Saturn. The parameter needed is the Julian Day Efemeris (*JDE*) for the given date. It returns,
/// in order, a tuple with the values ```(a, l, k, h, q, p)``` of the VSOP87 solution.
///
/// # Examples
///
/// ```
/// extern crate vsop87;
/// use vsop87::vsop87::*;
///
/// let (a, l, k, h, q, p) = jupiter(2305445.0);
///
/// assert!(a > 9.5727100002 && a < 9.5727100004);
/// assert!(l > 3.5107821038 && l < 3.5107821040);
/// assert!(k > -0.0048218813 && k < -0.0048218811);
/// assert!(h > 0.0575514202 && h < 0.0575514204);
/// assert!(q > -0.0090348990 && q < -0.0090348988);
/// assert!(p > 0.0196579375 && p < 0.0196579377);
/// ```
pub fn saturn(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    // TODO
    (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
}

/// Calculates VSOP87 solution for Jupiter
///
/// This function calculates the VSOP87 solution (the heliocentric orbital elements) for the planet
/// Jupiter. The parameter needed is the Julian Day Efemeris (*JDE*) for the given date. It
/// returns, in order, a tuple with the values ```(a, l, k, h, q, p)``` of the VSOP87 solution.
///
/// # Examples
///
/// ```
/// extern crate vsop87;
/// use vsop87::vsop87::*;
///
/// let (a, l, k, h, q, p) = jupiter(2378495.0);
///
/// assert!(a > 5.2027276672 && a < 5.2027276674);
/// assert!(l > 1.4820596291 && l < 1.4820596293);
/// assert!(k > 0.0464780412 && k < 0.0464780414);
/// assert!(h > 0.0116460263 && h < 0.0116460265);
/// assert!(q > -0.0019921307 && q < -0.0019921305);
/// assert!(p > 0.0112348477 && p < 0.0112348479);
/// ```
pub fn uranus(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    // TODO
    (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
}

/// Calculates VSOP87 solution for Jupiter
///
/// This function calculates the VSOP87 solution (the heliocentric orbital elements) for the planet
/// Jupiter. The parameter needed is the Julian Day Efemeris (*JDE*) for the given date. It
/// returns, in order, a tuple with the values ```(a, l, k, h, q, p)``` of the VSOP87 solution.
///
/// # Examples
///
/// ```
/// extern crate vsop87;
/// use vsop87::vsop87::*;
///
/// let (a, l, k, h, q, p) = jupiter(2378495.0);
///
/// assert!(a > 5.2027276672 && a < 5.2027276674);
/// assert!(l > 1.4820596291 && l < 1.4820596293);
/// assert!(k > 0.0464780412 && k < 0.0464780414);
/// assert!(h > 0.0116460263 && h < 0.0116460265);
/// assert!(q > -0.0019921307 && q < -0.0019921305);
/// assert!(p > 0.0112348477 && p < 0.0112348479);
/// ```
pub fn neptune(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    // TODO
    (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
}
