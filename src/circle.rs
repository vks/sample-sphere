//! Sample the unit circle.

use std::f64::consts::PI;

use rand::Rng;
use rand::distributions::{Distribution, Uniform};

/// Sample the unit circle in two Cartesian dimensions.
#[inline]
pub fn trigonometric<R: Rng>(rng: &mut R) -> [f64; 2] {
    let uniform = Uniform::new(0., 2.*PI);
    let theta = uniform.sample(rng);
    [theta.cos(), theta.sin()]
}

/// Sample the unit circle in two Cartesian dimensions.
#[inline]
pub fn cook_neumann<R: Rng>(rng: &mut R) -> [f64; 2] {
    let uniform = Uniform::new(-1., 1.);
    let mut x1;
    let mut x2;
    loop {
        x1 = uniform.sample(rng);
        x2 = uniform.sample(rng);
        if x1*x1 + x2*x2 < 1. {
            break;
        }
    }
    [(x1*x1 - x2*x2) / (x1*x1 + x2*x2), 2.*x1*x2 / (x1*x1 + x2*x2)]
}

/// Sample the unit circle in two Cartesian dimensions.
#[inline]
pub fn trigonometric_pythagoras<R: Rng>(rng: &mut R) -> [f64; 2] {
    let uniform = Uniform::<f64>::new(-1., 1.);
    let x = uniform.sample(rng).sin();
    [x, (1. - x*x).sqrt()]
}
