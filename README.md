# sample-sphere

A collection of algorithms to sample from the unit sphere surface and volume,
the unit circle and the unit disk, implemented in Rust.


## Usage

Use `cargo bench` to run the benchmarks. This will create
plots similar to the following:

![Sampling the unit sphere surface](plots/sphere_surface.svg)
![Sampling the unit sphere volume](plots/sphere_volume.svg)
![Sampling the unit circle](plots/circle.svg)
![Sampling the unit disk](plots/disk.svg)

The time corresponds to using the benchmarked algorithm to sample 1000 points.
`rng_overhead` corresponds to the minimal work required by the random number
generator to create the sample, giving a lower bound on the runtime of any
algorithm.
