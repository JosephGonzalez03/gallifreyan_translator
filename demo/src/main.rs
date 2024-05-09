use geomath::prelude::coordinates::Polar;
use geomath::vector::Vector2;
use language::letters::*;
use plotters::{
    prelude::*,
    style::full_palette::{BLUE, BROWN},
};
use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI},
    io,
    str::FromStr,
};

struct Plot {
    part: Part,
    vector: Vector2,
    radius: f64,
    offset: f64,
}

struct Drawing {
    series: Vec<Vec<(f32, f32)>>,
    style: plotters::style::ShapeStyle,
}

const SENTENCE_RADIUS: f64 = 10.0;
const WORD_RADIUS: f64 = 5.0;
const LETTER_RADIUS: f64 = 1.0;

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
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-20f32..20f32, -20f32..20f32)?;
    chart.configure_mesh().draw()?;
    let word_count: f64 = word.split_terminator(" ").map(|_| 1.0).sum();
    let notch_offset: f64 = PI / word_count;

    // Create word and letter plots.
    let mut sentence_circle_plots: Vec<Plot> = word
        .to_uppercase()
        .replace("\n", "")
        .split_terminator(" ")
        .into_iter()
        .scan(-FRAC_PI_2, |word_origin, word| {
            /* Step 1:
                Parse each character in a word into a Gallifreyan token. Keep in mind that some
                Gallifreyan letters combine two english letters.
            */
            let mut tokens: Vec<GallifreyanToken> = Vec::new();
            let mut char_iter = word.chars().into_iter().peekable();

            while let Some(current_letter) = char_iter.next() {
                let letter = if (['C', 'P', 'W', 'S', 'T', 'G'].contains(&current_letter)
                    && 'H'.eq(char_iter.peek().unwrap_or(&'>')))
                    || ('Q'.eq(&current_letter) && 'U'.eq(char_iter.peek().unwrap_or(&'>')))
                    || ('N'.eq(&current_letter) && 'G'.eq(char_iter.peek().unwrap_or(&'>')))
                {
                    current_letter.to_string() + &char_iter.next().unwrap().to_string()
                } else {
                    current_letter.to_string()
                };
                tokens.push(GallifreyanToken::from_str(letter.as_str()).unwrap());
            }

            /* Step 2:
                Determine the positions of the letters on the circle. Keep in mind that vowels
                that follow a consonant are placed on the consonant.
            */
            let number_of_letter_positions: f64 = tokens
                .iter()
                .filter(|token| token.is_letter())
                .scan(false, |is_previous_letter_a_consonant, letter| {
                    let result = if *is_previous_letter_a_consonant && letter.is_vowel() {
                        0.0
                    } else {
                        1.0
                    };
                    *is_previous_letter_a_consonant = !letter.is_vowel();
                    return Some(result);
                })
                .sum();

            /* Step 3:
                Decompose the letters from (1) into their parts and their positions vectors with the
                number of positions from (2).
            */
            let mut word_circle_plots: Vec<Plot> = tokens
                .iter()
                .scan(false, |is_previous_letter_a_consonant, token| {
                    let is_stand_alone_letter =
                        !(*is_previous_letter_a_consonant && token.is_vowel());
                    *is_previous_letter_a_consonant = !token.is_vowel();
                    return Some(is_stand_alone_letter);
                })
                .zip(tokens.iter())
                .enumerate()
                .scan(
                    -FRAC_PI_2,
                    |letter_origin, (index, (is_stand_alone_letter, token))| {
                        if is_stand_alone_letter && index != 0 {
                            *letter_origin += (2.0 * PI) / number_of_letter_positions;
                        }

                        let letter_circle_plots: Vec<Plot> = token
                            .parts()
                            .into_iter()
                            .map(|part| {
                                let mut letter_vector = Vector2::from_polar(
                                    match part {
                                        Part::Crescent => CRESCENT_BASE_RATIO,
                                        Part::Full => FULL_BASE_RATIO,
                                        Part::Moon(_) => MOON_BASE_RATIO,
                                        _ if part.is_modifier() => match token
                                            .parts()
                                            .into_iter()
                                            .filter(|part| part.is_base())
                                            .nth(0)
                                            .unwrap()
                                        {
                                            Part::Crescent => CRESCENT_BASE_RATIO,
                                            Part::Full => FULL_BASE_RATIO,
                                            _ => 0.0,
                                        },
                                        _ => 0.0,
                                    } * LETTER_RADIUS,
                                    letter_origin.as_f64()
                                        + match part {
                                            Part::Moon(offset) | Part::VowelLine1(offset) => offset,
                                            _ => 0.0,
                                        },
                                );

                                if !is_stand_alone_letter {
                                    letter_vector += Vector2::from_polar(
                                        match tokens
                                            .iter()
                                            .nth(index - 1)
                                            .unwrap()
                                            .parts()
                                            .into_iter()
                                            .filter(|part| part.is_base())
                                            .nth(0)
                                            .unwrap()
                                        {
                                            Part::Crescent => CRESCENT_BASE_RATIO,
                                            Part::Full => FULL_BASE_RATIO,
                                            _ => 0.0,
                                        } * LETTER_RADIUS,
                                        letter_origin.as_f64(),
                                    );
                                }

                                Plot {
                                    part,
                                    vector: Vector2::from_polar(SENTENCE_RADIUS, *word_origin)
                                        + Vector2::from_polar(WORD_RADIUS, *letter_origin)
                                        - letter_vector,
                                    radius: match part {
                                        Part::Moon(_) | Part::Core | Part::VowelLine1(_) => {
                                            LETTER_RADIUS / 3.0
                                        }
                                        _ => LETTER_RADIUS,
                                    },
                                    offset: letter_origin.clone(),
                                }
                            })
                            .collect();
                        Some(letter_circle_plots)
                    },
                )
                .flatten()
                .collect();

            /* Step 4:
                Create the edges that connect the letters together using the parts of letters
                (Crescent and Quarter) that touch the word circle.
            */
            let mut word_circle_edges: Vec<f64> = word_circle_plots
                .iter()
                .filter(|plot| [Part::Crescent, Part::Quarter].contains(&plot.part))
                .flat_map(|plot| {
                    let edge_offset = ((LETTER_RADIUS
                        * match plot.part {
                            Part::Crescent => CRESCENT_BASE_OFFSET,
                            Part::Quarter => QUARTER_BASE_OFFSET,
                            _ => panic!("Should not be here!"),
                        }
                        .sin())
                        / WORD_RADIUS)
                        .asin();
                    vec![plot.offset - edge_offset, plot.offset + edge_offset]
                })
                .collect();

            let mut word_circle_edge_plots: Vec<Plot> = if word_circle_edges.len() == 0 {
                vec![Plot {
                    part: Part::Edge(0.0, 2.0 * PI),
                    vector: Vector2::from_polar(SENTENCE_RADIUS, *word_origin),
                    radius: WORD_RADIUS,
                    offset: 0.0,
                }]
            } else {
                word_circle_edges.rotate_left(1);
                word_circle_edges
                    .chunks_exact(2)
                    .map(|edges| Plot {
                        part: Part::Edge(*edges.first().unwrap(), *edges.get(1).unwrap()),
                        vector: Vector2::from_polar(SENTENCE_RADIUS, *word_origin),
                        radius: WORD_RADIUS,
                        offset: 0.0,
                    })
                    .collect()
            };

            /* Step 5:
                Add notch between current and next words on the inner sentence circle.
            */
            word_circle_plots.push(Plot {
                part: Part::Notch,
                vector: Vector2::from_polar(
                    INNER_CIRCLE_RATIO * SENTENCE_RADIUS,
                    *word_origin + notch_offset,
                ) - Vector2::from_polar(
                    NOTCH_BASE_RATIO * WORD_RADIUS,
                    *word_origin + notch_offset,
                ),
                radius: WORD_RADIUS,
                offset: *word_origin + notch_offset + PI,
            });

            /* Step 6:
                Add edge plots to the plots collection and return all of them from this function.
            */
            word_circle_plots.append(&mut word_circle_edge_plots);
            *word_origin += (2.0 * PI) / word_count;
            Some(word_circle_plots)
        })
        .flatten()
        .collect();

    // Create inner sentence circle plots.
    let mut inner_sentence_circle_edges: Vec<f64> = sentence_circle_plots
        .iter()
        .filter(|plot| plot.part == Part::Notch)
        .flat_map(|plot| {
            let sentence_notch_offset = ((WORD_RADIUS * NOTCH_BASE_OFFSET.sin())
                / (INNER_CIRCLE_RATIO * SENTENCE_RADIUS))
                .asin();
            vec![
                plot.offset - sentence_notch_offset,
                plot.offset + sentence_notch_offset,
            ]
        })
        .collect();
    inner_sentence_circle_edges.rotate_left(1);
    let mut inner_sentence_circle_edge_plots: Vec<Plot> = inner_sentence_circle_edges
        .chunks_exact(2)
        .scan(0.0, |word_origin, edges| {
            let notch_edge = Plot {
                part: Part::Edge(*edges.first().unwrap(), *edges.get(1).unwrap()),
                vector: Vector2::from_polar(0.0, *word_origin),
                radius: INNER_CIRCLE_RATIO * SENTENCE_RADIUS,
                offset: 0.0,
            };
            *word_origin += (2.0 * PI) / word_count;
            Some(notch_edge)
        })
        .collect();
    sentence_circle_plots.append(&mut inner_sentence_circle_edge_plots);

    // Create outer sentence circle plots.
    sentence_circle_plots.push(Plot {
        part: Part::New,
        vector: Vector2::from_polar(0.0, 0.0),
        radius: OUTTER_CIRCLE_RATIO * SENTENCE_RADIUS,
        offset: 0.0,
    });

    // Draw plots.
    sentence_circle_plots
        .into_iter()
        .map(|plot| match plot.part {
            Part::Edge(start, end) => Drawing {
                series: vec![draw_base(
                    plot.vector,
                    plot.radius,
                    (start, end),
                    plot.offset,
                )],
                style: GREEN.stroke_width(2),
            },
            Part::Moon(_) => Drawing {
                series: vec![draw_base(
                    plot.vector,
                    plot.radius,
                    (0.0, 2.0 * PI),
                    plot.offset,
                )],
                style: BLUE.stroke_width(1),
            },
            Part::Core => Drawing {
                series: vec![draw_base(
                    plot.vector,
                    plot.radius,
                    (0.0, 2.0 * PI),
                    plot.offset,
                )],
                style: BLUE.stroke_width(1),
            },
            Part::Crescent => Drawing {
                series: vec![draw_base(
                    plot.vector,
                    plot.radius,
                    (CRESCENT_BASE_OFFSET, -CRESCENT_BASE_OFFSET),
                    plot.offset,
                )],
                style: BLUE.stroke_width(1),
            },
            Part::Full => Drawing {
                series: vec![draw_base(
                    plot.vector,
                    plot.radius,
                    (0.0, 2.0 * PI),
                    plot.offset,
                )],
                style: BLUE.stroke_width(1),
            },
            Part::Quarter => Drawing {
                series: vec![draw_base(
                    plot.vector,
                    plot.radius,
                    (QUARTER_BASE_OFFSET, -QUARTER_BASE_OFFSET),
                    plot.offset,
                )],
                style: BLUE.stroke_width(1),
            },
            Part::New => Drawing {
                series: vec![draw_base(
                    plot.vector,
                    plot.radius,
                    (0.0, 2.0 * PI),
                    plot.offset,
                )],
                style: BLUE.stroke_width(1),
            },
            Part::Dot1 => Drawing {
                series: draw_dots(plot.vector, plot.radius, vec![0.0], plot.offset),
                style: RED.filled(),
            },
            Part::Dot2 => Drawing {
                series: draw_dots(
                    plot.vector,
                    plot.radius,
                    vec![-FRAC_PI_4, FRAC_PI_4],
                    plot.offset,
                ),
                style: RED.filled(),
            },
            Part::Dot3 => Drawing {
                series: draw_dots(
                    plot.vector,
                    plot.radius,
                    vec![-FRAC_PI_4, 0.0, FRAC_PI_4],
                    plot.offset,
                ),
                style: RED.filled(),
            },
            Part::Dot4 => Drawing {
                series: draw_dots(
                    plot.vector,
                    plot.radius,
                    vec![-FRAC_PI_4, -FRAC_PI_8, FRAC_PI_8, FRAC_PI_4],
                    plot.offset,
                ),
                style: RED.filled(),
            },
            Part::VowelLine1(offset) => Drawing {
                series: draw_lines(plot.vector, plot.radius, vec![0.0], plot.offset + offset),
                style: BLUE.stroke_width(1),
            },
            Part::Line1 => Drawing {
                series: draw_lines(plot.vector, plot.radius, vec![PI], plot.offset + PI),
                style: BLUE.stroke_width(1),
            },
            Part::Line2 => Drawing {
                series: draw_lines(
                    plot.vector,
                    plot.radius,
                    vec![-FRAC_PI_4, FRAC_PI_4],
                    plot.offset,
                ),
                style: BLUE.stroke_width(1),
            },
            Part::Line3 => Drawing {
                series: draw_lines(
                    plot.vector,
                    plot.radius,
                    vec![-FRAC_PI_4, 0.0, FRAC_PI_4],
                    plot.offset,
                ),
                style: BLUE.stroke_width(1),
            },
            Part::Notch => Drawing {
                series: vec![draw_base(
                    plot.vector,
                    plot.radius,
                    (-NOTCH_BASE_OFFSET, NOTCH_BASE_OFFSET),
                    plot.offset,
                )],
                style: BROWN.stroke_width(1),
            },
        })
        .for_each(|drawing| {
            drawing.series.into_iter().for_each(|points| {
                if points.len() == 1 {
                    chart
                        .draw_series(LineSeries::new(points, drawing.style).point_size(2))
                        .unwrap();
                } else {
                    chart
                        .draw_series(LineSeries::new(points, drawing.style))
                        .unwrap();
                }
            });
        });
    Ok(())
}

