//! VSOP87E algorithm: Barycentric ecliptic rectangular coordinates for the equinox J2000.0.
//!
//! This module calculates barycentric ecliptic rectangular coordinates for the equinox J2000.0
//! for the planets in the solar system. These coordinates are centered in the
//! [barycenter](https://en.wikipedia.org/wiki/Barycenter) or center of masses of the solar system.
//! This means that the Sun is not at (0, 0, 0), and its position has to be calculated
//! independently.
//!
//! # Example
//!
//! Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
//! position of the Sun in the solar system using rectangular coordinates. In this case, we
//! calculate where the Sun was in January 1st, 2000.
//!
//! ```
//! use vsop87::vsop87e;
//!
//! let coordinates = vsop87e::sun(2451545.0);
//!
//! assert!(coordinates.x > -0.0071415280 && coordinates.x < -0.0071415278);
//! assert!(coordinates.y > -0.0027881716 && coordinates.y < -0.0027881714);
//! assert!(coordinates.z > 0.0002041 && coordinates.z < 0.0002081);

mod earth;
mod jupiter;
mod mars;
mod mercury;
mod neptune;
mod saturn;
mod sun;
mod uranus;
mod venus;

use super::{calculate_t, calculate_var, RectangularCoordinates};

#[cfg(feature = "no_std")]
use core::num::Float;

/// Calculates VSOP87E solution for the Sun.
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the Sun. The parameter needed is the Julian Day (*JD*) for the given
/// date. It returns the VSOP87E solution in a `RectangularCoordinates` structure. Those values are
/// the rectangular coordinates of the Sun, in *AU*, with the barycenter of the solar system in the
/// center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the Sun in the solar system using rectangular coordinates. In this case, we
/// calculate where the Sun was in January 1st, 2000.
///
/// ```
/// use vsop87::vsop87e;
///
/// let coordinates = vsop87e::sun(2451545.0);
///
/// assert!(coordinates.x > -0.0071415280 && coordinates.x < -0.0071415278);
/// assert!(coordinates.y > -0.0027881716 && coordinates.y < -0.0027881714);
/// assert!(coordinates.z > 0.0002041 && coordinates.z < 0.0002081);
/// ```
pub fn sun(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &sun::X0);
    let x1 = calculate_var(t, &sun::X1);
    let x2 = calculate_var(t, &sun::X2);
    let x3 = calculate_var(t, &sun::X3);
    let x4 = calculate_var(t, &sun::X4);
    let x5 = calculate_var(t, &sun::X5);

    let y0 = calculate_var(t, &sun::Y0);
    let y1 = calculate_var(t, &sun::Y1);
    let y2 = calculate_var(t, &sun::Y2);
    let y3 = calculate_var(t, &sun::Y3);
    let y4 = calculate_var(t, &sun::Y4);
    let y5 = calculate_var(t, &sun::Y5);

    let z0 = calculate_var(t, &sun::Z0);
    let z1 = calculate_var(t, &sun::Z1);
    let z2 = calculate_var(t, &sun::Z2);
    let z3 = calculate_var(t, &sun::Z3);
    let z4 = calculate_var(t, &sun::Z4);

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4) + x5 * t.powi(5);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4) + y5 * t.powi(5);
    let z = z0 + z1 * t + z2 * t * t + z3 * t.powi(3) + z4 * t.powi(4);

    RectangularCoordinates { x: x, y: y, z: z }
}

/// Calculates VSOP87E solution for Mercury.
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Mercury. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87E solution in a `RectangularCoordinates` structure. Those
/// values are the rectangular coordinates of the planet, in *AU*, with the barycenter of the solar
/// system in the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Mercury in the solar system using rectangular coordinates. In this case,
/// we calculate where Mercury was in December 31st, 1899.
///
/// ```
/// use vsop87::vsop87e;
///
/// let coordinates = vsop87e::mercury(2415020.0);
///
/// assert!(coordinates.x > -0.3865370328 && coordinates.x < -0.3865370326);
/// assert!(coordinates.y > -0.1438666202 && coordinates.y < -0.1438666200);
/// assert!(coordinates.z > 0.0235142 && coordinates.z < 0.0235182);
/// ```
pub fn mercury(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &mercury::X0);
    let x1 = calculate_var(t, &mercury::X1);
    let x2 = calculate_var(t, &mercury::X2);
    let x3 = calculate_var(t, &mercury::X3);
    let x4 = calculate_var(t, &mercury::X4);
    let x5 = calculate_var(t, &mercury::X5);

    let y0 = calculate_var(t, &mercury::Y0);
    let y1 = calculate_var(t, &mercury::Y1);
    let y2 = calculate_var(t, &mercury::Y2);
    let y3 = calculate_var(t, &mercury::Y3);
    let y4 = calculate_var(t, &mercury::Y4);
    let y5 = calculate_var(t, &mercury::Y5);

    let z0 = calculate_var(t, &mercury::Z0);
    let z1 = calculate_var(t, &mercury::Z1);
    let z2 = calculate_var(t, &mercury::Z2);
    let z3 = calculate_var(t, &mercury::Z3);
    let z4 = calculate_var(t, &mercury::Z4);

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4) + x5 * t.powi(5);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4) + y5 * t.powi(5);
    let z = z0 + z1 * t + z2 * t * t + z3 * t.powi(3) + z4 * t.powi(4);

    RectangularCoordinates { x: x, y: y, z: z }
}

