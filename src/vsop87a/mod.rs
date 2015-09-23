//! VSOP87A implementation
//!
//! This module calculates heliocentric ecliptic rectangular coordinates for the equinox J2000.0
//! for the planets in the solar system.

mod mercury;
mod venus;
mod earth;
mod earth_moon;
mod mars;
mod jupiter;
mod saturn;
mod uranus;
mod neptune;

use super::{calculate_t, calculate_var};

/// Calculates VSOP87A solution for Mercury
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Mercury. The parameter needed is the Julian Day
/// Efemeris (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*,
/// *z* of the VSOP87A solution. Those values are the rectangular coordinates of the planet, in
/// *AU*, with the Sun in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87a;
///
/// let (x, y, z) = vsop87a::mercury(2415020.0);
///
/// assert!(x > -0.3897246932 && x < -0.3897246930);
/// assert!(y > -0.1502242200 && y < -0.1502242198);
/// assert!(z > 0.023618 && z < 0.023622);
/// ```
pub fn mercury(jde: f64) -> (f64, f64, f64) {
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

    let x = x0 + x1*t + x2*t*t + x3*t.powi(3) + x4*t.powi(4) + x5*t.powi(5);
    let y = y0 + y1*t + y2*t*t + y3*t.powi(3) + y4*t.powi(4) + y5*t.powi(5);
    let z = z0 + z1*t + z2*t*t + z3*t.powi(3) + z4*t.powi(4);

    (x, y, z)
}

/// Calculates VSOP87A solution for Venus
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Venus. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87A solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// Sun in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87a;
///
/// let (x, y, z) = vsop87a::venus(2122820.0);
///
/// assert!(x > -0.6660158466 && x < -0.6660158464);
/// assert!(y > -0.2753592312 && y < -0.2753592310);
/// assert!(z > 0.035785 && z < 0.035789);
/// ```
pub fn venus(jde: f64) -> (f64, f64, f64) {
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

    let x = x0 + x1*t + x2*t*t + x3*t.powi(3) + x4*t.powi(4) + x5*t.powi(5);
    let y = y0 + y1*t + y2*t*t + y3*t.powi(3) + y4*t.powi(4) + y5*t.powi(5);
    let z = z0 + z1*t + z2*t*t + z3*t.powi(3) + z4*t.powi(4);

    (x, y, z)
}

/// Calculates VSOP87A solution for Earth
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Earth. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87A solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// Sun in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87a;
///
/// let (x, y, z) = vsop87a::earth(2341970.0);
///
/// assert!(x > -0.2104654653 && x < -0.2104654651);
/// assert!(y > 0.9603579953 && y < 0.9603579955);
/// assert!(z > 0.000645 && z < 0.000649);
/// ```
pub fn earth(jde: f64) -> (f64, f64, f64) {
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

    let x = x0 + x1*t + x2*t*t + x3*t.powi(3) + x4*t.powi(4) + x5*t.powi(5);
    let y = y0 + y1*t + y2*t*t + y3*t.powi(3) + y4*t.powi(4) + y5*t.powi(5);
    let z = z0 + z1*t + z2*t*t + z3*t.powi(3) + z4*t.powi(4);

    (x, y, z)
}

/// Calculates VSOP87A solution for Earth - Moon barycenter
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the Earth - Moon barycenter. The parameter needed is the Julian
/// Day Efemeris (*JDE*) for the given date. It returns, in order, a tuple with the values *x*,
/// *y*, *z* of the VSOP87A solution. Those values are the rectangular coordinates of the planet,
/// in *AU*, with the Sun in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87a;
///
/// let (x, y, z) = vsop87a::earth_moon(2159345.0);
///
/// assert!(x > -0.2654471687 && x < -0.2654471685);
/// assert!(y > 0.9464953235 && y < 0.9464953237);
/// assert!(z > 0.001703 && z < 0.001707);
/// ```
pub fn earth_moon(jde: f64) -> (f64, f64, f64) {
    let t = calculate_t(jde);

    let x0 = calculate_var(t, &earth_moon::X0);
    let x1 = calculate_var(t, &earth_moon::X1);
    let x2 = calculate_var(t, &earth_moon::X2);
    let x3 = calculate_var(t, &earth_moon::X3);
    let x4 = calculate_var(t, &earth_moon::X4);
    let x5 = calculate_var(t, &earth_moon::X5);

    let y0 = calculate_var(t, &earth_moon::Y0);
    let y1 = calculate_var(t, &earth_moon::Y1);
    let y2 = calculate_var(t, &earth_moon::Y2);
    let y3 = calculate_var(t, &earth_moon::Y3);
    let y4 = calculate_var(t, &earth_moon::Y4);
    let y5 = calculate_var(t, &earth_moon::Y5);

    let z0 = calculate_var(t, &earth_moon::Z0);
    let z1 = calculate_var(t, &earth_moon::Z1);
    let z2 = calculate_var(t, &earth_moon::Z2);
    let z3 = calculate_var(t, &earth_moon::Z3);
    let z4 = calculate_var(t, &earth_moon::Z4);

    let x = x0 + x1*t + x2*t*t + x3*t.powi(3) + x4*t.powi(4) + x5*t.powi(5);
    let y = y0 + y1*t + y2*t*t + y3*t.powi(3) + y4*t.powi(4) + y5*t.powi(5);
    let z = z0 + z1*t + z2*t*t + z3*t.powi(3) + z4*t.powi(4);

    (x, y, z)
}

