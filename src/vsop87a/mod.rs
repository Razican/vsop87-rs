//! VSOP87A algorithm: Heliocentric ecliptic rectangular coordinates for the equinox J2000.0.
//!
//! This module contains the functions to calculate heliocentric ecliptic rectangular coordinates
//! for the equinox J2000.0 for the planets in the solar system. The most useful when converting to
//! geocentric positions and later plot the position on a star chart.
//!
//! # Example
//!
//! Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
//! position of the planet Mercury in the solar system using rectangular coordinates. In this case,
//! we calculate where Mercury was in December 31st, 1899.
//!
//! ```
//! use vsop87::vsop87a;
//!
//! let coordinates = vsop87a::mercury(2415020.0);
//!
//! assert!(coordinates.x > -0.3897246932 && coordinates.x < -0.3897246930);
//! assert!(coordinates.y > -0.1502242200 && coordinates.y < -0.1502242198);
//! assert!(coordinates.z > 0.023618 && coordinates.z < 0.023622);

mod earth;
mod earth_moon;
mod jupiter;
mod mars;
mod mercury;
mod neptune;
mod saturn;
mod uranus;
mod venus;

use super::{calculate_t, calculate_var, RectangularCoordinates};
#[cfg(feature = "no_std")]
use core::num::Float;

/// Calculates VSOP87A solution for Mercury.
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Mercury. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87A solution in a `RectangularCoordinates` structure.
/// Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in the center
/// and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Mercury in the solar system using rectangular coordinates. In this case,
/// we calculate where Mercury was in December 31st, 1899.
///
/// ```
/// use vsop87::vsop87a;
///
/// let coordinates = vsop87a::mercury(2415020.0);
///
/// assert!(coordinates.x > -0.3897246932 && coordinates.x < -0.3897246930);
/// assert!(coordinates.y > -0.1502242200 && coordinates.y < -0.1502242198);
/// assert!(coordinates.z > 0.023618 && coordinates.z < 0.023622);
/// ```
pub fn mercury(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &mercury::X0[0], &mercury::X0[1], &mercury::X0[2]);
    let x1 = calculate_var(t, &mercury::X1[0], &mercury::X1[1], &mercury::X1[2]);
    let x2 = calculate_var(t, &mercury::X2[0], &mercury::X2[1], &mercury::X2[2]);
    let x3 = calculate_var(t, &mercury::X3[0], &mercury::X3[1], &mercury::X3[2]);
    let x4 = calculate_var(t, &mercury::X4[0], &mercury::X4[1], &mercury::X4[2]);
    let x5 = calculate_var(t, &mercury::X5[0], &mercury::X5[1], &mercury::X5[2]);

    let y0 = calculate_var(t, &mercury::Y0[0], &mercury::Y0[1], &mercury::Y0[2]);
    let y1 = calculate_var(t, &mercury::Y1[0], &mercury::Y1[1], &mercury::Y1[2]);
    let y2 = calculate_var(t, &mercury::Y2[0], &mercury::Y2[1], &mercury::Y2[2]);
    let y3 = calculate_var(t, &mercury::Y3[0], &mercury::Y3[1], &mercury::Y3[2]);
    let y4 = calculate_var(t, &mercury::Y4[0], &mercury::Y4[1], &mercury::Y4[2]);
    let y5 = calculate_var(t, &mercury::Y5[0], &mercury::Y5[1], &mercury::Y5[2]);

    let z0 = calculate_var(t, &mercury::Z0[0], &mercury::Z0[1], &mercury::Z0[2]);
    let z1 = calculate_var(t, &mercury::Z1[0], &mercury::Z1[1], &mercury::Z1[2]);
    let z2 = calculate_var(t, &mercury::Z2[0], &mercury::Z2[1], &mercury::Z2[2]);
    let z3 = calculate_var(t, &mercury::Z3[0], &mercury::Z3[1], &mercury::Z3[2]);
    let z4 = calculate_var(t, &mercury::Z4[0], &mercury::Z4[1], &mercury::Z4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let x = x0 + x1 * t + x2 * t2 + x3 * t3 + x4 * t4 + x5 * t5;
    let y = y0 + y1 * t + y2 * t2 + y3 * t3 + y4 * t4 + y5 * t5;
    let z = z0 + z1 * t + z2 * t2 + z3 * t3 + z4 * t4;

    RectangularCoordinates { x, y, z }
}

