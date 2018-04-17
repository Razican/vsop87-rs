//! This library implements the *VSOP87* solutions to calculate the positions of the planets in the
//! solar system. To use it you must include the following in your crate:
//!
//! ```
//! extern crate vsop87;
//! ```
//!
//! The main module calculates heliocentric ecliptic orbital elements for the equinox J2000.0 for
//! the planets in the solar system, the basic *VSOP87* solution. There is one module per other
//! *VSOP87* implementation: [*VSOP87A*](./vsop87a/index.html), [*VSOP87B*](./vsop87b/index.html),
//! [*VSOP87C*](./vsop87c/index.html), [*VSOP87D*](./vsop87d/index.html) and
//! [*VSOP87E*](./vsop87e/index.html). More information can be found
//! [here](https://www.caglow.com/info/compute/vsop87) and
//! [here](https://en.wikipedia.org/wiki/VSOP_(planets)).
//!
//! Each module has its own documentation, and here is the documentation on the base *VSOP87*
//! solution. The *VSOP87* algorithm has great precission (under 1") for **4,000 years** before and
//! after J2000 epoch for Mercury, Venus, Earth-Moon barycenter and Mars, for **2,000 years** in
//! the case of Jupiter and Saturn and for **6,000 years** for Uranus and Neptune.
//!
//! The base *VSOP87* solution calculates the
//! [orbital elements](https://en.wikipedia.org/wiki/Orbital_elements) of the planets arount the
//! Sun. The returned elements are a special VSOP87 orbital elements, that can be converted into
//! usual keplerian elements using the `Into` trait. These elements are ideal to get an idea on how
//! the orbits are changing over time. It can also be used for other complex orbital computations.
//!
//! # Example
//!
//! As an example, here we calculate the orbital parameters for Mercury on the January 1st, 2000.
//! The *VSOP87* algorithm requires dates to be entered as
//! [Julian Day](https://en.wikipedia.org/wiki/Julian_day) (*JD*). In our case, that date is
//! `2451545.0`.
//!
//! We first calculate the VSOP87 elements:
//!
//! ```
//! let vsop87_elts = vsop87::mercury(2451545.0);
//!
//! assert!(vsop87_elts.a > 0.3870982121 && vsop87_elts.a < 0.3870982123);
//! assert!(vsop87_elts.l > 4.4026057778 && vsop87_elts.l < 4.4026057780);
//! assert!(vsop87_elts.k > 0.0446647517 && vsop87_elts.k < 0.0446647519);
//! assert!(vsop87_elts.h > 0.2007208957 && vsop87_elts.h < 0.2007208959);
//! assert!(vsop87_elts.q > 0.0406161540 && vsop87_elts.q < 0.0406161542);
//! assert!(vsop87_elts.p > 0.04563512 && vsop87_elts.p < 0.04563588);
//! ```
//!
//! Note that the `>` and `<` comparisons are there because floats should not be compared using
//! `==`. Those numbers are retrieved from the test data of the *VSOP87* algorithm.
//!
//! We can then convert them into keplerian elements, by using both `KeplerianElements::from()` or
//! the `into()` function in the *VSOP87* elements. This also works the other way around:
//!
//! ```
//! use vsop87::{KeplerianElements, VSOP87Elements};
//!
//! # let vsop87_elts = vsop87::mercury(2451545.0);
//! #
//! # assert!(vsop87_elts.a > 0.3870982121 && vsop87_elts.a < 0.3870982123);
//! # assert!(vsop87_elts.l > 4.4026057778 && vsop87_elts.l < 4.4026057780);
//! # assert!(vsop87_elts.k > 0.0446647517 && vsop87_elts.k < 0.0446647519);
//! # assert!(vsop87_elts.h > 0.2007208957 && vsop87_elts.h < 0.2007208959);
//! # assert!(vsop87_elts.q > 0.0406161540 && vsop87_elts.q < 0.0406161542);
//! # assert!(vsop87_elts.p > 0.04563512 && vsop87_elts.p < 0.04563588);
//! #
//! let elements = KeplerianElements::from(vsop87_elts);
//! let convert_back: VSOP87Elements = elements.into();
//!
//! assert!(elements.semimajor_axis() > 0.387097 && elements.semimajor_axis() < 0.387099);
//! assert!(elements.eccentricity() > 0.205629 && elements.eccentricity() < 0.205631);
//! assert!(elements.inclination() > 0.122260 && elements.inclination() < 0.122261);
//! assert!(elements.ascending_node() > 0.843525 && elements.ascending_node() < 0.843527);
//! assert!(elements.periapsis() > 1.35183 && elements.periapsis() < 1.35185);
//! assert!(elements.mean_anomaly() > 4.40259 && elements.mean_anomaly() < 4.40261);
//! ```
//!
//! As you can see, these numbers perfectly match
//! [those from NASA](http://solarsystem.nasa.gov/planets/mercury/facts).

