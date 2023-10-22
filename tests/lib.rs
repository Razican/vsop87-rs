use vsop87::*;

#[test]
fn it_kepler() {
    let vsop87_elts = vsop87::mercury(2451545.0);

    assert!(vsop87_elts.a > 0.3870982121 && vsop87_elts.a < 0.3870982123);
    assert!(vsop87_elts.l > 4.4026057778 && vsop87_elts.l < 4.4026057780);
    assert!(vsop87_elts.k > 0.0446647517 && vsop87_elts.k < 0.0446647519);
    assert!(vsop87_elts.h > 0.2007208957 && vsop87_elts.h < 0.2007208959);
    assert!(vsop87_elts.q > 0.0406161540 && vsop87_elts.q < 0.0406161542);
    assert!(vsop87_elts.p > 0.04563512 && vsop87_elts.p < 0.04563588);

    let elements = KeplerianElements::from(vsop87_elts);

    assert!(elements.semimajor_axis() > 0.387097 && elements.semimajor_axis() < 0.387099);
    assert!(elements.eccentricity() > 0.205629 && elements.eccentricity() < 0.205631);
    assert!(elements.inclination() > 0.122260 && elements.inclination() < 0.122261);
    assert!(elements.ascending_node() > 0.843525 && elements.ascending_node() < 0.843527);
    assert!(elements.periapsis() > 1.35183 && elements.periapsis() < 1.35185);
    assert!(elements.mean_anomaly() > 4.40259 && elements.mean_anomaly() < 4.40261);
}

#[test]
fn it_convert() {
    let vsop87_elts = vsop87::mercury(2451545.0);

    assert!(vsop87_elts.a > 0.3870982121 && vsop87_elts.a < 0.3870982123);
    assert!(vsop87_elts.l > 4.4026057778 && vsop87_elts.l < 4.4026057780);
    assert!(vsop87_elts.k > 0.0446647517 && vsop87_elts.k < 0.0446647519);
    assert!(vsop87_elts.h > 0.2007208957 && vsop87_elts.h < 0.2007208959);
    assert!(vsop87_elts.q > 0.0406161540 && vsop87_elts.q < 0.0406161542);
    assert!(vsop87_elts.p > 0.04563512 && vsop87_elts.p < 0.04563588);

    let kepler_elements = KeplerianElements::from(vsop87_elts);
    let new_vsop87_elts: VSOP87Elements = kepler_elements.into();

    assert!(new_vsop87_elts.a > 0.3870982121 && new_vsop87_elts.a < 0.3870982123);
    assert!(new_vsop87_elts.l > 4.4026057778 && new_vsop87_elts.l < 4.4026057780);
    assert!(new_vsop87_elts.k > 0.0446647517 && new_vsop87_elts.k < 0.0446647519);
    assert!(new_vsop87_elts.h > 0.2007208957 && new_vsop87_elts.h < 0.2007208959);
    assert!(new_vsop87_elts.q > 0.0406161540 && new_vsop87_elts.q < 0.0406161542);
    assert!(new_vsop87_elts.p > 0.04563512 && new_vsop87_elts.p < 0.04563588);
}