/// Calculates VSOP87A solution for Venus.
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Venus. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87A solution in a `RectangularCoordinates` structure.
/// Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in the center
/// and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Venus in the solar system using rectangular coordinates. In this case,
/// we calculate where Venus was in December 19th, 1099.
///
/// ```
/// use vsop87::vsop87a;
///
/// let coordinates = vsop87a::venus(2122820.0);
///
/// assert!(coordinates.x > -0.6660158466 && coordinates.x < -0.6660158464);
/// assert!(coordinates.y > -0.2753592312 && coordinates.y < -0.2753592310);
/// assert!(coordinates.z > 0.035785 && coordinates.z < 0.035789);
/// ```
pub fn venus(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &venus::X0[0], &venus::X0[1], &venus::X0[2]);
    let x1 = calculate_var(t, &venus::X1[0], &venus::X1[1], &venus::X1[2]);
    let x2 = calculate_var(t, &venus::X2[0], &venus::X2[1], &venus::X2[2]);
    let x3 = calculate_var(t, &venus::X3[0], &venus::X3[1], &venus::X3[2]);
    let x4 = calculate_var(t, &venus::X4[0], &venus::X4[1], &venus::X4[2]);
    let x5 = calculate_var(t, &venus::X5[0], &venus::X5[1], &venus::X5[2]);

    let y0 = calculate_var(t, &venus::Y0[0], &venus::Y0[1], &venus::Y0[2]);
    let y1 = calculate_var(t, &venus::Y1[0], &venus::Y1[1], &venus::Y1[2]);
    let y2 = calculate_var(t, &venus::Y2[0], &venus::Y2[1], &venus::Y2[2]);
    let y3 = calculate_var(t, &venus::Y3[0], &venus::Y3[1], &venus::Y3[2]);
    let y4 = calculate_var(t, &venus::Y4[0], &venus::Y4[1], &venus::Y4[2]);
    let y5 = calculate_var(t, &venus::Y5[0], &venus::Y5[1], &venus::Y5[2]);

    let z0 = calculate_var(t, &venus::Z0[0], &venus::Z0[1], &venus::Z0[2]);
    let z1 = calculate_var(t, &venus::Z1[0], &venus::Z1[1], &venus::Z1[2]);
    let z2 = calculate_var(t, &venus::Z2[0], &venus::Z2[1], &venus::Z2[2]);
    let z3 = calculate_var(t, &venus::Z3[0], &venus::Z3[1], &venus::Z3[2]);
    let z4 = calculate_var(t, &venus::Z4[0], &venus::Z4[1], &venus::Z4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let x = x0 + x1 * t + x2 * t2 + x3 * t3 + x4 * t4 + x5 * t5;
    let y = y0 + y1 * t + y2 * t2 + y3 * t3 + y4 * t4 + y5 * t5;
    let z = z0 + z1 * t + z2 * t2 + z3 * t3 + z4 * t4;

    RectangularCoordinates { x, y, z }
}

