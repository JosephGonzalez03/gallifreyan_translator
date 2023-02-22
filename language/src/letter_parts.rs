use std::mem::discriminant;

use crate::math_util::*;

pub const CRESCENT_HEIGHT: f32 = 0.9;
pub const FULL_HEIGHT: f32 = 1.2;
pub const DEFAULT_BASE_HEIGHT: f32 = 0.0;
const DOT_OFFSET_HEIGHT: f32 = 0.2;
const CRESCENT_BASE_OFFSET: Degree = Degree(30.0);
const QUARTER_BASE_OFFSET: Degree = Degree(100.0);

pub enum Base {
    Vowel(Polar, Polar),
    Crescent(Polar, f32),
    Full(Polar, f32),
    Quarter(Polar, f32),
    New(Polar, f32),
}

impl PartialEq for Base {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Base {
    pub fn position(&self) -> &Polar {
        match self {
            Base::Vowel(position, _) => position,
            Base::Crescent(position, _) => position,
            Base::Full(position, _) => position,
            Base::Quarter(position, _) => position,
            Base::New(position, _) => position,
        }
    }

    pub fn to_drawing(&self) -> Drawing {
        match self {
            Base::Vowel(position, vowel_position) => draw_arc_3d(
                &position,
                &Polar::new(position.radius() / 3.0, vowel_position.angle()),
                &vowel_position.radius(),
                (Degree(0.0), Degree(360.0)),
            ),
            Base::Crescent(word, letter) => draw_arc_3d(
                &word,
                &Polar::new(letter * CRESCENT_HEIGHT, word.angle() + Degree(180.0)),
                letter,
                (
                    word.angle() + CRESCENT_BASE_OFFSET,
                    word.angle() - CRESCENT_BASE_OFFSET,
                ),
            ),
            Base::Full(word, letter) => draw_arc_3d(
                &word,
                &Polar::new(letter * FULL_HEIGHT, word.angle()),
                letter,
                (Degree(0.0), Degree(360.0)),
            ),
            Base::Quarter(word, letter) => draw_arc_3d(
                &word,
                &Polar::new(DEFAULT_BASE_HEIGHT, Degree(180.0)),
                letter,
                (
                    word.angle() + QUARTER_BASE_OFFSET,
                    word.angle() - QUARTER_BASE_OFFSET,
                ),
            ),
            Base::New(word, letter) => draw_arc_3d(
                &word,
                &Polar::new(DEFAULT_BASE_HEIGHT, Degree(0.0)),
                letter,
                (Degree(0.0), Degree(360.0)),
            ),
        }
    }

    fn starting_angle(&self) -> Option<Degree> {
        match self {
            Base::Crescent(word, letter) => Some(
                word.angle() - law_of_sines_angle(
                    &word.radius(),
                    letter,
                    CRESCENT_BASE_OFFSET,
                ),
            ),
            Base::Quarter(word, letter) => Some(
                word.angle() - law_of_sines_angle(
                    &word.radius(),
                    letter,
                    QUARTER_BASE_OFFSET,
                )
            ),
            _ => None,
        }
    }

    fn ending_angle(&self) -> Option<Degree> {
        match self {
            Base::Crescent(word, letter) => Some(
                word.angle() + law_of_sines_angle(
                    &word.radius(),
                    letter,
                    CRESCENT_BASE_OFFSET,
                ),
            ),
            Base::Quarter(word, letter) => Some(
                word.angle() + law_of_sines_angle(
                    &word.radius(),
                    letter,
                    QUARTER_BASE_OFFSET,
                )
            ),
            _ => None,
        }
    }

    fn has_edge(&self) -> bool {
        match self {
            Base::Crescent(_, _) |
            Base::Quarter(_, _) => true,
            _ => false,
        }
    }

    pub fn buffer(&self, next: &Base) -> Option<Drawing> {
        if !self.has_edge() || !next.has_edge() {
            return None
        }

        let edge1 = self
            .ending_angle()
            .expect("There is an early return for bases that don't have edges.");
        let edge2 = next
            .starting_angle()
            .expect("There is an early return for bases that don't have edges.");

        Some(draw_arc(
            self.position().radius(),
            (edge1, edge2)
        ))
    }
}

fn draw_dots(word: &Polar, letter: &f32, base: &f32, angles: Vec<Degree>) -> Vec<Drawing> {
    angles
        .into_iter()
        .map(|angle| {
            draw_dot_3d(
                word,
                &Polar::new(letter * (base + DOT_OFFSET_HEIGHT), word.angle() + Degree(180.0)),
                &Polar::new(*letter, word.angle() + angle + Degree(180.0)),
            )
        })
        .collect()
}

fn draw_lines(
    word: &Polar,
    letter: &f32,
    base: &f32,
    angles: Vec<Degree>,
) -> Vec<Drawing> {
    angles
        .into_iter()
        .map(|angle| {
            draw_line_3d(
                word,
                &Polar::new(letter * base, word.angle() + Degree(180.0)),
                &Polar::new(*letter, word.angle() + angle),
            )
        })
        .collect()
}

pub enum Modifier {
    Dot1(Polar, f32, f32),
    Dot2(Polar, f32, f32),
    Dot3(Polar, f32, f32),
    Dot4(Polar, f32, f32),
    Line1(Polar, f32, f32, Degree),
    Line2(Polar, f32, f32),
    Line3(Polar, f32, f32),
}

impl Modifier {
    pub fn to_drawings(&self) -> Vec<Drawing> {
        match self {
            Modifier::Dot1(word, letter, base) => {
                draw_dots(word, letter, base, vec![Degree(0.0)])
            }
            Modifier::Dot2(word, letter, base) => {
                draw_dots(word, letter, base, vec![Degree(-45.0), Degree(45.0)])
            }
            Modifier::Dot3(word, letter, base) => draw_dots(
                word,
                letter,
                base,
                vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
            ),
            Modifier::Dot4(word, letter, base) => draw_dots(
                word,
                letter,
                base,
                vec![Degree(-45.0), Degree(-22.5), Degree(22.5), Degree(45.0)],
            ),
            Modifier::Line1(word, letter, base, degree) => {
                draw_lines(word, letter, base, vec![*degree])
            }
            Modifier::Line2(word, letter, base) => {
                draw_lines(word, letter, base, vec![Degree(-45.0), Degree(45.0)])
            }
            Modifier::Line3(word, letter, base) => draw_lines(
                word,
                letter,
                base,
                vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
            ),
        }
    }
}
