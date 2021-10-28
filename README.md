# A (tiny) Spline Rust library



# Usage


Spliny is developed as part of a family of three crates, but can be used independently too:

- **splinify** fits (non-uniform) [B-Spline](b-splines) curves to input data,
and results in a fitted as a `spliny`-crate `CurveSpline`.
Data inputs are `x` and `y` vectors for 1 dimensional curves,
and `u` and `xyn` vectors in case of N-dimensional curves.

- Use **spliny** to to use the generated splines, for example to calculate curve coordinates, or a spline-curve's derivatives.
This package also implements basic tools for input and output of spline representations in form of JSON files, and spline plots.
It is completely written in Rust, and does **not** require a Fortran compiler. 

- **dierckx-sys** contains Fortran foreign function interfaces to Paul Dierckx' FITPACK library. 
It is used by `splinify`, but ---unless you want to explore Paul Dierckx library yourself--- can be ignored as concerned to using `splinify` and `spliny`.

To use this library add this to your `Cargo.toml` file:

```
[dependencies]
spliny = "0.1"
```

# Spline Curve
The base spline representation in `Spliny` is the `SplineCurve<K,N>` object ---a wrapper for an vector of knots, and
fit coefficients--- with *K* the spline degree, *N* the space dimension of the curve spline.

For convenience, the following aliases have been defined:

|          Alias        | K | N |
|-----------------------|:-:|:-:|
| `LinearSpline`        | 1 | 1 |
| `CubicSpline`         | 3 | 1 |
| `QuinticSpline`       | 5 | 1 |
| `LinearSpline2D`      | 1 | 2 |
| `CubicSpline2D`       | 3 | 2 |
| `QuinticSpline2D`     | 5 | 2 |
| `LinearSpline3D`      | 1 | 3 |
| `CubicSpline3D`       | 3 | 3 |
| `QuinticSpline3D`     | 5 | 3 |


# License
All content &copy;2021 Harbers Bik LLC, and licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>?)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
