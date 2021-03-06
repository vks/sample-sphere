//! Sample the unit sphere surface.

use std::f64::consts::PI;

use rand::Rng;
use rand::distributions::{Distribution, Uniform, Normal};

/// Sample the unit sphere surface in spherical coordinates (theta, phi).
#[inline]
pub fn spherical<R: Rng>(rng: &mut R) -> [f64; 2] {
    let uniform = Uniform::new(0., 1.);
    let (u, v) = (uniform.sample(rng), uniform.sample(rng));
    [2.*PI*u, (2.*v - 1.).acos()]
}

/// Convert from spherical coordinates to cartesian coordinates.
#[inline]
pub fn spherical_to_cartesian(x: [f64; 3]) -> [f64; 3] {
    let (r, theta, phi) = (x[0], x[1], x[2]);
    [r * theta.cos() * phi.sin(), r * theta.sin() * phi.sin(), r * phi.cos()]
}

/// Sample the unit sphere surface in three Cartesian dimensions.
#[inline]
pub fn trigonometric<R: Rng>(rng: &mut R) -> [f64; 3] {
    let uniform01 = Uniform::new(0., 1.);
    let uniform11 = Uniform::new(-1., 1.);
    let (u, v) = (uniform11.sample(rng), uniform01.sample(rng));
    let theta = 2.*PI*v;
    let (s, c) = theta.sin_cos();
    let factor = (1.0_f64 - u*u).sqrt();
    [factor * c, factor * s, u]
}

/// Sample the unit sphere surface in three Cartesian dimensions.
#[inline]
pub fn marsaglia<R: Rng>(rng: &mut R) -> [f64; 3] {
    let uniform = Uniform::new(-1., 1.);

    loop {
        let (x1, x2) = (uniform.sample(rng), uniform.sample(rng));
        let (x1_sq, x2_sq) = (x1*x1, x2*x2);
        let sum = x1_sq + x2_sq;
        if sum >= 1. {
            continue;
        }
        let factor = (1.0_f64 - x1_sq - x2_sq).sqrt();
        return [2.*x1 * factor, 2.*x2 * factor, 1. - 2.*sum];
    }
}

/// Sample the unit sphere surface in three Cartesian dimensions.
#[inline]
pub fn cook_neumann<R: Rng>(rng: &mut R) -> [f64; 3] {
    let uniform = Uniform::new(-1., 1.);
    loop {
        let (x0, x1, x2, x3) =
            (uniform.sample(rng), uniform.sample(rng),
             uniform.sample(rng), uniform.sample(rng));
        let (x0_sq, x1_sq, x2_sq, x3_sq) = (x0*x0, x1*x1, x2*x2, x3*x3);
        let sum = x0_sq + x1_sq + x2_sq + x3_sq;
        if sum >= 1. {
            continue;
        }
        let inv_sum = 1. / sum;
        return [
            2.*(x1*x3 + x0*x2) * inv_sum,
            2.*(x2*x3 - x0*x1) * inv_sum,
            (x0_sq + x3_sq - x1_sq - x2_sq) * inv_sum,
        ];
    }
}

/// Sample the unit sphere surface in three Cartesian dimensions.
#[inline]
pub fn normal<R: Rng>(rng: &mut R) -> [f64; 3] {
    let dist = Normal::new(0., 1.);
    let (x, y, z) =
        (dist.sample(rng), dist.sample(rng), dist.sample(rng));
    let factor = 1. / (x*x + y*y + z*z).sqrt();
    [x * factor, y * factor, z * factor]
}
