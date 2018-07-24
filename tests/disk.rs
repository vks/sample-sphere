extern crate rand;

extern crate sample_sphere;

use rand::FromEntropy;
use sample_sphere::disk;

macro_rules! make_norm_test {
    ($name:ident, $fun:path) => (
        #[test]
        fn $name() {
            let mut rng = rand::rngs::SmallRng::from_entropy();
            for _ in 0..1000 {
                let x = $fun(&mut rng);
                assert!(x[0]*x[0] + x[1]*x[1] <= 1.);
            }
        }
    );
}

make_norm_test!(trigonometric_norm, disk::trigonometric);
make_norm_test!(trigonometric_scaled_norm, disk::trigonometric_scaled);
make_norm_test!(rejection_norm, disk::rejection);