#![forbid(missing_docs, warnings, anonymous_parameters, unused_extern_crates,
          unused_import_braces, missing_copy_implementations, trivial_casts,
          variant_size_differences, missing_debug_implementations, trivial_numeric_casts)]
// Debug trait derivation will show an error if forbidden.
#![deny(unused_qualifications, unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
// FIXME: Maybe we should start writing proper variable names.
#![cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]
#![cfg_attr(feature = "cargo-clippy", warn(clippy_pedantic))]
// All the "allow by default" lints
#![warn(box_pointers, unused_results)]
#![cfg_attr(feature = "unstable", feature(stdsimd, target_feature, cfg_target_feature))]

pub mod vsop87a;
pub mod vsop87b;
pub mod vsop87c;
pub mod vsop87d;
pub mod vsop87e;

mod mercury;
mod venus;
mod earth_moon;
mod mars;
mod jupiter;
mod saturn;
mod uranus;
mod neptune;

use std::f64::consts::PI;

/// Structure representing the keplerian elements of an orbit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeplerianElements {
    ecc: f64,
    sma: f64,
    incl: f64,
    lan: f64,
    lper: f64,
    l0: f64,
}

impl KeplerianElements {
    /// Gets the eccentricity of the orbit (*e*).
    ///
    /// A number smaller to one would be a closed ellipse, while 0 would be a circle orbit. Values
    /// higher than one would be hyperbolic orbits, that are not closed, while a 1 would be a
    /// parabolical orbit. Negative values cannot exist.
    pub fn eccentricity(&self) -> f64 {
        self.ecc
    }

    /// Gets the semimajor axis of an orbit (*a*), in *AU* (Astronomical units).
    ///
    /// This value represents the average distance from the orbiting body to the center of mass.
    pub fn semimajor_axis(&self) -> f64 {
        self.sma
    }

    /// Gets the inclination of the orbit (*i*), in radians.
    ///
    /// This value represents the inclination of the plane where the object is orbiting with
    /// respect to the reference plane.
    pub fn inclination(&self) -> f64 {
        self.incl
    }

    /// Gets the longitude of the ascending node of the orbit (*Ω*), in radians.
    ///
    /// This value represents the angle in the orbit ellipse of the point where the reference plane
    /// and the orbit plane cross when the orbiting body crosses the plane *ascending* in the orbit.
    pub fn ascending_node(&self) -> f64 {
        self.lan
    }

    /// Gets the longitude of the periapsis of the orbit (*ϖ*), in radians.
    ///
    /// This value represents the angle in the orbit ellipse of the nearest point of the orbit to
    /// the center of mass of the system.
    pub fn periapsis(&self) -> f64 {
        self.lper
    }

    /// Gets the mean anomaly of the orbiting object at the given epoch.
    ///
    /// This value represents the angle in the orbit ellipse of the orbiting body at the given
    /// moment.
    pub fn mean_anomaly(&self) -> f64 {
        self.l0
    }
}