/// Calculates VSOP87A solution for Earth.
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Earth. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87A solution in a `RectangularCoordinates` structure.
/// Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in the center
/// and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Earth in the solar system using rectangular coordinates. In this case,
/// we calculate where the Earth was in December 29th, 1699.
///
/// ```
/// use vsop87::vsop87a;
///
/// let coordinates = vsop87a::earth(2341970.0);
///
/// assert!(coordinates.x > -0.2104654653 && coordinates.x < -0.2104654651);
/// assert!(coordinates.y > 0.9603579953 && coordinates.y < 0.9603579955);
/// assert!(coordinates.z > 0.000645 && coordinates.z < 0.000649);
/// ```
pub fn earth(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &earth::X0[0], &earth::X0[1], &earth::X0[2]);
    let x1 = calculate_var(t, &earth::X1[0], &earth::X1[1], &earth::X1[2]);
    let x2 = calculate_var(t, &earth::X2[0], &earth::X2[1], &earth::X2[2]);
    let x3 = calculate_var(t, &earth::X3[0], &earth::X3[1], &earth::X3[2]);
    let x4 = calculate_var(t, &earth::X4[0], &earth::X4[1], &earth::X4[2]);
    let x5 = calculate_var(t, &earth::X5[0], &earth::X5[1], &earth::X5[2]);

    let y0 = calculate_var(t, &earth::Y0[0], &earth::Y0[1], &earth::Y0[2]);
    let y1 = calculate_var(t, &earth::Y1[0], &earth::Y1[1], &earth::Y1[2]);
    let y2 = calculate_var(t, &earth::Y2[0], &earth::Y2[1], &earth::Y2[2]);
    let y3 = calculate_var(t, &earth::Y3[0], &earth::Y3[1], &earth::Y3[2]);
    let y4 = calculate_var(t, &earth::Y4[0], &earth::Y4[1], &earth::Y4[2]);
    let y5 = calculate_var(t, &earth::Y5[0], &earth::Y5[1], &earth::Y5[2]);

    let z0 = calculate_var(t, &earth::Z0[0], &earth::Z0[1], &earth::Z0[2]);
    let z1 = calculate_var(t, &earth::Z1[0], &earth::Z1[1], &earth::Z1[2]);
    let z2 = calculate_var(t, &earth::Z2[0], &earth::Z2[1], &earth::Z2[2]);
    let z3 = calculate_var(t, &earth::Z3[0], &earth::Z3[1], &earth::Z3[2]);
    let z4 = calculate_var(t, &earth::Z4[0], &earth::Z4[1], &earth::Z4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let x = x0 + x1 * t + x2 * t2 + x3 * t3 + x4 * t4 + x5 * t5;
    let y = y0 + y1 * t + y2 * t2 + y3 * t3 + y4 * t4 + y5 * t5;
    let z = z0 + z1 * t + z2 * t2 + z3 * t3 + z4 * t4;

    RectangularCoordinates { x, y, z }
}

/// Calculates VSOP87A solution for Earth - Moon barycenter.
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the Earth - Moon barycenter or center of masses. The parameter
/// needed is the Julian Day (*JD*) for the given date. It returns the VSOP87A solution in a
/// `RectangularCoordinates` structure. Those values are the rectangular coordinates of the planet,
/// in *AU*, with the Sun in the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the Earth - Moon barycenter in the solar system using rectangular coordinates. In
/// this case, we calculate where the barycenter was in December 19th, 1199.
///
/// ```
/// use vsop87::vsop87a;
///
/// let coordinates = vsop87a::earth_moon(2159345.0);
///
/// assert!(coordinates.x > -0.2654471687 && coordinates.x < -0.2654471685);
/// assert!(coordinates.y > 0.9464953235 && coordinates.y < 0.9464953237);
/// assert!(coordinates.z > 0.001703 && coordinates.z < 0.001707);
/// ```
pub fn earth_moon(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(
        t,
        &earth_moon::X0[0],
        &earth_moon::X0[1],
        &earth_moon::X0[2],
    );
    let x1 = calculate_var(
        t,
        &earth_moon::X1[0],
        &earth_moon::X1[1],
        &earth_moon::X1[2],
    );
    let x2 = calculate_var(
        t,
        &earth_moon::X2[0],
        &earth_moon::X2[1],
        &earth_moon::X2[2],
    );
    let x3 = calculate_var(
        t,
        &earth_moon::X3[0],
        &earth_moon::X3[1],
        &earth_moon::X3[2],
    );
    let x4 = calculate_var(
        t,
        &earth_moon::X4[0],
        &earth_moon::X4[1],
        &earth_moon::X4[2],
    );
    let x5 = calculate_var(
        t,
        &earth_moon::X5[0],
        &earth_moon::X5[1],
        &earth_moon::X5[2],
    );

    let y0 = calculate_var(
        t,
        &earth_moon::Y0[0],
        &earth_moon::Y0[1],
        &earth_moon::Y0[2],
    );
    let y1 = calculate_var(
        t,
        &earth_moon::Y1[0],
        &earth_moon::Y1[1],
        &earth_moon::Y1[2],
    );
    let y2 = calculate_var(
        t,
        &earth_moon::Y2[0],
        &earth_moon::Y2[1],
        &earth_moon::Y2[2],
    );
    let y3 = calculate_var(
        t,
        &earth_moon::Y3[0],
        &earth_moon::Y3[1],
        &earth_moon::Y3[2],
    );
    let y4 = calculate_var(
        t,
        &earth_moon::Y4[0],
        &earth_moon::Y4[1],
        &earth_moon::Y4[2],
    );
    let y5 = calculate_var(
        t,
        &earth_moon::Y5[0],
        &earth_moon::Y5[1],
        &earth_moon::Y5[2],
    );

    let z0 = calculate_var(
        t,
        &earth_moon::Z0[0],
        &earth_moon::Z0[1],
        &earth_moon::Z0[2],
    );
    let z1 = calculate_var(
        t,
        &earth_moon::Z1[0],
        &earth_moon::Z1[1],
        &earth_moon::Z1[2],
    );
    let z2 = calculate_var(
        t,
        &earth_moon::Z2[0],
        &earth_moon::Z2[1],
        &earth_moon::Z2[2],
    );
    let z3 = calculate_var(
        t,
        &earth_moon::Z3[0],
        &earth_moon::Z3[1],
        &earth_moon::Z3[2],
    );
    let z4 = calculate_var(
        t,
        &earth_moon::Z4[0],
        &earth_moon::Z4[1],
        &earth_moon::Z4[2],
    );

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let x = x0 + x1 * t + x2 * t2 + x3 * t3 + x4 * t4 + x5 * t5;
    let y = y0 + y1 * t + y2 * t2 + y3 * t3 + y4 * t4 + y5 * t5;
    let z = z0 + z1 * t + z2 * t2 + z3 * t3 + z4 * t4;

    RectangularCoordinates { x, y, z }
}

