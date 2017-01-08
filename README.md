# VSOP87 Rust implementation #
[![Build Status](https://travis-ci.org/Razican/vsop87-rs.svg?branch=master)](https://travis-ci.org/Razican/vsop87-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/g028p4t0ekvcypu3?svg=true)](https://ci.appveyor.com/project/Razican/vsop87-rs)
[![Coverage Status](https://coveralls.io/repos/Razican/vsop87-rs/badge.svg?branch=develop&service=github)](https://coveralls.io/github/Razican/vsop87-rs?branch=master)
[![Crates.io](https://meritbadge.herokuapp.com/vsop87)](https://crates.io/crates/vsop87)

This library implements the *VSOP87* solutions to calculate the positions of the planets in the
solar system. Full **documentation** can be found [here](https://docs.rs/vsop87/). To use it you
must include the following in your crate:

```rust
extern crate vsop87;
```

The main module calculates heliocentric ecliptic orbital elements for the equinox J2000.0 for
the planets in the solar system, the basic *VSOP87* solution. There is one module per other
*VSOP87* implementation: *VSOP87A*, *VSOP87B*, *VSOP87C*, *VSOP87D* and *VSOP87E*. More
information can be found [here](https://www.caglow.com/info/compute/vsop87) and
[here](https://en.wikipedia.org/wiki/VSOP_(planets)).

Each module has its own documentation, and here is the documentation on the base *VSOP87*
solution. The *VSOP87* algorithm has great precission (under 1") for **4,000 years** before and
after J2000 epoch for Mercury, Venus, Earth-Moon barycenter and Mars, for **2,000 years** in
the case of Jupiter and Saturn and for **6,000 years** for Uranus and Neptune.

The base *VSOP87* solution calculates the
[orbital elements](https://en.wikipedia.org/wiki/Orbital_elements) of the planets arount the
Sun. The returned elements are a special VSOP87 orbital elements, that can be converted into
usual keplerian elements using the `Into` trait. These elements are ideal to get an idea on how
the orbits are changing over time. It can also be used for other complex orbital computations.

## Example

As an example, here we calculate the orbital parameters for Mercury on the January 1st, 2000.
The *VSOP87* algorithm requires dates to be entered as
[Julian Day](https://en.wikipedia.org/wiki/Julian_day) (*JD*). In our case, that date is
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
`==`. Those numbers are retrieved from the test data of the *VSOP87* algorithm.
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

As you can see, these numbers perfectly match
[those from NASA](http://solarsystem.nasa.gov/planets/mercury/facts).

## License ##

This library is distributed under the terms of both the MIT license and the
Apache License (Version 2.0), at your option. See LICENSE-APACHE, and LICENSE-MIT files for
details.