impl From<VSOP87Elements> for KeplerianElements {
    fn from(elts: VSOP87Elements) -> KeplerianElements {
        let ecc = (elts.h * elts.h + elts.k * elts.k).sqrt();
        let i = (1_f64 - 2_f64 * (elts.p * elts.p + elts.q * elts.q)).acos();
        let lan = (elts.p / elts.q).atan();
        let lper = (elts.h / ecc).asin();

        KeplerianElements {
            ecc: ecc,
            sma: elts.a,
            incl: i,
            lan: lan,
            lper: lper,
            l0: elts.l,
        }
    }
}

/// Structure representing 3 dimensional rectangular coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RectangularCoordinates {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Z coordinate.
    pub z: f64,
}

/// Structure representing spherical coordinates of a body.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphericalCoordinates {
    lon: f64,
    lat: f64,
    dist: f64,
}

impl SphericalCoordinates {
    /// Gets the ecliptic longitude of the body, in radians.
    ///
    /// This value represents the angular distance of an object along the ecliptic plane from the
    /// primary direction. In the case of heliocentric coordinates, it represents the *l* parameter,
    /// in geocentric coordinates, represents the *λ* parameter.
    pub fn longitude(&self) -> f64 {
        self.lon
    }

    /// Gets the ecliptic latitude of the body, in radians.
    ///
    /// This value represents the angular distance of an object from the ecliptic towards the north
    /// ecliptic pole. In the case of heliocentric coordinates, it represents the *b* parameter, in
    /// geocentric coordinates, represents the *β* parameter.
    pub fn latitude(&self) -> f64 {
        self.lat
    }

    /// Gets the distance to the center of mass, in astronomical units (*AU*).
    ///
    /// In the case of heliocentric coordinates, it represents the *r* parameter, in geocentric
    /// coordinates, represents the *Δ* parameter.
    pub fn distance(&self) -> f64 {
        self.dist
    }
}

/// Calculates the `t` value.
#[inline]
fn calculate_t(jde: f64) -> f64 {
    (jde - 2_451_545_f64) / 365_250_f64
}

/// Calculates the given variable.
#[inline]
#[cfg(not(feature = "unstable"))]
fn calculate_var(t: f64, var: &[(f64, f64, f64)]) -> f64 {
    calculate_var_default(t, var)
}

/// Calculates the given variable.
#[inline]
#[cfg(feature = "unstable")]
fn calculate_var(t: f64, var: &[(f64, f64, f64)]) -> f64 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[allow(unsafe_code)]
    {
        if is_x86_feature_detected!("avx") {
            unsafe { calculate_var_avx(t, var) }
        } else {
            calculate_var_default(t, var)
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        calculate_var_default(t, var)
    }
}

/// Default method to calculate the variable.
#[inline]
fn calculate_var_default(t: f64, var: &[(f64, f64, f64)]) -> f64 {
    var.iter()
        .fold(0_f64, |term, &(a, b, c)| term + a * (b + c * t).cos())
}

/// Calculate the given variable using AVX-2.
#[cfg(all(feature = "unstable", any(target_arch = "x86", target_arch = "x86_64")))]
#[cfg_attr(feature = "unstable", target_feature(enable = "avx"))]
#[allow(unsafe_code)]
unsafe fn calculate_var_avx(t: f64, var: &[(f64, f64, f64)]) -> f64 {
    // #[cfg(target_arch = "x86")]
    // use std::arch::x86::_mm256_add_epi64;
    // #[cfg(target_arch = "x86_64")]
    // use std::arch::x86_64::_mm256_add_epi64;
    //
    // _mm256_add_epi64();

    calculate_var_default()
}

/// Elements used by the VSOP87 solution. Can be converted into keplerian elements.
///
/// More information can be found [here](http://totaleclipse.eu/Astronomy/VSOP87.html).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VSOP87Elements {
    /// Semimajor axis in astronomical units (*AU*).
    pub a: f64,
    /// Mean longitude at epoch.
    pub l: f64,
    /// `e * lper.cos()``, where *e* is the eccentricity and *lper* is the longitude of the
    /// perihelion.
    pub k: f64,
    /// `e * lper.sin()``, where *e* is the eccentricity and *lper* is the longitude of the
    /// perihelion (*ϖ*).
    pub h: f64,
    /// `(i/2.0).sin() * lan.cos()` where *i* is inclination and *lan is the longitude of the
    /// ascending node (*Ω*).
    pub q: f64,
    /// `(i/2.0).sin() * lan.sin()` where *i* is inclination and *lan is the longitude of the
    /// ascending node (*Ω*).
    pub p: f64,
}

