# j-Invariant Approximation in Rust

This project computes an approximation of the modular function $j(\tau)$ using the truncated $q$-expansion. The implementation is written in Rust using the [`num-complex`](https://docs.rs/num-complex) crate.

## Overview

The modular $j$-invariant is a fundamental object in the theory of elliptic curves and modular forms. It is computed using the formula:

$j(\tau) = \frac{1}{q} + 744 + 196884q + 21493760q^2 + 864299970q^3 + \dots$

where $q = e^{2\pi i \tau}$ and $\tau$ is a complex number in the upper half-plane.

This implementation approximates $j(\tau)$ for $\tau = \frac{-1 + \sqrt{11}i}{2}$, which corresponds to a discriminant $D = -11$.

## Usage

### Prerequisites

Ensure you have Rust installed. If not, install it using [rustup](https://rustup.rs/).

### Installation

Clone this repository:

```sh
git clone https://github.com/cypriansakwa/Computing_the_j_Invariant_in_Rust_Using_Complex_Arithmetic_and_Modular_Functions.git
cd Computing_the_j_Invariant_in_Rust_Using_Complex_Arithmetic_and_Modular_Functions
```
## Running the Program
To run the Rust program, execute:
```
cargo run
```
## Expected Output
The program prints an approximation of the $j$-invariant, rounding its real part:
```
j-invariant approximation (real part, rounded): -32768
```
## Code
```
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
```
## Dependencies
This project uses the `num-complex` crate. Add it to your `Cargo.toml`:
```
[dependencies]
num-bigint = "0.4"
num-complex = "0.4"
num-traits = "0.2"
```

