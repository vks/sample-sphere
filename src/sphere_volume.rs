//! Sample the unit sphere volume.

use std::f64::consts::PI;

use rand::Rng;
use rand::distributions::{Distribution, Uniform};

/// Sample the unit sphere volume in three Cartesian dimensions.
#[inline]
pub fn rejection<R: Rng>(rng: &mut R) -> [f64; 3] {
    let uniform = Uniform::<f64>::new_inclusive(-1., 1.);
    let mut x1;
    let mut x2;
    let mut x3;
    loop {
        x1 = uniform.sample(rng);
        x2 = uniform.sample(rng);
        x3 = uniform.sample(rng);
        if x1*x1 + x2*x2 + x3*x3 <= 1. {
            break;
        }
    }
    [x1, x2, x3]
}

/// Sample the unit sphere volume in three Cartesian dimensions.
#[inline]
pub fn trigonometric<R: Rng>(rng: &mut R) -> [f64; 3] {
    let uniform_u = Uniform::<f64>::new_inclusive(0., 1.);
    let uniform_v = Uniform::<f64>::new_inclusive(-1., 1.);
    let uniform_theta = Uniform::new(0., 2.*PI);
    let u = uniform_u.sample(rng);
    let v = uniform_v.sample(rng);
    let theta = uniform_theta.sample(rng);
    let r = u.cbrt();
    let factor = r * (1. - v*v).sqrt();
    [
        factor * theta.cos(),
        factor * theta.sin(),
        r * v,
    ]
}