impl From<KeplerianElements> for VSOP87Elements {
    fn from(elts: KeplerianElements) -> VSOP87Elements {
        VSOP87Elements {
            a: elts.sma,
            l: elts.l0,
            k: elts.ecc * elts.lper.cos(),
            h: elts.ecc * elts.lper.sin(),
            q: (elts.incl / 2.0).sin() * elts.lan.cos(),
            p: (elts.incl / 2.0).sin() * elts.lan.sin(),
        }
    }
}

/// Calculates VSOP87 solution for Mercury.
///
/// This function calculates the VSOP87 solution (heliocentric ecliptic orbital elements for the
/// equinox J2000.0) for the planet Mercury. The parameter needed is the Julian Day (*JD*) for the
/// given date. It returns the `VSOP87Elements` of the VSOP87 solution.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// orbit of the planet Mercury. In this case, we calculate the orbit of Mercury in December 31st,
/// 1899.
///
/// ```
/// let vsop87_elts = vsop87::mercury(2415020.0);
///
/// assert!(vsop87_elts.a > 0.3870977205 && vsop87_elts.a < 0.3870977207);
/// assert!(vsop87_elts.l > 3.1341564064 && vsop87_elts.l < 3.1341564066);
/// assert!(vsop87_elts.k > 0.0452159417 && vsop87_elts.k < 0.0452159419);
/// assert!(vsop87_elts.h > 0.2005915793 && vsop87_elts.h < 0.2005915795);
/// assert!(vsop87_elts.q > 0.0405500077 && vsop87_elts.q < 0.0405500079);
/// assert!(vsop87_elts.p > 0.04576328 && vsop87_elts.p < 0.04576404);
/// ```
///
/// It can then be converted into keplerian elements:
///
/// ```
/// use vsop87::{KeplerianElements, VSOP87Elements};
///
/// # let vsop87_elts = vsop87::mercury(2415020.0);
/// #
/// let k_elements: KeplerianElements = vsop87_elts.into();
/// let convert_back = VSOP87Elements::from(k_elements);
/// ```
pub fn mercury(jde: f64) -> VSOP87Elements {
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

    let a = a0 + a1 * t + a2 * t * t;
    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3)) % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t * t + k3 * t.powi(3) + k4 * t.powi(4) + k5 * t.powi(5);
    let h = h0 + h1 * t + h2 * t * t + h3 * t.powi(3) + h4 * t.powi(4) + h5 * t.powi(5);
    let q = q0 + q1 * t + q2 * t * t + q3 * t.powi(3) + q4 * t.powi(4) + q5 * t.powi(5);
    let p = p0 + p1 * t + p2 * t * t + p3 * t.powi(3) + p4 * t.powi(4);

    VSOP87Elements {
        a,
        l: if l > 0_f64 { l } else { 2_f64 * PI + l },
        k,
        h,
        q,
        p,
    }
}

