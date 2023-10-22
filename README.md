# VSOP87 Rust implementation

[![Build Status][build_badge]][build_link]
[![codecov](https://codecov.io/gh/Razican/vsop87-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/Razican/vsop87-rs)
[![Crates.io](https://img.shields.io/crates/v/vsop87.svg)](https://crates.io/crates/vsop87)
[![Docs.rs](https://docs.rs/vsop87/badge.svg)](https://docs.rs/vsop87)

[build_badge]: https://github.com/Razican/vsop87-rs/actions/workflows/rust.yml/badge.svg?event=push&branch=main
[build_link]: https://github.com/Razican/vsop87-rs/actions/workflows/rust.yml?query=event%3Apush+branch%3Amain

This library implements the *VSOP87* solutions to calculate the positions of the planets in the
solar system. Full **documentation** can be found [here][docs_link].

The main module calculates heliocentric ecliptic orbital elements for the equinox J2000.0 for
the planets in the solar system, the basic *VSOP87* solution. There is one module per other
*VSOP87* implementation: *VSOP87A*, *VSOP87B*, *VSOP87C*, *VSOP87D* and *VSOP87E*. More
information can be found [here][vsop87_compute] and [here][vsop87_wiki].

Each module has its own documentation, and here is the documentation on the base *VSOP87*
solution. The *VSOP87* algorithm has great precision (under 1") for **4,000 years** before and
after J2000 epoch for Mercury, Venus, Earth-Moon barycenter and Mars, for **2,000 years** in
the case of Jupiter and Saturn and for **6,000 years** for Uranus and Neptune.

The base *VSOP87* solution calculates the [orbital elements][orb_elem_wiki] of the planets around
the Sun. The returned elements are a special VSOP87 orbital elements, that can be converted into
usual keplerian elements using the `Into` trait. These elements are ideal to get an idea on how
the orbits are changing over time. It can also be used for other complex orbital computations.

## Example

As an example, here we calculate the orbital parameters for Mercury on the January 1st, 2000.
The *VSOP87* algorithm requires dates to be entered as
[Julian Day][julian_day_wiki] (*JD*). In our case, that date is
`2451545.0`.

We first calculate the VSOP87 elements:

```rust
let vsop87_elts = vsop87::mercury(2451545.0);

assert!(vsop87_elts.a > 0.3870982121 && vsop87_elts.a < 0.3870982123);
assert!(vsop87_elts.l > 4.4026057778 && vsop87_elts.l < 4.4026057780);
assert!(vsop87_elts.k > 0.0446647517 && vsop87_elts.k < 0.0446647519);
assert!(vsop87_elts.h > 0.2007208957 && vsop87_elts.h < 0.2007208959);
assert!(vsop87_elts.q > 0.0406161540 && vsop87_elts.q < 0.0406161542);
assert!(vsop87_elts.p > 0.04563512 && vsop87_elts.p < 0.04563588);
```

Note that the `>` and `<` comparisons are there because floats should not be compared using
`==`. Those numbers are retrieved from the original test data of the *VSOP87* algorithm.
We can then convert them into keplerian elements, by using both `KeplerianElements::from()` or
the `into()` function in the *VSOP87* elements. This also works the other way around:

```rust
use vsop87::{KeplerianElements, VSOP87Elements};

let elements = KeplerianElements::from(vsop87_elts);
let convert_back: VSOP87Elements = elements.into();

assert!(elements.semimajor_axis() > 0.387097 && elements.semimajor_axis() < 0.387099);
assert!(elements.eccentricity() > 0.205629 && elements.eccentricity() < 0.205631);
assert!(elements.inclination() > 0.122260 && elements.inclination() < 0.122261);
assert!(elements.ascending_node() > 0.843525 && elements.ascending_node() < 0.843527);
assert!(elements.periapsis() > 1.35183 && elements.periapsis() < 1.35185);
assert!(elements.mean_anomaly() > 4.40259 && elements.mean_anomaly() < 4.40261);
```

As you can see, these numbers perfectly match [those from NASA][nasa_mercury_facts].

## License

This library is distributed under the terms of both the MIT license and the
Apache License (Version 2.0), at your option. See LICENSE-APACHE, and LICENSE-MIT files for
details.

[docs_link]: https://docs.rs/vsop87/
[vsop87_compute]: https://www.caglow.com/info/compute/vsop87
[vsop87_wiki]: https://en.wikipedia.org/wiki/VSOP_(planets)
[julian_day_wiki]: https://en.wikipedia.org/wiki/Julian_day
[orb_elem_wiki]: https://en.wikipedia.org/wiki/Orbital_elements
[nasa_mercury_facts]: https://solarsystem.nasa.gov/planets/mercury/facts