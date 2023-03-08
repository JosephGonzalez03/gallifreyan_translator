use geomath::{prelude::coordinates::Polar, vector::Vector2};
use language::{glyphs::*, letters::*};
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
    let gallifreyan_word: Vec<GallifreyanCharacter> = GallifreyanWord::from("TCHXD")
        .unwrap()
        .to_gallifreyan_characters(6.0);

    gallifreyan_word.iter().for_each(|gallifreyan_character| {
        chart
            .draw_series(LineSeries::new(
                gallifreyan_character.draw_base().to_vec(),
                BLUE,
            ))
            .unwrap();

        match gallifreyan_character.modifier {
            Some(Modifier::Line1 | Modifier::Line2 | Modifier::Line3) => {
                gallifreyan_character
                    .draw_modifier()
                    .expect("Already checked if modifier exists.")
                    .iter()
                    .for_each(|drawing| {
                        chart
                            .draw_series(LineSeries::new(drawing.to_vec(), BLUE))
                            .unwrap();
                    });
            }
            Some(Modifier::Dot1 | Modifier::Dot2 | Modifier::Dot3 | Modifier::Dot4) => {
                gallifreyan_character
                    .draw_modifier()
                    .expect("Already checked if modifier exists.")
                    .iter()
                    .for_each(|drawing| {
                        chart
                            .draw_series(
                                LineSeries::new(drawing.to_vec(), BLUE.filled()).point_size(2),
                            )
                            .unwrap();
                    });
            }
            None => (),
        }
    });

    let mut characters_with_edges: Vec<GallifreyanCharacter> = gallifreyan_word
        .into_iter()
        .filter(|gallifreyan_character| gallifreyan_character.has_edge())
        .collect::<GallifreyanCharacterCollection>()
        .0;

    characters_with_edges.extend_from_within(..1);
    characters_with_edges
        .as_slice()
        .windows(2)
        .map(|letters| {
            let edge1 = letters[0]
                .ending_angle()
                .expect("The Gallifreyan character should have an edge.");
            let edge2 = letters[1]
                .starting_angle()
                .expect("The Gallifreyan character should have an edge.");

            draw_base(
                Vector2::from_polar(0.0, 0.0),
                letters[0].origin.rho(),
                (edge1, edge2),
                0.0,
            )
        })
        .for_each(|drawing| {
            chart
                .draw_series(LineSeries::new(drawing.to_vec(), BLUE))
                .unwrap();
        });
    Ok(())
}