/// Calculates VSOP87 solution for Venus.
///
/// This function calculates the VSOP87 solution (heliocentric ecliptic orbital elements for the
/// equinox J2000.0) for the planet Venus. The parameter needed is the Julian Day (*JD*) for the
/// given date. It returns the `VSOP87Elements` of the VSOP87 solution.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// orbit of the planet Venus. In this case, we calculate the orbit of Venus in January 1st, 2000.
///
/// ```
/// let vsop87_elts = vsop87::venus(2451545.0);
///
/// assert!(vsop87_elts.a > 0.7233269303 && vsop87_elts.a < 0.7233269305);
/// assert!(vsop87_elts.l > 3.1761350909 && vsop87_elts.l < 3.1761350911);
/// assert!(vsop87_elts.k > -0.0045086078 && vsop87_elts.k < -0.0045086076);
/// assert!(vsop87_elts.h > 0.0050312181 && vsop87_elts.h < 0.0050312183);
/// assert!(vsop87_elts.q > 0.0068248057 && vsop87_elts.q < 0.0068248059);
/// assert!(vsop87_elts.p > 0.02882177 && vsop87_elts.p < 0.02882253);
/// ```
///
/// It can then be converted into keplerian elements:
///
/// ```
/// use vsop87::{KeplerianElements, VSOP87Elements};
///
/// # let vsop87_elts = vsop87::venus(2451545.0);
/// #
/// let k_elements: KeplerianElements = vsop87_elts.into();
/// let convert_back = VSOP87Elements::from(k_elements);
/// ```
pub fn venus(jde: f64) -> VSOP87Elements {
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

    let a = a0 + a1 * t + a2 * t * t;
    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3)) % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t * t + k3 * t.powi(3) + k4 * t.powi(4) + k5 * t.powi(5);
    let h = h0 + h1 * t + h2 * t * t + h3 * t.powi(3) + h4 * t.powi(4) + h5 * t.powi(5);
    let q = q0 + q1 * t + q2 * t * t + q3 * t.powi(3) + q4 * t.powi(4) + q5 * t.powi(5);
    let p = p0 + p1 * t + p2 * t * t + p3 * t.powi(3) + p4 * t.powi(4);

    VSOP87Elements {
        a,
        l: if l > 0_f64 { l } else { 2_f64 * PI + l },
        k,
        h,
        q,
        p,
    }
}

/// Calculates VSOP87 solution for Earth - Moon barycenter.
///
/// This function calculates the VSOP87 solution (heliocentric ecliptic orbital elements for the
/// equinox J2000.0) for the Earth - Moon barycenter. The parameter needed is the Julian Day (*JD*)
/// for the given date. It returns the `VSOP87Elements` of the VSOP87 solution.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// orbit of the Earth - Moon barycenter (the center of masses between the Earth and the Moon, not
/// exactly the center of the Earth). In this case, we calculate the orbit of the Earth - Moon
/// barycenter in December 19th, 1099.
/// ```
/// let vsop87_elts = vsop87::earth_moon(2122820.0);
///
/// assert!(vsop87_elts.a > 1.0000134925 && vsop87_elts.a < 1.0000134927);
/// assert!(vsop87_elts.l > 1.8519621672 && vsop87_elts.l < 1.8519621674);
/// assert!(vsop87_elts.k > -0.0029638176 && vsop87_elts.k < -0.0029638174);
/// assert!(vsop87_elts.h > 0.0168402193 && vsop87_elts.h < 0.0168402195);
/// assert!(vsop87_elts.q > 0.0010301900 && vsop87_elts.q < 0.0010301902);
/// assert!(vsop87_elts.p > -0.00005346 && vsop87_elts.p < -0.00005270);
/// ```
///
/// It can then be converted into keplerian elements:
///
/// ```
/// use vsop87::{KeplerianElements, VSOP87Elements};
///
/// # let vsop87_elts = vsop87::earth_moon(2122820.0);
/// #
/// let k_elements: KeplerianElements = vsop87_elts.into();
/// let convert_back = VSOP87Elements::from(k_elements);
/// ```
pub fn earth_moon(jde: f64) -> VSOP87Elements {
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

    let a = a0 + a1 * t + a2 * t * t;
    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t * t + k3 * t.powi(3) + k4 * t.powi(4) + k5 * t.powi(5);
    let h = h0 + h1 * t + h2 * t * t + h3 * t.powi(3) + h4 * t.powi(4) + h5 * t.powi(5);
    let q = q0 + q1 * t + q2 * t * t + q3 * t.powi(3) + q4 * t.powi(4) + q5 * t.powi(5);
    let p = p0 + p1 * t + p2 * t * t + p3 * t.powi(3) + p4 * t.powi(4);

    VSOP87Elements {
        a,
        l: if l > 0_f64 { l } else { 2_f64 * PI + l },
        k,
        h,
        q,
        p,
    }
}

