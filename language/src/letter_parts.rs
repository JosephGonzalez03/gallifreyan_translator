use std::mem::discriminant;

use crate::math_util::*;

pub const CRESCENT_HEIGHT: f32 = 0.9;
pub const FULL_HEIGHT: f32 = 1.2;
pub const DEFAULT_BASE_HEIGHT: f32 = 0.0;
const DOT_OFFSET_HEIGHT: f32 = 0.2;

fn draw_dots(letter: &Polar, base: &Polar, base_height: &f32, angles: Vec<Degree>) -> Vec<Drawing> {
    angles
        .into_iter()
        .map(|angle| {
            dot(
                letter,
                &(base * &Polar::new(base_height + DOT_OFFSET_HEIGHT, Degree(0.0))),
                &(base * &Polar::new(1.0, angle)),
            )
        })
        .collect()
}

fn draw_lines(
    letter: &Polar,
    base: &Polar,
    base_height: &f32,
    angles: Vec<Degree>,
) -> Vec<Drawing> {
    angles
        .into_iter()
        .map(|angle| {
            normal_line(
                letter,
                &(base * &Polar::new(*base_height, Degree(0.0))),
                &(base * &Polar::new(1.0, angle)),
            )
        })
        .collect()
}

pub enum Base {
    Vowel(Polar, Polar),
    Crescent(Polar, Polar),
    Full(Polar, Polar),
    Quarter(Polar, Polar),
    New(Polar, Polar),
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
            Base::Vowel(position, vowel_position) => arc3_d(
                &position,
                &Polar::new(position.radius() / 3.0, vowel_position.angle()),
                vowel_position.radius(),
                (Degree(0.0), Degree(360.0)),
            ),
            Base::Crescent(letter, base) => arc3_d(
                &letter,
                &(base * &Polar::new(CRESCENT_HEIGHT, Degree(180.0))),
                base.radius(),
                (
                    letter.angle() + Degree(30.0),
                    letter.angle() + Degree(330.0),
                ),
            ),
            Base::Full(letter, base) => arc3_d(
                &letter,
                &(base * &Polar::new(FULL_HEIGHT, Degree(0.0))),
                base.radius(),
                (Degree(0.0), Degree(360.0)),
            ),
            Base::Quarter(letter, base) => arc3_d(
                &letter,
                &Polar::new(DEFAULT_BASE_HEIGHT, Degree(180.0)),
                base.radius(),
                (
                    letter.angle() + Degree(95.0),
                    letter.angle() + Degree(265.0),
                ),
            ),
            Base::New(letter, base) => arc3_d(
                &letter,
                &Polar::new(DEFAULT_BASE_HEIGHT, Degree(0.0)),
                base.radius(),
                (letter.angle() + Degree(0.0), letter.angle() + Degree(360.0)),
            ),
        }
    }

    fn starting_angle(&self) -> Option<Degree> {
        match self {
            Base::Crescent(letter, base) => Some(
                letter.angle() + law_of_sines_angle(
                    &letter.radius(),
                    &base.radius(),
                    Degree(30.0),
                ),
            ),
            Base::Quarter(letter, base) => Some(
                letter.angle() + law_of_sines_angle(
                    &letter.radius(),
                    &base.radius(),
                    Degree(90.0),
                )
            ),
            _ => None,
        }
    }

    fn ending_angle(&self) -> Option<Degree> {
        match self {
            Base::Crescent(letter, base) => Some(
                letter.angle() - law_of_sines_angle(
                    &letter.radius(),
                    &base.radius(),
                    Degree(30.0),
                ),
            ),
            Base::Quarter(letter, base) => Some(
                letter.angle() - law_of_sines_angle(
                    &letter.radius(),
                    &base.radius(),
                    Degree(90.0),
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

        Some(arc(
            self.position().radius(),
            (edge1, edge2)
        ))
    }
}

pub enum Modifier {
    Dot1(Polar, Polar, f32),
    Dot2(Polar, Polar, f32),
    Dot3(Polar, Polar, f32),
    Dot4(Polar, Polar, f32),
    Line1(Polar, Polar, f32, Degree),
    Line2(Polar, Polar, f32),
    Line3(Polar, Polar, f32),
}

impl Modifier {
    pub fn to_drawings(&self) -> Vec<Drawing> {
        match self {
            Modifier::Dot1(letter, base, base_height) => {
                draw_dots(letter, base, base_height, vec![Degree(0.0)])
            }
            Modifier::Dot2(letter, base, base_height) => {
                draw_dots(letter, base, base_height, vec![Degree(-45.0), Degree(45.0)])
            }
            Modifier::Dot3(letter, base, base_height) => draw_dots(
                letter,
                base,
                base_height,
                vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
            ),
            Modifier::Dot4(letter, base, base_height) => draw_dots(
                letter,
                base,
                base_height,
                vec![Degree(-45.0), Degree(-22.5), Degree(22.5), Degree(45.0)],
            ),
            Modifier::Line1(letter, base, base_height, degree) => {
                draw_lines(letter, base, base_height, vec![*degree])
            }
            Modifier::Line2(letter, base, base_height) => {
                draw_lines(letter, base, base_height, vec![Degree(-45.0), Degree(45.0)])
            }
            Modifier::Line3(letter, base, base_height) => draw_lines(
                letter,
                base,
                base_height,
                vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
            ),
        }
    }
}