/// Calculates VSOP87A solution for Mars.
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Mars. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87A solution in a `RectangularCoordinates` structure. Those
/// values are the rectangular coordinates of the planet, in *AU*, with the Sun in the center and
/// the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Mars in the solar system using rectangular coordinates. In this case, we
/// calculate where Mars was in December 19th, 1399.
///
/// ```
/// use vsop87::vsop87a;
///
/// let coordinates = vsop87a::mars(2232395.0);
///
/// assert!(coordinates.x > 1.3910394545 && coordinates.x < 1.3910394547);
/// assert!(coordinates.y > -0.0543839268 && coordinates.y < -0.0543839266);
/// assert!(coordinates.z > -0.037103 && coordinates.z < -0.037099);
/// ```
pub fn mars(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &mars::X0[0], &mars::X0[1], &mars::X0[2]);
    let x1 = calculate_var(t, &mars::X1[0], &mars::X1[1], &mars::X1[2]);
    let x2 = calculate_var(t, &mars::X2[0], &mars::X2[1], &mars::X2[2]);
    let x3 = calculate_var(t, &mars::X3[0], &mars::X3[1], &mars::X3[2]);
    let x4 = calculate_var(t, &mars::X4[0], &mars::X4[1], &mars::X4[2]);
    let x5 = calculate_var(t, &mars::X5[0], &mars::X5[1], &mars::X5[2]);

    let y0 = calculate_var(t, &mars::Y0[0], &mars::Y0[1], &mars::Y0[2]);
    let y1 = calculate_var(t, &mars::Y1[0], &mars::Y1[1], &mars::Y1[2]);
    let y2 = calculate_var(t, &mars::Y2[0], &mars::Y2[1], &mars::Y2[2]);
    let y3 = calculate_var(t, &mars::Y3[0], &mars::Y3[1], &mars::Y3[2]);
    let y4 = calculate_var(t, &mars::Y4[0], &mars::Y4[1], &mars::Y4[2]);
    let y5 = calculate_var(t, &mars::Y5[0], &mars::Y5[1], &mars::Y5[2]);

    let z0 = calculate_var(t, &mars::Z0[0], &mars::Z0[1], &mars::Z0[2]);
    let z1 = calculate_var(t, &mars::Z1[0], &mars::Z1[1], &mars::Z1[2]);
    let z2 = calculate_var(t, &mars::Z2[0], &mars::Z2[1], &mars::Z2[2]);
    let z3 = calculate_var(t, &mars::Z3[0], &mars::Z3[1], &mars::Z3[2]);
    let z4 = calculate_var(t, &mars::Z4[0], &mars::Z4[1], &mars::Z4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let x = x0 + x1 * t + x2 * t2 + x3 * t3 + x4 * t4 + x5 * t5;
    let y = y0 + y1 * t + y2 * t2 + y3 * t3 + y4 * t4 + y5 * t5;
    let z = z0 + z1 * t + z2 * t2 + z3 * t3 + z4 * t4;

    RectangularCoordinates { x, y, z }
}

