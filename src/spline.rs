use super::Result;
#[cfg(feature="plot")]
use super::plot::plot_base;


/// Maximum supported spline degree plus one. Used as a fixed array size for the de Boor
/// working buffer. Spline degrees 1 through 5 (linear through quintic) are supported.
const DE_BOOR_SIZE: usize = 6;

/// A B-spline curve of degree `K` in `N`-dimensional space.
///
/// The curve is stored in its knot/coefficient form:
/// - `t`: the full clamped knot vector (length `n`); the first and last `K` values must repeat
///   the boundary knots so the curve interpolates its endpoints.
/// - `c`: the B-spline control coefficients, stored dimension-major: all `n-K-1` x-values first,
///   then all y-values, etc. Total length must be `N * (n - K - 1)`.
///
/// Use the type aliases in [`crate`] (e.g. [`crate::CubicSpline2D`]) for the most common
/// degree/dimension combinations.
#[derive(Debug, Clone)]
pub struct SplineCurve<const K: usize, const N: usize> {
    pub t: Vec<f64>, // knot vector (clamped; length n)
    pub c: Vec<f64>, // B-spline coefficients (dimension-major; length N*(n-K-1))
    i: Option<usize>, // cached knot interval index from the last `eval` call
}

impl<const K: usize, const N: usize> SplineCurve<K, N> {
    /// Creates a spline from a knot vector `t` and coefficient array `c` without validating
    /// the coefficient count against the knot vector length.
    ///
    /// Prefer [`try_new`](Self::try_new) when constructing splines from untrusted input.
    ///
    /// # Panics
    /// Panics if `K >= 6` (degrees above quintic are not supported).
    pub fn new(t: Vec<f64>, c: Vec<f64>) -> Self {
        assert!(K < DE_BOOR_SIZE, "Spline degree K={} exceeds maximum supported degree {}", K, DE_BOOR_SIZE - 1);
        Self { t, c, i: None }
    }