/// Calculates VSOP87 solution for Mars.
///
/// This function calculates the VSOP87 solution (heliocentric ecliptic orbital elements for the
/// equinox J2000.0) for the planet Mars. The parameter needed is the Julian Day (*JD*) for the
/// given date. It returns the `VSOP87Elements` of the VSOP87 solution.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// orbit of the planet Mars. In this case, we calculate the orbit of Mars in December 19th, 1199.
///
/// ```
/// let vsop87_elts = vsop87::mars(2159345.0);
///
/// assert!(vsop87_elts.a > 1.5237578877 && vsop87_elts.a < 1.5237578879);
/// assert!(vsop87_elts.l > 4.0669853278 && vsop87_elts.l < 4.0669853280);
/// assert!(vsop87_elts.k > 0.0821906316 && vsop87_elts.k < 0.0821906318);
/// assert!(vsop87_elts.h > -0.0427917583 && vsop87_elts.h < -0.0427917581);
/// assert!(vsop87_elts.q > 0.0103081045 && vsop87_elts.q < 0.0103081047);
/// assert!(vsop87_elts.p > 0.01313608 && vsop87_elts.p < 0.01313684);
/// ```
///
/// It can then be converted into keplerian elements:
///
/// ```
/// use vsop87::{KeplerianElements, VSOP87Elements};
///
/// # let vsop87_elts = vsop87::mars(2159345.0);
/// #
/// let k_elements: KeplerianElements = vsop87_elts.into();
/// let convert_back = VSOP87Elements::from(k_elements);
/// ```
pub fn mars(jde: f64) -> VSOP87Elements {
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

    let a = a0 + a1 * t + a2 * t * t;
    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t * t + k3 * t.powi(3) + k4 * t.powi(4) + k5 * t.powi(5);
    let h = h0 + h1 * t + h2 * t * t + h3 * t.powi(3) + h4 * t.powi(4) + h5 * t.powi(5);
    let q = q0 + q1 * t + q2 * t * t + q3 * t.powi(3) + q4 * t.powi(4) + q5 * t.powi(5);
    let p = p0 + p1 * t + p2 * t * t + p3 * t.powi(3);

    VSOP87Elements {
        a,
        l: if l > 0_f64 { l } else { 2_f64 * PI + l },
        k,
        h,
        q,
        p,
    }
}

/// Calculates VSOP87 solution for Jupiter.
///
/// This function calculates the VSOP87 solution (heliocentric ecliptic orbital elements for the
/// equinox J2000.0) for the planet Jupiter. The parameter needed is the Julian Day (*JD*) for the
/// given date. It returns the `VSOP87Elements` of the VSOP87 solution.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// orbit of the planet Jupiter. In this case, we calculate the orbit of Jupiter in December 30th,
/// 1799.
///
/// ```
/// let vsop87_elts = vsop87::jupiter(2378495.0);
///
/// assert!(vsop87_elts.a > 5.2027276672 && vsop87_elts.a < 5.2027276674);
/// assert!(vsop87_elts.l > 1.4820596291 && vsop87_elts.l < 1.4820596293);
/// assert!(vsop87_elts.k > 0.0464780412 && vsop87_elts.k < 0.0464780414);
/// assert!(vsop87_elts.h > 0.0116460263 && vsop87_elts.h < 0.0116460265);
/// assert!(vsop87_elts.q > -0.0019921307 && vsop87_elts.q < -0.0019921305);
/// assert!(vsop87_elts.p > 0.01123447 && vsop87_elts.p < 0.01123523);
/// ```
///
/// It can then be converted into keplerian elements:
///
/// ```
/// use vsop87::{KeplerianElements, VSOP87Elements};
///
/// # let vsop87_elts = vsop87::jupiter(2378495.0);
/// #
/// let k_elements: KeplerianElements = vsop87_elts.into();
/// let convert_back = VSOP87Elements::from(k_elements);
/// ```
pub fn jupiter(jde: f64) -> VSOP87Elements {
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

    let a = a0 + a1 * t + a2 * t * t + a3 * t.powi(3) + a4 * t.powi(4) + a5 * t.powi(5);
    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t * t + k3 * t.powi(3) + k4 * t.powi(4);
    let h = h0 + h1 * t + h2 * t * t + h3 * t.powi(3) + h4 * t.powi(4);
    let q = q0 + q1 * t + q2 * t * t + q3 * t.powi(3);
    let p = p0 + p1 * t + p2 * t * t;

    VSOP87Elements {
        a,
        l: if l > 0_f64 { l } else { 2_f64 * PI + l },
        k,
        h,
        q,
        p,
    }
}

