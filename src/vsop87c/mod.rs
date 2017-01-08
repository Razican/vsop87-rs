//! VSOP87C algorithm: Heliocentric ecliptic rectangular coordinates for the equinox of the day.
//!
//! This module contains the functions to calculate heliocentric ecliptic rectangular coordinates
//! for the equinox of the day for the planets in the solar system. The most useful when converting
//! to geocentric positions and later compute e.g. rise/set/culmination times, or the altitude and
//! azimuth relative to your local horizon.
//!
//! # Example
//!
//! Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
//! position of the planet Venus in the solar system using rectangular coordinates. In this case,
//! we calculate where Venus was in December 31st, 1899.
//!
//! ```
//! use vsop87::vsop87c;
//!
//! let coordinates = vsop87c::venus(2415020.0);
//!
//! assert!(coordinates.x > 0.6919778853 && coordinates.x < 0.6919778855);
//! assert!(coordinates.y > -0.2203045664 && coordinates.y < -0.2203045662);
//! assert!(coordinates.z > -0.04298775 && coordinates.z < -0.04298715);

mod mercury;
mod venus;
mod earth;
mod mars;
mod jupiter;
mod saturn;
mod uranus;
mod neptune;

use super::{calculate_t, calculate_var, RectangularCoordinates};

/// Calculates VSOP87C solution for Mercury.
///
/// This function calculates the VSOP87C solution (heliocentric ecliptic rectangular coordinates
/// for the equinox of the day) for the planet Mercury. The parameter needed is the Julian Day
/// (*JD*) for the given date. It returns the VSOP87C solution in a `RectangularCoordinates`
/// structure. Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in
/// the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Mercury in the solar system using rectangular coordinates. In this case,
/// we calculate where Mercury was in January 1st, 2000.
///
/// ```
/// use vsop87::vsop87c;
///
/// let coordinates = vsop87c::mercury(2451545.0);
///
/// assert!(coordinates.x > -0.1300934113 && coordinates.x < -0.1300934111);
/// assert!(coordinates.y > -0.4472876718 && coordinates.y < -0.4472876716);
/// assert!(coordinates.z > -0.02459868 && coordinates.z < -0.02459808);
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

/// Calculates VSOP87C solution for Venus.
///
/// This function calculates the VSOP87C solution (heliocentric ecliptic rectangular coordinates
/// for the equinox of the day) for the planet Venus. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87C solution in a `RectangularCoordinates` structure.
/// Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in the center
/// and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Venus in the solar system using rectangular coordinates. In this case,
/// we calculate where Venus was in December 31st, 1899.
///
/// ```
/// use vsop87::vsop87c;
///
/// let coordinates = vsop87c::venus(2415020.0);
///
/// assert!(coordinates.x > 0.6919778853 && coordinates.x < 0.6919778855);
/// assert!(coordinates.y > -0.2203045664 && coordinates.y < -0.2203045662);
/// assert!(coordinates.z > -0.04298775 && coordinates.z < -0.04298715);
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

/// Calculates VSOP87C solution for Earth.
///
/// This function calculates the VSOP87C solution (heliocentric ecliptic rectangular coordinates
/// for the equinox of the day) for the planet Earth. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87C solution in a `RectangularCoordinates` structure.
/// Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in the center
/// and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Earth in the solar system using rectangular coordinates. In this case,
/// we calculate where the Earth was in December 30th, 1799.
///
/// ```
/// use vsop87::vsop87c;
///
/// let coordinates = vsop87c::earth(2378495.0);
///
/// assert!(coordinates.x > -0.1522449492 && coordinates.x < -0.1522449490);
/// assert!(coordinates.y > 0.9713689618 && coordinates.y < 0.9713689620);
/// assert!(coordinates.z > -0.00000010 && coordinates.z < 0.00000050);
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

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4) + x5 * t.powi(5);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4) + y5 * t.powi(5);
    let z = z0 + z1 * t + z2 * t * t + z3 * t.powi(3);

    RectangularCoordinates { x: x, y: y, z: z }
}

/// Calculates VSOP87C solution for Mars.
///
/// This function calculates the VSOP87C solution (heliocentric ecliptic rectangular coordinates
/// for the equinox of the day) for the planet Mars. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the VSOP87C solution in a `RectangularCoordinates` structure.
/// Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in the center
/// and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Mars in the solar system using rectangular coordinates. In this case, we
/// calculate where Mars was in December 29th, 1699.
///
/// ```
/// use vsop87::vsop87c;
///
/// let coordinates = vsop87c::mars(2341970.0);
///
/// assert!(coordinates.x > -1.6160583004 && coordinates.x < -1.6160583002);
/// assert!(coordinates.y > 0.3697531113 && coordinates.y < 0.3697531115);
/// assert!(coordinates.z > 0.04647523 && coordinates.z < 0.04647583);
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

/// Calculates VSOP87C solution for Jupiter.
///
/// This function calculates the VSOP87C solution (heliocentric ecliptic rectangular coordinates
/// for the equinox of the day) for the planet Jupiter. The parameter needed is the Julian Day
/// (*JD*) for the given date. It returns the VSOP87C solution in a `RectangularCoordinates`
/// structure. Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in
/// the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Jupiter in the solar system using rectangular coordinates. In this case,
/// we calculate where Jupiter was in December 29th, 1599.
///
/// ```
/// use vsop87::vsop87c;
///
/// let coordinates = vsop87c::jupiter(2305445.0);
///
/// assert!(coordinates.x > -3.6969935265 && coordinates.x < -3.6969935263);
/// assert!(coordinates.y > 3.8580245749 && coordinates.y < 3.8580245751);
/// assert!(coordinates.z > 0.07509245 && coordinates.z < 0.07509305);
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

