//! VSOP87D algorithm: Heliocentric ecliptic spherical coordinates for the equinox of the day.
//!
//! This module contains the functions to calculate heliocentric ecliptic spherical coordinates for
//! the equinox of the day for the planets in the solar system.
//!
//! # Example
//!
//! Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
//! position of the planet Jupiter in the solar system using spherical coordinates. In this case,
//! we calculate where Jupiter was in December 19th, 1399.
//!
//! ```
//! use vsop87::vsop87d;
//!
//! let coordinates = vsop87d::jupiter(2232395.0);
//!
//! assert!(coordinates.longitude() > 3.0889515349 && coordinates.longitude() < 3.0889515351);
//! assert!(coordinates.latitude() > 0.0231157946 && coordinates.latitude() < 0.0231157948);
//! assert!(coordinates.distance() > 5.44915664 && coordinates.distance() < 5.44915740);
//! ```

mod earth;
mod jupiter;
mod mars;
mod mercury;
mod neptune;
mod saturn;
mod uranus;
mod venus;

use super::{calculate_t, calculate_var, SphericalCoordinates};

#[cfg(feature = "no_std")]
use core::f64::consts::PI;
#[cfg(not(feature = "no_std"))]
use std::f64::consts::PI;

#[cfg(feature = "no_std")]
use core::num::Float;

