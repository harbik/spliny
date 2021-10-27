use spline_curves::{Result, SplineCurve};
use std::iter::repeat;

pub fn main() -> Result<()> {

    let mut t: Vec<f64> = Vec::new();
    t.extend(repeat(0.0).take(3));
    t.extend((0..=180u32).into_iter().step_by(45).map(|v|(v as f64).to_radians()));
    t.extend(repeat(180f64.to_radians()).take(3));



    /*

    let spline: SplineCurve<3,2> = SplineCurve::new(
        [1.0, 1.0, 1.0, 2.0, 3.0, 3.0, 3.0]
    );
    */


    Ok(())
}
