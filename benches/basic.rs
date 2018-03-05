#[macro_use]
extern crate criterion;
extern crate rand;

extern crate sample_sphere;

use criterion::{Criterion, Fun};
use rand::Rng;

fn sample_sphere(c: &mut Criterion) {
    c.bench_functions("sample_sphere",
        vec![
            {
                let mut rng = rand::weak_rng();
                Fun::new("rng_overhead", move |b, _| b.iter(
                    || [rng.next_f64(), rng.next_f64()]))
            },
            {
                let mut rng = rand::weak_rng();
                Fun::new("spherical", move |b, _| b.iter(
                    || sample_sphere::spherical(&mut rng)))
            },
            {
                let mut rng = rand::weak_rng();
                Fun::new("trigonometric", move |b, _| b.iter(
                    || sample_sphere::trigonometric(&mut rng)))
            },
            {
                let mut rng = rand::weak_rng();
                Fun::new("marsaglia", move |b, _| b.iter(
                    || sample_sphere::marsaglia(&mut rng)))
            },
            {
                let mut rng = rand::weak_rng();
                Fun::new("cook_neumann", move |b, _| b.iter(
                    || sample_sphere::cook_neumann(&mut rng)))
            },
            {
                let mut rng = rand::weak_rng();
                Fun::new("normal", move |b, _| b.iter(
                    || sample_sphere::normal(&mut rng)))
            },
        ],
        ());
}

criterion_group!(benches, sample_sphere);
criterion_main!(benches);
