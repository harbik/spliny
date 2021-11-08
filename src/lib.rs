#![doc = include_str!("../README.md")]

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


pub type LinearSpline = spline::SplineCurve::<1,1>;
pub type CubicSpline = spline::SplineCurve::<3,1>;
pub type QuinticSpline = spline::SplineCurve::<5,1>;
pub type LinearSpline2D = spline::SplineCurve::<1,2>;
pub type CubicSpline2D = spline::SplineCurve::<3,2>;
pub type QuinticSpline2D = spline::SplineCurve::<5,2>;
pub type LinearSpline3D = spline::SplineCurve::<1,3>;
pub type CubicSpline3D = spline::SplineCurve::<3,3>;
pub type QuinticSpline3D = spline::SplineCurve::<5,3>;

pub mod spline;
pub use spline::*;

pub mod splines;
pub use splines::*;

pub mod plot;
