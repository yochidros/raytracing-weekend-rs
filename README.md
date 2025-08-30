# Ray Tracing in Rust

https://raytracing.github.io/books/RayTracingInOneWeekend.html

A compact ray tracer implemented in Rust. It renders PPM images and demonstrates core techniques: ray generation, surface intersection, materials and scattering, and a simple camera model.

## Overview

- Purpose: educational, readable implementation of a basic path tracer.
- Output: writes a ".ppm" image to the repo root (e.g., "test.ppm").
- Performance: supports debug and optimized builds; quality vs. speed is adjustable via image size, samples per pixel, and recursion depth.

## Features

- Rays, vectors, and color utilities (`ray.rs`, `vec3.rs`, `color.rs`).
- Geometry and hit logic (`sphere.rs`, `hit_record.rs`, `interval.rs`).
- Materials with diffuse/metal/dielectric scattering (`material.rs`).
- Camera with FOV, focus, sampling, and aspect control (`camera.rs`).
- Minimal scene setup in `main.rs` producing a PPM image.

## Workspace

This repository is a Cargo workspace. The current CPU renderer lives in the `single-cpu` crate.

- Build: `cargo build` or `cargo build -p single-cpu`
- Run (debug): `cargo run -p single-cpu`
- Run (release): `cargo run -p single-cpu --release`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Format: `cargo fmt --all`

The renderer writes a PPM image (e.g., `test.ppm`) to the repository root when run from the workspace root.