/// Calculates VSOP87E solution for Venus.
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Venus. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87E solution in a `RectangularCoordinates` structure. Those
/// values are the rectangular coordinates of the planet, in *AU*, with the barycenter of the solar
/// system in the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Venus in the solar system using rectangular coordinates. In this case,
/// we calculate where Venus was in December 30th, 1799.
///
/// ```
/// use vsop87::vsop87e;
///
/// let coordinates = vsop87e::venus(2378495.0);
///
/// assert!(coordinates.x > -0.5948645228 && coordinates.x < -0.5948645226);
/// assert!(coordinates.y > 0.3900421674 && coordinates.y < 0.3900421676);
/// assert!(coordinates.z > 0.0397542 && coordinates.z < 0.0397582);
/// ```
pub fn venus(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &venus::X0);
    let x1 = calculate_var(t, &venus::X1);
    let x2 = calculate_var(t, &venus::X2);
    let x3 = calculate_var(t, &venus::X3);
    let x4 = calculate_var(t, &venus::X4);
    let x5 = calculate_var(t, &venus::X5);

    let y0 = calculate_var(t, &venus::Y0);
    let y1 = calculate_var(t, &venus::Y1);
    let y2 = calculate_var(t, &venus::Y2);
    let y3 = calculate_var(t, &venus::Y3);
    let y4 = calculate_var(t, &venus::Y4);
    let y5 = calculate_var(t, &venus::Y5);

    let z0 = calculate_var(t, &venus::Z0);
    let z1 = calculate_var(t, &venus::Z1);
    let z2 = calculate_var(t, &venus::Z2);
    let z3 = calculate_var(t, &venus::Z3);
    let z4 = calculate_var(t, &venus::Z4);

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4) + x5 * t.powi(5);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4) + y5 * t.powi(5);
    let z = z0 + z1 * t + z2 * t * t + z3 * t.powi(3) + z4 * t.powi(4);

    RectangularCoordinates { x: x, y: y, z: z }
}

/// Calculates VSOP87E solution for Earth.
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Earth. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87E solution in a `RectangularCoordinates` structure. Those
/// values are the rectangular coordinates of the planet, in *AU*, with the barycenter of the solar
/// system in the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Earth in the solar system using rectangular coordinates. In this case,
/// we calculate where the Earth was in December 29th, 1699.
///
/// ```
/// use vsop87::vsop87e;
///
/// let coordinates = vsop87e::earth(2341970.0);
///
/// assert!(coordinates.x > -0.2155959338 && coordinates.x < -0.2155959336);
/// assert!(coordinates.y > 0.9651943804 && coordinates.y < 0.9651943806);
/// assert!(coordinates.z > 0.0007705 && coordinates.z < 0.0007745);
/// ```
pub fn earth(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &earth::X0);
    let x1 = calculate_var(t, &earth::X1);
    let x2 = calculate_var(t, &earth::X2);
    let x3 = calculate_var(t, &earth::X3);
    let x4 = calculate_var(t, &earth::X4);
    let x5 = calculate_var(t, &earth::X5);

    let y0 = calculate_var(t, &earth::Y0);
    let y1 = calculate_var(t, &earth::Y1);
    let y2 = calculate_var(t, &earth::Y2);
    let y3 = calculate_var(t, &earth::Y3);
    let y4 = calculate_var(t, &earth::Y4);
    let y5 = calculate_var(t, &earth::Y5);

    let z0 = calculate_var(t, &earth::Z0);
    let z1 = calculate_var(t, &earth::Z1);
    let z2 = calculate_var(t, &earth::Z2);
    let z3 = calculate_var(t, &earth::Z3);
    let z4 = calculate_var(t, &earth::Z4);

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4) + x5 * t.powi(5);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4) + y5 * t.powi(5);
    let z = z0 + z1 * t + z2 * t * t + z3 * t.powi(3) + z4 * t.powi(4);

    RectangularCoordinates { x: x, y: y, z: z }
}

