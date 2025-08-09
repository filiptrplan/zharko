# Zharko - a simple ray tracer written in Rust

Hello! This is a simple ray tracer written in Rust for my mathematical modelling course. It is not meant to be
a serious project, I just took the opportunity to combine my course work and my Rust learning journey.

## Techniques

*   **Ray-Sphere Intersection:** Implements the mathematical solution to find where a ray intersects with a sphere, a fundamental operation in ray tracing. This follows standard geometric algorithms.
*   **Lambertian Diffuse Reflection:** Simulates matte surfaces using Lambert's cosine law for diffuse reflection. Rays are scattered in random directions weighted by the cosine of the angle between the ray and the surface normal.
*   **Metal Reflection:** Simulates reflective surfaces using vector reflection. Incident rays are reflected across the surface normal.
*   **Gamma Correction:** Applies gamma correction (square root) to linear color values before output to ensure correct brightness on displays. This is a crucial step in any rendering pipeline.
*   **Anti-aliasing (MSAA):** Reduces jagged edges (aliasing) by taking multiple samples per pixel and averaging their colors. This is a form of Multisample Anti-Aliasing (MSAA).

## Features

*   **Progressive Rendering with Progress Bar:** Renders the image sample by sample, providing visual feedback via a progress bar (using the `indicatif` crate). This is useful for long renders.
*   **PPM Image Output:** Renders directly to the PPM image format, a simple and portable format for storing images.

## How to run examples

The individual examples are contained in the `examples/` directory. They can be run with `cargo` like so:

```bash
cargo run --example <example>
```