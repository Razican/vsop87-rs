//! VSOP87B algorithm: Heliocentric ecliptic spherical coordinates for the equinox J2000.0.
//!
//! This module calculates heliocentric ecliptic spherical coordinates for the equinox J2000.0 for
//! the planets in the solar system.
//!
//! # Example
//!
//! Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
//! position of the planet Earth in the solar system using spherical coordinates. In this case, we
//! calculate where the Earth was in December 30th, 1799.
//!
//! ```
//! use vsop87::vsop87b;
//!
//! let coordinates = vsop87b::earth(2378495.0);
//!
//! assert!(coordinates.longitude() > 1.7750058557 && coordinates.longitude() < 1.7750058559);
//! assert!(coordinates.latitude() > 0.0004381094 && coordinates.latitude() < 0.0004381096);
//! assert!(coordinates.distance() > 0.9832270 && coordinates.distance() < 0.9832278);

mod mercury;
mod venus;
mod earth;
mod mars;
mod jupiter;
mod saturn;
mod uranus;
mod neptune;

use super::{calculate_t, calculate_var, SphericalCoordinates};
use std::f64::consts::PI;

/// Calculates VSOP87B solution for Mercury.
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates for
/// the equinox J2000.0) for the planet Mercury. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87B solution ina `SphericalCoordinates` struct. Those values
/// are the spherical coordinates of the planet, with the Sun in the center and the ecliptic plane
/// as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Mercury in the solar system using spherical coordinates. In this case,
/// we calculate where Mercury was in January 1st, 2000.
///
/// ```
/// use vsop87::vsop87b;
///
/// let coordinates = vsop87b::mercury(2451545.0);
///
/// assert!(coordinates.longitude() > 4.4293481042 && coordinates.longitude() < 4.4293481044);
/// assert!(coordinates.latitude() > -0.0527573412 && coordinates.latitude() < -0.0527573410);
/// assert!(coordinates.distance() > 0.4664711 && coordinates.distance() < 0.4664719);
/// ```
pub fn mercury(jde: f64) -> SphericalCoordinates {
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87B solution for Venus.
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates for
/// the equinox J2000.0) for the planet Venus. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87B solution ina `SphericalCoordinates` struct. Those values
/// are the spherical coordinates of the planet, with the Sun in the center and the ecliptic plane
/// as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Venus in the solar system using spherical coordinates. In this case, we
/// calculate where Venus was in December 31st, 1899.
///
/// ```
/// use vsop87::vsop87b;
///
/// let coordinates = vsop87b::venus(2415020.0);
///
/// assert!(coordinates.longitude() > 5.9993518123 && coordinates.longitude() < 5.9993518125);
/// assert!(coordinates.latitude() > -0.0591709805 && coordinates.latitude() < -0.0591709803);
/// assert!(coordinates.distance() > 0.7274715 && coordinates.distance() < 0.7274723);
/// ```
pub fn venus(jde: f64) -> SphericalCoordinates {
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87B solution for Earth.
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates for
/// the equinox J2000.0) for the planet Earth. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87B solution ina `SphericalCoordinates` struct. Those values
/// are the spherical coordinates of the planet, with the Sun in the center and the ecliptic plane
/// as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Earth in the solar system using spherical coordinates. In this case, we
/// calculate where the Earth was in December 30th, 1799.
///
/// ```
/// use vsop87::vsop87b;
///
/// let coordinates = vsop87b::earth(2378495.0);
///
/// assert!(coordinates.longitude() > 1.7750058557 && coordinates.longitude() < 1.7750058559);
/// assert!(coordinates.latitude() > 0.0004381094 && coordinates.latitude() < 0.0004381096);
/// assert!(coordinates.distance() > 0.9832270 && coordinates.distance() < 0.9832278);
/// ```
pub fn earth(jde: f64) -> SphericalCoordinates {
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87B solution for Mars.
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates for
/// the equinox J2000.0) for the planet Mars. The parameter needed is the Julian Day (*JD*) for the
/// given date. It returns the VSOP87B solution ina `SphericalCoordinates` struct. Those values are
/// the spherical coordinates of the planet, with the Sun in the center and the ecliptic plane as
/// reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Mars in the solar system using spherical coordinates. In this case, we
/// calculate where Mars was in December 29th, 1699.
///
/// ```
/// use vsop87::vsop87b;
///
/// let coordinates = vsop87b::mars(2341970.0);
///
/// assert!(coordinates.longitude() > 2.9897807829 && coordinates.longitude() < 2.9897807831);
/// assert!(coordinates.latitude() > 0.0280781216 && coordinates.latitude() < 0.0280781218);
/// assert!(coordinates.distance() > 1.6584693 && coordinates.distance() < 1.6584701);
/// ```
pub fn mars(jde: f64) -> SphericalCoordinates {
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87B solution for Jupiter.
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates for
/// the equinox J2000.0) for the planet Jupiter. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87B solution ina `SphericalCoordinates` struct. Those values
/// are the spherical coordinates of the planet, with the Sun in the center and the ecliptic plane
/// as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Jupiter in the solar system using spherical coordinates. In this case,
/// we calculate where Jupiter was in December 29th, 1599.
///
/// ```
/// use vsop87::vsop87b;
///
/// let coordinates = vsop87b::jupiter(2305445.0);
///
/// assert!(coordinates.longitude() > 2.4323346133 && coordinates.longitude() < 2.4323346135);
/// assert!(coordinates.latitude() > 0.0145957281 && coordinates.latitude() < 0.0145957283);
/// assert!(coordinates.distance() > 5.3439451 && coordinates.distance() < 5.3439459);
/// ```
pub fn jupiter(jde: f64) -> SphericalCoordinates {
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87B solution for Saturn.
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates for
/// the equinox J2000.0) for the planet Saturn. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87B solution ina `SphericalCoordinates` struct. Those values
/// are the spherical coordinates of the planet, with the Sun in the center and the ecliptic plane
/// as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Saturn in the solar system using spherical coordinates. In this case, we
/// calculate where Saturn was in December 19th, 1499.
///
/// ```
/// use vsop87::vsop87b;
///
/// let coordinates = vsop87b::saturn(2268920.0);
///
/// assert!(coordinates.longitude() > 0.9812189104 && coordinates.longitude() < 0.9812189106);
/// assert!(coordinates.latitude() > -0.0369435534 && coordinates.latitude() < -0.0369435532);
/// assert!(coordinates.distance() > 9.0669210 && coordinates.distance() < 9.0669218);
/// ```
pub fn saturn(jde: f64) -> SphericalCoordinates {
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3) + b4 * t.powi(4) + b5 * t.powi(5);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3) + r4 * t.powi(4);

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87B solution for Uranus.
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates for
/// the equinox J2000.0) for the planet Uranus. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87B solution ina `SphericalCoordinates` struct. Those values
/// are the spherical coordinates of the planet, with the Sun in the center and the ecliptic plane
/// as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Uranus in the solar system using spherical coordinates. In this case, we
/// calculate where Uranus was in December 19th, 1399.
///
/// ```
/// use vsop87::vsop87b;
///
/// let coordinates = vsop87b::uranus(2232395.0);
///
/// assert!(coordinates.longitude() > 4.6715450661 && coordinates.longitude() < 4.6715450663);
/// assert!(coordinates.latitude() > -0.0033027750 && coordinates.latitude() < -0.0033027748);
/// assert!(coordinates.distance() > 19.2694309 && coordinates.distance() < 19.2694317);
/// ```
pub fn uranus(jde: f64) -> SphericalCoordinates {
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4)) % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3);

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}

/// Calculates VSOP87B solution for Neptune.
///
/// This function calculates the VSOP87B solution (heliocentric ecliptic spherical coordinates for
/// the equinox J2000.0) for the planet Neptune. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87B solution ina `SphericalCoordinates` struct. Those values
/// are the spherical coordinates of the planet, with the Sun in the center and the ecliptic plane
/// as reference `latitude = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Neptuine in the solar system using spherical coordinates. In this case,
/// we calculate where Neptune was in December 19th, 1299.
///
/// ```
/// use vsop87::vsop87b;
///
/// let coordinates = vsop87b::neptune(2195870.0);
///
/// assert!(coordinates.longitude() > 3.7635416327 && coordinates.longitude() < 3.7635416329);
/// assert!(coordinates.latitude() > 0.0306777429 && coordinates.latitude() < 0.0306777431);
/// assert!(coordinates.distance() > 30.3109111 && coordinates.distance() < 30.3109119);
/// ```
pub fn neptune(jde: f64) -> SphericalCoordinates {
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

    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3)) % (2_f64 * PI);
    let b = b0 + b1 * t + b2 * t * t + b3 * t.powi(3);
    let r = r0 + r1 * t + r2 * t * t + r3 * t.powi(3);

    SphericalCoordinates {
        lon: if l > 0_f64 { l } else { 2_f64 * PI + l },
        lat: b,
        dist: r,
    }
}