    /// Creates a spline from a knot vector `t` and coefficient array `c`, returning an error
    /// if there are fewer than `N*(K+1)` coefficients.
    ///
    /// # Panics
    /// Panics if `K >= 6` (degrees above quintic are not supported).
    pub fn try_new(t: Vec<f64>, c: Vec<f64>) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        assert!(K < DE_BOOR_SIZE, "Spline degree K={} exceeds maximum supported degree {}", K, DE_BOOR_SIZE - 1);
        let nc = c.len() / N;
        if nc < (K+1) {
            return Err(format!("Need at least {} coefficients for a degree-{} spline", N*(K+1), K).into());
        }
        Ok(Self { t, c, i: None })
    }

    /// Plots the spline curve to a PNG file.
    ///
    /// Only supported for 1D (`N=1`) and 2D (`N=2`) curves.
    /// For 1D curves, the parameter is used as the x-axis and the spline value as the y-axis.
    /// For 2D curves, the first dimension is used as x and the second as y.
    ///
    /// # Panics
    /// Panics if `N > 2`.
    #[cfg(feature="plot")]
    pub fn plot(&self, filepath: &str, wxh: (u32, u32)) -> Result<()> {
        assert!(N <= 2, "Plotting is only supported for 1D and 2D curves (N=1 or N=2), got N={}", N);
        Ok(plot_base(self.clone(), filepath, wxh, None, None, false)?)
    }

    /// Plots the spline curve with explicit parameter values highlighted.
    ///
    /// Only supported for 1D (`N=1`) and 2D (`N=2`) curves. See [`plot`](Self::plot) for details.
    ///
    /// # Panics
    /// Panics if `N > 2`.
    #[cfg(feature="plot")]
    pub fn plot_with_parameter(&self, filepath: &str, wxh: (u32, u32), u:Option<&[f64]>) -> Result<()> {
        assert!(N <= 2, "Plotting is only supported for 1D and 2D curves (N=1 or N=2), got N={}", N);
        Ok(plot_base(self.clone(), filepath, wxh, u, None, false)?)
    }

    /// Plots the spline curve together with its control points.
    ///
    /// Only supported for 1D (`N=1`) and 2D (`N=2`) curves. See [`plot`](Self::plot) for details.
    ///
    /// # Panics
    /// Panics if `N > 2`.
    #[cfg(feature="plot")]
    pub fn plot_with_control_points(&self, filepath: &str, wxh: (u32, u32)) -> Result<()> {
        assert!(N <= 2, "Plotting is only supported for 1D and 2D curves (N=1 or N=2), got N={}", N);
        Ok(plot_base(self.clone(), filepath, wxh, None, None, true)?)
    }

    /// Plots the spline curve together with reference data points.
    ///
    /// Only supported for 1D (`N=1`) and 2D (`N=2`) curves. See [`plot`](Self::plot) for details.
    ///
    /// # Panics
    /// Panics if `N > 2`.
    #[cfg(feature="plot")]
    pub fn plot_with_data(&self, filepath: &str, wxh: (u32, u32), xy: &[f64]) -> Result<()> {
        assert!(N <= 2, "Plotting is only supported for 1D and 2D curves (N=1 or N=2), got N={}", N);
        Ok(plot_base(self.clone(), filepath, wxh, None, Some(xy), false)?)
    }

    /// Plots the spline curve together with its control points and reference data points.
    ///
    /// Only supported for 1D (`N=1`) and 2D (`N=2`) curves. See [`plot`](Self::plot) for details.
    ///
    /// # Panics
    /// Panics if `N > 2`.
    #[cfg(feature="plot")]
    pub fn plot_with_control_points_and_data(&self, filepath: &str, wxh: (u32, u32), xy: &[f64]) -> Result<()> {
        assert!(N <= 2, "Plotting is only supported for 1D and 2D curves (N=1 or N=2), got N={}", N);
        Ok(plot_base(self.clone(), filepath, wxh, None, Some(xy), true)?)
    }

    /// Calculates spline coordinates for a collection of parameter values.
    ///
    /// `u` must be sorted in strictly increasing order. Values outside the knot range are clamped
    /// to the nearest boundary.
    ///
    /// The output is a flat interleaved array: the N coordinates of the first point, then the N
    /// coordinates of the second, and so on. For a 2D curve: `[x0, y0, x1, y1, x2, y2, ...]`.
    /// Use [`crate::transpose`] to split the result into per-dimension vectors.
    ///
    /// # Errors
    /// Returns an error if the coefficient count does not match the knot vector, or if `u` is not
    /// strictly increasing.
    pub fn evaluate(&self, u: &[f64]) -> Result<Vec<f64>> {
        let n = self.t.len();
        let nc = self.c.len() / N;
        if nc < (K+1) {
            return Err(format!("Need at least {} coefficients for a degree-{} spline", N*(K+1), K).into());
        }
        if nc != n-(K+1) {
            return Err(format!("Expected {} coefficient values, got {}", N*(n - K - 1), N*nc).into());
        }
        let mut v: Vec<f64> = Vec::with_capacity(u.len() * N);

        let mut i = K;
        let mut u_prev = f64::NEG_INFINITY;
        // Ideally [f64; K+1], but const generic expressions in array lengths are not yet stable.
        let mut d = [0.0; DE_BOOR_SIZE];

        for &t in u {
            if t <= u_prev {
                return Err("parameter values must be in strictly increasing order".into());
            } else {
                u_prev = t;
            };

            // clamp to the valid knot interval
            let arg = if t < self.t[K] || t > self.t[n - K - 1] {
                t.clamp(self.t[K], self.t[n - K - 1])
            } else {
                t
            };

            // find knot interval which contains arg
            while !(arg >= self.t[i] && arg <= self.t[i + 1]) {
                i += 1
            }

            // calculate spline values
            for dim in 0..N {
                // copy relevant c values into d
                for (j, dm) in d.iter_mut().enumerate().take(K + 1) {
                    *dm = self.c[dim * nc + j + i - K];
                }

                v.push(self.deboor(i, arg, &mut d))
            }
        }
        Ok(v)
    }


    /// Evaluates a single parameter value on the spline.
    ///
    /// Returns `Ok(values)` with N-dimensional output, or `Err(distance)` if `t` is outside the
    /// knot range. Only indices `0..N` of the returned array are populated; the rest are zero.
    ///
    /// Note: the return array length is fixed at `DE_BOOR_SIZE` (= 6) due to a current compiler
    /// limitation preventing `[f64; N]` as a return type. Read only `values[0..N]`.
    pub fn eval(&mut self, t: f64) -> std::result::Result<[f64; DE_BOOR_SIZE], f64> {
        let n = self.t.len();
        let nc = self.c.len() / N;

        if t < self.t[K] {
            Err(t - self.t[K])
        } else if t > self.t[n - K - 1] {
            Err(t - self.t[n - K - 1])
        } else {
            let mut i = if let Some(i_prev) = self.i {
                if t > self.t[i_prev] {
                    i_prev
                } else {
                    K
                }
            } else {
                K
            };

            while !(t >= self.t[i] && t <= self.t[i + 1]) {
                i += 1
            }
            self.i = Some(i);

            let mut result = [0.0; DE_BOOR_SIZE];
            let mut d = [0.0; DE_BOOR_SIZE];

            for dim in 0..N {
                for (j, dm) in d.iter_mut().enumerate().take(K + 1) {
                    *dm = self.c[dim * nc + j + i - K];
                }
                result[dim] = self.deboor(i, t, &mut d);
            }
            Ok(result)
        }
    }

    /// Evaluates one dimension of the B-spline at parameter `x` using the de Boor algorithm.
    ///
    /// `i` is the knot interval index such that `t[i] <= x <= t[i+1]`. `d` must be pre-loaded
    /// with the K+1 relevant coefficients for the dimension being evaluated; it is modified
    /// in-place and the result is returned as `d[K]`.
    pub(crate) fn deboor(&self, i: usize, x: f64, d: &mut [f64; DE_BOOR_SIZE]) -> f64 {

        for r in 1..K + 1 {
            for j in (r..=K).into_iter().rev() {
                let alpha =
                    (x - self.t[j + i - K]) / (self.t[j + 1 + i - r] - self.t[j + i - K]);
                d[j] = (1.0 - alpha) * d[j - 1] + alpha * d[j]
            }
        }
        d[K]
    }

}

