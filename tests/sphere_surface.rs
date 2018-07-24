extern crate rand;

extern crate sample_sphere;

use rand::FromEntropy;
use sample_sphere::sphere_surface;

/// Assert that two numbers are almost equal to each other.
///
/// On panic, this macro will print the values of the expressions with their
/// debug representations.
macro_rules! assert_almost_eq {
    ($a:expr, $b:expr, $prec:expr) => (
        let diff = ($a - $b).abs();
        if diff > $prec {
            panic!(format!(
                "assertion failed: `abs(left - right) = {:.1e} < {:e}`, \
                 (left: `{}`, right: `{}`)",
                diff, $prec, $a, $b));
        }
    );
}

macro_rules! make_norm_test {
    ($name:ident, $fun:path) => (
        #[test]
        fn $name() {
            let mut rng = rand::rngs::SmallRng::from_entropy();
            for _ in 0..1000 {
                let x = $fun(&mut rng);
                assert_almost_eq!(x[0]*x[0] + x[1]*x[1] + x[2]*x[2], 1., 1e-15);
            }
        }
    );
}

#[inline]
fn from_spherical<R: rand::Rng>(rng: &mut R) -> [f64; 3] {
    let y = sphere_surface::spherical(rng);
    let theta = y[0];
    let phi = y[1];
    sphere_surface::spherical_to_cartesian([1., theta, phi])
}

make_norm_test!(spherical_norm, from_spherical);
make_norm_test!(trigonometric_norm, sphere_surface::trigonometric);
make_norm_test!(marsaglia_norm, sphere_surface::marsaglia);
make_norm_test!(cook_neumann_norm, sphere_surface::cook_neumann);
make_norm_test!(normal_norm, sphere_surface::normal);