/// Calculates VSOP87A solution for Mars
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Mars. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87A solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// Sun in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87a;
///
/// let (x, y, z) = vsop87a::mars(2232395.0);
///
/// assert!(x > 1.3910394545 && x < 1.3910394547);
/// assert!(y > -0.0543839268 && y < -0.0543839266);
/// assert!(z > -0.037103 && z < -0.037099);
/// ```
pub fn mars(jde: f64) -> (f64, f64, f64) {
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

    let x = x0 + x1*t + x2*t*t + x3*t.powi(3) + x4*t.powi(4) + x5*t.powi(5);
    let y = y0 + y1*t + y2*t*t + y3*t.powi(3) + y4*t.powi(4) + y5*t.powi(5);
    let z = z0 + z1*t + z2*t*t + z3*t.powi(3) + z4*t.powi(4);

    (x, y, z)
}

/// Calculates VSOP87A solution for Jupiter
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Jupiter. The parameter needed is the Julian Day
/// Efemeris (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*,
/// *z* of the VSOP87A solution. Those values are the rectangular coordinates of the planet, in
/// *AU*, with the Sun in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87a;
///
/// let (x, y, z) = vsop87a::jupiter(2451545.0);
///
/// assert!(x > 4.0011740267 && x < 4.0011740269);
/// assert!(y > 2.9385810076 && y < 2.9385810078);
/// assert!(z > -0.101786 && z < -0.101782);
/// ```
pub fn jupiter(jde: f64) -> (f64, f64, f64) {
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

    let x = x0 + x1*t + x2*t*t + x3*t.powi(3) + x4*t.powi(4) + x5*t.powi(5);
    let y = y0 + y1*t + y2*t*t + y3*t.powi(3) + y4*t.powi(4) + y5*t.powi(5);
    let z = z0 + z1*t + z2*t*t + z3*t.powi(3) + z4*t.powi(4);

    (x, y, z)
}

/// Calculates VSOP87A solution for Saturn
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Saturn. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87A solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// Sun in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87a;
///
/// let (x, y, z) = vsop87a::saturn(2122820.0);
///
/// assert!(x > -7.9395559174 && x < -7.9395559172);
/// assert!(y > -5.8435867017 && y < -5.8435867015);
/// assert!(z > 0.416558 && z < 0.416562);
/// ```
pub fn saturn(jde: f64) -> (f64, f64, f64) {
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

    let x = x0 + x1*t + x2*t*t + x3*t.powi(3) + x4*t.powi(4) + x5*t.powi(5);
    let y = y0 + y1*t + y2*t*t + y3*t.powi(3) + y4*t.powi(4) + y5*t.powi(5);
    let z = z0 + z1*t + z2*t*t + z3*t.powi(3) + z4*t.powi(4);

    (x, y, z)
}

/// Calculates VSOP87A solution for Uranus
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Uranus. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87A solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// Sun in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87a;
///
/// let (x, y, z) = vsop87a::uranus(2159345.0);
///
/// assert!(x > -9.8287104598 && x < -9.8287104596);
/// assert!(y > 15.7711888604 && y < 15.7711888606);
/// assert!(z > 0.191480 && z < 0.191484);
/// ```
pub fn uranus(jde: f64) -> (f64, f64, f64) {
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

    let x = x0 + x1*t + x2*t*t + x3*t.powi(3) + x4*t.powi(4);
    let y = y0 + y1*t + y2*t*t + y3*t.powi(3) + y4*t.powi(4);
    let z = z0 + z1*t + z2*t*t;

    (x, y, z)
}

/// Calculates VSOP87A solution for Neptune
///
/// This function calculates the VSOP87A solution (heliocentric ecliptic rectangular coordinates
/// for the equinox J2000.0) for the planet Neptune. The parameter needed is the Julian Day
/// Efemeris (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*,
/// *z* of the VSOP87A solution. Those values are the rectangular coordinates of the planet, in
/// *AU*, with the Sun in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87a;
///
/// let (x, y, z) = vsop87a::neptune(2195870.0);
///
/// assert!(x > -24.6234347579 && x < -24.6234347577);
/// assert!(y > -17.6514428047 && y < -17.6514428045);
/// assert!(z > 0.929722 && z < 0.929726);
/// ```
pub fn neptune(jde: f64) -> (f64, f64, f64) {
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

    let x = x0 + x1*t + x2*t*t + x3*t.powi(3) + x4*t.powi(4);
    let y = y0 + y1*t + y2*t*t + y3*t.powi(3) + y4*t.powi(4);
    let z = z0 + z1*t + z2*t*t;

    (x, y, z)
}
