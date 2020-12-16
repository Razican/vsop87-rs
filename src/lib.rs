//! This library implements the *VSOP87* solutions to calculate the positions of the planets in the
//! solar system.
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

#![forbid(
    missing_docs,
    anonymous_parameters,
    unused_extern_crates,
    unused_import_braces,
    missing_copy_implementations,
    trivial_casts,
    variant_size_differences,
    missing_debug_implementations,
    trivial_numeric_casts
)]
// Debug trait derivation will show an error if forbidden.
#![deny(unused_qualifications, unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::many_single_char_names,
    clippy::unreadable_literal,
    clippy::excessive_precision,
    clippy::must_use_candidate
)]
#![cfg_attr(all(test, feature = "no_std"), allow(unused_imports))]
// Features
#![cfg_attr(feature = "no_std", no_std)]
// All the "allow by default" lints
#![warn(box_pointers, unused_results)]

pub mod vsop87a;
pub mod vsop87b;
pub mod vsop87c;
pub mod vsop87d;
pub mod vsop87e;

mod earth_moon;
mod jupiter;
mod mars;
mod mercury;
mod neptune;
mod saturn;
mod uranus;
mod venus;

#[cfg(feature = "no_std")]
use core::f64::consts::PI;
#[cfg(feature = "no_std")]
use libm::{acos, asin, atan, cos, sin, sincos, sqrt};

#[cfg(not(feature = "no_std"))]
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
    /// parabolic orbit. Negative values cannot exist.
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
    fn from(elts: VSOP87Elements) -> Self {
        #[cfg(feature = "no_std")]
        {
            let ecc = sqrt(elts.h * elts.h + elts.k * elts.k);
            let i = acos(1_f64 - 2_f64 * (elts.p * elts.p + elts.q * elts.q));
            let lan = atan(elts.p / elts.q);
            let lper = asin(elts.h / ecc);

            Self {
                ecc,
                sma: elts.a,
                incl: i,
                lan,
                lper,
                l0: elts.l,
            }
        }

        #[cfg(not(feature = "no_std"))]
        {
            let ecc = (elts.h * elts.h + elts.k * elts.k).sqrt();
            let i = (1_f64 - 2_f64 * (elts.p * elts.p + elts.q * elts.q)).acos();
            let lan = (elts.p / elts.q).atan();
            let lper = (elts.h / ecc).asin();

            Self {
                ecc,
                sma: elts.a,
                incl: i,
                lan,
                lper,
                l0: elts.l,
            }
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

/// Calculates the time variable for VSOP87.
#[inline]
fn calculate_t(jde: f64) -> f64 {
    (jde - 2_451_545_f64) / 365_250_f64
}

/// Calculates the given variable.
#[inline]
fn calculate_var(t: f64, a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "simd",
        not(feature = "no_std")
    ))]
    #[allow(unsafe_code)]
    {
        if is_x86_feature_detected!("avx") {
            // Safe because we already checked that we have AVX instruction set.
            unsafe { calculate_var_avx(t, a, b, c) }
        } else {
            calculate_var_fallback(t, a, b, c)
        }
    }

    #[cfg(any(
        all(not(target_arch = "x86"), not(target_arch = "x86_64")),
        not(feature = "simd"),
        feature = "no_std"
    ))]
    {
        calculate_var_fallback(t, a, b, c)
    }
}

/// Fallback implementation of the variable calculation.
///
/// Used in systems without SIMD support.
#[inline]
fn calculate_var_fallback(t: f64, a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    #[cfg(not(feature = "no_std"))]
    {
        a.iter()
            .zip(b)
            .zip(c)
            .fold(0_f64, |term, ((a, b), c)| term + a * (b + c * t).cos())
    }

    #[cfg(feature = "no_std")]
    {
        a.iter()
            .zip(b)
            .zip(c)
            .fold(0_f64, |term, ((a, b), c)| term + a * cos(b + c * t))
    }
}

