//! VSOP87D implementation
//!
//! This module calculates heliocentric ecliptic spherical coordinates for the equinox of the day
//! for the planets in the solar system.

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

/// Calculates VSOP87D solution for Mercury
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Mercury. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *l*, *b*, *r* of the
/// VSOP87D solution. Those values are the spherical coordinates of the planet, with the Sun in the
/// center and the ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87d;
///
/// let (l, b, r) = vsop87d::mercury(2378495.0);
///
/// assert!(l > 2.0737894887 && l < 2.0737894889);
/// assert!(b > 0.1168184803 && b < 0.1168184805);
/// assert!(r > 0.32339057 && r < 0.32339133);
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5)) %
            (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    (if l > 0_f64 { l } else { 2_f64 * PI + l }, b, r)
}

/// Calculates VSOP87D solution for Venus
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Venus. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *l*, *b*, *r* of the
/// VSOP87D solution. Those values are the spherical coordinates of the planet, with the Sun in the
/// center and the ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87d;
///
/// let (l, b, r) = vsop87d::venus(2341970.0);
///
/// assert!(l > 5.3115708035 && l < 5.3115708037);
/// assert!(b > -0.0455979905 && b < -0.0455979903);
/// assert!(r > 0.72834037 && r < 0.72834113);
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5)) %
            (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    (if l > 0_f64 { l } else { 2_f64 * PI + l }, b, r)
}

/// Calculates VSOP87D solution for Earth
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Earth. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *l*, *b*, *r* of the
/// VSOP87D solution. Those values are the spherical coordinates of the planet, with the Sun in the
/// center and the ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87d;
///
/// let (l, b, r) = vsop87d::earth(2305445.0);
///
/// assert!(l > 1.7006065937 && l < 1.7006065939);
/// assert!(b > -0.0000016360 && b < -0.0000016358);
/// assert!(r > 0.98312506 && r < 0.98312582);
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

    let r0 = calculate_var(t, &earth::R0);
    let r1 = calculate_var(t, &earth::R1);
    let r2 = calculate_var(t, &earth::R2);
    let r3 = calculate_var(t, &earth::R3);
    let r4 = calculate_var(t, &earth::R4);

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5)) %
            (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    (if l > 0_f64 { l } else { 2_f64 * PI + l }, b, r)
}

/// Calculates VSOP87D solution for Mars
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Mars. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *l*, *b*, *r* of the
/// VSOP87D solution. Those values are the spherical coordinates of the planet, with the Sun in the
/// center and the ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87d;
///
/// let (l, b, r) = vsop87d::mars(2268920.0);
///
/// assert!(l > 1.0050966938 && l < 1.0050966940);
/// assert!(b > 0.0066676097 && b < 0.0066676099);
/// assert!(r > 1.51236189 && r < 1.51236265);
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5)) %
            (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    (if l > 0_f64 { l } else { 2_f64 * PI + l }, b, r)
}

/// Calculates VSOP87D solution for Jupiter
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Jupiter. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *l*, *b*, *r* of the
/// VSOP87D solution. Those values are the spherical coordinates of the planet, with the Sun in the
/// center and the ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87d;
///
/// let (l, b, r) = vsop87d::jupiter(2232395.0);
///
/// assert!(l > 3.0889515349 && l < 3.0889515351);
/// assert!(b > 0.0231157946 && b < 0.0231157948);
/// assert!(r > 5.44915664 && r < 5.44915740);
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5)) %
            (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    (if l > 0_f64 { l } else { 2_f64 * PI + l }, b, r)
}

/// Calculates VSOP87D solution for Saturn
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Saturn. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *l*, *b*, *r* of the
/// VSOP87D solution. Those values are the spherical coordinates of the planet, with the Sun in the
/// center and the ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87d;
///
/// let (l, b, r) = vsop87d::saturn(2195870.0);
///
/// assert!(l > 2.2948875822 && l < 2.2948875824);
/// assert!(b > 0.0178533696 && b < 0.0178533698);
/// assert!(r > 9.18575957 && r < 9.18576033);
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5)) %
            (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    (if l > 0_f64 { l } else { 2_f64 * PI + l }, b, r)
}

/// Calculates VSOP87D solution for Uranus
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Uranus. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *l*, *b*, *r* of the
/// VSOP87D solution. Those values are the spherical coordinates of the planet, with the Sun in the
/// center and the ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87d;
///
/// let (l, b, r) = vsop87d::uranus(2159345.0);
///
/// assert!(l > 1.9333853934 && l < 1.9333853936);
/// assert!(b > 0.0088045917 && b < 0.0088045919);
/// assert!(r > 18.58414975 && r < 18.58415051);
/// ```
pub fn uranus(jde: f64) -> (f64, f64, f64) {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &uranus::L0);
    let l1 = calculate_var(t, &uranus::L1);
    let l2 = calculate_var(t, &uranus::L2);
    let l3 = calculate_var(t, &uranus::L3);
    let l4 = calculate_var(t, &uranus::L4);
    let l5 = calculate_var(t, &uranus::L5);

    let b0 = calculate_var(t, &uranus::B0);
    let b1 = calculate_var(t, &uranus::B1);
    let b2 = calculate_var(t, &uranus::B2);
    let b3 = calculate_var(t, &uranus::B3);
    let b4 = calculate_var(t, &uranus::B4);

    let r0 = calculate_var(t, &uranus::R0);
    let r1 = calculate_var(t, &uranus::R1);
    let r2 = calculate_var(t, &uranus::R2);
    let r3 = calculate_var(t, &uranus::R3);

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5)) %
            (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3);

    (if l > 0_f64 { l } else { 2_f64 * PI + l }, b, r)
}

/// Calculates VSOP87D solution for Neptune
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Neptune. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *l*, *b*, *r* of the
/// VSOP87D solution. Those values are the spherical coordinates of the planet, with the Sun in the
/// center and the ecliptic plane as reference ```b = 0```:
///
/// - Ecliptic longitude (*L*), in radians
/// - Ecliptic latitude (*B*), in radians
/// - Radial distance (*R*), in *AU*
///
/// # Examples
///
/// ```
/// use vsop87::vsop87d;
///
/// let (l, b, r) = vsop87d::neptune(2122820.0);
///
/// assert!(l > 2.2124988266 && l < 2.2124988268);
/// assert!(b > 0.0027498092 && b < 0.0027498094);
/// assert!(r > 30.06536898 && r < 30.06536974);
/// ```
pub fn neptune(jde: f64) -> (f64, f64, f64) {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &neptune::L0);
    let l1 = calculate_var(t, &neptune::L1);
    let l2 = calculate_var(t, &neptune::L2);
    let l3 = calculate_var(t, &neptune::L3);
    let l4 = calculate_var(t, &neptune::L4);
    let l5 = calculate_var(t, &neptune::L5);

    let b0 = calculate_var(t, &neptune::B0);
    let b1 = calculate_var(t, &neptune::B1);
    let b2 = calculate_var(t, &neptune::B2);
    let b3 = calculate_var(t, &neptune::B3);
    let b4 = calculate_var(t, &neptune::B4);
    let b5 = calculate_var(t, &neptune::B5);

    let r0 = calculate_var(t, &neptune::R0);
    let r1 = calculate_var(t, &neptune::R1);
    let r2 = calculate_var(t, &neptune::R2);
    let r3 = calculate_var(t, &neptune::R3);

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5)) %
            (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3);

    (if l > 0_f64 { l } else { 2_f64 * PI + l }, b, r)
}