/// Splits an interleaved coordinate array (as returned by [`SplineCurve::evaluate`]) into
/// per-dimension vectors.
///
/// `n` is the number of dimensions. For example, an input `&[x0, y0, x1, y1, x2, y2]` with
/// `n=2` returns `vec![vec![x0, x1, x2], vec![y0, y1, y2]]`.
pub fn transpose(xyn: &[f64], n: usize) -> Vec<Vec<f64>>{
    let m = xyn.len()/n;
    let mut vn: Vec<Vec<f64>> = std::iter::repeat(Vec::with_capacity(m)).take(n).collect();
    for v in xyn.chunks(n) {
        for (i,x) in v.iter().enumerate() {
            vn[i].push(*x);
        }
    }
    vn
}

#[cfg(test)]
mod tests {
    use super::{transpose, SplineCurve};
    use approx::assert_abs_diff_eq;

    // spline test values from https://docs.rs/bspline/1.0.0/bspline/index.html crate

    // --- error path tests ---

    #[test]
    fn try_new_rejects_too_few_coefficients() {
        // K=3 requires at least K+1=4 coefficients per dimension
        let result: std::result::Result<SplineCurve<3, 1>, _> = SplineCurve::try_new(
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
            vec![0.0, 0.0, 0.0], // only 3, need ≥ 4
        );
        assert!(result.is_err());
    }