/// Calculates the given variable using the AVX instruction set.
#[target_feature(enable = "avx")]
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd",
    not(feature = "no_std")
))]
#[allow(unsafe_code)]
unsafe fn calculate_var_avx(t: f64, a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    #[cfg(feature = "no_std")]
    use core::{f64, mem};
    #[cfg(not(feature = "no_std"))]
    use std::{f64, mem};

    #[cfg(all(feature = "no_std", target_arch = "x86_64"))]
    use core::arch::x86_64::{_mm256_add_pd, _mm256_mul_pd, _mm256_set1_pd, _mm256_set_pd};
    #[cfg(all(not(feature = "no_std"), target_arch = "x86_64"))]
    use std::arch::x86_64::{_mm256_add_pd, _mm256_mul_pd, _mm256_set1_pd, _mm256_set_pd};

    #[cfg(all(feature = "no_std", target_arch = "x86"))]
    use core::arch::x86::{_mm256_add_pd, _mm256_mul_pd, _mm256_set1_pd, _mm256_set_pd};
    #[cfg(all(not(feature = "no_std"), target_arch = "x86"))]
    use std::arch::x86::{_mm256_add_pd, _mm256_mul_pd, _mm256_set1_pd, _mm256_set_pd};

    /// Vectorizes the calculation of 4 terms at the same time.
    unsafe fn vector_term(
        (a1, b1, c1): (f64, f64, f64),
        (a2, b2, c2): (f64, f64, f64),
        (a3, b3, c3): (f64, f64, f64),
        (a4, b4, c4): (f64, f64, f64),
        t: f64,
    ) -> (f64, f64, f64, f64) {
        let a = _mm256_set_pd(a1, a2, a3, a4);
        let b = _mm256_set_pd(b1, b2, b3, b4);
        let c = _mm256_set_pd(c1, c2, c3, c4);
        let t = _mm256_set1_pd(t);

        // Safe because both values are created properly and checked.
        let ct = _mm256_mul_pd(c, t);
        // Safe because both values are created properly and checked.
        let bct = _mm256_add_pd(b, ct);

        // Safe because bct_unpacked is 4 f64 long.
        let bct_unpacked: (f64, f64, f64, f64) = mem::transmute(bct);

        // Safe because bct_unpacked is 4 f64 long, and x84/x86_64 is little endian.
        let bct = _mm256_set_pd(
            bct_unpacked.3.cos(),
            bct_unpacked.2.cos(),
            bct_unpacked.1.cos(),
            bct_unpacked.0.cos(),
        );

        // Safe because both values are created properly and checked.
        let term = _mm256_mul_pd(a, bct);
        let term_unpacked: (f64, f64, f64, f64) = mem::transmute(term);

        term_unpacked
    }

    let iter1 = a.chunks_exact(4);
    let iter2 = b.chunks_exact(4);
    let iter3 = c.chunks_exact(4);

    let remainder = match (iter1.remainder(), iter2.remainder(), iter3.remainder()) {
        (&[a1, a2, a3], &[b1, b2, b3], &[c1, c2, c3]) => {
            // The result is little endian in x86/x86_64.
            let (_term4, term3, term2, term1) = vector_term(
                (a1, b1, c1),
                (a2, b2, c2),
                (a3, b3, c3),
                (f64::NAN, f64::NAN, f64::NAN),
                t,
            );

            term1 + term2 + term3
        }
        (&[a1, a2], &[b1, b2], &[c1, c2]) => a1 * (b1 + c1 * t).cos() + a2 * (b2 + c2 * t).cos(),
        (&[a], &[b], &[c]) => a * (b + c * t).cos(),
        (&[], &[], &[]) => 0_f64,
        _ => unreachable!(),
    };

    let res = iter1
        .zip(iter2)
        .zip(iter3)
        .map(|vars| match vars {
            ((&[a1, a2, a3, a4], &[b1, b2, b3, b4]), &[c1, c2, c3, c4]) => {
                // The result is little endian in x86/x86_64.
                let (term4, term3, term2, term1) =
                    vector_term((a1, b1, c1), (a2, b2, c2), (a3, b3, c3), (a4, b4, c4), t);

                term1 + term2 + term3 + term4
            }
            _ => unreachable!(),
        })
        .sum::<f64>();

    res + remainder
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
    fn from(elts: KeplerianElements) -> Self {
        #[cfg(feature = "no_std")]
        {
            let (lper_sin, lper_cos) = sincos(elts.lper);
            let (lan_sin, lan_cos) = sincos(elts.lan);
            let incl_sin = sin(elts.incl / 2.0);
            Self {
                a: elts.sma,
                l: elts.l0,
                k: elts.ecc * lper_cos,
                h: elts.ecc * lper_sin,
                q: incl_sin * lan_cos,
                p: incl_sin * lan_sin,
            }
        }

        #[cfg(not(feature = "no_std"))]
        {
            let (lper_sin, lper_cos) = elts.lper.sin_cos();
            let (lan_sin, lan_cos) = elts.lan.sin_cos();
            let incl_sin = (elts.incl / 2.0).sin();
            Self {
                a: elts.sma,
                l: elts.l0,
                k: elts.ecc * lper_cos,
                h: elts.ecc * lper_sin,
                q: incl_sin * lan_cos,
                p: incl_sin * lan_sin,
            }
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

    let a0 = calculate_var(t, &mercury::A0[0], &mercury::A0[1], &mercury::A0[2]);
    let a1 = calculate_var(t, &mercury::A1[0], &mercury::A1[1], &mercury::A1[2]);
    let a2 = calculate_var(t, &mercury::A2[0], &mercury::A2[1], &mercury::A2[2]);

    let l0 = calculate_var(t, &mercury::L0[0], &mercury::L0[1], &mercury::L0[2]);
    let l1 = calculate_var(t, &mercury::L1[0], &mercury::L1[1], &mercury::L1[2]);
    let l2 = calculate_var(t, &mercury::L2[0], &mercury::L2[1], &mercury::L2[2]);
    let l3 = calculate_var(t, &mercury::L3[0], &mercury::L3[1], &mercury::L3[2]);

    let k0 = calculate_var(t, &mercury::K0[0], &mercury::K0[1], &mercury::K0[2]);
    let k1 = calculate_var(t, &mercury::K1[0], &mercury::K1[1], &mercury::K1[2]);
    let k2 = calculate_var(t, &mercury::K2[0], &mercury::K2[1], &mercury::K2[2]);
    let k3 = calculate_var(t, &mercury::K3[0], &mercury::K3[1], &mercury::K3[2]);
    let k4 = calculate_var(t, &mercury::K4[0], &mercury::K4[1], &mercury::K4[2]);
    let k5 = calculate_var(t, &mercury::K5[0], &mercury::K5[1], &mercury::K5[2]);

    let h0 = calculate_var(t, &mercury::H0[0], &mercury::H0[1], &mercury::H0[2]);
    let h1 = calculate_var(t, &mercury::H1[0], &mercury::H1[1], &mercury::H1[2]);
    let h2 = calculate_var(t, &mercury::H2[0], &mercury::H2[1], &mercury::H2[2]);
    let h3 = calculate_var(t, &mercury::H3[0], &mercury::H3[1], &mercury::H3[2]);
    let h4 = calculate_var(t, &mercury::H4[0], &mercury::H4[1], &mercury::H4[2]);
    let h5 = calculate_var(t, &mercury::H5[0], &mercury::H5[1], &mercury::H5[2]);

    let q0 = calculate_var(t, &mercury::Q0[0], &mercury::Q0[1], &mercury::Q0[2]);
    let q1 = calculate_var(t, &mercury::Q1[0], &mercury::Q1[1], &mercury::Q1[2]);
    let q2 = calculate_var(t, &mercury::Q2[0], &mercury::Q2[1], &mercury::Q2[2]);
    let q3 = calculate_var(t, &mercury::Q3[0], &mercury::Q3[1], &mercury::Q3[2]);
    let q4 = calculate_var(t, &mercury::Q4[0], &mercury::Q4[1], &mercury::Q4[2]);
    let q5 = calculate_var(t, &mercury::Q5[0], &mercury::Q5[1], &mercury::Q5[2]);

    let p0 = calculate_var(t, &mercury::P0[0], &mercury::P0[1], &mercury::P0[2]);
    let p1 = calculate_var(t, &mercury::P1[0], &mercury::P1[1], &mercury::P1[2]);
    let p2 = calculate_var(t, &mercury::P2[0], &mercury::P2[1], &mercury::P2[2]);
    let p3 = calculate_var(t, &mercury::P3[0], &mercury::P3[1], &mercury::P3[2]);
    let p4 = calculate_var(t, &mercury::P4[0], &mercury::P4[1], &mercury::P4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t2 * t2;
    let t5 = t2 * t3;

    let a = a0 + a1 * t + a2 * t2;
    let l = (l0 + l1 * t + l2 * t2 + l3 * t3) % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t2 + k3 * t3 + k4 * t4 + k5 * t5;
    let h = h0 + h1 * t + h2 * t2 + h3 * t3 + h4 * t4 + h5 * t5;
    let q = q0 + q1 * t + q2 * t2 + q3 * t3 + q4 * t4 + q5 * t5;
    let p = p0 + p1 * t + p2 * t2 + p3 * t3 + p4 * t4;

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

    let a0 = calculate_var(t, &venus::A0[0], &venus::A0[1], &venus::A0[2]);
    let a1 = calculate_var(t, &venus::A1[0], &venus::A1[1], &venus::A1[2]);
    let a2 = calculate_var(t, &venus::A2[0], &venus::A2[1], &venus::A2[2]);

    let l0 = calculate_var(t, &venus::L0[0], &venus::L0[1], &venus::L0[2]);
    let l1 = calculate_var(t, &venus::L1[0], &venus::L1[1], &venus::L1[2]);
    let l2 = calculate_var(t, &venus::L2[0], &venus::L2[1], &venus::L2[2]);
    let l3 = calculate_var(t, &venus::L3[0], &venus::L3[1], &venus::L3[2]);

    let k0 = calculate_var(t, &venus::K0[0], &venus::K0[1], &venus::K0[2]);
    let k1 = calculate_var(t, &venus::K1[0], &venus::K1[1], &venus::K1[2]);
    let k2 = calculate_var(t, &venus::K2[0], &venus::K2[1], &venus::K2[2]);
    let k3 = calculate_var(t, &venus::K3[0], &venus::K3[1], &venus::K3[2]);
    let k4 = calculate_var(t, &venus::K4[0], &venus::K4[1], &venus::K4[2]);
    let k5 = calculate_var(t, &venus::K5[0], &venus::K5[1], &venus::K5[2]);

    let h0 = calculate_var(t, &venus::H0[0], &venus::H0[1], &venus::H0[2]);
    let h1 = calculate_var(t, &venus::H1[0], &venus::H1[1], &venus::H1[2]);
    let h2 = calculate_var(t, &venus::H2[0], &venus::H2[1], &venus::H2[2]);
    let h3 = calculate_var(t, &venus::H3[0], &venus::H3[1], &venus::H3[2]);
    let h4 = calculate_var(t, &venus::H4[0], &venus::H4[1], &venus::H4[2]);
    let h5 = calculate_var(t, &venus::H5[0], &venus::H5[1], &venus::H5[2]);

    let q0 = calculate_var(t, &venus::Q0[0], &venus::Q0[1], &venus::Q0[2]);
    let q1 = calculate_var(t, &venus::Q1[0], &venus::Q1[1], &venus::Q1[2]);
    let q2 = calculate_var(t, &venus::Q2[0], &venus::Q2[1], &venus::Q2[2]);
    let q3 = calculate_var(t, &venus::Q3[0], &venus::Q3[1], &venus::Q3[2]);
    let q4 = calculate_var(t, &venus::Q4[0], &venus::Q4[1], &venus::Q4[2]);
    let q5 = calculate_var(t, &venus::Q5[0], &venus::Q5[1], &venus::Q5[2]);

    let p0 = calculate_var(t, &venus::P0[0], &venus::P0[1], &venus::P0[2]);
    let p1 = calculate_var(t, &venus::P1[0], &venus::P1[1], &venus::P1[2]);
    let p2 = calculate_var(t, &venus::P2[0], &venus::P2[1], &venus::P2[2]);
    let p3 = calculate_var(t, &venus::P3[0], &venus::P3[1], &venus::P3[2]);
    let p4 = calculate_var(t, &venus::P4[0], &venus::P4[1], &venus::P4[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t2 * t2;
    let t5 = t2 * t3;

    let a = a0 + a1 * t + a2 * t2;
    let l = (l0 + l1 * t + l2 * t2 + l3 * t3) % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t2 + k3 * t3 + k4 * t4 + k5 * t5;
    let h = h0 + h1 * t + h2 * t2 + h3 * t3 + h4 * t4 + h5 * t5;
    let q = q0 + q1 * t + q2 * t2 + q3 * t3 + q4 * t4 + q5 * t5;
    let p = p0 + p1 * t + p2 * t2 + p3 * t3 + p4 * t4;

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
#[allow(clippy::too_many_lines)]
pub fn earth_moon(jde: f64) -> VSOP87Elements {
    let t = calculate_t(jde);

    let a0 = calculate_var(
        t,
        &earth_moon::A0[0],
        &earth_moon::A0[1],
        &earth_moon::A0[2],
    );
    let a1 = calculate_var(
        t,
        &earth_moon::A1[0],
        &earth_moon::A1[1],
        &earth_moon::A1[2],
    );
    let a2 = calculate_var(
        t,
        &earth_moon::A2[0],
        &earth_moon::A2[1],
        &earth_moon::A2[2],
    );

    let l0 = calculate_var(
        t,
        &earth_moon::L0[0],
        &earth_moon::L0[1],
        &earth_moon::L0[2],
    );
    let l1 = calculate_var(
        t,
        &earth_moon::L1[0],
        &earth_moon::L1[1],
        &earth_moon::L1[2],
    );
    let l2 = calculate_var(
        t,
        &earth_moon::L2[0],
        &earth_moon::L2[1],
        &earth_moon::L2[2],
    );
    let l3 = calculate_var(
        t,
        &earth_moon::L3[0],
        &earth_moon::L3[1],
        &earth_moon::L3[2],
    );
    let l4 = calculate_var(
        t,
        &earth_moon::L4[0],
        &earth_moon::L4[1],
        &earth_moon::L4[2],
    );
    let l5 = calculate_var(
        t,
        &earth_moon::L5[0],
        &earth_moon::L5[1],
        &earth_moon::L5[2],
    );

    let k0 = calculate_var(
        t,
        &earth_moon::K0[0],
        &earth_moon::K0[1],
        &earth_moon::K0[2],
    );
    let k1 = calculate_var(
        t,
        &earth_moon::K1[0],
        &earth_moon::K1[1],
        &earth_moon::K1[2],
    );
    let k2 = calculate_var(
        t,
        &earth_moon::K2[0],
        &earth_moon::K2[1],
        &earth_moon::K2[2],
    );
    let k3 = calculate_var(
        t,
        &earth_moon::K3[0],
        &earth_moon::K3[1],
        &earth_moon::K3[2],
    );
    let k4 = calculate_var(
        t,
        &earth_moon::K4[0],
        &earth_moon::K4[1],
        &earth_moon::K4[2],
    );
    let k5 = calculate_var(
        t,
        &earth_moon::K5[0],
        &earth_moon::K5[1],
        &earth_moon::K5[2],
    );

    let h0 = calculate_var(
        t,
        &earth_moon::H0[0],
        &earth_moon::H0[1],
        &earth_moon::H0[2],
    );
    let h1 = calculate_var(
        t,
        &earth_moon::H1[0],
        &earth_moon::H1[1],
        &earth_moon::H1[2],
    );
    let h2 = calculate_var(
        t,
        &earth_moon::H2[0],
        &earth_moon::H2[1],
        &earth_moon::H2[2],
    );
    let h3 = calculate_var(
        t,
        &earth_moon::H3[0],
        &earth_moon::H3[1],
        &earth_moon::H3[2],
    );
    let h4 = calculate_var(
        t,
        &earth_moon::H4[0],
        &earth_moon::H4[1],
        &earth_moon::H4[2],
    );
    let h5 = calculate_var(
        t,
        &earth_moon::H5[0],
        &earth_moon::H5[1],
        &earth_moon::H5[2],
    );

    let q0 = calculate_var(
        t,
        &earth_moon::Q0[0],
        &earth_moon::Q0[1],
        &earth_moon::Q0[2],
    );
    let q1 = calculate_var(
        t,
        &earth_moon::Q1[0],
        &earth_moon::Q1[1],
        &earth_moon::Q1[2],
    );
    let q2 = calculate_var(
        t,
        &earth_moon::Q2[0],
        &earth_moon::Q2[1],
        &earth_moon::Q2[2],
    );
    let q3 = calculate_var(
        t,
        &earth_moon::Q3[0],
        &earth_moon::Q3[1],
        &earth_moon::Q3[2],
    );
    let q4 = calculate_var(
        t,
        &earth_moon::Q4[0],
        &earth_moon::Q4[1],
        &earth_moon::Q4[2],
    );
    let q5 = calculate_var(
        t,
        &earth_moon::Q5[0],
        &earth_moon::Q5[1],
        &earth_moon::Q5[2],
    );

    let p0 = calculate_var(
        t,
        &earth_moon::P0[0],
        &earth_moon::P0[1],
        &earth_moon::P0[2],
    );
    let p1 = calculate_var(
        t,
        &earth_moon::P1[0],
        &earth_moon::P1[1],
        &earth_moon::P1[2],
    );
    let p2 = calculate_var(
        t,
        &earth_moon::P2[0],
        &earth_moon::P2[1],
        &earth_moon::P2[2],
    );
    let p3 = calculate_var(
        t,
        &earth_moon::P3[0],
        &earth_moon::P3[1],
        &earth_moon::P3[2],
    );
    let p4 = calculate_var(
        t,
        &earth_moon::P4[0],
        &earth_moon::P4[1],
        &earth_moon::P4[2],
    );

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t2 * t2;
    let t5 = t2 * t3;

    let a = a0 + a1 * t + a2 * t2;
    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t2 + k3 * t3 + k4 * t4 + k5 * t5;
    let h = h0 + h1 * t + h2 * t2 + h3 * t3 + h4 * t4 + h5 * t5;
    let q = q0 + q1 * t + q2 * t2 + q3 * t3 + q4 * t4 + q5 * t5;
    let p = p0 + p1 * t + p2 * t2 + p3 * t3 + p4 * t4;

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

    let a0 = calculate_var(t, &mars::A0[0], &mars::A0[1], &mars::A0[2]);
    let a1 = calculate_var(t, &mars::A1[0], &mars::A1[1], &mars::A1[2]);
    let a2 = calculate_var(t, &mars::A2[0], &mars::A2[1], &mars::A2[2]);

    let l0 = calculate_var(t, &mars::L0[0], &mars::L0[1], &mars::L0[2]);
    let l1 = calculate_var(t, &mars::L1[0], &mars::L1[1], &mars::L1[2]);
    let l2 = calculate_var(t, &mars::L2[0], &mars::L2[1], &mars::L2[2]);
    let l3 = calculate_var(t, &mars::L3[0], &mars::L3[1], &mars::L3[2]);
    let l4 = calculate_var(t, &mars::L4[0], &mars::L4[1], &mars::L4[2]);
    let l5 = calculate_var(t, &mars::L5[0], &mars::L5[1], &mars::L5[2]);

    let k0 = calculate_var(t, &mars::K0[0], &mars::K0[1], &mars::K0[2]);
    let k1 = calculate_var(t, &mars::K1[0], &mars::K1[1], &mars::K1[2]);
    let k2 = calculate_var(t, &mars::K2[0], &mars::K2[1], &mars::K2[2]);
    let k3 = calculate_var(t, &mars::K3[0], &mars::K3[1], &mars::K3[2]);
    let k4 = calculate_var(t, &mars::K4[0], &mars::K4[1], &mars::K4[2]);
    let k5 = calculate_var(t, &mars::K5[0], &mars::K5[1], &mars::K5[2]);

    let h0 = calculate_var(t, &mars::H0[0], &mars::H0[1], &mars::H0[2]);
    let h1 = calculate_var(t, &mars::H1[0], &mars::H1[1], &mars::H1[2]);
    let h2 = calculate_var(t, &mars::H2[0], &mars::H2[1], &mars::H2[2]);
    let h3 = calculate_var(t, &mars::H3[0], &mars::H3[1], &mars::H3[2]);
    let h4 = calculate_var(t, &mars::H4[0], &mars::H4[1], &mars::H4[2]);
    let h5 = calculate_var(t, &mars::H5[0], &mars::H5[1], &mars::H5[2]);

    let q0 = calculate_var(t, &mars::Q0[0], &mars::Q0[1], &mars::Q0[2]);
    let q1 = calculate_var(t, &mars::Q1[0], &mars::Q1[1], &mars::Q1[2]);
    let q2 = calculate_var(t, &mars::Q2[0], &mars::Q2[1], &mars::Q2[2]);
    let q3 = calculate_var(t, &mars::Q3[0], &mars::Q3[1], &mars::Q3[2]);
    let q4 = calculate_var(t, &mars::Q4[0], &mars::Q4[1], &mars::Q4[2]);
    let q5 = calculate_var(t, &mars::Q5[0], &mars::Q5[1], &mars::Q5[2]);

    let p0 = calculate_var(t, &mars::P0[0], &mars::P0[1], &mars::P0[2]);
    let p1 = calculate_var(t, &mars::P1[0], &mars::P1[1], &mars::P1[2]);
    let p2 = calculate_var(t, &mars::P2[0], &mars::P2[1], &mars::P2[2]);
    let p3 = calculate_var(t, &mars::P3[0], &mars::P3[1], &mars::P3[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t2 * t2;
    let t5 = t2 * t3;

    let a = a0 + a1 * t + a2 * t2;
    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t2 + k3 * t3 + k4 * t4 + k5 * t5;
    let h = h0 + h1 * t + h2 * t2 + h3 * t3 + h4 * t4 + h5 * t5;
    let q = q0 + q1 * t + q2 * t2 + q3 * t3 + q4 * t4 + q5 * t5;
    let p = p0 + p1 * t + p2 * t2 + p3 * t3;

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

    let a0 = calculate_var(t, &jupiter::A0[0], &jupiter::A0[1], &jupiter::A0[2]);
    let a1 = calculate_var(t, &jupiter::A1[0], &jupiter::A1[1], &jupiter::A1[2]);
    let a2 = calculate_var(t, &jupiter::A2[0], &jupiter::A2[1], &jupiter::A2[2]);
    let a3 = calculate_var(t, &jupiter::A3[0], &jupiter::A3[1], &jupiter::A3[2]);
    let a4 = calculate_var(t, &jupiter::A4[0], &jupiter::A4[1], &jupiter::A4[2]);
    let a5 = calculate_var(t, &jupiter::A5[0], &jupiter::A5[1], &jupiter::A5[2]);

    let l0 = calculate_var(t, &jupiter::L0[0], &jupiter::L0[1], &jupiter::L0[2]);
    let l1 = calculate_var(t, &jupiter::L1[0], &jupiter::L1[1], &jupiter::L1[2]);
    let l2 = calculate_var(t, &jupiter::L2[0], &jupiter::L2[1], &jupiter::L2[2]);
    let l3 = calculate_var(t, &jupiter::L3[0], &jupiter::L3[1], &jupiter::L3[2]);
    let l4 = calculate_var(t, &jupiter::L4[0], &jupiter::L4[1], &jupiter::L4[2]);
    let l5 = calculate_var(t, &jupiter::L5[0], &jupiter::L5[1], &jupiter::L5[2]);

    let k0 = calculate_var(t, &jupiter::K0[0], &jupiter::K0[1], &jupiter::K0[2]);
    let k1 = calculate_var(t, &jupiter::K1[0], &jupiter::K1[1], &jupiter::K1[2]);
    let k2 = calculate_var(t, &jupiter::K2[0], &jupiter::K2[1], &jupiter::K2[2]);
    let k3 = calculate_var(t, &jupiter::K3[0], &jupiter::K3[1], &jupiter::K3[2]);
    let k4 = calculate_var(t, &jupiter::K4[0], &jupiter::K4[1], &jupiter::K4[2]);

    let h0 = calculate_var(t, &jupiter::H0[0], &jupiter::H0[1], &jupiter::H0[2]);
    let h1 = calculate_var(t, &jupiter::H1[0], &jupiter::H1[1], &jupiter::H1[2]);
    let h2 = calculate_var(t, &jupiter::H2[0], &jupiter::H2[1], &jupiter::H2[2]);
    let h3 = calculate_var(t, &jupiter::H3[0], &jupiter::H3[1], &jupiter::H3[2]);
    let h4 = calculate_var(t, &jupiter::H4[0], &jupiter::H4[1], &jupiter::H4[2]);

    let q0 = calculate_var(t, &jupiter::Q0[0], &jupiter::Q0[1], &jupiter::Q0[2]);
    let q1 = calculate_var(t, &jupiter::Q1[0], &jupiter::Q1[1], &jupiter::Q1[2]);
    let q2 = calculate_var(t, &jupiter::Q2[0], &jupiter::Q2[1], &jupiter::Q2[2]);
    let q3 = calculate_var(t, &jupiter::Q3[0], &jupiter::Q3[1], &jupiter::Q3[2]);

    let p0 = calculate_var(t, &jupiter::P0[0], &jupiter::P0[1], &jupiter::P0[2]);
    let p1 = calculate_var(t, &jupiter::P1[0], &jupiter::P1[1], &jupiter::P1[2]);
    let p2 = calculate_var(t, &jupiter::P2[0], &jupiter::P2[1], &jupiter::P2[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t2 * t2;
    let t5 = t2 * t3;

    let a = a0 + a1 * t + a2 * t2 + a3 * t3 + a4 * t4 + a5 * t5;
    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t2 + k3 * t3 + k4 * t4;
    let h = h0 + h1 * t + h2 * t2 + h3 * t3 + h4 * t4;
    let q = q0 + q1 * t + q2 * t2 + q3 * t3;
    let p = p0 + p1 * t + p2 * t2;

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

    let a0 = calculate_var(t, &saturn::A0[0], &saturn::A0[1], &saturn::A0[2]);
    let a1 = calculate_var(t, &saturn::A1[0], &saturn::A1[1], &saturn::A1[2]);
    let a2 = calculate_var(t, &saturn::A2[0], &saturn::A2[1], &saturn::A2[2]);
    let a3 = calculate_var(t, &saturn::A3[0], &saturn::A3[1], &saturn::A3[2]);
    let a4 = calculate_var(t, &saturn::A4[0], &saturn::A4[1], &saturn::A4[2]);
    let a5 = calculate_var(t, &saturn::A5[0], &saturn::A5[1], &saturn::A5[2]);

    let l0 = calculate_var(t, &saturn::L0[0], &saturn::L0[1], &saturn::L0[2]);
    let l1 = calculate_var(t, &saturn::L1[0], &saturn::L1[1], &saturn::L1[2]);
    let l2 = calculate_var(t, &saturn::L2[0], &saturn::L2[1], &saturn::L2[2]);
    let l3 = calculate_var(t, &saturn::L3[0], &saturn::L3[1], &saturn::L3[2]);
    let l4 = calculate_var(t, &saturn::L4[0], &saturn::L4[1], &saturn::L4[2]);
    let l5 = calculate_var(t, &saturn::L5[0], &saturn::L5[1], &saturn::L5[2]);

    let k0 = calculate_var(t, &saturn::K0[0], &saturn::K0[1], &saturn::K0[2]);
    let k1 = calculate_var(t, &saturn::K1[0], &saturn::K1[1], &saturn::K1[2]);
    let k2 = calculate_var(t, &saturn::K2[0], &saturn::K2[1], &saturn::K2[2]);
    let k3 = calculate_var(t, &saturn::K3[0], &saturn::K3[1], &saturn::K3[2]);
    let k4 = calculate_var(t, &saturn::K4[0], &saturn::K4[1], &saturn::K4[2]);
    let k5 = calculate_var(t, &saturn::K5[0], &saturn::K5[1], &saturn::K5[2]);

    let h0 = calculate_var(t, &saturn::H0[0], &saturn::H0[1], &saturn::H0[2]);
    let h1 = calculate_var(t, &saturn::H1[0], &saturn::H1[1], &saturn::H1[2]);
    let h2 = calculate_var(t, &saturn::H2[0], &saturn::H2[1], &saturn::H2[2]);
    let h3 = calculate_var(t, &saturn::H3[0], &saturn::H3[1], &saturn::H3[2]);
    let h4 = calculate_var(t, &saturn::H4[0], &saturn::H4[1], &saturn::H4[2]);
    let h5 = calculate_var(t, &saturn::H5[0], &saturn::H5[1], &saturn::H5[2]);

    let q0 = calculate_var(t, &saturn::Q0[0], &saturn::Q0[1], &saturn::Q0[2]);
    let q1 = calculate_var(t, &saturn::Q1[0], &saturn::Q1[1], &saturn::Q1[2]);
    let q2 = calculate_var(t, &saturn::Q2[0], &saturn::Q2[1], &saturn::Q2[2]);
    let q3 = calculate_var(t, &saturn::Q3[0], &saturn::Q3[1], &saturn::Q3[2]);
    let q4 = calculate_var(t, &saturn::Q4[0], &saturn::Q4[1], &saturn::Q4[2]);

    let p0 = calculate_var(t, &saturn::P0[0], &saturn::P0[1], &saturn::P0[2]);
    let p1 = calculate_var(t, &saturn::P1[0], &saturn::P1[1], &saturn::P1[2]);
    let p2 = calculate_var(t, &saturn::P2[0], &saturn::P2[1], &saturn::P2[2]);
    let p3 = calculate_var(t, &saturn::P3[0], &saturn::P3[1], &saturn::P3[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t2 * t2;
    let t5 = t2 * t3;

    let a = a0 + a1 * t + a2 * t2 + a3 * t3 + a4 * t4 + a5 * t5;
    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t2 + k3 * t3 + k4 * t4 + k5 * t5;
    let h = h0 + h1 * t + h2 * t2 + h3 * t3 + h4 * t4 + h5 * t5;
    let q = q0 + q1 * t + q2 * t2 + q3 * t3 + q4 * t4;
    let p = p0 + p1 * t + p2 * t2 + p3 * t3;

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

    let a0 = calculate_var(t, &uranus::A0[0], &uranus::A0[1], &uranus::A0[2]);
    let a1 = calculate_var(t, &uranus::A1[0], &uranus::A1[1], &uranus::A1[2]);
    let a2 = calculate_var(t, &uranus::A2[0], &uranus::A2[1], &uranus::A2[2]);
    let a3 = calculate_var(t, &uranus::A3[0], &uranus::A3[1], &uranus::A3[2]);
    let a4 = calculate_var(t, &uranus::A4[0], &uranus::A4[1], &uranus::A4[2]);
    let a5 = calculate_var(t, &uranus::A5[0], &uranus::A5[1], &uranus::A5[2]);

    let l0 = calculate_var(t, &uranus::L0[0], &uranus::L0[1], &uranus::L0[2]);
    let l1 = calculate_var(t, &uranus::L1[0], &uranus::L1[1], &uranus::L1[2]);
    let l2 = calculate_var(t, &uranus::L2[0], &uranus::L2[1], &uranus::L2[2]);
    let l3 = calculate_var(t, &uranus::L3[0], &uranus::L3[1], &uranus::L3[2]);
    let l4 = calculate_var(t, &uranus::L4[0], &uranus::L4[1], &uranus::L4[2]);
    let l5 = calculate_var(t, &uranus::L5[0], &uranus::L5[1], &uranus::L5[2]);

    let k0 = calculate_var(t, &uranus::K0[0], &uranus::K0[1], &uranus::K0[2]);
    let k1 = calculate_var(t, &uranus::K1[0], &uranus::K1[1], &uranus::K1[2]);
    let k2 = calculate_var(t, &uranus::K2[0], &uranus::K2[1], &uranus::K2[2]);
    let k3 = calculate_var(t, &uranus::K3[0], &uranus::K3[1], &uranus::K3[2]);
    let k4 = calculate_var(t, &uranus::K4[0], &uranus::K4[1], &uranus::K4[2]);

    let h0 = calculate_var(t, &uranus::H0[0], &uranus::H0[1], &uranus::H0[2]);
    let h1 = calculate_var(t, &uranus::H1[0], &uranus::H1[1], &uranus::H1[2]);
    let h2 = calculate_var(t, &uranus::H2[0], &uranus::H2[1], &uranus::H2[2]);
    let h3 = calculate_var(t, &uranus::H3[0], &uranus::H3[1], &uranus::H3[2]);
    let h4 = calculate_var(t, &uranus::H4[0], &uranus::H4[1], &uranus::H4[2]);

    let q0 = calculate_var(t, &uranus::Q0[0], &uranus::Q0[1], &uranus::Q0[2]);
    let q1 = calculate_var(t, &uranus::Q1[0], &uranus::Q1[1], &uranus::Q1[2]);
    let q2 = calculate_var(t, &uranus::Q2[0], &uranus::Q2[1], &uranus::Q2[2]);
    let q3 = calculate_var(t, &uranus::Q3[0], &uranus::Q3[1], &uranus::Q3[2]);

    let p0 = calculate_var(t, &uranus::P0[0], &uranus::P0[1], &uranus::P0[2]);
    let p1 = calculate_var(t, &uranus::P1[0], &uranus::P1[1], &uranus::P1[2]);
    let p2 = calculate_var(t, &uranus::P2[0], &uranus::P2[1], &uranus::P2[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t2 * t2;
    let t5 = t2 * t3;

    let a = a0 + a1 * t + a2 * t2 + a3 * t3 + a4 * t4 + a5 * t5;
    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t2 + k3 * t3 + k4 * t4;
    let h = h0 + h1 * t + h2 * t2 + h3 * t3 + h4 * t4;
    let q = q0 + q1 * t + q2 * t2 + q3 * t3;
    let p = p0 + p1 * t + p2 * t2;

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

    let a0 = calculate_var(t, &neptune::A0[0], &neptune::A0[1], &neptune::A0[2]);
    let a1 = calculate_var(t, &neptune::A1[0], &neptune::A1[1], &neptune::A1[2]);
    let a2 = calculate_var(t, &neptune::A2[0], &neptune::A2[1], &neptune::A2[2]);
    let a3 = calculate_var(t, &neptune::A3[0], &neptune::A3[1], &neptune::A3[2]);
    let a4 = calculate_var(t, &neptune::A4[0], &neptune::A4[1], &neptune::A4[2]);
    let a5 = calculate_var(t, &neptune::A5[0], &neptune::A5[1], &neptune::A5[2]);

    let l0 = calculate_var(t, &neptune::L0[0], &neptune::L0[1], &neptune::L0[2]);
    let l1 = calculate_var(t, &neptune::L1[0], &neptune::L1[1], &neptune::L1[2]);
    let l2 = calculate_var(t, &neptune::L2[0], &neptune::L2[1], &neptune::L2[2]);
    let l3 = calculate_var(t, &neptune::L3[0], &neptune::L3[1], &neptune::L3[2]);
    let l4 = calculate_var(t, &neptune::L4[0], &neptune::L4[1], &neptune::L4[2]);
    let l5 = calculate_var(t, &neptune::L5[0], &neptune::L5[1], &neptune::L5[2]);

    let k0 = calculate_var(t, &neptune::K0[0], &neptune::K0[1], &neptune::K0[2]);
    let k1 = calculate_var(t, &neptune::K1[0], &neptune::K1[1], &neptune::K1[2]);
    let k2 = calculate_var(t, &neptune::K2[0], &neptune::K2[1], &neptune::K2[2]);
    let k3 = calculate_var(t, &neptune::K3[0], &neptune::K3[1], &neptune::K3[2]);
    let k4 = calculate_var(t, &neptune::K4[0], &neptune::K4[1], &neptune::K4[2]);
    let k5 = calculate_var(t, &neptune::K5[0], &neptune::K5[1], &neptune::K5[2]);

    let h0 = calculate_var(t, &neptune::H0[0], &neptune::H0[1], &neptune::H0[2]);
    let h1 = calculate_var(t, &neptune::H1[0], &neptune::H1[1], &neptune::H1[2]);
    let h2 = calculate_var(t, &neptune::H2[0], &neptune::H2[1], &neptune::H2[2]);
    let h3 = calculate_var(t, &neptune::H3[0], &neptune::H3[1], &neptune::H3[2]);
    let h4 = calculate_var(t, &neptune::H4[0], &neptune::H4[1], &neptune::H4[2]);
    let h5 = calculate_var(t, &neptune::H5[0], &neptune::H5[1], &neptune::H5[2]);

    let q0 = calculate_var(t, &neptune::Q0[0], &neptune::Q0[1], &neptune::Q0[2]);
    let q1 = calculate_var(t, &neptune::Q1[0], &neptune::Q1[1], &neptune::Q1[2]);
    let q2 = calculate_var(t, &neptune::Q2[0], &neptune::Q2[1], &neptune::Q2[2]);
    let q3 = calculate_var(t, &neptune::Q3[0], &neptune::Q3[1], &neptune::Q3[2]);

    let p0 = calculate_var(t, &neptune::P0[0], &neptune::P0[1], &neptune::P0[2]);
    let p1 = calculate_var(t, &neptune::P1[0], &neptune::P1[1], &neptune::P1[2]);
    let p2 = calculate_var(t, &neptune::P2[0], &neptune::P2[1], &neptune::P2[2]);

    // We calculate the `t` potencies beforehand for easy re-use.
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t2 * t2;
    let t5 = t2 * t3;

    let a = a0 + a1 * t + a2 * t2 + a3 * t3 + a4 * t4 + a5 * t5;
    let l = (l0 + l1 * t + l2 * t2 + l3 * t3 + l4 * t4 + l5 * t5) % (2_f64 * PI);
    let k = k0 + k1 * t + k2 * t2 + k3 * t3 + k4 * t4 + k5 * t5;
    let h = h0 + h1 * t + h2 * t2 + h3 * t3 + h4 * t4 + h5 * t5;
    let q = q0 + q1 * t + q2 * t2 + q3 * t3;
    let p = p0 + p1 * t + p2 * t2;

    VSOP87Elements {
        a,
        l: if l > 0_f64 { l } else { 2_f64 * PI + l },
        k,
        h,
        q,
        p,
    }
}
