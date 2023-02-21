use language::{letters::*, math_util::*};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("0.png", (640, 640)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f32..10f32, -10f32..10f32)?;

    chart.configure_mesh().draw()?;
    let b = GallifreyanLetter::CH(Polar::new(6.0, Degree(0.0)));
    let c = GallifreyanLetter::CH(Polar::new(6.0, Degree(180.0)));
    chart.draw_series(LineSeries::new(
        b.letter().base().to_drawing().to_points().to_vec(),
        GREEN,
    ))?;
    chart.draw_series(LineSeries::new(
        c.letter().base().to_drawing().to_points().to_vec(),
        GREEN,
    ))?;
    chart.draw_series(LineSeries::new(
        b.letter().base().buffer(c.letter().base()).unwrap().to_points().to_vec(),
        RED,
    ))?;
    b.letter()
        .modifiers()
        .as_ref()
        .expect("")
        .to_drawings()
        .iter()
        .for_each(|drawing| {
            //chart.draw_series(LineSeries::new(part.to_vec(), GREEN));
            chart
                .draw_series(
                    LineSeries::new(drawing.to_points().to_vec(), RED.filled()).point_size(2),
                )
                .unwrap();
        });
    c.letter()
        .modifiers()
        .as_ref()
        .expect("")
        .to_drawings()
        .iter()
        .for_each(|drawing| {
            //chart.draw_series(LineSeries::new(part.to_vec(), GREEN));
            chart
                .draw_series(
                    LineSeries::new(drawing.to_points().to_vec(), RED.filled()).point_size(2),
                )
                .unwrap();
        });
    Ok(())
}