/// Calculates VSOP87 solution for Saturn.
///
/// This function calculates the VSOP87 solution (heliocentric ecliptic orbital elements for the
/// equinox J2000.0) for the planet Saturn. The parameter needed is the Julian Day (*JD*) for the
/// given date. It returns the `VSOP87Elements` of the VSOP87 solution.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// orbit of the planet Saturn. In this case, we calculate the orbit of Saturn in December 29th,
/// 1599.
///
/// ```
/// let vsop87_elts = vsop87::saturn(2305445.0);
///
/// assert!(vsop87_elts.a > 9.5727100002 && vsop87_elts.a < 9.5727100004);
/// assert!(vsop87_elts.l > 3.5107821038 && vsop87_elts.l < 3.5107821040);
/// assert!(vsop87_elts.k > -0.0048218813 && vsop87_elts.k < -0.0048218811);
/// assert!(vsop87_elts.h > 0.0575514202 && vsop87_elts.h < 0.0575514204);
/// assert!(vsop87_elts.q > -0.0090348990 && vsop87_elts.q < -0.0090348988);
/// assert!(vsop87_elts.p > 0.01965756 && vsop87_elts.p < 0.01965832);
/// ```
///
/// It can then be converted into keplerian elements:
///
/// ```
/// use vsop87::{KeplerianElements, VSOP87Elements};
///
/// # let vsop87_elts = vsop87::saturn(2305445.0);
/// #
/// let k_elements: KeplerianElements = vsop87_elts.into();
/// let convert_back = VSOP87Elements::from(k_elements);
/// ```
pub fn saturn(jde: f64) -> VSOP87Elements {
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

    let a = a0 + a1 * t + a2 * t * t + a3 * t.powi(3) + a4 * t.powi(4) + a5 * t.powi(5);
    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t * t + k3 * t.powi(3) + k4 * t.powi(4) + k5 * t.powi(5);
    let h = h0 + h1 * t + h2 * t * t + h3 * t.powi(3) + h4 * t.powi(4) + h5 * t.powi(5);
    let q = q0 + q1 * t + q2 * t * t + q3 * t.powi(3) + q4 * t.powi(4);
    let p = p0 + p1 * t + p2 * t * t + p3 * t.powi(3);

    VSOP87Elements {
        a,
        l: if l > 0_f64 { l } else { 2_f64 * PI + l },
        k,
        h,
        q,
        p,
    }
}