/// Calculates VSOP87D solution for Mercury.
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Mercury. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87D solution ina `SphericalCoordinates` struct. Those
/// values are the spherical coordinates of the planet, with the Sun in the center and the ecliptic
/// plane as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Mercury in the solar system using spherical coordinates. In this case,
/// we calculate where Mercury was in December 30th, 1799.
///
/// ```
/// use vsop87::vsop87d;
///
/// let coordinates = vsop87d::mercury(2378495.0);
///
/// assert!(coordinates.longitude() > 2.0737894887 && coordinates.longitude() < 2.0737894889);
/// assert!(coordinates.latitude() > 0.1168184803 && coordinates.latitude() < 0.1168184805);
/// assert!(coordinates.distance() > 0.32339057 && coordinates.distance() < 0.32339133);
/// ```
pub fn mercury(jde: f64) -> SphericalCoordinates {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &mercury::L0[0], &mercury::L0[1], &mercury::L0[2]);
    let l1 = calculate_var(t, &mercury::L1[0], &mercury::L1[1], &mercury::L1[2]);
    let l2 = calculate_var(t, &mercury::L2[0], &mercury::L2[1], &mercury::L2[2]);
    let l3 = calculate_var(t, &mercury::L3[0], &mercury::L3[1], &mercury::L3[2]);
    let l4 = calculate_var(t, &mercury::L4[0], &mercury::L4[1], &mercury::L4[2]);
    let l5 = calculate_var(t, &mercury::L5[0], &mercury::L5[1], &mercury::L5[2]);

    let b0 = calculate_var(t, &mercury::B0[0], &mercury::B0[1], &mercury::B0[2]);
    let b1 = calculate_var(t, &mercury::B1[0], &mercury::B1[1], &mercury::B1[2]);
    let b2 = calculate_var(t, &mercury::B2[0], &mercury::B2[1], &mercury::B2[2]);
    let b3 = calculate_var(t, &mercury::B3[0], &mercury::B3[1], &mercury::B3[2]);
    let b4 = calculate_var(t, &mercury::B4[0], &mercury::B4[1], &mercury::B4[2]);
    let b5 = calculate_var(t, &mercury::B5[0], &mercury::B5[1], &mercury::B5[2]);

    let r0 = calculate_var(t, &mercury::R0[0], &mercury::R0[1], &mercury::R0[2]);
    let r1 = calculate_var(t, &mercury::R1[0], &mercury::R1[1], &mercury::R1[2]);
    let r2 = calculate_var(t, &mercury::R2[0], &mercury::R2[1], &mercury::R2[2]);
    let r3 = calculate_var(t, &mercury::R3[0], &mercury::R3[1], &mercury::R3[2]);
    let r4 = calculate_var(t, &mercury::R4[0], &mercury::R4[1], &mercury::R4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t2 + b3 * t3 + b4 * t4 + b5 * t5;
    let r = r0 + r1 * t + r2 * t2 + r3 * t3 + r4 * t4;

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87D solution for Venus.
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Venus. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87D solution ina `SphericalCoordinates` struct. Those values
/// are the spherical coordinates of the planet, with the Sun in the center and the ecliptic plane
/// as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Venus in the solar system using spherical coordinates. In this case, we
/// calculate where Venus was in December 29th, 1699.
///
/// ```
/// use vsop87::vsop87d;
///
/// let coordinates = vsop87d::venus(2341970.0);
///
/// assert!(coordinates.longitude() > 5.3115708035 && coordinates.longitude() < 5.3115708037);
/// assert!(coordinates.latitude() > -0.0455979905 && coordinates.latitude() < -0.0455979903);
/// assert!(coordinates.distance() > 0.72834037 && coordinates.distance() < 0.72834113);
/// ```
pub fn venus(jde: f64) -> SphericalCoordinates {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &venus::L0[0], &venus::L0[1], &venus::L0[2]);
    let l1 = calculate_var(t, &venus::L1[0], &venus::L1[1], &venus::L1[2]);
    let l2 = calculate_var(t, &venus::L2[0], &venus::L2[1], &venus::L2[2]);
    let l3 = calculate_var(t, &venus::L3[0], &venus::L3[1], &venus::L3[2]);
    let l4 = calculate_var(t, &venus::L4[0], &venus::L4[1], &venus::L4[2]);
    let l5 = calculate_var(t, &venus::L5[0], &venus::L5[1], &venus::L5[2]);

    let b0 = calculate_var(t, &venus::B0[0], &venus::B0[1], &venus::B0[2]);
    let b1 = calculate_var(t, &venus::B1[0], &venus::B1[1], &venus::B1[2]);
    let b2 = calculate_var(t, &venus::B2[0], &venus::B2[1], &venus::B2[2]);
    let b3 = calculate_var(t, &venus::B3[0], &venus::B3[1], &venus::B3[2]);
    let b4 = calculate_var(t, &venus::B4[0], &venus::B4[1], &venus::B4[2]);
    let b5 = calculate_var(t, &venus::B5[0], &venus::B5[1], &venus::B5[2]);

    let r0 = calculate_var(t, &venus::R0[0], &venus::R0[1], &venus::R0[2]);
    let r1 = calculate_var(t, &venus::R1[0], &venus::R1[1], &venus::R1[2]);
    let r2 = calculate_var(t, &venus::R2[0], &venus::R2[1], &venus::R2[2]);
    let r3 = calculate_var(t, &venus::R3[0], &venus::R3[1], &venus::R3[2]);
    let r4 = calculate_var(t, &venus::R4[0], &venus::R4[1], &venus::R4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t2 + b3 * t3 + b4 * t4 + b5 * t5;
    let r = r0 + r1 * t + r2 * t2 + r3 * t3 + r4 * t4;

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87D solution for Earth.
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Earth. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87D solution ina `SphericalCoordinates` struct. Those values
/// are the spherical coordinates of the planet, with the Sun in the center and the ecliptic plane
/// as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Earth in the solar system using spherical coordinates. In this case, we
/// calculate where the Earth was in December 29th, 1599.
///
/// ```
/// use vsop87::vsop87d;
///
/// let coordinates = vsop87d::earth(2305445.0);
///
/// assert!(coordinates.longitude() > 1.7006065937 && coordinates.longitude() < 1.7006065939);
/// assert!(coordinates.latitude() > -0.0000016360 && coordinates.latitude() < -0.0000016358);
/// assert!(coordinates.distance() > 0.98312506 && coordinates.distance() < 0.98312582);
/// ```
pub fn earth(jde: f64) -> SphericalCoordinates {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &earth::L0[0], &earth::L0[1], &earth::L0[2]);
    let l1 = calculate_var(t, &earth::L1[0], &earth::L1[1], &earth::L1[2]);
    let l2 = calculate_var(t, &earth::L2[0], &earth::L2[1], &earth::L2[2]);
    let l3 = calculate_var(t, &earth::L3[0], &earth::L3[1], &earth::L3[2]);
    let l4 = calculate_var(t, &earth::L4[0], &earth::L4[1], &earth::L4[2]);
    let l5 = calculate_var(t, &earth::L5[0], &earth::L5[1], &earth::L5[2]);

    let b0 = calculate_var(t, &earth::B0[0], &earth::B0[1], &earth::B0[2]);
    let b1 = calculate_var(t, &earth::B1[0], &earth::B1[1], &earth::B1[2]);
    let b2 = calculate_var(t, &earth::B2[0], &earth::B2[1], &earth::B2[2]);
    let b3 = calculate_var(t, &earth::B3[0], &earth::B3[1], &earth::B3[2]);
    let b4 = calculate_var(t, &earth::B4[0], &earth::B4[1], &earth::B4[2]);

    let r0 = calculate_var(t, &earth::R0[0], &earth::R0[1], &earth::R0[2]);
    let r1 = calculate_var(t, &earth::R1[0], &earth::R1[1], &earth::R1[2]);
    let r2 = calculate_var(t, &earth::R2[0], &earth::R2[1], &earth::R2[2]);
    let r3 = calculate_var(t, &earth::R3[0], &earth::R3[1], &earth::R3[2]);
    let r4 = calculate_var(t, &earth::R4[0], &earth::R4[1], &earth::R4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t2 + b3 * t3 + b4 * t4;
    let r = r0 + r1 * t + r2 * t2 + r3 * t3 + r4 * t4;

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87D solution for Mars.
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Mars. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87D solution ina `SphericalCoordinates` struct. Those values
/// are the spherical coordinates of the planet, with the Sun in the center and the ecliptic plane
/// as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Mars in the solar system using spherical coordinates. In this case, we
/// calculate where Mercury was in December 19th, 1499.
///
/// ```
/// use vsop87::vsop87d;
///
/// let coordinates = vsop87d::mars(2268920.0);
///
/// assert!(coordinates.longitude() > 1.0050966938 && coordinates.longitude() < 1.0050966940);
/// assert!(coordinates.latitude() > 0.0066676097 && coordinates.latitude() < 0.0066676099);
/// assert!(coordinates.distance() > 1.51236189 && coordinates.distance() < 1.51236265);
/// ```
pub fn mars(jde: f64) -> SphericalCoordinates {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &mars::L0[0], &mars::L0[1], &mars::L0[2]);
    let l1 = calculate_var(t, &mars::L1[0], &mars::L1[1], &mars::L1[2]);
    let l2 = calculate_var(t, &mars::L2[0], &mars::L2[1], &mars::L2[2]);
    let l3 = calculate_var(t, &mars::L3[0], &mars::L3[1], &mars::L3[2]);
    let l4 = calculate_var(t, &mars::L4[0], &mars::L4[1], &mars::L4[2]);
    let l5 = calculate_var(t, &mars::L5[0], &mars::L5[1], &mars::L5[2]);

    let b0 = calculate_var(t, &mars::B0[0], &mars::B0[1], &mars::B0[2]);
    let b1 = calculate_var(t, &mars::B1[0], &mars::B1[1], &mars::B1[2]);
    let b2 = calculate_var(t, &mars::B2[0], &mars::B2[1], &mars::B2[2]);
    let b3 = calculate_var(t, &mars::B3[0], &mars::B3[1], &mars::B3[2]);
    let b4 = calculate_var(t, &mars::B4[0], &mars::B4[1], &mars::B4[2]);
    let b5 = calculate_var(t, &mars::B5[0], &mars::B5[1], &mars::B5[2]);

    let r0 = calculate_var(t, &mars::R0[0], &mars::R0[1], &mars::R0[2]);
    let r1 = calculate_var(t, &mars::R1[0], &mars::R1[1], &mars::R1[2]);
    let r2 = calculate_var(t, &mars::R2[0], &mars::R2[1], &mars::R2[2]);
    let r3 = calculate_var(t, &mars::R3[0], &mars::R3[1], &mars::R3[2]);
    let r4 = calculate_var(t, &mars::R4[0], &mars::R4[1], &mars::R4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t2 + b3 * t3 + b4 * t4 + b5 * t5;
    let r = r0 + r1 * t + r2 * t2 + r3 * t3 + r4 * t4;

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87D solution for Jupiter.
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Jupiter. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87D solution ina `SphericalCoordinates` struct. Those
/// values are the spherical coordinates of the planet, with the Sun in the center and the ecliptic
/// plane as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Jupiter in the solar system using spherical coordinates. In this case,
/// we calculate where Jupiter was in December 19th, 1399.
///
/// ```
/// use vsop87::vsop87d;
///
/// let coordinates = vsop87d::jupiter(2232395.0);
///
/// assert!(coordinates.longitude() > 3.0889515349 && coordinates.longitude() < 3.0889515351);
/// assert!(coordinates.latitude() > 0.0231157946 && coordinates.latitude() < 0.0231157948);
/// assert!(coordinates.distance() > 5.44915664 && coordinates.distance() < 5.44915740);
/// ```
pub fn jupiter(jde: f64) -> SphericalCoordinates {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &jupiter::L0[0], &jupiter::L0[1], &jupiter::L0[2]);
    let l1 = calculate_var(t, &jupiter::L1[0], &jupiter::L1[1], &jupiter::L1[2]);
    let l2 = calculate_var(t, &jupiter::L2[0], &jupiter::L2[1], &jupiter::L2[2]);
    let l3 = calculate_var(t, &jupiter::L3[0], &jupiter::L3[1], &jupiter::L3[2]);
    let l4 = calculate_var(t, &jupiter::L4[0], &jupiter::L4[1], &jupiter::L4[2]);
    let l5 = calculate_var(t, &jupiter::L5[0], &jupiter::L5[1], &jupiter::L5[2]);

    let b0 = calculate_var(t, &jupiter::B0[0], &jupiter::B0[1], &jupiter::B0[2]);
    let b1 = calculate_var(t, &jupiter::B1[0], &jupiter::B1[1], &jupiter::B1[2]);
    let b2 = calculate_var(t, &jupiter::B2[0], &jupiter::B2[1], &jupiter::B2[2]);
    let b3 = calculate_var(t, &jupiter::B3[0], &jupiter::B3[1], &jupiter::B3[2]);
    let b4 = calculate_var(t, &jupiter::B4[0], &jupiter::B4[1], &jupiter::B4[2]);
    let b5 = calculate_var(t, &jupiter::B5[0], &jupiter::B5[1], &jupiter::B5[2]);

    let r0 = calculate_var(t, &jupiter::R0[0], &jupiter::R0[1], &jupiter::R0[2]);
    let r1 = calculate_var(t, &jupiter::R1[0], &jupiter::R1[1], &jupiter::R1[2]);
    let r2 = calculate_var(t, &jupiter::R2[0], &jupiter::R2[1], &jupiter::R2[2]);
    let r3 = calculate_var(t, &jupiter::R3[0], &jupiter::R3[1], &jupiter::R3[2]);
    let r4 = calculate_var(t, &jupiter::R4[0], &jupiter::R4[1], &jupiter::R4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t2 + b3 * t3 + b4 * t4 + b5 * t5;
    let r = r0 + r1 * t + r2 * t2 + r3 * t3 + r4 * t4;

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87D solution for Saturn.
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Saturn. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87D solution ina `SphericalCoordinates` struct. Those
/// values are the spherical coordinates of the planet, with the Sun in the center and the ecliptic
/// plane as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Saturn in the solar system using spherical coordinates. In this case, we
/// calculate where Saturn was in December 19th, 1299.
///
/// ```
/// use vsop87::vsop87d;
///
/// let coordinates = vsop87d::saturn(2195870.0);
///
/// assert!(coordinates.longitude() > 2.2948875822 && coordinates.longitude() < 2.2948875824);
/// assert!(coordinates.latitude() > 0.0178533696 && coordinates.latitude() < 0.0178533698);
/// assert!(coordinates.distance() > 9.18575957 && coordinates.distance() < 9.18576033);
/// ```
pub fn saturn(jde: f64) -> SphericalCoordinates {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &saturn::L0[0], &saturn::L0[1], &saturn::L0[2]);
    let l1 = calculate_var(t, &saturn::L1[0], &saturn::L1[1], &saturn::L1[2]);
    let l2 = calculate_var(t, &saturn::L2[0], &saturn::L2[1], &saturn::L2[2]);
    let l3 = calculate_var(t, &saturn::L3[0], &saturn::L3[1], &saturn::L3[2]);
    let l4 = calculate_var(t, &saturn::L4[0], &saturn::L4[1], &saturn::L4[2]);
    let l5 = calculate_var(t, &saturn::L5[0], &saturn::L5[1], &saturn::L5[2]);

    let b0 = calculate_var(t, &saturn::B0[0], &saturn::B0[1], &saturn::B0[2]);
    let b1 = calculate_var(t, &saturn::B1[0], &saturn::B1[1], &saturn::B1[2]);
    let b2 = calculate_var(t, &saturn::B2[0], &saturn::B2[1], &saturn::B2[2]);
    let b3 = calculate_var(t, &saturn::B3[0], &saturn::B3[1], &saturn::B3[2]);
    let b4 = calculate_var(t, &saturn::B4[0], &saturn::B4[1], &saturn::B4[2]);
    let b5 = calculate_var(t, &saturn::B5[0], &saturn::B5[1], &saturn::B5[2]);

    let r0 = calculate_var(t, &saturn::R0[0], &saturn::R0[1], &saturn::R0[2]);
    let r1 = calculate_var(t, &saturn::R1[0], &saturn::R1[1], &saturn::R1[2]);
    let r2 = calculate_var(t, &saturn::R2[0], &saturn::R2[1], &saturn::R2[2]);
    let r3 = calculate_var(t, &saturn::R3[0], &saturn::R3[1], &saturn::R3[2]);
    let r4 = calculate_var(t, &saturn::R4[0], &saturn::R4[1], &saturn::R4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t2 + b3 * t3 + b4 * t4 + b5 * t5;
    let r = r0 + r1 * t + r2 * t2 + r3 * t3 + r4 * t4;

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87D solution for Uranus.
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Uranus. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87D solution ina `SphericalCoordinates` struct. Those
/// values are the spherical coordinates of the planet, with the Sun in the center and the ecliptic
/// plane as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Uranus in the solar system using spherical coordinates. In this case, we
/// calculate where Uranus was in December 19th, 1199.
///
/// ```
/// use vsop87::vsop87d;
///
/// let coordinates = vsop87d::uranus(2159345.0);
///
/// assert!(coordinates.longitude() > 1.9333853934 && coordinates.longitude() < 1.9333853936);
/// assert!(coordinates.latitude() > 0.0088045917 && coordinates.latitude() < 0.0088045919);
/// assert!(coordinates.distance() > 18.58414975 && coordinates.distance() < 18.58415051);
/// ```
pub fn uranus(jde: f64) -> SphericalCoordinates {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &uranus::L0[0], &uranus::L0[1], &uranus::L0[2]);
    let l1 = calculate_var(t, &uranus::L1[0], &uranus::L1[1], &uranus::L1[2]);
    let l2 = calculate_var(t, &uranus::L2[0], &uranus::L2[1], &uranus::L2[2]);
    let l3 = calculate_var(t, &uranus::L3[0], &uranus::L3[1], &uranus::L3[2]);
    let l4 = calculate_var(t, &uranus::L4[0], &uranus::L4[1], &uranus::L4[2]);
    let l5 = calculate_var(t, &uranus::L5[0], &uranus::L5[1], &uranus::L5[2]);

    let b0 = calculate_var(t, &uranus::B0[0], &uranus::B0[1], &uranus::B0[2]);
    let b1 = calculate_var(t, &uranus::B1[0], &uranus::B1[1], &uranus::B1[2]);
    let b2 = calculate_var(t, &uranus::B2[0], &uranus::B2[1], &uranus::B2[2]);
    let b3 = calculate_var(t, &uranus::B3[0], &uranus::B3[1], &uranus::B3[2]);
    let b4 = calculate_var(t, &uranus::B4[0], &uranus::B4[1], &uranus::B4[2]);

    let r0 = calculate_var(t, &uranus::R0[0], &uranus::R0[1], &uranus::R0[2]);
    let r1 = calculate_var(t, &uranus::R1[0], &uranus::R1[1], &uranus::R1[2]);
    let r2 = calculate_var(t, &uranus::R2[0], &uranus::R2[1], &uranus::R2[2]);
    let r3 = calculate_var(t, &uranus::R3[0], &uranus::R3[1], &uranus::R3[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t2 + b3 * t3 + b4 * t4;
    let r = r0 + r1 * t + r2 * t2 + r3 * t3;

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87D solution for Neptune.
///
/// This function calculates the VSOP87D solution (heliocentric ecliptic spherical coordinates for
/// the equinox of the day) for the planet Neptune. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87D solution ina `SphericalCoordinates` struct. Those
/// values are the spherical coordinates of the planet, with the Sun in the center and the ecliptic
/// plane as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Neptune in the solar system using spherical coordinates. In this case,
/// we calculate where Neptune was in December 19th, 1099.
///
/// ```
/// use vsop87::vsop87d;
///
/// let coordinates = vsop87d::neptune(2122820.0);
///
/// assert!(coordinates.longitude() > 2.2124988266 && coordinates.longitude() < 2.2124988268);
/// assert!(coordinates.latitude() > 0.0027498092 && coordinates.latitude() < 0.0027498094);
/// assert!(coordinates.distance() > 30.06536898 && coordinates.distance() < 30.06536974);
/// ```
pub fn neptune(jde: f64) -> SphericalCoordinates {
    let t = calculate_t(jde);

    let l0 = calculate_var(t, &neptune::L0[0], &neptune::L0[1], &neptune::L0[2]);
    let l1 = calculate_var(t, &neptune::L1[0], &neptune::L1[1], &neptune::L1[2]);
    let l2 = calculate_var(t, &neptune::L2[0], &neptune::L2[1], &neptune::L2[2]);
    let l3 = calculate_var(t, &neptune::L3[0], &neptune::L3[1], &neptune::L3[2]);
    let l4 = calculate_var(t, &neptune::L4[0], &neptune::L4[1], &neptune::L4[2]);
    let l5 = calculate_var(t, &neptune::L5[0], &neptune::L5[1], &neptune::L5[2]);

    let b0 = calculate_var(t, &neptune::B0[0], &neptune::B0[1], &neptune::B0[2]);
    let b1 = calculate_var(t, &neptune::B1[0], &neptune::B1[1], &neptune::B1[2]);
    let b2 = calculate_var(t, &neptune::B2[0], &neptune::B2[1], &neptune::B2[2]);
    let b3 = calculate_var(t, &neptune::B3[0], &neptune::B3[1], &neptune::B3[2]);
    let b4 = calculate_var(t, &neptune::B4[0], &neptune::B4[1], &neptune::B4[2]);
    let b5 = calculate_var(t, &neptune::B5[0], &neptune::B5[1], &neptune::B5[2]);

    let r0 = calculate_var(t, &neptune::R0[0], &neptune::R0[1], &neptune::R0[2]);
    let r1 = calculate_var(t, &neptune::R1[0], &neptune::R1[1], &neptune::R1[2]);
    let r2 = calculate_var(t, &neptune::R2[0], &neptune::R2[1], &neptune::R2[2]);
    let r3 = calculate_var(t, &neptune::R3[0], &neptune::R3[1], &neptune::R3[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t2 + b3 * t3 + b4 * t4 + b5 * t5;
    let r = r0 + r1 * t + r2 * t2 + r3 * t3;

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}