/// Calculates VSOP87E solution for Mars.
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Mars. The parameter needed is the Julian Day (*JD*) for the
/// given date. It returns the VSOP87E solution in a `RectangularCoordinates` structure. Those
/// values are the rectangular coordinates of the planet, in *AU*, with the barycenter of the solar
/// system in the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Mars in the solar system using rectangular coordinates. In this case, we
/// calculate where Mercury was in December 29th, 1599.
///
/// ```
/// use vsop87::vsop87e;
///
/// let coordinates = vsop87e::mars(2305445.0);
///
/// assert!(coordinates.x > -0.8237565239 && coordinates.x < -0.8237565237);
/// assert!(coordinates.y > 1.4065798250 && coordinates.y < 1.4065798252);
/// assert!(coordinates.z > 0.0502476 && coordinates.z < 0.0502516);
/// ```
pub fn mars(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &mars::X0);
    let x1 = calculate_var(t, &mars::X1);
    let x2 = calculate_var(t, &mars::X2);
    let x3 = calculate_var(t, &mars::X3);
    let x4 = calculate_var(t, &mars::X4);
    let x5 = calculate_var(t, &mars::X5);

    let y0 = calculate_var(t, &mars::Y0);
    let y1 = calculate_var(t, &mars::Y1);
    let y2 = calculate_var(t, &mars::Y2);
    let y3 = calculate_var(t, &mars::Y3);
    let y4 = calculate_var(t, &mars::Y4);
    let y5 = calculate_var(t, &mars::Y5);

    let z0 = calculate_var(t, &mars::Z0);
    let z1 = calculate_var(t, &mars::Z1);
    let z2 = calculate_var(t, &mars::Z2);
    let z3 = calculate_var(t, &mars::Z3);
    let z4 = calculate_var(t, &mars::Z4);

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4) + x5 * t.powi(5);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4) + y5 * t.powi(5);
    let z = z0 + z1 * t + z2 * t * t + z3 * t.powi(3) + z4 * t.powi(4);

    RectangularCoordinates { x: x, y: y, z: z }
}

/// Calculates VSOP87E solution for Jupiter.
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Jupiter. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87E solution in a `RectangularCoordinates` structure. Those
/// values are the rectangular coordinates of the planet, in *AU*, with the barycenter of the solar
/// system in the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Jupiter in the solar system using rectangular coordinates. In this case,
/// we calculate where Jupiter was in December 19th, 1499.
///
/// ```
/// use vsop87::vsop87e;
///
/// let coordinates = vsop87e::jupiter(2268920.0);
///
/// assert!(coordinates.x > 4.5819830418 && coordinates.x < 4.5819830420);
/// assert!(coordinates.y > -1.9854861384 && coordinates.y < -1.9854861382);
/// assert!(coordinates.z > -0.0959289 && coordinates.z < -0.0959249);
/// ```
pub fn jupiter(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &jupiter::X0);
    let x1 = calculate_var(t, &jupiter::X1);
    let x2 = calculate_var(t, &jupiter::X2);
    let x3 = calculate_var(t, &jupiter::X3);
    let x4 = calculate_var(t, &jupiter::X4);
    let x5 = calculate_var(t, &jupiter::X5);

    let y0 = calculate_var(t, &jupiter::Y0);
    let y1 = calculate_var(t, &jupiter::Y1);
    let y2 = calculate_var(t, &jupiter::Y2);
    let y3 = calculate_var(t, &jupiter::Y3);
    let y4 = calculate_var(t, &jupiter::Y4);
    let y5 = calculate_var(t, &jupiter::Y5);

    let z0 = calculate_var(t, &jupiter::Z0);
    let z1 = calculate_var(t, &jupiter::Z1);
    let z2 = calculate_var(t, &jupiter::Z2);
    let z3 = calculate_var(t, &jupiter::Z3);
    let z4 = calculate_var(t, &jupiter::Z4);

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4) + x5 * t.powi(5);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4) + y5 * t.powi(5);
    let z = z0 + z1 * t + z2 * t * t + z3 * t.powi(3) + z4 * t.powi(4);

    RectangularCoordinates { x: x, y: y, z: z }
}

