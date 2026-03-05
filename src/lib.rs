#![doc = include_str!("../README.md")]

/// Convenience `Result` type used throughout this crate, with a boxed dynamic error.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// 1D linear spline. Supports plotting with the `plot` feature.
pub type LinearSpline = spline::SplineCurve::<1,1>;
/// 1D cubic spline. Supports plotting with the `plot` feature.
pub type CubicSpline = spline::SplineCurve::<3,1>;
/// 1D quintic spline. Supports plotting with the `plot` feature.
pub type QuinticSpline = spline::SplineCurve::<5,1>;
/// 2D linear spline. Supports plotting with the `plot` feature.
pub type LinearSpline2D = spline::SplineCurve::<1,2>;
/// 2D cubic spline. Supports plotting with the `plot` feature.
pub type CubicSpline2D = spline::SplineCurve::<3,2>;
/// 2D quintic spline. Supports plotting with the `plot` feature.
pub type QuinticSpline2D = spline::SplineCurve::<5,2>;
/// 3D linear spline. Plotting is **not** supported.
pub type LinearSpline3D = spline::SplineCurve::<1,3>;
/// 3D cubic spline. Plotting is **not** supported.
pub type CubicSpline3D = spline::SplineCurve::<3,3>;
/// 3D quintic spline. Plotting is **not** supported.
pub type QuinticSpline3D = spline::SplineCurve::<5,3>;

pub mod spline;
pub use spline::*;

pub mod splines;
pub use splines::*;

#[cfg(feature = "plot")]
pub mod plot;
