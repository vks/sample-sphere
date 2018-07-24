#[macro_use]
extern crate criterion;
extern crate rand;

extern crate sample_sphere;

use criterion::{Criterion, Fun};
use rand::{Rng, FromEntropy};
use sample_sphere::sphere_surface;

fn sample_sphere(c: &mut Criterion) {
    c.bench_functions("sample_sphere",
        vec![
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("rng_overhead", move |b, _| b.iter(
                    || [rng.gen::<f64>(), rng.gen::<f64>()]))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("marsaglia", move |b, _| b.iter(
                    || sphere_surface::marsaglia(&mut rng)))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("spherical", move |b, _| b.iter(
                    || sphere_surface::spherical(&mut rng)))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("normal", move |b, _| b.iter(
                    || sphere_surface::normal(&mut rng)))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("cook_neumann", move |b, _| b.iter(
                    || sphere_surface::cook_neumann(&mut rng)))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("trigonometric", move |b, _| b.iter(
                    || sphere_surface::trigonometric(&mut rng)))
            },
        ],
        ());
}

criterion_group!(benches, sample_sphere);
criterion_main!(benches);