/// Calculates VSOP87A solution for Jupiter.
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Jupiter. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87A solution in a `RectangularCoordinates` structure.
/// Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in the center
/// and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Jupiter in the solar system using rectangular coordinates. In this case,
/// we calculate where Jupiter was in January 1st, 2000.
///
/// ```
/// use vsop87::vsop87a;
///
/// let coordinates = vsop87a::jupiter(2451545.0);
///
/// assert!(coordinates.x > 4.0011740267 && coordinates.x < 4.0011740269);
/// assert!(coordinates.y > 2.9385810076 && coordinates.y < 2.9385810078);
/// assert!(coordinates.z > -0.101786 && coordinates.z < -0.101782);
/// ```
pub fn jupiter(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &jupiter::X0[0], &jupiter::X0[1], &jupiter::X0[2]);
    let x1 = calculate_var(t, &jupiter::X1[0], &jupiter::X1[1], &jupiter::X1[2]);
    let x2 = calculate_var(t, &jupiter::X2[0], &jupiter::X2[1], &jupiter::X2[2]);
    let x3 = calculate_var(t, &jupiter::X3[0], &jupiter::X3[1], &jupiter::X3[2]);
    let x4 = calculate_var(t, &jupiter::X4[0], &jupiter::X4[1], &jupiter::X4[2]);
    let x5 = calculate_var(t, &jupiter::X5[0], &jupiter::X5[1], &jupiter::X5[2]);

    let y0 = calculate_var(t, &jupiter::Y0[0], &jupiter::Y0[1], &jupiter::Y0[2]);
    let y1 = calculate_var(t, &jupiter::Y1[0], &jupiter::Y1[1], &jupiter::Y1[2]);
    let y2 = calculate_var(t, &jupiter::Y2[0], &jupiter::Y2[1], &jupiter::Y2[2]);
    let y3 = calculate_var(t, &jupiter::Y3[0], &jupiter::Y3[1], &jupiter::Y3[2]);
    let y4 = calculate_var(t, &jupiter::Y4[0], &jupiter::Y4[1], &jupiter::Y4[2]);
    let y5 = calculate_var(t, &jupiter::Y5[0], &jupiter::Y5[1], &jupiter::Y5[2]);

    let z0 = calculate_var(t, &jupiter::Z0[0], &jupiter::Z0[1], &jupiter::Z0[2]);
    let z1 = calculate_var(t, &jupiter::Z1[0], &jupiter::Z1[1], &jupiter::Z1[2]);
    let z2 = calculate_var(t, &jupiter::Z2[0], &jupiter::Z2[1], &jupiter::Z2[2]);
    let z3 = calculate_var(t, &jupiter::Z3[0], &jupiter::Z3[1], &jupiter::Z3[2]);
    let z4 = calculate_var(t, &jupiter::Z4[0], &jupiter::Z4[1], &jupiter::Z4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let x = x0 + x1 * t + x2 * t2 + x3 * t3 + x4 * t4 + x5 * t5;
    let y = y0 + y1 * t + y2 * t2 + y3 * t3 + y4 * t4 + y5 * t5;
    let z = z0 + z1 * t + z2 * t2 + z3 * t3 + z4 * t4;

    RectangularCoordinates { x, y, z }
}

