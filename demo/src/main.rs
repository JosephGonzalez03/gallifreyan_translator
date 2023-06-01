use language::{glyphs::*, letters::*};
use plotters::prelude::*;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("clear").status().unwrap();
    let mut word = String::new();
    println!("Enter word: ");
    io::stdin()
        .read_line(&mut word)
        .expect("Smart user. Valid word.");
    let root = BitMapBackend::new("gallifreyan-message.png", (640, 640)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .build_cartesian_2d(-20f32..20f32, -20f32..20f32)?;

    let gallifreyan_word = GallifreyanWord::from(&word.trim_end_matches("\n"));

    gallifreyan_word
        .to_gallifreyan_characters()
        .iter()
        .for_each(|gallifreyan_character| {
            chart
                .draw_series(LineSeries::new(
                    gallifreyan_character.draw_base().to_vec(),
                    BLUE,
                ))
                .unwrap();

            match gallifreyan_character.modifier {
                Some(
                    Modifier::Line1 | Modifier::VowelLine1(_) | Modifier::Line2 | Modifier::Line3,
                ) => {
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

    gallifreyan_word
        .draw_edges()
        .into_iter()
        .for_each(|drawing| {
            chart
                .draw_series(LineSeries::new(drawing.to_vec(), BLUE))
                .unwrap();
        });
    Ok(())
}
