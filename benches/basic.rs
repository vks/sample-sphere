#[macro_use]
extern crate criterion;
extern crate rand;

extern crate sample_sphere;

use criterion::{Criterion, Fun};
use rand::{Rng, FromEntropy};
use sample_sphere::{sphere_surface, sphere_volume, circle, disk};

fn sample_sphere_surface(c: &mut Criterion) {
    c.bench_functions("sample_sphere_surface",
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

fn sample_sphere_volume(c: &mut Criterion) {
    c.bench_functions("sample_sphere_volume",
        vec![
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("rng_overhead", move |b, _| b.iter(
                    || [rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()]))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("rejection", move |b, _| b.iter(
                    || sphere_volume::rejection(&mut rng)))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("trigonometric", move |b, _| b.iter(
                    || sphere_volume::trigonometric(&mut rng)))
            },
        ],
        ());
}

fn sample_circle(c: &mut Criterion) {
    c.bench_functions("sample_circle",
        vec![
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("rng_overhead", move |b, _| b.iter(
                    || rng.gen::<f64>()))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("cook_neumann", move |b, _| b.iter(
                    || circle::cook_neumann(&mut rng)))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("trigonometric_pythagoras", move |b, _| b.iter(
                    || circle::trigonometric_pythagoras(&mut rng)))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("trigonometric", move |b, _| b.iter(
                    || circle::trigonometric(&mut rng)))
            },
        ],
        ());
}

fn sample_disk(c: &mut Criterion) {
    c.bench_functions("sample_disk",
        vec![
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("rng_overhead", move |b, _| b.iter(
                    || [rng.gen::<f64>(), rng.gen::<f64>()]))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("rejection", move |b, _| b.iter(
                    || disk::rejection(&mut rng)))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("trigonometric", move |b, _| b.iter(
                    || disk::trigonometric(&mut rng)))
            },
            {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                Fun::new("trigonometric_scaled", move |b, _| b.iter(
                    || disk::trigonometric_scaled(&mut rng)))
            },
        ],
        ());
}

criterion_group!(benches,
    sample_sphere_surface, sample_sphere_volume, sample_circle, sample_disk);
criterion_main!(benches);