/// Calculates VSOP87 solution for Uranus.
///
/// This function calculates the VSOP87 solution (heliocentric ecliptic orbital elements for the
/// equinox J2000.0) for the planet Uranus. The parameter needed is the Julian Day (*JD*) for the
/// given date. It returns the `VSOP87Elements` of the VSOP87 solution.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// orbit of the planet Uranus. In this case, we calculate the orbit of Uranus in December 19th,
/// 1399.
///
/// ```
/// let vsop87_elts = vsop87::uranus(2232395.0);
///
/// assert!(vsop87_elts.a > 19.2497356422 && vsop87_elts.a < 19.2497356424);
/// assert!(vsop87_elts.l > 4.5777275752 && vsop87_elts.l < 4.5777275754);
/// assert!(vsop87_elts.k > -0.0466529112 && vsop87_elts.k < -0.0466529110);
/// assert!(vsop87_elts.h > 0.0051308956 && vsop87_elts.h < 0.0051308958);
/// assert!(vsop87_elts.q > 0.0019206656 && vsop87_elts.q < 0.0019206658);
/// assert!(vsop87_elts.p > 0.00655819 && vsop87_elts.p < 0.00655895);
/// ```
///
/// It can then be converted into keplerian elements:
///
/// ```
/// use vsop87::{KeplerianElements, VSOP87Elements};
///
/// # let vsop87_elts = vsop87::uranus(2232395.0);
/// #
/// let k_elements: KeplerianElements = vsop87_elts.into();
/// let convert_back = VSOP87Elements::from(k_elements);
/// ```
pub fn uranus(jde: f64) -> VSOP87Elements {
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

    let a = a0 + a1 * t + a2 * t * t + a3 * t.powi(3) + a4 * t.powi(4) + a5 * t.powi(5);
    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t * t + k3 * t.powi(3) + k4 * t.powi(4);
    let h = h0 + h1 * t + h2 * t * t + h3 * t.powi(3) + h4 * t.powi(4);
    let q = q0 + q1 * t + q2 * t * t + q3 * t.powi(3);
    let p = p0 + p1 * t + p2 * t * t;

    VSOP87Elements {
        a,
        l: if l > 0_f64 { l } else { 2_f64 * PI + l },
        k,
        h,
        q,
        p,
    }
}

/// Calculates VSOP87 solution for Neptune.
///
/// This function calculates the VSOP87 solution (heliocentric ecliptic orbital elements for the
/// equinox J2000.0) for the planet Neptune. The parameter needed is the Julian Day (*JD*) for the
/// given date. It returns the `VSOP87Elements` of the VSOP87 solution.
///
/// # Example
///
/// Given a date in [*JD*](http://aa.usno.navy.mil/data/docs/JulianDate.php), we can get the
/// orbit of the planet Neptune. In this case, we calculate the orbit of Neptune in December 19th,
/// 1499.
///
/// ```
/// let vsop87_elts = vsop87::neptune(2268920.0);
///
/// assert!(vsop87_elts.a > 30.1963044187 && vsop87_elts.a < 30.1963044189);
/// assert!(vsop87_elts.l > 5.1088676118 && vsop87_elts.l < 5.1088676120);
/// assert!(vsop87_elts.k > 0.0091964091 && vsop87_elts.k < 0.0091964093);
/// assert!(vsop87_elts.h > 0.0031103619 && vsop87_elts.h < 0.0031103621);
/// assert!(vsop87_elts.q > -0.0102800265 && vsop87_elts.q < -0.0102800263);
/// assert!(vsop87_elts.p > 0.01148076 && vsop87_elts.p < 0.01148152);
/// ```
///
/// It can then be converted into keplerian elements:
///
/// ```
/// use vsop87::{KeplerianElements, VSOP87Elements};
///
/// # let vsop87_elts = vsop87::neptune(2268920.0);
/// #
/// let k_elements: KeplerianElements = vsop87_elts.into();
/// let convert_back = VSOP87Elements::from(k_elements);
/// ```
pub fn neptune(jde: f64) -> VSOP87Elements {
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

    let a = a0 + a1 * t + a2 * t * t + a3 * t.powi(3) + a4 * t.powi(4) + a5 * t.powi(5);
    let l = (l0 + l1 * t + l2 * t * t + l3 * t.powi(3) + l4 * t.powi(4) + l5 * t.powi(5))
        % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t * t + k3 * t.powi(3) + k4 * t.powi(4) + k5 * t.powi(5);
    let h = h0 + h1 * t + h2 * t * t + h3 * t.powi(3) + h4 * t.powi(4) + h5 * t.powi(5);
    let q = q0 + q1 * t + q2 * t * t + q3 * t.powi(3);
    let p = p0 + p1 * t + p2 * t * t;

    VSOP87Elements {
        a,
        l: if l > 0_f64 { l } else { 2_f64 * PI + l },
        k,
        h,
        q,
        p,
    }
}