/// Calculates VSOP87C solution for Saturn.
///
/// This function calculates the VSOP87C solution (heliocentric ecliptic rectangular coordinates
/// for the equinox of the day) for the planet Saturn. The parameter needed is the Julian Day
/// (*JD*) for the given date. It returns the VSOP87C solution in a `RectangularCoordinates`
/// structure. Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in
/// the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Saturn in the solar system using rectangular coordinates. In this case,
/// we calculate where Saturn was in December 19th, 1499.
///
/// ```
/// use vsop87::vsop87c;
///
/// let coordinates = vsop87c::saturn(2268920.0);
///
/// assert!(coordinates.x > 5.9153005787 && coordinates.x < 5.9153005789);
/// assert!(coordinates.y > 6.8629464079 && coordinates.y < 6.8629464081);
/// assert!(coordinates.z > -0.34387152 && coordinates.z < -0.34387092);
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

/// Calculates VSOP87C solution for Uranus.
///
/// This function calculates the VSOP87C solution (heliocentric ecliptic rectangular coordinates
/// for the equinox of the day) for the planet Uranus. The parameter needed is the Julian Day
/// (*JD*) for the given date. It returns the VSOP87C solution in a `RectangularCoordinates`
/// structure. Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in
/// the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Uranus in the solar system using rectangular coordinates. In this case,
/// we calculate where Uranus was in December 19th, 1399.
///
/// ```
/// use vsop87::vsop87c;
///
/// let coordinates = vsop87c::uranus(2232395.0);
///
/// assert!(coordinates.x > -3.5812895194 && coordinates.x < -3.5812895192);
/// assert!(coordinates.y > -18.9336732632 && coordinates.y < -18.9336732630);
/// assert!(coordinates.z > -0.03719665 && coordinates.z < -0.03719605);
/// ```
pub fn uranus(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &uranus::X0);
    let x1 = calculate_var(t, &uranus::X1);
    let x2 = calculate_var(t, &uranus::X2);
    let x3 = calculate_var(t, &uranus::X3);
    let x4 = calculate_var(t, &uranus::X4);
    let x5 = calculate_var(t, &uranus::X5);

    let y0 = calculate_var(t, &uranus::Y0);
    let y1 = calculate_var(t, &uranus::Y1);
    let y2 = calculate_var(t, &uranus::Y2);
    let y3 = calculate_var(t, &uranus::Y3);
    let y4 = calculate_var(t, &uranus::Y4);
    let y5 = calculate_var(t, &uranus::Y5);

    let z0 = calculate_var(t, &uranus::Z0);
    let z1 = calculate_var(t, &uranus::Z1);
    let z2 = calculate_var(t, &uranus::Z2);
    let z3 = calculate_var(t, &uranus::Z3);
    let z4 = calculate_var(t, &uranus::Z4);

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4) + x5 * t.powi(5);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4) + y5 * t.powi(5);
    let z = z0 + z1 * t + z2 * t * t + z3 * t.powi(3) + z4 * t.powi(4);

    RectangularCoordinates { x: x, y: y, z: z }
}

/// Calculates VSOP87C solution for Neptune.
///
/// This function calculates the VSOP87C solution (heliocentric ecliptic rectangular coordinates
/// for the equinox of the day) for the planet Neptune. The parameter needed is the Julian Day
/// (*JD*) for the given date. It returns the VSOP87C solution in a `RectangularCoordinates`
/// structure. Those values are the rectangular coordinates of the planet, in *AU*, with the Sun in
/// the center and the ecliptic plane as reference `z = 0`.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// position of the planet Neptune in the solar system using rectangular coordinates. In this case,
/// we calculate where Neptune was in December 19th, 1299.
///
/// ```
/// use vsop87::vsop87c;
///
/// let coordinates = vsop87c::neptune(2195870.0);
///
/// assert!(coordinates.x > -27.2598513120 && coordinates.x < -27.2598513118);
/// assert!(coordinates.y > -13.2185335841 && coordinates.y < -13.2185335839);
/// assert!(coordinates.z > 0.96032790 && coordinates.z < 0.96032849);
/// ```
pub fn neptune(jde: f64) -> RectangularCoordinates {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &neptune::X0);
    let x1 = calculate_var(t, &neptune::X1);
    let x2 = calculate_var(t, &neptune::X2);
    let x3 = calculate_var(t, &neptune::X3);
    let x4 = calculate_var(t, &neptune::X4);
    let x5 = calculate_var(t, &neptune::X5);

    let y0 = calculate_var(t, &neptune::Y0);
    let y1 = calculate_var(t, &neptune::Y1);
    let y2 = calculate_var(t, &neptune::Y2);
    let y3 = calculate_var(t, &neptune::Y3);
    let y4 = calculate_var(t, &neptune::Y4);
    let y5 = calculate_var(t, &neptune::Y5);

    let z0 = calculate_var(t, &neptune::Z0);
    let z1 = calculate_var(t, &neptune::Z1);
    let z2 = calculate_var(t, &neptune::Z2);
    let z3 = calculate_var(t, &neptune::Z3);
    let z4 = calculate_var(t, &neptune::Z4);

    let x = x0 + x1 * t + x2 * t * t + x3 * t.powi(3) + x4 * t.powi(4) + x5 * t.powi(5);
    let y = y0 + y1 * t + y2 * t * t + y3 * t.powi(3) + y4 * t.powi(4) + y5 * t.powi(5);
    let z = z0 + z1 * t + z2 * t * t + z3 * t.powi(3) + z4 * t.powi(4);

    RectangularCoordinates { x: x, y: y, z: z }
}