/// Calculates VSOP87E solution for Saturn.
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Saturn. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87E solution in a `RectangularCoordinates` structure. Those
/// values are the rectangular coordinates of the planet, in *AU*, with the barycenter of the solar
/// system in the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Saturn in the solar system using rectangular coordinates. In this case,
/// we calculate where Saturn was in December 19th, 1399.
///
/// ```
/// use vsop87::vsop87e;
///
/// let coordinates = vsop87e::saturn(2232395.0);
///
/// assert!(coordinates.x > 1.2645936161 && coordinates.x < 1.2645936163);
/// assert!(coordinates.y > -10.0240954526 && coordinates.y < -10.0240954524);
/// assert!(coordinates.z > 0.1345888 && coordinates.z < 0.1345928);
/// ```
pub fn saturn(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &saturn::X0);
    let x1 = calculate_var(t, &saturn::X1);
    let x2 = calculate_var(t, &saturn::X2);
    let x3 = calculate_var(t, &saturn::X3);
    let x4 = calculate_var(t, &saturn::X4);
    let x5 = calculate_var(t, &saturn::X5);

    let y0 = calculate_var(t, &saturn::Y0);
    let y1 = calculate_var(t, &saturn::Y1);
    let y2 = calculate_var(t, &saturn::Y2);
    let y3 = calculate_var(t, &saturn::Y3);
    let y4 = calculate_var(t, &saturn::Y4);
    let y5 = calculate_var(t, &saturn::Y5);

    let z0 = calculate_var(t, &saturn::Z0);
    let z1 = calculate_var(t, &saturn::Z1);
    let z2 = calculate_var(t, &saturn::Z2);
    let z3 = calculate_var(t, &saturn::Z3);
    let z4 = calculate_var(t, &saturn::Z4);

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4) + x5 * t.powi(5);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4) + y5 * t.powi(5);
    let z = z0 + z1 * t + z2 * t * t + z3 * t.powi(3) + z4 * t.powi(4);

    RectangularCoordinates { x: x, y: y, z: z }
}

/// Calculates VSOP87E solution for Uranus.
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Uranus. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87E solution in a `RectangularCoordinates` structure. Those
/// values are the rectangular coordinates of the planet, in *AU*, with the barycenter of the solar
/// system in the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Uranus in the solar system using rectangular coordinates. In this case,
/// we calculate where Neptune was in December 19th, 1299.
///
/// ```
/// use vsop87::vsop87e;
///
/// let coordinates = vsop87e::uranus(2195870.0);
///
/// assert!(coordinates.x > -17.6538791198 && coordinates.x < -17.6538791196);
/// assert!(coordinates.y > -5.1666300881 && coordinates.y < -5.1666300879);
/// assert!(coordinates.z > 0.2124594 && coordinates.z < 0.2124634);
/// ```
pub fn uranus(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &uranus::X0);
    let x1 = calculate_var(t, &uranus::X1);
    let x2 = calculate_var(t, &uranus::X2);
    let x3 = calculate_var(t, &uranus::X3);
    let x4 = calculate_var(t, &uranus::X4);

    let y0 = calculate_var(t, &uranus::Y0);
    let y1 = calculate_var(t, &uranus::Y1);
    let y2 = calculate_var(t, &uranus::Y2);
    let y3 = calculate_var(t, &uranus::Y3);
    let y4 = calculate_var(t, &uranus::Y4);

    let z0 = calculate_var(t, &uranus::Z0);
    let z1 = calculate_var(t, &uranus::Z1);
    let z2 = calculate_var(t, &uranus::Z2);

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4);
    let z = z0 + z1 * t + z2 * t * t;

    RectangularCoordinates { x: x, y: y, z: z }
}

/// Calculates VSOP87E solution for Neptune.
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Neptune. The parameter needed is the Julian Day (*JD*) for
/// the given date. It returns the VSOP87E solution in a `RectangularCoordinates` structure. Those
/// values are the rectangular coordinates of the planet, in *AU*, with the barycenter of the solar
/// system in the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Neptune in the solar system using rectangular coordinates. In this case,
/// we calculate where Neptune was in December 19th, 1199.
///
/// ```
/// use vsop87::vsop87e;
///
/// let coordinates = vsop87e::neptune(2159345.0);
///
/// assert!(coordinates.x > 29.8297729718 && coordinates.x < 29.8297729720);
/// assert!(coordinates.y > -2.0298541973 && coordinates.y < -2.0298541971);
/// assert!(coordinates.z > -0.6440972 && coordinates.z < -0.6440932);
/// ```
pub fn neptune(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &neptune::X0);
    let x1 = calculate_var(t, &neptune::X1);
    let x2 = calculate_var(t, &neptune::X2);
    let x3 = calculate_var(t, &neptune::X3);
    let x4 = calculate_var(t, &neptune::X4);

    let y0 = calculate_var(t, &neptune::Y0);
    let y1 = calculate_var(t, &neptune::Y1);
    let y2 = calculate_var(t, &neptune::Y2);
    let y3 = calculate_var(t, &neptune::Y3);
    let y4 = calculate_var(t, &neptune::Y4);

    let z0 = calculate_var(t, &neptune::Z0);
    let z1 = calculate_var(t, &neptune::Z1);
    let z2 = calculate_var(t, &neptune::Z2);

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4);
    let z = z0 + z1 * t + z2 * t * t;

    RectangularCoordinates { x: x, y: y, z: z }
}