/// Calculates VSOP87A solution for Saturn.
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Saturn. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87A solution in a `RectangularCoordinates` structure.
/// Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in the center
/// and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Saturn in the solar system using rectangular coordinates. In this case,
/// we calculate where Saturn was in December 19th, 1099.
///
/// ```
/// use vsop87::vsop87a;
///
/// let coordinates = vsop87a::saturn(2122820.0);
///
/// assert!(coordinates.x > -7.9395559174 && coordinates.x < -7.9395559172);
/// assert!(coordinates.y > -5.8435867017 && coordinates.y < -5.8435867015);
/// assert!(coordinates.z > 0.416558 && coordinates.z < 0.416562);
/// ```
pub fn saturn(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &saturn::X0[0], &saturn::X0[1], &saturn::X0[2]);
    let x1 = calculate_var(t, &saturn::X1[0], &saturn::X1[1], &saturn::X1[2]);
    let x2 = calculate_var(t, &saturn::X2[0], &saturn::X2[1], &saturn::X2[2]);
    let x3 = calculate_var(t, &saturn::X3[0], &saturn::X3[1], &saturn::X3[2]);
    let x4 = calculate_var(t, &saturn::X4[0], &saturn::X4[1], &saturn::X4[2]);
    let x5 = calculate_var(t, &saturn::X5[0], &saturn::X5[1], &saturn::X5[2]);

    let y0 = calculate_var(t, &saturn::Y0[0], &saturn::Y0[1], &saturn::Y0[2]);
    let y1 = calculate_var(t, &saturn::Y1[0], &saturn::Y1[1], &saturn::Y1[2]);
    let y2 = calculate_var(t, &saturn::Y2[0], &saturn::Y2[1], &saturn::Y2[2]);
    let y3 = calculate_var(t, &saturn::Y3[0], &saturn::Y3[1], &saturn::Y3[2]);
    let y4 = calculate_var(t, &saturn::Y4[0], &saturn::Y4[1], &saturn::Y4[2]);
    let y5 = calculate_var(t, &saturn::Y5[0], &saturn::Y5[1], &saturn::Y5[2]);

    let z0 = calculate_var(t, &saturn::Z0[0], &saturn::Z0[1], &saturn::Z0[2]);
    let z1 = calculate_var(t, &saturn::Z1[0], &saturn::Z1[1], &saturn::Z1[2]);
    let z2 = calculate_var(t, &saturn::Z2[0], &saturn::Z2[1], &saturn::Z2[2]);
    let z3 = calculate_var(t, &saturn::Z3[0], &saturn::Z3[1], &saturn::Z3[2]);
    let z4 = calculate_var(t, &saturn::Z4[0], &saturn::Z4[1], &saturn::Z4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;
    let t5 = t.powi(5);

    let x = x0 + x1 * t + x2 * t2 + x3 * t3 + x4 * t4 + x5 * t5;
    let y = y0 + y1 * t + y2 * t2 + y3 * t3 + y4 * t4 + y5 * t5;
    let z = z0 + z1 * t + z2 * t2 + z3 * t3 + z4 * t4;

    RectangularCoordinates { x, y, z }
}

