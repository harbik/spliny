# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-03-04

### Added

- Runtime assertion in `new()` and `try_new()` that `K < DE_BOOR_SIZE`, preventing silent out-of-bounds access for spline degrees ≥ 6.
- Runtime assertion in all `plot*` methods that `N <= 2`, with a clear panic message for unsupported curve dimensions.
- Doc comments on all `plot*` methods clarifying they only support 1D and 2D curves.
- Doc comments on type aliases in `lib.rs` indicating plot support (1D/2D) or lack thereof (3D).

### Changed

- Plot methods (`plot`, `plot_with_parameter`, `plot_with_control_points`, `plot_with_data`, `plot_with_control_points_and_data`) now take `&self` instead of consuming `self`.
- `SplineCurves::spline_curve` iterates over `&self.keys` by reference instead of by value.
- Test `.plot()` calls are now guarded behind `#[cfg(feature = "plot")]`.

### Fixed

- `eval()` now correctly evaluates all N dimensions via the de Boor algorithm (previously only the last dimension's coefficients were loaded and a single `f64` was returned). The method now returns `[f64; DE_BOOR_SIZE]` with all dimensions populated.
- `SplineCurve` no longer stores redundant `k` and `n` fields that duplicated the const generic parameters `K` and `N`.
- Error messages in `try_new()` and `evaluate()` reported an incorrect required coefficient count; they now correctly state `N*(K+1)` (minimum) and `N*(n-K-1)` (exact expected).
- All internal uses of `self.k` replaced with the const generic `K`, removing the runtime field indirection.
- `plot.rs`: `range_spline` used `step_by(1)` (a no-op) when computing per-axis bounds for 2D curves; corrected to `step_by(2)` so x and y ranges are computed independently.
- `plot.rs`: replaced `if xy.is_some() { xy.unwrap() }` with an idiomatic `if let Some(xy) = xy` pattern.
- `plot.rs`: removed a hardcoded nanometer-wavelength axis label formatter that was specific to spectral data and had leaked into the generic plotting utility.
- README Example 1 doctest marked `ignore` (it depends on the `splinify` crate and cannot compile in isolation).
- Removed `serde` and `serde_json` from hard dependencies; neither was used by the library itself.
- Removed dead commented-out `deboor_derivative` stub.
- Removed duplicate `evaluate` calls in `linear_bspline` and `quartic_bspline` tests.
- Added documentation to `SplineCurves` explaining the interior-knot storage format, boundary-repetition convention, and the `i32` knot type.
- Added tests for error paths (`try_new` with too few coefficients, `evaluate` with non-monotonic params, `evaluate` with coefficient count mismatch), out-of-range clamping, `eval` signed-distance error return, `eval` interval-cache reset on backward step, 2D curve evaluation (`SplineCurve<1,2>`), `transpose` for 2D and 3D coordinate vectors, and `SplineCurves::spline_curve` with a missing key.

## [0.2.0] - 2024-09-17

### Added

- Optional plotting support behind the `plot` feature (Plotters backend).
- 2D cubic spline plotting utility (example: `cubic2d`).

### Changed

- Documentation updates and additional examples.

## [0.1.0] - 2021-12-27

### Added

- Initial release: spline curve types based on knots/control points.
- JSON (de)serialization via `serde`.

[0.3.0]: https://github.com/harbik/spliny/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/harbik/spliny/releases/tag/v0.2.0
[0.1.0]: https://github.com/harbik/spliny/releases/tag/v0.1.0
