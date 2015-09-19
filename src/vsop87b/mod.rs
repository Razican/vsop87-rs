//! VSOP87B implementation
//!
//! This module calculates heliocentric ecliptic spherical coordinates for the planets in the
//! solar system.

mod mercury;
mod venus;
mod earth;
mod mars;
mod jupiter;
mod saturn;
mod uranus;
mod neptune;

use super::{calculate_t, calculate_var};
use std::f64::consts::PI;

/// Calculates VSOP87B solution for Mercury
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates)
/// for the planet Mercury. The parameter needed is the Julian Day Efemeris (*JDE*) for the given
/// date. It returns, in order, a tuple with the values *l*, *b*, *r* of the VSOP87B solution.
/// Those values are the spherical coordinates of the planet, with the Sun in the center and the
/// ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87b;
///
/// let (l, b, r) = vsop87b::mercury(2451545.0);
///
/// assert!(l > 4.4293481042 && l < 4.4293481044);
/// assert!(b > -0.0527573412 && b < -0.0527573410);
/// assert!(r > 0.4664711 && r < 0.4664719);
/// ```
pub fn mercury(jde: f64) -> (f64, f64, f64) {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &mercury::L0);
    let l1 = calculate_var(t, &mercury::L1);
    let l2 = calculate_var(t, &mercury::L2);
    let l3 = calculate_var(t, &mercury::L3);
    let l4 = calculate_var(t, &mercury::L4);
    let l5 = calculate_var(t, &mercury::L5);

    let b0 = calculate_var(t, &mercury::B0);
    let b1 = calculate_var(t, &mercury::B1);
    let b2 = calculate_var(t, &mercury::B2);
    let b3 = calculate_var(t, &mercury::B3);
    let b4 = calculate_var(t, &mercury::B4);
    let b5 = calculate_var(t, &mercury::B5);

    let r0 = calculate_var(t, &mercury::R0);
    let r1 = calculate_var(t, &mercury::R1);
    let r2 = calculate_var(t, &mercury::R2);
    let r3 = calculate_var(t, &mercury::R3);
    let r4 = calculate_var(t, &mercury::R4);

    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let b = b0 + b1*t + b2*t*t + b3*t.powi(3) + b4*t.powi(4) + b5*t.powi(5);
    let r = r0 + r1*t + r2*t*t + r3*t.powi(3) + r4*t.powi(4);

    (if l > 0_f64 {l} else {2_f64*PI+l}, b, r)
}

/// Calculates VSOP87B solution for Venus
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates)
/// for the planet Venus. The parameter needed is the Julian Day Efemeris (*JDE*) for the given
/// date. It returns, in order, a tuple with the values *l*, *b*, *r* of the VSOP87B solution.
/// Those values are the spherical coordinates of the planet, with the Sun in the center and the
/// ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87b;
///
/// let (l, b, r) = vsop87b::venus(2415020.0);
///
/// assert!(l > 5.9993518123 && l < 5.9993518125);
/// assert!(b > -0.0591709805 && b < -0.0591709803);
/// assert!(r > 0.7274715 && r < 0.7274723);
/// ```
pub fn venus(jde: f64) -> (f64, f64, f64) {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &venus::L0);
    let l1 = calculate_var(t, &venus::L1);
    let l2 = calculate_var(t, &venus::L2);
    let l3 = calculate_var(t, &venus::L3);
    let l4 = calculate_var(t, &venus::L4);
    let l5 = calculate_var(t, &venus::L5);

    let b0 = calculate_var(t, &venus::B0);
    let b1 = calculate_var(t, &venus::B1);
    let b2 = calculate_var(t, &venus::B2);
    let b3 = calculate_var(t, &venus::B3);
    let b4 = calculate_var(t, &venus::B4);
    let b5 = calculate_var(t, &venus::B5);

    let r0 = calculate_var(t, &venus::R0);
    let r1 = calculate_var(t, &venus::R1);
    let r2 = calculate_var(t, &venus::R2);
    let r3 = calculate_var(t, &venus::R3);
    let r4 = calculate_var(t, &venus::R4);

    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let b = b0 + b1*t + b2*t*t + b3*t.powi(3) + b4*t.powi(4) + b5*t.powi(5);
    let r = r0 + r1*t + r2*t*t + r3*t.powi(3) + r4*t.powi(4);

    (if l > 0_f64 {l} else {2_f64*PI+l}, b, r)
}

