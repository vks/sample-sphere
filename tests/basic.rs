extern crate rand;

extern crate sample_sphere;

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

macro_rules! make_basic_test {
    ($name:ident, $fun:path) => (
        #[test]
        fn $name() {
            let mut rng = rand::weak_rng();
            for _ in 0..1000 {
                let x = $fun(&mut rng);
                assert_almost_eq!(x[0]*x[0] + x[1]*x[1] + x[2]*x[2], 1., 1e-15);
            }
        }
    );
}

#[inline]
fn from_spherical<R: rand::Rng>(rng: &mut R) -> [f64; 3] {
    let y = sample_sphere::spherical(rng);
    let theta = y[0];
    let phi = y[1];
    sample_sphere::spherical_to_cartesian([1., theta, phi])
}

make_basic_test!(spherical, from_spherical);
make_basic_test!(trigonometric, sample_sphere::trigonometric);
make_basic_test!(marsaglia, sample_sphere::marsaglia);
make_basic_test!(cook_neumann, sample_sphere::cook_neumann);
make_basic_test!(normal, sample_sphere::normal);