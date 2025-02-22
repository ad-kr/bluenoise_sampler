# Precomputed blue noise for fast sampling

This crate provides precomputed blue noise for fast sampling. The blue noise is generated with the
[`bluenoise`](https://docs.rs/bluenoise) crate and is stored in an array for later use.

# Scaling

The blue noise can be scaled based on a desired radius. The radius is the distance between each sample, just like in
a regular Poisson disk sampling algorithm.

# Example

```rs
use bluenoise_sampler::BlueNoiseSampler;

let noise_sampler = BlueNoiseSampler::new(512.0, 256.0, 4.0);
let samples = noise_sampler.get_samples(); // Returns Vec<(f32, f32)>
```