/// Calculates VSOP87B solution for Earth
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates)
/// for the planet Earth. The parameter needed is the Julian Day Efemeris (*JDE*) for the given
/// date. It returns, in order, a tuple with the values *l*, *b*, *r* of the VSOP87B solution.
/// Those values are the spherical coordinates of the planet, with the Sun in the center and the
/// ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87b;
///
/// let (l, b, r) = vsop87b::earth(2378495.0);
///
/// assert!(l > 1.7750058557 && l < 1.7750058559);
/// assert!(b > 0.0004381094 && b < 0.0004381096);
/// assert!(r > 0.9832270 && r < 0.9832278);
/// ```
pub fn earth(jde: f64) -> (f64, f64, f64) {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &earth::L0);
    let l1 = calculate_var(t, &earth::L1);
    let l2 = calculate_var(t, &earth::L2);
    let l3 = calculate_var(t, &earth::L3);
    let l4 = calculate_var(t, &earth::L4);
    let l5 = calculate_var(t, &earth::L5);

    let b0 = calculate_var(t, &earth::B0);
    let b1 = calculate_var(t, &earth::B1);
    let b2 = calculate_var(t, &earth::B2);
    let b3 = calculate_var(t, &earth::B3);
    let b4 = calculate_var(t, &earth::B4);
    let b5 = calculate_var(t, &earth::B5);

    let r0 = calculate_var(t, &earth::R0);
    let r1 = calculate_var(t, &earth::R1);
    let r2 = calculate_var(t, &earth::R2);
    let r3 = calculate_var(t, &earth::R3);
    let r4 = calculate_var(t, &earth::R4);

    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let b = b0 + b1*t + b2*t*t + b3*t.powi(3) + b4*t.powi(4) + b5*t.powi(5);
    let r = r0 + r1*t + r2*t*t + r3*t.powi(3) + r4*t.powi(4);

    (if l > 0_f64 {l} else {2_f64*PI+l}, b, r)
}

/// Calculates VSOP87B solution for Mars
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates)
/// for the planet Mars. The parameter needed is the Julian Day Efemeris (*JDE*) for the given
/// date. It returns, in order, a tuple with the values *l*, *b*, *r* of the VSOP87B solution.
/// Those values are the spherical coordinates of the planet, with the Sun in the center and the
/// ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87b;
///
/// let (l, b, r) = vsop87b::mars(2341970.0);
///
/// assert!(l > 2.9897807829 && l < 2.9897807831);
/// assert!(b > 0.0280781216 && b < 0.0280781218);
/// assert!(r > 1.6584693 && r < 1.6584701);
/// ```
pub fn mars(jde: f64) -> (f64, f64, f64) {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &mars::L0);
    let l1 = calculate_var(t, &mars::L1);
    let l2 = calculate_var(t, &mars::L2);
    let l3 = calculate_var(t, &mars::L3);
    let l4 = calculate_var(t, &mars::L4);
    let l5 = calculate_var(t, &mars::L5);

    let b0 = calculate_var(t, &mars::B0);
    let b1 = calculate_var(t, &mars::B1);
    let b2 = calculate_var(t, &mars::B2);
    let b3 = calculate_var(t, &mars::B3);
    let b4 = calculate_var(t, &mars::B4);
    let b5 = calculate_var(t, &mars::B5);

    let r0 = calculate_var(t, &mars::R0);
    let r1 = calculate_var(t, &mars::R1);
    let r2 = calculate_var(t, &mars::R2);
    let r3 = calculate_var(t, &mars::R3);
    let r4 = calculate_var(t, &mars::R4);

    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let b = b0 + b1*t + b2*t*t + b3*t.powi(3) + b4*t.powi(4) + b5*t.powi(5);
    let r = r0 + r1*t + r2*t*t + r3*t.powi(3) + r4*t.powi(4);

    (if l > 0_f64 {l} else {2_f64*PI+l}, b, r)
}

