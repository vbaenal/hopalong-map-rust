#![allow(non_snake_case)]

mod hopalong;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let colors = [&RED, &BLUE, &GREEN, &WHITE, &MAGENTA, &CYAN, &YELLOW];

    let root =
    SVGBackend::new("hopalong.svg", (800, 800)).into_drawing_area();

    root.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(0)
        .build_ranged(-20.1f64..20.1f64, -20.1f64..20.1f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let (xss, yss): (Vec<Vec<f64>>, Vec<Vec<f64>>) = hopalong::hopalong(2.1,-2.6,0.0,50,5000,None,None);
    
    for (xs, ys) in xss.iter().zip(yss.iter()) {
        for (i, (x, y)) in (xs.iter().zip(ys.iter())).enumerate() {
            plotting_area.draw_pixel((*x, *y), colors[i%colors.len()])?;
        }
    }

    Ok(())
}