    #[test]
    fn evaluate_rejects_non_monotonic_params() {
        let s: SplineCurve<1, 1> = SplineCurve::new(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0]);
        assert!(s.evaluate(&[0.2, 0.5, 0.4]).is_err()); // 0.4 < 0.5 is not strictly increasing
    }

    #[test]
    fn evaluate_rejects_coefficient_mismatch() {
        // n=6 knots, K=1 → expects nc = n-(K+1) = 4 coefficients; providing 3 should error
        let s: SplineCurve<1, 1> = SplineCurve::new(
            vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0],
            vec![0.0, 1.0, 2.0],
        );
        assert!(s.evaluate(&[0.5]).is_err());
    }

    // --- clamping test ---

    #[test]
    fn evaluate_clamps_out_of_range_params() {
        // Linear spline from y=0 at t=0 to y=1 at t=1; params outside [0,1] should clamp
        let s: SplineCurve<1, 1> = SplineCurve::new(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0]);
        let yt = s.evaluate(&[-0.5, 1.5]).unwrap();
        assert_abs_diff_eq!(yt[0], 0.0, epsilon = 1E-10); // clamped to t=0
        assert_abs_diff_eq!(yt[1], 1.0, epsilon = 1E-10); // clamped to t=1
    }

    // --- eval() tests ---

    #[test]
    fn eval_returns_err_with_signed_distance_when_out_of_range() {
        let mut s: SplineCurve<3, 1> = SplineCurve::new(
            vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0],
            vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0],
        );
        // Below range: distance = t - t_min = -3 - (-2) = -1
        assert_abs_diff_eq!(s.eval(-3.0).unwrap_err(), -1.0, epsilon = 1E-10);
        // Above range: distance = t - t_max = 3 - 2 = 1
        assert_abs_diff_eq!(s.eval(3.0).unwrap_err(), 1.0, epsilon = 1E-10);
    }

    #[test]
    fn eval_cache_resets_correctly_on_backward_step() {
        let mut s: SplineCurve<3, 1> = SplineCurve::new(
            vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0],
            vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0],
        );
        // Forward: populates cache to a later interval
        assert_abs_diff_eq!(s.eval(0.5).unwrap()[0], 2.875, epsilon = 1E-7);
        // Backward: must reset cache; result must still be correct
        assert_abs_diff_eq!(s.eval(-1.0).unwrap()[0], 1.0, epsilon = 1E-7);
        // Forward again: cache reset again, result must be correct
        assert_abs_diff_eq!(s.eval(0.5).unwrap()[0], 2.875, epsilon = 1E-7);
    }

    // --- 2D curve test ---

    #[test]
    fn linear_2d_spline() {
        // Straight-line parametric curve from (0,0) to (1,1)
        // Coefficients: first half = x-controls [0,1], second half = y-controls [0,1]
        let s: SplineCurve<1, 2> = SplineCurve::new(
            vec![0.0, 0.0, 1.0, 1.0],
            vec![0.0, 1.0, 0.0, 1.0],
        );
        let r = s.evaluate(&[0.0, 0.5, 1.0]).unwrap();
        // output is interleaved: [x0, y0, x1, y1, x2, y2]
        assert_abs_diff_eq!(r[0], 0.0, epsilon = 1E-10); // x at t=0
        assert_abs_diff_eq!(r[1], 0.0, epsilon = 1E-10); // y at t=0
        assert_abs_diff_eq!(r[2], 0.5, epsilon = 1E-10); // x at t=0.5
        assert_abs_diff_eq!(r[3], 0.5, epsilon = 1E-10); // y at t=0.5
        assert_abs_diff_eq!(r[4], 1.0, epsilon = 1E-10); // x at t=1
        assert_abs_diff_eq!(r[5], 1.0, epsilon = 1E-10); // y at t=1
    }

    // --- transpose tests ---

    #[test]
    fn transpose_splits_2d_coordinates() {
        let flat = vec![1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0]; // [x0,y0, x1,y1, x2,y2]
        let vecs = transpose(&flat, 2);
        assert_eq!(vecs[0], vec![1.0, 3.0, 5.0]); // x values
        assert_eq!(vecs[1], vec![2.0, 4.0, 6.0]); // y values
    }

    #[test]
    fn transpose_splits_3d_coordinates() {
        let flat = vec![1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0]; // [x0,y0,z0, x1,y1,z1]
        let vecs = transpose(&flat, 3);
        assert_eq!(vecs[0], vec![1.0, 4.0]);
        assert_eq!(vecs[1], vec![2.0, 5.0]);
        assert_eq!(vecs[2], vec![3.0, 6.0]);
    }

    #[test]
    fn linear_bspline() {
        let x = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
        let y = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];

        let s: SplineCurve<1, 1> = SplineCurve::new(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0]);
        let yt = s.evaluate(&x).unwrap();
        y.iter()
            .zip(yt.iter())
            .for_each(|(&a, &b)| assert_abs_diff_eq!(a, b, epsilon = 1E-8));

        #[cfg(feature = "plot")]
        s.plot("test.png", (1500,1000)).unwrap();
    }
    #[test]
    fn quadratic_bspline() {
        let x = [0.0, 0.5, 1.0, 1.4, 1.5, 1.6, 2.0, 2.5, 3.0];
        let y = [0.0, 0.125, 0.5, 0.74, 0.75, 0.74, 0.5, 0.125, 0.0];

        let s: SplineCurve<2, 1> = SplineCurve::new(
            vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0],
            vec![0.0, 0.0, 1.0, 0.0, 0.0],
        );
        let yt = s.evaluate(&x).unwrap();
        y.iter()
            .zip(yt.iter())
            .for_each(|(&a, &b)| assert_abs_diff_eq!(a, b, epsilon = 1E-8));

        #[cfg(feature = "plot")]
        s.plot("test.png", (1500,1000)).unwrap();
    }

    #[test]
    fn cubic_bspline() {
        // expected
        let x = vec![-2.0, -1.5, -1.0, -0.6, 0.0, 0.5, 1.5, 2.0];
        let y = vec![0.0, 0.125, 1.0, 2.488, 4.0, 2.875, 0.12500001, 0.0];

        let s: SplineCurve<3, 1> = SplineCurve::new(
            vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0],
            vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0],
        );

        let yt = s.evaluate(&x).unwrap();
        y.iter()
            .zip(yt.iter())
            .for_each(|(&a, &b)| assert_abs_diff_eq!(a, b, epsilon = 1E-7));

        #[cfg(feature = "plot")]
        s.plot("test.png", (1000,1000)).unwrap();

    }

    #[test]
    fn cubic_bspline_single_values() {
        // expected
        let x = vec![-2.0, -1.5, -1.0, -0.6, 0.0, 0.5, 1.5, 2.0];
        let y = vec![0.0, 0.125, 1.0, 2.488, 4.0, 2.875, 0.12500001, 0.0];

        let mut s: SplineCurve<3, 1> = SplineCurve::new(
            vec![-2.0, -2.0, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0, 2.0],
            vec![0.0, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0],
        );

        let yt: Vec<f64> = x.into_iter().map(|x|s.eval(x).unwrap()[0]).collect();
        y.iter()
            .zip(yt.iter())
            .for_each(|(&a, &b)| assert_abs_diff_eq!(a, b, epsilon = 1E-7));

        #[cfg(feature = "plot")]
        s.plot("test.png", (1000,1000)).unwrap();

    }


    #[test]
    fn quartic_bspline() {
        let x = vec![0.0, 0.4, 1.0, 1.5, 2.0, 2.5, 3.0, 3.2, 4.1, 4.5, 5.0];
        let y = vec![
            0.0,
            0.0010666668,
            0.041666668,
            0.19791667,
            0.4583333,
            0.5989583,
            0.4583333,
            0.35206667,
            0.02733751,
            0.002604167,
            0.0,
        ];
        let s: SplineCurve<4, 1> = SplineCurve::new(
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0, 5.0,
            ],
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        );
        let yt = s.evaluate(&x).unwrap();
        y.iter()
            .zip(yt.iter())
            .for_each(|(&a, &b)| assert_abs_diff_eq!(a, b, epsilon = 1E-7));

        #[cfg(feature = "plot")]
        s.plot("test.png", (2000,1000)).unwrap();
    }
}
