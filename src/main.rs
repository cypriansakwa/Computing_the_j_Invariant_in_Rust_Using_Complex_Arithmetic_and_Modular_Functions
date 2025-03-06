use num_complex::Complex;
use std::f64::consts::PI;

fn main() {
    let sqrt_11 = (11.0_f64).sqrt();
    let tau = Complex::new(-1.0, sqrt_11) / 2.0;

    let two_pi_i = Complex::new(0.0, 2.0 * PI);
    let q = (two_pi_i * tau).exp();

    // Compute j(τ) using the truncated q-expansion
    let j_approx = Complex::new(1.0, 0.0) / q
        + Complex::new(744.0, 0.0)
        + Complex::new(196884.0, 0.0) * q
        + Complex::new(21493760.0, 0.0) * q * q
        + Complex::new(864299970.0, 0.0) * q * q * q;

        println!("j-invariant approximation (real part, rounded): {}", j_approx.re.round());

}
