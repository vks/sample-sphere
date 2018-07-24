extern crate rand;

extern crate sample_sphere;

use rand::FromEntropy;
use sample_sphere::sphere_volume;

macro_rules! make_norm_test {
    ($name:ident, $fun:path) => (
        #[test]
        fn $name() {
            let mut rng = rand::rngs::SmallRng::from_entropy();
            for _ in 0..1000 {
                let x = $fun(&mut rng);
                assert!(x[0]*x[0] + x[1]*x[1] + x[2]*x[2] <= 1.);
            }
        }
    );
}

make_norm_test!(trigonometric_norm, sphere_volume::trigonometric);
make_norm_test!(rejection_norm, sphere_volume::rejection);