const INNER_CIRCLE_RATIO: f64 = 1.6;
const OUTTER_CIRCLE_RATIO: f64 = 1.8;
const NOTCH_BASE_RATIO: f64 = 0.15;
const CRESCENT_BASE_RATIO: f64 = 0.9;
const FULL_BASE_RATIO: f64 = 1.2;
const MOON_BASE_RATIO: f64 = 1.0;
const NOTCH_BASE_OFFSET: f64 = FRAC_PI_2;
const CRESCENT_BASE_OFFSET: f64 = FRAC_PI_6;
const QUARTER_BASE_OFFSET: f64 = 5.0 * PI / 9.0;
const DOT_OFFSET: f64 = 0.4;

pub fn draw_base(origin: Vector2, size: f64, range: (f64, f64), offset: f64) -> Vec<(f32, f32)> {
    let start_range = ((range.0 * 180.0) / PI).round() as i64;
    let end_range = ((range.1 * 180.0) / PI).round() as i64;

    match start_range < end_range {
        true => start_range..=end_range,
        false => start_range..=end_range + 360,
    }
    .map(|angle| origin + Vector2::from_polar(size, (angle as f64 * PI) / 180.0 + offset))
    .map(|vector| (vector.x as f32, vector.y as f32))
    .collect::<Vec<(f32, f32)>>()
}

pub fn draw_dots(
    origin: Vector2,
    size: f64,
    angles: Vec<f64>,
    offset: f64,
) -> Vec<Vec<(f32, f32)>> {
    angles
        .into_iter()
        .map(|angle| origin - Vector2::from_polar(size + DOT_OFFSET, angle + offset))
        .map(|vector| vec![(vector.x as f32, vector.y as f32)])
        .collect::<Vec<Vec<(f32, f32)>>>()
}

pub fn draw_lines(
    origin: Vector2,
    size: f64,
    angles: Vec<f64>,
    offset: f64,
) -> Vec<Vec<(f32, f32)>> {
    angles
        .into_iter()
        .map(|angle| {
            let point1 = origin - Vector2::from_polar(size, angle + offset);
            let point2 = origin - Vector2::from_polar(1.5 * size, angle + offset);
            vec![
                (point1.x as f32, point1.y as f32),
                (point2.x as f32, point2.y as f32),
            ]
        })
        .collect::<Vec<Vec<(f32, f32)>>>()
}
