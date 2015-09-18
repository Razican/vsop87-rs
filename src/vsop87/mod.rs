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
/// returns, in order, a tuple with the values *a*, *l*, *k*, *h*, *q*, *p*) of the VSOP87
/// solution.
///
/// # Examples
///
/// ```
/// use vsop87::*;
///
/// let (a, l, k, h, q, p) = vsop87::mercury(2415020.0);
///
/// assert!(a > 0.3870977205 && a < 0.3870977207);
/// assert!(l > 3.1341564064 && l < 3.1341564066);
/// assert!(k > 0.0452159417 && k < 0.0452159419);
/// assert!(h > 0.2005915793 && h < 0.2005915795);
/// assert!(q > 0.0405500077 && q < 0.0405500079);
/// assert!(p > 0.04576328 && p < 0.04576404);
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

    let a = a0 + a1*t + a2*t*t;
    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3)) % (2_f64*PI);
    let k = k0 + k1*t + k2*t*t + k3*t.powi(3) + k4*t.powi(4) + k5*t.powi(5);
    let h = h0 + h1*t + h2*t*t + h3*t.powi(3) + h4*t.powi(4) + h5*t.powi(5);
    let q = q0 + q1*t + q2*t*t + q3*t.powi(3) + q4*t.powi(4) + q5*t.powi(5);
    let p = p0 + p1*t + p2*t*t + p3*t.powi(3) + p4*t.powi(4);

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
/// use vsop87::*;
///
/// let (a, l, k, h, q, p) = vsop87::venus(2451545.0);
///
/// assert!(a > 0.7233269303 && a < 0.7233269305);
/// assert!(l > 3.1761350909 && l < 3.1761350911);
/// assert!(k > -0.0045086078 && k < -0.0045086076);
/// assert!(h > 0.0050312181 && h < 0.0050312183);
/// assert!(q > 0.0068248057 && q < 0.0068248059);
/// assert!(p > 0.02882177 && p < 0.02882253);
/// ```
pub fn venus(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t = calculate_t(jde);

    let a0 = calculate_var(t, &venus::A0);
    let a1 = calculate_var(t, &venus::A1);
    let a2 = calculate_var(t, &venus::A2);

    let l0 = calculate_var(t, &venus::L0);
    let l1 = calculate_var(t, &venus::L1);
    let l2 = calculate_var(t, &venus::L2);
    let l3 = calculate_var(t, &venus::L3);

    let k0 = calculate_var(t, &venus::K0);
    let k1 = calculate_var(t, &venus::K1);
    let k2 = calculate_var(t, &venus::K2);
    let k3 = calculate_var(t, &venus::K3);
    let k4 = calculate_var(t, &venus::K4);
    let k5 = calculate_var(t, &venus::K5);

    let h0 = calculate_var(t, &venus::H0);
    let h1 = calculate_var(t, &venus::H1);
    let h2 = calculate_var(t, &venus::H2);
    let h3 = calculate_var(t, &venus::H3);
    let h4 = calculate_var(t, &venus::H4);
    let h5 = calculate_var(t, &venus::H5);

    let q0 = calculate_var(t, &venus::Q0);
    let q1 = calculate_var(t, &venus::Q1);
    let q2 = calculate_var(t, &venus::Q2);
    let q3 = calculate_var(t, &venus::Q3);
    let q4 = calculate_var(t, &venus::Q4);
    let q5 = calculate_var(t, &venus::Q5);

    let p0 = calculate_var(t, &venus::P0);
    let p1 = calculate_var(t, &venus::P1);
    let p2 = calculate_var(t, &venus::P2);
    let p3 = calculate_var(t, &venus::P3);
    let p4 = calculate_var(t, &venus::P4);

    let a = a0 + a1*t + a2*t*t;
    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3)) % (2_f64*PI);
    let k = k0 + k1*t + k2*t*t + k3*t.powi(3) + k4*t.powi(4) + k5*t.powi(5);
    let h = h0 + h1*t + h2*t*t + h3*t.powi(3) + h4*t.powi(4) + h5*t.powi(5);
    let q = q0 + q1*t + q2*t*t + q3*t.powi(3) + q4*t.powi(4) + q5*t.powi(5);
    let p = p0 + p1*t + p2*t*t + p3*t.powi(3) + p4*t.powi(4);

    (a, if l > 0_f64 {l} else {2_f64*PI+l}, k, h, q, p)
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
/// use vsop87::*;
///
/// let (a, l, k, h, q, p) = vsop87::earth_moon(2122820.0);
///
/// assert!(a > 1.0000134925 && a < 1.0000134927);
/// assert!(l > 1.8519621672 && l < 1.8519621674);
/// assert!(k > -0.0029638176 && k < -0.0029638174);
/// assert!(h > 0.0168402193 && h < 0.0168402195);
/// assert!(q > 0.0010301900 && q < 0.0010301902);
/// assert!(p > -0.00005346 && p < -0.00005270);
/// ```
pub fn earth_moon(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t = calculate_t(jde);

    let a0 = calculate_var(t, &earth_moon::A0);
    let a1 = calculate_var(t, &earth_moon::A1);
    let a2 = calculate_var(t, &earth_moon::A2);

    let l0 = calculate_var(t, &earth_moon::L0);
    let l1 = calculate_var(t, &earth_moon::L1);
    let l2 = calculate_var(t, &earth_moon::L2);
    let l3 = calculate_var(t, &earth_moon::L3);
    let l4 = calculate_var(t, &earth_moon::L4);
    let l5 = calculate_var(t, &earth_moon::L5);

    let k0 = calculate_var(t, &earth_moon::K0);
    let k1 = calculate_var(t, &earth_moon::K1);
    let k2 = calculate_var(t, &earth_moon::K2);
    let k3 = calculate_var(t, &earth_moon::K3);
    let k4 = calculate_var(t, &earth_moon::K4);
    let k5 = calculate_var(t, &earth_moon::K5);

    let h0 = calculate_var(t, &earth_moon::H0);
    let h1 = calculate_var(t, &earth_moon::H1);
    let h2 = calculate_var(t, &earth_moon::H2);
    let h3 = calculate_var(t, &earth_moon::H3);
    let h4 = calculate_var(t, &earth_moon::H4);
    let h5 = calculate_var(t, &earth_moon::H5);

    let q0 = calculate_var(t, &earth_moon::Q0);
    let q1 = calculate_var(t, &earth_moon::Q1);
    let q2 = calculate_var(t, &earth_moon::Q2);
    let q3 = calculate_var(t, &earth_moon::Q3);
    let q4 = calculate_var(t, &earth_moon::Q4);
    let q5 = calculate_var(t, &earth_moon::Q5);

    let p0 = calculate_var(t, &earth_moon::P0);
    let p1 = calculate_var(t, &earth_moon::P1);
    let p2 = calculate_var(t, &earth_moon::P2);
    let p3 = calculate_var(t, &earth_moon::P3);
    let p4 = calculate_var(t, &earth_moon::P4);

    let a = a0 + a1*t + a2*t*t;
    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let k = k0 + k1*t + k2*t*t + k3*t.powi(3) + k4*t.powi(4) + k5*t.powi(5);
    let h = h0 + h1*t + h2*t*t + h3*t.powi(3) + h4*t.powi(4) + h5*t.powi(5);
    let q = q0 + q1*t + q2*t*t + q3*t.powi(3) + q4*t.powi(4) + q5*t.powi(5);
    let p = p0 + p1*t + p2*t*t + p3*t.powi(3) + p4*t.powi(4);

    (a, if l > 0_f64 {l} else {2_f64*PI+l}, k, h, q, p)
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
/// use vsop87::*;
///
/// let (a, l, k, h, q, p) = vsop87::mars(2159345.0);
///
/// assert!(a > 1.5237578877 && a < 1.5237578879);
/// assert!(l > 4.0669853278 && l < 4.0669853280);
/// assert!(k > 0.0821906316 && k < 0.0821906318);
/// assert!(h > -0.0427917583 && h < -0.0427917581);
/// assert!(q > 0.0103081045 && q < 0.0103081047);
/// assert!(p > 0.01313608 && p < 0.01313684);
/// ```
pub fn mars(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t = calculate_t(jde);

    let a0 = calculate_var(t, &mars::A0);
    let a1 = calculate_var(t, &mars::A1);
    let a2 = calculate_var(t, &mars::A2);

    let l0 = calculate_var(t, &mars::L0);
    let l1 = calculate_var(t, &mars::L1);
    let l2 = calculate_var(t, &mars::L2);
    let l3 = calculate_var(t, &mars::L3);
    let l4 = calculate_var(t, &mars::L4);
    let l5 = calculate_var(t, &mars::L5);

    let k0 = calculate_var(t, &mars::K0);
    let k1 = calculate_var(t, &mars::K1);
    let k2 = calculate_var(t, &mars::K2);
    let k3 = calculate_var(t, &mars::K3);
    let k4 = calculate_var(t, &mars::K4);
    let k5 = calculate_var(t, &mars::K5);

    let h0 = calculate_var(t, &mars::H0);
    let h1 = calculate_var(t, &mars::H1);
    let h2 = calculate_var(t, &mars::H2);
    let h3 = calculate_var(t, &mars::H3);
    let h4 = calculate_var(t, &mars::H4);
    let h5 = calculate_var(t, &mars::H5);

    let q0 = calculate_var(t, &mars::Q0);
    let q1 = calculate_var(t, &mars::Q1);
    let q2 = calculate_var(t, &mars::Q2);
    let q3 = calculate_var(t, &mars::Q3);
    let q4 = calculate_var(t, &mars::Q4);
    let q5 = calculate_var(t, &mars::Q5);

    let p0 = calculate_var(t, &mars::P0);
    let p1 = calculate_var(t, &mars::P1);
    let p2 = calculate_var(t, &mars::P2);
    let p3 = calculate_var(t, &mars::P3);

    let a = a0 + a1*t + a2*t*t;
    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let k = k0 + k1*t + k2*t*t + k3*t.powi(3) + k4*t.powi(4) + k5*t.powi(5);
    let h = h0 + h1*t + h2*t*t + h3*t.powi(3) + h4*t.powi(4) + h5*t.powi(5);
    let q = q0 + q1*t + q2*t*t + q3*t.powi(3) + q4*t.powi(4) + q5*t.powi(5);
    let p = p0 + p1*t + p2*t*t + p3*t.powi(3);

    (a, if l > 0_f64 {l} else {2_f64*PI+l}, k, h, q, p)
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
/// use vsop87::*;
///
/// let (a, l, k, h, q, p) = vsop87::jupiter(2378495.0);
///
/// assert!(a > 5.2027276672 && a < 5.2027276674);
/// assert!(l > 1.4820596291 && l < 1.4820596293);
/// assert!(k > 0.0464780412 && k < 0.0464780414);
/// assert!(h > 0.0116460263 && h < 0.0116460265);
/// assert!(q > -0.0019921307 && q < -0.0019921305);
/// assert!(p > 0.01123447 && p < 0.01123523);
/// ```
pub fn jupiter(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t = calculate_t(jde);

    let a0 = calculate_var(t, &jupiter::A0);
    let a1 = calculate_var(t, &jupiter::A1);
    let a2 = calculate_var(t, &jupiter::A2);
    let a3 = calculate_var(t, &jupiter::A3);
    let a4 = calculate_var(t, &jupiter::A4);
    let a5 = calculate_var(t, &jupiter::A5);

    let l0 = calculate_var(t, &jupiter::L0);
    let l1 = calculate_var(t, &jupiter::L1);
    let l2 = calculate_var(t, &jupiter::L2);
    let l3 = calculate_var(t, &jupiter::L3);
    let l4 = calculate_var(t, &jupiter::L4);
    let l5 = calculate_var(t, &jupiter::L5);

    let k0 = calculate_var(t, &jupiter::K0);
    let k1 = calculate_var(t, &jupiter::K1);
    let k2 = calculate_var(t, &jupiter::K2);
    let k3 = calculate_var(t, &jupiter::K3);
    let k4 = calculate_var(t, &jupiter::K4);

    let h0 = calculate_var(t, &jupiter::H0);
    let h1 = calculate_var(t, &jupiter::H1);
    let h2 = calculate_var(t, &jupiter::H2);
    let h3 = calculate_var(t, &jupiter::H3);
    let h4 = calculate_var(t, &jupiter::H4);

    let q0 = calculate_var(t, &jupiter::Q0);
    let q1 = calculate_var(t, &jupiter::Q1);
    let q2 = calculate_var(t, &jupiter::Q2);
    let q3 = calculate_var(t, &jupiter::Q3);

    let p0 = calculate_var(t, &jupiter::P0);
    let p1 = calculate_var(t, &jupiter::P1);
    let p2 = calculate_var(t, &jupiter::P2);

    let a = a0 + a1*t + a2*t*t + a3*t.powi(3) + a4*t.powi(4) + a5*t.powi(5);
    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let k = k0 + k1*t + k2*t*t + k3*t.powi(3) + k4*t.powi(4);
    let h = h0 + h1*t + h2*t*t + h3*t.powi(3) + h4*t.powi(4);
    let q = q0 + q1*t + q2*t*t + q3*t.powi(3);
    let p = p0 + p1*t + p2*t*t;

    (a, if l > 0_f64 {l} else {2_f64*PI+l}, k, h, q, p)
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
/// use vsop87::*;
///
/// let (a, l, k, h, q, p) = vsop87::saturn(2305445.0);
///
/// assert!(a > 9.5727100002 && a < 9.5727100004);
/// assert!(l > 3.5107821038 && l < 3.5107821040);
/// assert!(k > -0.0048218813 && k < -0.0048218811);
/// assert!(h > 0.0575514202 && h < 0.0575514204);
/// assert!(q > -0.0090348990 && q < -0.0090348988);
/// assert!(p > 0.01965756 && p < 0.01965832);
/// ```
pub fn saturn(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t = calculate_t(jde);

    let a0 = calculate_var(t, &saturn::A0);
    let a1 = calculate_var(t, &saturn::A1);
    let a2 = calculate_var(t, &saturn::A2);
    let a3 = calculate_var(t, &saturn::A3);
    let a4 = calculate_var(t, &saturn::A4);
    let a5 = calculate_var(t, &saturn::A5);

    let l0 = calculate_var(t, &saturn::L0);
    let l1 = calculate_var(t, &saturn::L1);
    let l2 = calculate_var(t, &saturn::L2);
    let l3 = calculate_var(t, &saturn::L3);
    let l4 = calculate_var(t, &saturn::L4);
    let l5 = calculate_var(t, &saturn::L5);

    let k0 = calculate_var(t, &saturn::K0);
    let k1 = calculate_var(t, &saturn::K1);
    let k2 = calculate_var(t, &saturn::K2);
    let k3 = calculate_var(t, &saturn::K3);
    let k4 = calculate_var(t, &saturn::K4);
    let k5 = calculate_var(t, &saturn::K5);

    let h0 = calculate_var(t, &saturn::H0);
    let h1 = calculate_var(t, &saturn::H1);
    let h2 = calculate_var(t, &saturn::H2);
    let h3 = calculate_var(t, &saturn::H3);
    let h4 = calculate_var(t, &saturn::H4);
    let h5 = calculate_var(t, &saturn::H5);

    let q0 = calculate_var(t, &saturn::Q0);
    let q1 = calculate_var(t, &saturn::Q1);
    let q2 = calculate_var(t, &saturn::Q2);
    let q3 = calculate_var(t, &saturn::Q3);
    let q4 = calculate_var(t, &saturn::Q4);

    let p0 = calculate_var(t, &saturn::P0);
    let p1 = calculate_var(t, &saturn::P1);
    let p2 = calculate_var(t, &saturn::P2);
    let p3 = calculate_var(t, &saturn::P3);

    let a = a0 + a1*t + a2*t*t + a3*t.powi(3) + a4*t.powi(4) + a5*t.powi(5);
    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let k = k0 + k1*t + k2*t*t + k3*t.powi(3) + k4*t.powi(4) + k5*t.powi(5);
    let h = h0 + h1*t + h2*t*t + h3*t.powi(3) + h4*t.powi(4) + h5*t.powi(5);
    let q = q0 + q1*t + q2*t*t + q3*t.powi(3) + q4*t.powi(4);
    let p = p0 + p1*t + p2*t*t + p3*t.powi(3);

    (a, if l > 0_f64 {l} else {2_f64*PI+l}, k, h, q, p)
}

/// Calculates VSOP87 solution for Uranus
///
/// This function calculates the VSOP87 solution (the heliocentric orbital elements) for the planet
/// Uranus. The parameter needed is the Julian Day Efemeris (*JDE*) for the given date. It returns,
/// in order, a tuple with the values ```(a, l, k, h, q, p)``` of the VSOP87 solution.
///
/// # Examples
///
/// ```
/// use vsop87::*;
///
/// let (a, l, k, h, q, p) = vsop87::uranus(2232395.0);
///
/// assert!(a > 19.2497356422 && a < 19.2497356424);
/// assert!(l > 4.5777275752 && l < 4.5777275754);
/// assert!(k > -0.0466529112 && k < -0.0466529110);
/// assert!(h > 0.0051308956 && h < 0.0051308958);
/// assert!(q > 0.0019206656 && q < 0.0019206658);
/// assert!(p > 0.00655819 && p < 0.00655895);
/// ```
pub fn uranus(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t = calculate_t(jde);

    let a0 = calculate_var(t, &uranus::A0);
    let a1 = calculate_var(t, &uranus::A1);
    let a2 = calculate_var(t, &uranus::A2);
    let a3 = calculate_var(t, &uranus::A3);
    let a4 = calculate_var(t, &uranus::A4);
    let a5 = calculate_var(t, &uranus::A5);

    let l0 = calculate_var(t, &uranus::L0);
    let l1 = calculate_var(t, &uranus::L1);
    let l2 = calculate_var(t, &uranus::L2);
    let l3 = calculate_var(t, &uranus::L3);
    let l4 = calculate_var(t, &uranus::L4);
    let l5 = calculate_var(t, &uranus::L5);

    let k0 = calculate_var(t, &uranus::K0);
    let k1 = calculate_var(t, &uranus::K1);
    let k2 = calculate_var(t, &uranus::K2);
    let k3 = calculate_var(t, &uranus::K3);
    let k4 = calculate_var(t, &uranus::K4);

    let h0 = calculate_var(t, &uranus::H0);
    let h1 = calculate_var(t, &uranus::H1);
    let h2 = calculate_var(t, &uranus::H2);
    let h3 = calculate_var(t, &uranus::H3);
    let h4 = calculate_var(t, &uranus::H4);

    let q0 = calculate_var(t, &uranus::Q0);
    let q1 = calculate_var(t, &uranus::Q1);
    let q2 = calculate_var(t, &uranus::Q2);
    let q3 = calculate_var(t, &uranus::Q3);

    let p0 = calculate_var(t, &uranus::P0);
    let p1 = calculate_var(t, &uranus::P1);
    let p2 = calculate_var(t, &uranus::P2);

    let a = a0 + a1*t + a2*t*t + a3*t.powi(3) + a4*t.powi(4) + a5*t.powi(5);
    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let k = k0 + k1*t + k2*t*t + k3*t.powi(3) + k4*t.powi(4);
    let h = h0 + h1*t + h2*t*t + h3*t.powi(3) + h4*t.powi(4);
    let q = q0 + q1*t + q2*t*t + q3*t.powi(3);
    let p = p0 + p1*t + p2*t*t;

    (a, if l > 0_f64 {l} else {2_f64*PI+l}, k, h, q, p)
}

/// Calculates VSOP87 solution for Neptune
///
/// This function calculates the VSOP87 solution (the heliocentric orbital elements) for the planet
/// Neptune. The parameter needed is the Julian Day Efemeris (*JDE*) for the given date. It
/// returns, in order, a tuple with the values ```(a, l, k, h, q, p)``` of the VSOP87 solution.
///
/// # Examples
///
/// ```
/// use vsop87::*;
///
/// let (a, l, k, h, q, p) = vsop87::neptune(2268920.0);
///
/// assert!(a > 30.1963044187 && a < 30.1963044189);
/// assert!(l > 5.1088676118 && l < 5.1088676120);
/// assert!(k > 0.0091964091 && k < 0.0091964093);
/// assert!(h > 0.0031103619 && h < 0.0031103621);
/// assert!(q > -0.0102800265 && q < -0.0102800263);
/// assert!(p > 0.01148076 && p < 0.01148152);
/// ```
pub fn neptune(jde: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t = calculate_t(jde);

    let a0 = calculate_var(t, &neptune::A0);
    let a1 = calculate_var(t, &neptune::A1);
    let a2 = calculate_var(t, &neptune::A2);
    let a3 = calculate_var(t, &neptune::A3);
    let a4 = calculate_var(t, &neptune::A4);
    let a5 = calculate_var(t, &neptune::A5);

    let l0 = calculate_var(t, &neptune::L0);
    let l1 = calculate_var(t, &neptune::L1);
    let l2 = calculate_var(t, &neptune::L2);
    let l3 = calculate_var(t, &neptune::L3);
    let l4 = calculate_var(t, &neptune::L4);
    let l5 = calculate_var(t, &neptune::L5);

    let k0 = calculate_var(t, &neptune::K0);
    let k1 = calculate_var(t, &neptune::K1);
    let k2 = calculate_var(t, &neptune::K2);
    let k3 = calculate_var(t, &neptune::K3);
    let k4 = calculate_var(t, &neptune::K4);
    let k5 = calculate_var(t, &neptune::K5);

    let h0 = calculate_var(t, &neptune::H0);
    let h1 = calculate_var(t, &neptune::H1);
    let h2 = calculate_var(t, &neptune::H2);
    let h3 = calculate_var(t, &neptune::H3);
    let h4 = calculate_var(t, &neptune::H4);
    let h5 = calculate_var(t, &neptune::H5);

    let q0 = calculate_var(t, &neptune::Q0);
    let q1 = calculate_var(t, &neptune::Q1);
    let q2 = calculate_var(t, &neptune::Q2);
    let q3 = calculate_var(t, &neptune::Q3);

    let p0 = calculate_var(t, &neptune::P0);
    let p1 = calculate_var(t, &neptune::P1);
    let p2 = calculate_var(t, &neptune::P2);

    let a = a0 + a1*t + a2*t*t + a3*t.powi(3) + a4*t.powi(4) + a5*t.powi(5);
    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let k = k0 + k1*t + k2*t*t + k3*t.powi(3) + k4*t.powi(4) + k5*t.powi(5);
    let h = h0 + h1*t + h2*t*t + h3*t.powi(3) + h4*t.powi(4) + h5*t.powi(5);
    let q = q0 + q1*t + q2*t*t + q3*t.powi(3);
    let p = p0 + p1*t + p2*t*t;

    (a, if l > 0_f64 {l} else {2_f64*PI+l}, k, h, q, p)
}