/// Calculates VSOP87A solution for Uranus.
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Uranus. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87A solution in a `RectangularCoordinates` structure.
/// Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in the center
/// and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Uranus in the solar system using rectangular coordinates. In this case,
/// we calculate where Uranus was in December 19th, 1199.
///
/// ```
/// use vsop87::vsop87a;
///
/// let coordinates = vsop87a::uranus(2159345.0);
///
/// assert!(coordinates.x > -9.8287104598 && coordinates.x < -9.8287104596);
/// assert!(coordinates.y > 15.7711888604 && coordinates.y < 15.7711888606);
/// assert!(coordinates.z > 0.191480 && coordinates.z < 0.191484);
/// ```
pub fn uranus(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &uranus::X0[0], &uranus::X0[1], &uranus::X0[2]);
    let x1 = calculate_var(t, &uranus::X1[0], &uranus::X1[1], &uranus::X1[2]);
    let x2 = calculate_var(t, &uranus::X2[0], &uranus::X2[1], &uranus::X2[2]);
    let x3 = calculate_var(t, &uranus::X3[0], &uranus::X3[1], &uranus::X3[2]);
    let x4 = calculate_var(t, &uranus::X4[0], &uranus::X4[1], &uranus::X4[2]);

    let y0 = calculate_var(t, &uranus::Y0[0], &uranus::Y0[1], &uranus::Y0[2]);
    let y1 = calculate_var(t, &uranus::Y1[0], &uranus::Y1[1], &uranus::Y1[2]);
    let y2 = calculate_var(t, &uranus::Y2[0], &uranus::Y2[1], &uranus::Y2[2]);
    let y3 = calculate_var(t, &uranus::Y3[0], &uranus::Y3[1], &uranus::Y3[2]);
    let y4 = calculate_var(t, &uranus::Y4[0], &uranus::Y4[1], &uranus::Y4[2]);

    let z0 = calculate_var(t, &uranus::Z0[0], &uranus::Z0[1], &uranus::Z0[2]);
    let z1 = calculate_var(t, &uranus::Z1[0], &uranus::Z1[1], &uranus::Z1[2]);
    let z2 = calculate_var(t, &uranus::Z2[0], &uranus::Z2[1], &uranus::Z2[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;

    let x = x0 + x1 * t + x2 * t2 + x3 * t3 + x4 * t4;
    let y = y0 + y1 * t + y2 * t2 + y3 * t3 + y4 * t4;
    let z = z0 + z1 * t + z2 * t2;

    RectangularCoordinates { x, y, z }
}

/// Calculates VSOP87A solution for Neptune
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Neptune. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87A solution in a `RectangularCoordinates` structure.
/// Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in the center
/// and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Neptune in the solar system using rectangular coordinates. In this case,
/// we calculate where Neptune was in December 19th, 1299.
///
/// ```
/// use vsop87::vsop87a;
///
/// let coordinates = vsop87a::neptune(2195870.0);
///
/// assert!(coordinates.x > -24.6234347579 && coordinates.x < -24.6234347577);
/// assert!(coordinates.y > -17.6514428047 && coordinates.y < -17.6514428045);
/// assert!(coordinates.z > 0.929722 && coordinates.z < 0.929726);
/// ```
pub fn neptune(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &neptune::X0[0], &neptune::X0[1], &neptune::X0[2]);
    let x1 = calculate_var(t, &neptune::X1[0], &neptune::X1[1], &neptune::X1[2]);
    let x2 = calculate_var(t, &neptune::X2[0], &neptune::X2[1], &neptune::X2[2]);
    let x3 = calculate_var(t, &neptune::X3[0], &neptune::X3[1], &neptune::X3[2]);
    let x4 = calculate_var(t, &neptune::X4[0], &neptune::X4[1], &neptune::X4[2]);

    let y0 = calculate_var(t, &neptune::Y0[0], &neptune::Y0[1], &neptune::Y0[2]);
    let y1 = calculate_var(t, &neptune::Y1[0], &neptune::Y1[1], &neptune::Y1[2]);
    let y2 = calculate_var(t, &neptune::Y2[0], &neptune::Y2[1], &neptune::Y2[2]);
    let y3 = calculate_var(t, &neptune::Y3[0], &neptune::Y3[1], &neptune::Y3[2]);
    let y4 = calculate_var(t, &neptune::Y4[0], &neptune::Y4[1], &neptune::Y4[2]);

    let z0 = calculate_var(t, &neptune::Z0[0], &neptune::Z0[1], &neptune::Z0[2]);
    let z1 = calculate_var(t, &neptune::Z1[0], &neptune::Z1[1], &neptune::Z1[2]);
    let z2 = calculate_var(t, &neptune::Z2[0], &neptune::Z2[1], &neptune::Z2[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t.powi(3);
    let t4 = t2 * t2;

    let x = x0 + x1 * t + x2 * t2 + x3 * t3 + x4 * t4;
    let y = y0 + y1 * t + y2 * t2 + y3 * t3 + y4 * t4;
    let z = z0 + z1 * t + z2 * t2;

    RectangularCoordinates { x, y, z }
}
