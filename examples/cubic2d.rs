// cargo run --examples/cubic2d.rs
use spliny::{CubicSpline2D, Result};

pub fn main() -> Result<()> {

    // set path to directory of example script, used for saving plot
    std::env::set_current_dir(std::path::Path::new(file!()).parent().unwrap())?;   

    let spline = CubicSpline2D::new(
        vec![1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0],
        vec![0.0, 0.5, 1.0, 3.0, 2.0, -3.0, 3.0, -3.0]
    );

    spline.plot_with_control_points("cubic2d.png", (800,800))?;

    Ok(())
}
