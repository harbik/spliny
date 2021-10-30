use super::Result;
use super::plot_base;
use serde::{Deserialize, Serialize};

/**
 * General B-Spline Curve Knot/Coefficient Representation
 */
#[derive(Clone, Serialize, Deserialize)]
pub struct SplineCurve<const K: usize, const N: usize> {
    pub t: Vec<f64>, // Knot values
    pub c: Vec<f64>, // b-Spline coefficients
    k: usize,        // Spline degree
    n: usize,        // Spline dimension
}

impl<const K: usize, const N: usize> SplineCurve<K, N> {
    pub fn new(t: Vec<f64>, c: Vec<f64>) -> Self {
        Self { t, c, k: K, n: N }
    }

    pub fn plot(self, filepath: &str, wxh: (u32, u32)) -> Result<()> {
        Ok(plot_base(self, filepath, wxh, None, None, false)?)
    }

    pub fn plot_with_parameter(self, filepath: &str, wxh: (u32, u32), u:Option<&[f64]>) -> Result<()> {
        Ok(plot_base(self, filepath, wxh, u, None, false)?)
    }

    pub fn plot_with_control_points(self, filepath: &str, wxh: (u32, u32)) -> Result<()> {
        Ok(plot_base(self, filepath, wxh, None, None, true)?)
    }

    pub fn plot_with_data(self, filepath: &str, wxh: (u32, u32), u:Option<&[f64]>, xy: Option<&[f64]>, plot_control_points: bool) -> Result<()> {
        Ok(plot_base(self, filepath, wxh, u, xy, plot_control_points)?)
    }

    pub fn evaluate(&self, us: &[f64]) -> Result<Vec<f64>> {
        let n = self.t.len();
        let nc = self.c.len() / N;
        let mut v: Vec<f64> = Vec::with_capacity(us.len() * N); // x,y,..x,y coordinates

        let mut i = self.k;
        let mut u_prev = f64::NEG_INFINITY;
        let mut d = [0.0; 6]; // want to use K+1 here, but currently not allowed yet by the compiler

        for &u in us {
            if u <= u_prev {
                return Err("x values should be sorted in strict increasing order".into());
            } else {
                u_prev = u;
            };

            // clamp x to interval tb..=te
            let arg = if u < self.t[self.k] || u > self.t[n - self.k - 1] {
                u.clamp(self.t[self.k], self.t[n - K - 1])
            } else {
                u
            };

            // find knot interval which contains x=arg
            while !(arg >= self.t[i] && arg <= self.t[i + 1]) {
                i += 1
            }

            // calculate spline values
            for dim in 0..N {
                // copy relevant c values into d
                for (j, dm) in d.iter_mut().enumerate().take(K + 1) {
                    *dm = self.c[dim * nc + j + i - self.k];
                }

                v.push(self.deboor(i, arg, &mut d))
            }
        }
        Ok(v)
    }

    pub fn deboor(&self, i: usize, x: f64, d: &mut [f64; 6]) -> f64 {
        /*
        for j in 0..K+1 {
            d[j] = self.c[(j + i - self.k)];
        }
        */

        for r in 1..self.k + 1 {
            for j in (r..=self.k).into_iter().rev() {
                let alpha =
                    (x - self.t[j + i - self.k]) / (self.t[j + 1 + i - r] - self.t[j + i - self.k]);
                d[j] = (1.0 - alpha) * d[j - 1] + alpha * d[j]
            }
        }
        d[self.k]
    }
}

#[cfg(test)]
mod tests {
    use super::SplineCurve;
    use approx::assert_abs_diff_eq;

    // spline test values from https://docs.rs/bspline/1.0.0/bspline/index.html crate

    #[test]
    fn linear_bspline() {
        let x = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
        let y = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];

        let s: SplineCurve<1, 1> = SplineCurve::new(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0]);
        let yt = s.evaluate(&x).unwrap();
        //println!("{:?}", yt);
        y.iter()
            .zip(yt.iter())
            .for_each(|(&a, &b)| assert_abs_diff_eq!(a, b, epsilon = 1E-8));

        let ytx = s.evaluate(&x).unwrap();
        //println!("{:?}", ytx);
        y.iter()
            .zip(ytx.iter())
            .for_each(|(&a, &b)| assert_abs_diff_eq!(a, b, epsilon = 1E-8));

    //    let u: Vec<f64>  = (0..100).into_iter().map(|v|v as f64/100.0).collect();
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
        //  println!("{:?}", yt);
        y.iter()
            .zip(yt.iter())
            .for_each(|(&a, &b)| assert_abs_diff_eq!(a, b, epsilon = 1E-8));

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

        //
        let yt = s.evaluate(&x).unwrap();
        y.iter()
            .zip(yt.iter())
            .for_each(|(&a, &b)| assert_abs_diff_eq!(a, b, epsilon = 1E-7));

        s.plot("test.png", (2000,1000)).unwrap();

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
        //println!("{:?}", yt);
        y.iter()
            .zip(yt.iter())
            .for_each(|(&a, &b)| assert_abs_diff_eq!(a, b, epsilon = 1E-7));

        let ytx = s.evaluate(&x).unwrap();
        //println!("{:?}", ytx);
        y.iter()
            .zip(ytx.iter())
            .for_each(|(&a, &b)| assert_abs_diff_eq!(a, b, epsilon = 1E-7));
        s.plot("test.png", (2000,1000)).unwrap();
    }
}
