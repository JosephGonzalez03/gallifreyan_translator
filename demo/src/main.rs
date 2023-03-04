use language::letters::*;
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
    if let Ok(drawings) = GallifreyanWord::from("TCHX") {
        drawings
            .to_drawings(6.0)
            .iter()
            .for_each(|drawing| {
                chart
                    .draw_series(
                        LineSeries::new(drawing
                            .to_points()
                            .to_vec(),
                        BLUE));
            });
    }



    /*
    word.letters().iter().for_each(|letter| {

        // draw base
        chart.draw_series(LineSeries::new(
            letter.to_drawing().to_vec(),
            GREEN,
        )).unwrap();

        // draw edge


        // draw modifier
        if let Some(modifier) = letter.modifier() {
            modifier
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
        };
    });
    */
    Ok(())
}
