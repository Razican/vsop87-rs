//! VSOP87E implementation
//!
//! This module calculates barycentric ecliptic rectangular coordinates for the equinox J2000.0
//! for the planets in the solar system. These coordinates are centered in the barycenter of the
//! solar system.

mod sun;
mod mercury;
mod venus;
mod earth;
mod mars;
mod jupiter;
mod saturn;
mod uranus;
mod neptune;

use super::{calculate_t, calculate_var};

/// Calculates VSOP87E solution for the Sun
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the Sun. The parameter needed is the Julian Day Efemeris (*JDE*) for
/// the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the VSOP87E
/// solution. Those values are the rectangular coordinates of the Sun, in *AU*, with the barycenter
/// of the solar system in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87e;
///
/// let (x, y, z) = vsop87e::sun(2451545.0);
/// ```
pub fn sun(jde: f64) -> (f64, f64, f64) {
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

    let x = x0 + x1*t + x2*t*t + x3*t.powi(3) + x4*t.powi(4) + x5*t.powi(5);
    let y = y0 + y1*t + y2*t*t + y3*t.powi(3) + y4*t.powi(4) + y5*t.powi(5);
    let z = z0 + z1*t + z2*t*t + z3*t.powi(3) + z4*t.powi(4);

    (x, y, z)
}


/// Calculates VSOP87E solution for Mercury
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Mercury. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87E solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// barycenter of the solar system in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87e;
///
/// let (x, y, z) = vsop87e::mercury(2451545.0);
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

/// Calculates VSOP87E solution for Venus
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Venus. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87E solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// barycenter of the solar system in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87e;
///
/// let (x, y, z) = vsop87e::venus(2451545.0);
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

/// Calculates VSOP87E solution for Earth
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Earth. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87E solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// barycenter of the solar system in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87e;
///
/// let (x, y, z) = vsop87e::earth(2451545.0);
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

/// Calculates VSOP87E solution for Mars
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Mars. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87E solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// barycenter of the solar system in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87e;
///
/// let (x, y, z) = vsop87e::mars(2451545.0);
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

/// Calculates VSOP87E solution for Jupiter
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Jupiter. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87E solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// barycenter of the solar system in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87e;
///
/// let (x, y, z) = vsop87e::jupiter(2451545.0);
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

/// Calculates VSOP87E solution for Saturn
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Saturn. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87E solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// barycenter of the solar system in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87e;
///
/// let (x, y, z) = vsop87e::saturn(2451545.0);
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

/// Calculates VSOP87E solution for Uranus
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Uranus. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87E solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// barycenter of the solar system in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87e;
///
/// let (x, y, z) = vsop87e::uranus(2451545.0);
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

/// Calculates VSOP87E solution for Neptune
///
/// This function calculates the VSOP87E solution (barycentric ecliptic rectangular coordinates for
/// the equinox J2000.0) for the planet Neptune. The parameter needed is the Julian Day Efemeris
/// (*JDE*) for the given date. It returns, in order, a tuple with the values *x*, *y*, *z* of the
/// VSOP87E solution. Those values are the rectangular coordinates of the planet, in *AU*, with the
/// barycenter of the solar system in the center and the ecliptic plane as reference ```z = 0```.
///
/// # Examples
///
/// ```
/// use vsop87::vsop87e;
///
/// let (x, y, z) = vsop87e::neptune(2451545.0);
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