/// Calculates VSOP87B solution for Jupiter
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates)
/// for the planet Jupiter. The parameter needed is the Julian Day Efemeris (*JDE*) for the given
/// date. It returns, in order, a tuple with the values *l*, *b*, *r* of the VSOP87B solution.
/// Those values are the spherical coordinates of the planet, with the Sun in the center and the
/// ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87b;
///
/// let (l, b, r) = vsop87b::jupiter(2305445.0);
///
/// assert!(l > 2.4323346133 && l < 2.4323346135);
/// assert!(b > 0.0145957281 && b < 0.0145957283);
/// assert!(r > 5.3439451 && r < 5.3439459);
/// ```
pub fn jupiter(jde: f64) -> (f64, f64, f64) {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &jupiter::L0);
    let l1 = calculate_var(t, &jupiter::L1);
    let l2 = calculate_var(t, &jupiter::L2);
    let l3 = calculate_var(t, &jupiter::L3);
    let l4 = calculate_var(t, &jupiter::L4);
    let l5 = calculate_var(t, &jupiter::L5);

    let b0 = calculate_var(t, &jupiter::B0);
    let b1 = calculate_var(t, &jupiter::B1);
    let b2 = calculate_var(t, &jupiter::B2);
    let b3 = calculate_var(t, &jupiter::B3);
    let b4 = calculate_var(t, &jupiter::B4);
    let b5 = calculate_var(t, &jupiter::B5);

    let r0 = calculate_var(t, &jupiter::R0);
    let r1 = calculate_var(t, &jupiter::R1);
    let r2 = calculate_var(t, &jupiter::R2);
    let r3 = calculate_var(t, &jupiter::R3);
    let r4 = calculate_var(t, &jupiter::R4);

    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let b = b0 + b1*t + b2*t*t + b3*t.powi(3) + b4*t.powi(4) + b5*t.powi(5);
    let r = r0 + r1*t + r2*t*t + r3*t.powi(3) + r4*t.powi(4);

    (if l > 0_f64 {l} else {2_f64*PI+l}, b, r)
}

/// Calculates VSOP87B solution for Saturn
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates)
/// for the planet Saturn. The parameter needed is the Julian Day Efemeris (*JDE*) for the given
/// date. It returns, in order, a tuple with the values *l*, *b*, *r* of the VSOP87B solution.
/// Those values are the spherical coordinates of the planet, with the Sun in the center and the
/// ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87b;
///
/// let (l, b, r) = vsop87b::saturn(2268920.0);
///
/// assert!(l > 0.9812189104 && l < 0.9812189106);
/// assert!(b > -0.0369435534 && b < -0.0369435532);
/// assert!(r > 9.0669210 && r < 9.0669218);
/// ```
pub fn saturn(jde: f64) -> (f64, f64, f64) {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &saturn::L0);
    let l1 = calculate_var(t, &saturn::L1);
    let l2 = calculate_var(t, &saturn::L2);
    let l3 = calculate_var(t, &saturn::L3);
    let l4 = calculate_var(t, &saturn::L4);
    let l5 = calculate_var(t, &saturn::L5);

    let b0 = calculate_var(t, &saturn::B0);
    let b1 = calculate_var(t, &saturn::B1);
    let b2 = calculate_var(t, &saturn::B2);
    let b3 = calculate_var(t, &saturn::B3);
    let b4 = calculate_var(t, &saturn::B4);
    let b5 = calculate_var(t, &saturn::B5);

    let r0 = calculate_var(t, &saturn::R0);
    let r1 = calculate_var(t, &saturn::R1);
    let r2 = calculate_var(t, &saturn::R2);
    let r3 = calculate_var(t, &saturn::R3);
    let r4 = calculate_var(t, &saturn::R4);

    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4) + l5*t.powi(5)) % (2_f64*PI);
    let b = b0 + b1*t + b2*t*t + b3*t.powi(3) + b4*t.powi(4) + b5*t.powi(5);
    let r = r0 + r1*t + r2*t*t + r3*t.powi(3) + r4*t.powi(4);

    (if l > 0_f64 {l} else {2_f64*PI+l}, b, r)
}

