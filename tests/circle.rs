extern crate rand;

extern crate sample_sphere;

#[macro_use] mod macros;

use rand::FromEntropy;
use sample_sphere::circle;

macro_rules! make_norm_test {
    ($name:ident, $fun:path) => (
        #[test]
        fn $name() {
            let mut rng = rand::rngs::SmallRng::from_entropy();
            for _ in 0..1000 {
                let x = $fun(&mut rng);
                assert_almost_eq!(x[0]*x[0] + x[1]*x[1], 1., 1e-15);
            }
        }
    );
}

make_norm_test!(trigonometric_norm, circle::trigonometric);
make_norm_test!(cook_neumann_norm, circle::cook_neumann);
