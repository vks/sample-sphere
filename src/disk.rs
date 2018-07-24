//! Sample the unit disk.

use std::f64::consts::PI;

use rand::Rng;
use rand::distributions::{Distribution, Uniform};

/// Sample the unit disk in two Cartesian dimensions.
#[inline]
pub fn trigonometric<R: Rng>(rng: &mut R) -> [f64; 2] {
    let uniform_r = Uniform::<f64>::new_inclusive(0., 1.);
    let uniform_theta = Uniform::new(0., 2.*PI);
    let r = uniform_r.sample(rng).sqrt();
    let theta = uniform_theta.sample(rng);
    [r * theta.cos(), r * theta.sin()]
}

/// Sample the unit disk in two Cartesian dimensions.
#[inline]
pub fn trigonometric_scaled<R: Rng>(rng: &mut R) -> [f64; 2] {
    let uniform = Uniform::new(0., 1.);
    let mut a = uniform.sample(rng);
    let mut b = uniform.sample(rng);
    if b < a {
        ::std::mem::swap(&mut a, &mut b);
    }
    let theta = 2.*PI*a/b;
    [b * theta.cos(), b * theta.sin()]
}

/// Sample the unit disk in two Cartesian dimensions.
#[inline]
pub fn rejection<R: Rng>(rng: &mut R) -> [f64; 2] {
    let uniform = Uniform::new_inclusive(-1., 1.);
    let mut x1;
    let mut x2;
    loop {
        x1 = uniform.sample(rng);
        x2 = uniform.sample(rng);
        if x1*x1 + x2*x2 <= 1. {
            break;
        }
    }
    [x1, x2]
}
