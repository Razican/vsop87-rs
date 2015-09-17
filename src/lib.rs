pub mod vsop87;
// pub mod vsop87a;
// pub mod vsop87b;
// pub mod vsop87c;
// pub mod vsop87d;
// pub mod vsop87e;

fn calculate_t(jde: f64) -> f64{
    return (jde - 2451545_f64)/365250_f64
}

fn calculate_var(t: f64, var: &[(f64, f64, f64)]) -> f64 {
    var.iter().fold(0_f64, |term, &(a, b, c)| term + a*(b + c*t).cos())
}
