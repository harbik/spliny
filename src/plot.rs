

use super::{Result, SplineCurve};
use plotters::{prelude::*};


pub fn range_knots<const K:usize, const N:usize>(s: &SplineCurve<K,N>) -> Result<[f64;4]> {
    let nc = s.c.len();
    let nc_2 = nc/2;
    let (x_min, x_max) = match N {
        1 => (
            s.t.iter().cloned().reduce(f64::min).unwrap(), 
            s.t.iter().cloned().reduce(f64::max).unwrap()
        ),
        2 => (
            s.c[0..nc_2].iter().cloned().reduce(f64::min).unwrap(),
            s.c[0..nc_2].iter().cloned().reduce(f64::max).unwrap(),
        ),
        _ => return Err("Only one and two dimensional curve splnies supported".into())

    };
    let (y_min, y_max) = match N {
        1 => (
            s.c[0..nc].iter().cloned().reduce(f64::min).unwrap(),
            s.c[0..nc].iter().cloned().reduce(f64::max).unwrap(),
        ),
        2 => (
            s.c[nc_2..nc].iter().cloned().reduce(f64::min).unwrap(),
            s.c[nc_2..nc].iter().cloned().reduce(f64::max).unwrap(),
        ),
        _ => return Err("Only one and two dimensional curve splnies supported".into())
    };
    Ok([x_min, x_max, y_min, y_max])
}

pub fn range_spline(u: &[f64], xn: &[f64]) -> Result<[f64;4]> {
    let n = xn.len()/u.len();
    let (x_min, x_max) = match n {
        1 => (
            u.iter().cloned().reduce(f64::min).unwrap(),
            u.iter().cloned().reduce(f64::max).unwrap(),
        ),
        2 => (
            // step-by starts at 0
            xn.iter().step_by(1).cloned().reduce(f64::min).unwrap(),
            xn.iter().step_by(1).cloned().reduce(f64::max).unwrap(),
        ),
        _ => return Err("only dimensions 0 and 1 supported".into())
    };
    let (y_min, y_max) = match n {
        1 => (
            xn.iter().cloned().reduce(f64::min).unwrap(),
            xn.iter().cloned().reduce(f64::max).unwrap(),
        ),
        2 => (
            xn.iter().skip(1).step_by(1).cloned().reduce(f64::min).unwrap(),
            xn.iter().skip(1).step_by(1).cloned().reduce(f64::max).unwrap(),
        ),
        _ => return Err("only dimensions 0 and 1 supported".into())
    };
    Ok([x_min, x_max, y_min, y_max])
}

pub fn avg(t: &[f64], k: usize) -> Vec<f64> {
     t.windows(k).map(|ts|ts.iter().sum::<f64>()/k as f64).collect()
}



/// Plots a two-dimensional (xy) spline curve for testing review
///  
pub fn plot<const K: usize, const N: usize>(
    s: SplineCurve<K,N>,
    filepath: &str,
    wxh: (u32, u32),
    u: &[f64],
    xy: Option<&[f64]>,
) -> Result<()> {
   // if N>2 { return Err("only 2D plots supported".into())}

    let s_xy = s.evaluate(&u)?;

    let [x_min, x_max, y_min, y_max] = match N {
           1 => range_spline(u, &s_xy)?,
           2 => range_knots(&s)?,
           _ => return Err("only 2D plots supported".into())
        };

    let width = x_max - x_min;
    let height = y_max - y_min;


    let spline_color = HSLColor(0.5, 0.5, 0.4);
    //let spline_coef_color = HSLColor(0.05, 0.5, 0.4);

    let mut chartarea = BitMapBackend::new(filepath, (wxh.0, wxh.1)).into_drawing_area();
    chartarea.fill(&WHITE)?;
    chartarea = chartarea.margin(100, 100, 100, 100);
    chartarea.fill(&HSLColor(0.1, 0.5, 0.95))?;

    let mut chart = ChartBuilder::on(&chartarea)
        .margin(50)
        .set_all_label_area_size(50)
        .build_cartesian_2d(
            x_min - width / 10.0..x_max + width / 10.0,
            y_min - height / 10.0..y_max + height / 10.0,
        )?;

    // draw the mesh
    chart.configure_mesh()
        .x_labels(10)
//        .x_label_formatter(&|v| format!("{:.1}", v))
        .label_style(TextStyle::from(("sans-serif", 20).into_font()))
        .draw()?;

    if N==2 {
        // draw control points, only in 2D case
        let nc = s.c.len();
        let nc_2 = nc/2;
        let c_x = &s.c[0..nc_2];
        let c_y =  &s.c[nc_2..nc];
       // let c: Vec<(f64,f64)> = c_x.iter().cloned().zip(c_y.iter().cloned()).collect();
        chart.draw_series(
           c_x.iter().cloned().zip(c_y.iter().cloned()) 
         //   control_points(&s)?
         //       .into_iter()
                .map(|xy|Circle::new(xy, 10, spline_color.filled())),
        )?;
    }

    // draw spline 
    if N==2 {
        chart.draw_series(LineSeries::new(
            s_xy.chunks(2).map(|xy|(xy[0],xy[1])),
            spline_color.mix(1.0).stroke_width(5)))?;
    } else if N ==1 {
        chart.draw_series(LineSeries::new(
            u.iter().skip((K+1)/2).zip(s_xy.iter()).map(|(&x,&y)|(x,y)),
            spline_color.mix(1.0).stroke_width(5)))?;

    }

    // xy value target of fit
    if xy.is_some() {
        chart.draw_series(LineSeries::new(
            xy.unwrap().chunks(2).map(|xy|(xy[0],xy[1])),
            BLACK.mix(1.0).stroke_width(2),
        ))?;
    }   

    /* 
        if flags.contains(Plot2DFlags::LEGEND) {
            let leg_txt = if let Some(e) = s.e {
                format!("{} knots  {:.1e} rms", s.t.len(), e)
            } else {
                format!("{} knots", s.t.len())
            };

            chartarea.draw(
                &(EmptyElement::at((1550, 100))
                    + Text::new( leg_txt, (0, 0), &"sans-serif".into_font().resize(40.0).color(&spline_color),
                    )),
            )?;
        }
         */

        chartarea.present()?;
    Ok(())
}