/// Calculates VSOP87B solution for Uranus
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates)
/// for the planet Uranus. The parameter needed is the Julian Day Efemeris (*JDE*) for the given
/// date. It returns, in order, a tuple with the values *l*, *b*, *r* of the VSOP87B solution.
/// Those values are the spherical coordinates of the planet, with the Sun in the center and the
/// ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87b;
///
/// let (l, b, r) = vsop87b::uranus(2232395.0);
///
/// assert!(l > 4.6715450661 && l < 4.6715450663);
/// assert!(b > -0.0033027750 && b < -0.0033027748);
/// assert!(r > 19.2694309 && r < 19.2694317);
/// ```
pub fn uranus(jde: f64) -> (f64, f64, f64) {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &uranus::L0);
    let l1 = calculate_var(t, &uranus::L1);
    let l2 = calculate_var(t, &uranus::L2);
    let l3 = calculate_var(t, &uranus::L3);
    let l4 = calculate_var(t, &uranus::L4);

    let b0 = calculate_var(t, &uranus::B0);
    let b1 = calculate_var(t, &uranus::B1);
    let b2 = calculate_var(t, &uranus::B2);
    let b3 = calculate_var(t, &uranus::B3);

    let r0 = calculate_var(t, &uranus::R0);
    let r1 = calculate_var(t, &uranus::R1);
    let r2 = calculate_var(t, &uranus::R2);
    let r3 = calculate_var(t, &uranus::R3);

    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3) + l4*t.powi(4)) % (2_f64*PI);
    let b = b0 + b1*t + b2*t*t + b3*t.powi(3);
    let r = r0 + r1*t + r2*t*t + r3*t.powi(3);

    (if l > 0_f64 {l} else {2_f64*PI+l}, b, r)
}

/// Calculates VSOP87B solution for Neptune
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates)
/// for the planet Neptune. The parameter needed is the Julian Day Efemeris (*JDE*) for the given
/// date. It returns, in order, a tuple with the values *l*, *b*, *r* of the VSOP87B solution.
/// Those values are the spherical coordinates of the planet, with the Sun in the center and the
/// ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87b;
///
/// let (l, b, r) = vsop87b::neptune(2195870.0);
///
/// assert!(l > 3.7635416327 && l < 3.7635416329);
/// assert!(b > 0.0306777429 && b < 0.0306777431);
/// assert!(r > 30.3109111 && r < 30.3109119);
/// ```
pub fn neptune(jde: f64) -> (f64, f64, f64) {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &neptune::L0);
    let l1 = calculate_var(t, &neptune::L1);
    let l2 = calculate_var(t, &neptune::L2);
    let l3 = calculate_var(t, &neptune::L3);

    let b0 = calculate_var(t, &neptune::B0);
    let b1 = calculate_var(t, &neptune::B1);
    let b2 = calculate_var(t, &neptune::B2);
    let b3 = calculate_var(t, &neptune::B3);

    let r0 = calculate_var(t, &neptune::R0);
    let r1 = calculate_var(t, &neptune::R1);
    let r2 = calculate_var(t, &neptune::R2);
    let r3 = calculate_var(t, &neptune::R3);

    let l = (l0 + l1*t + l2*t*t + l3*t.powi(3)) % (2_f64*PI);
    let b = b0 + b1*t + b2*t*t + b3*t.powi(3);
    let r = r0 + r1*t + r2*t*t + r3*t.powi(3);

    (if l > 0_f64 {l} else {2_f64*PI+l}, b, r)
}
