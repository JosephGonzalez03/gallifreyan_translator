use std::mem::discriminant;

use crate::math_util::*;

pub const CRESCENT_HEIGHT: f32 = 0.9;
pub const FULL_HEIGHT: f32 = 1.2;
pub const DEFAULT_BASE_HEIGHT: f32 = 0.0;
const DOT_OFFSET_HEIGHT: f32 = 0.2;
const CRESCENT_BASE_OFFSET: Degree = Degree(30.0);
const QUARTER_BASE_OFFSET: Degree = Degree(100.0);

pub enum Base {
    Vowel,
    Crescent,
    Full,
    Quarter,
    New,
}

impl PartialEq for Base {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Base {
    pub fn to_drawing(&self, word: Polar, letter: &f32) -> Drawing {
        match self {
            /*
            Base::Vowel => draw_arc_3d(
                &word,
                &Polar::new(word.radius() / 3.0, vowel_position.angle()),
                &vowel_position.radius(),
                (Degree(0.0), Degree(360.0)),
            ),
            */
            Base::Vowel => todo!(),
            Base::Crescent => draw_arc_3d(
                &word,
                &Polar::new(letter * CRESCENT_HEIGHT, word.angle() + Degree(180.0)),
                letter,
                (
                    word.angle() + CRESCENT_BASE_OFFSET,
                    word.angle() - CRESCENT_BASE_OFFSET,
                ),
            ),
            Base::Full => draw_arc_3d(
                &word,
                &Polar::new(letter * FULL_HEIGHT, word.angle() + Degree(180.0)),
                letter,
                (Degree(0.0), Degree(360.0)),
            ),
            Base::Quarter => draw_arc_3d(
                &word,
                &Polar::new(DEFAULT_BASE_HEIGHT, Degree(180.0)),
                letter,
                (
                    word.angle() + QUARTER_BASE_OFFSET,
                    word.angle() - QUARTER_BASE_OFFSET,
                ),
            ),
            Base::New => draw_arc_3d(
                &word,
                &Polar::new(DEFAULT_BASE_HEIGHT, Degree(0.0)),
                letter,
                (Degree(0.0), Degree(360.0)),
            ),
        }
    }

    fn starting_angle(&self, word: &Polar, letter: &f32) -> Option<Degree> {
        match self {
            Base::Crescent => Some(
                word.angle() - law_of_sines_angle(&word.radius(), letter, CRESCENT_BASE_OFFSET),
            ),
            Base::Quarter => {
                Some(word.angle() - law_of_sines_angle(&word.radius(), letter, QUARTER_BASE_OFFSET))
            }
            _ => None,
        }
    }

    fn ending_angle(&self, word: &Polar, letter: &f32) -> Option<Degree> {
        match self {
            Base::Crescent => Some(
                word.angle() + law_of_sines_angle(&word.radius(), letter, CRESCENT_BASE_OFFSET),
            ),
            Base::Quarter => {
                Some(word.angle() + law_of_sines_angle(&word.radius(), letter, QUARTER_BASE_OFFSET))
            }
            _ => None,
        }
    }

    pub fn has_edge(&self) -> bool {
        match self {
            Base::Crescent | Base::Quarter => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct DrawingCollection(Vec<Drawing>);

impl DrawingCollection {
    fn new() -> Self {
        Self(Vec::new())
    }

    pub fn drawings(self) -> Vec<Drawing> {
        self.0
    }

    pub fn append(&mut self, mut other: Vec<Drawing>) {
        self.0.append(&mut other)
    }

    fn push(&mut self, value: Drawing) {
        self.0.push(value)
    }
}

impl FromIterator<Vec<Drawing>> for DrawingCollection {
    fn from_iter<T: IntoIterator<Item = Vec<Drawing>>>(iter: T) -> Self {
        let mut drawings = DrawingCollection::new();

        for i in iter {
            drawings.append(i)
        }

        drawings
    }
}

fn draw_dots(word: &Polar, letter: &f32, base: &f32, angles: Vec<Degree>) -> Vec<Drawing> {
    angles
        .into_iter()
        .map(|angle| {
            draw_dot_3d(
                word,
                &Polar::new(
                    letter * (base + DOT_OFFSET_HEIGHT),
                    word.angle() + Degree(180.0),
                ),
                &Polar::new(*letter, word.angle() + angle + Degree(180.0)),
            )
        })
        .collect()
}

fn draw_lines(word: &Polar, letter: &f32, base: &f32, angles: Vec<Degree>) -> Vec<Drawing> {
    angles
        .into_iter()
        .map(|angle| {
            draw_line_3d(
                word,
                &Polar::new(letter * base, word.angle() + Degree(180.0)),
                &Polar::new(*letter, word.angle() + angle + Degree(180.0)),
            )
        })
        .collect()
}

pub enum Modifier {
    Dot1,
    Dot2,
    Dot3,
    Dot4,
    Line1,
    Line2,
    Line3,
}

impl Modifier {
    pub fn to_drawings(&self, word: &Polar, letter: &f32, base: &f32) -> Vec<Drawing> {
        match self {
            Modifier::Dot1 => draw_dots(word, letter, base, vec![Degree(0.0)]),
            Modifier::Dot2 => draw_dots(word, letter, base, vec![Degree(-45.0), Degree(45.0)]),
            Modifier::Dot3 => draw_dots(
                word,
                letter,
                base,
                vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
            ),
            Modifier::Dot4 => draw_dots(
                word,
                letter,
                base,
                vec![Degree(-45.0), Degree(-22.5), Degree(22.5), Degree(45.0)],
            ),
            Modifier::Line1 => draw_lines(word, letter, base, vec![Degree(90.0)]),
            Modifier::Line2 => draw_lines(word, letter, base, vec![Degree(-45.0), Degree(45.0)]),
            Modifier::Line3 => draw_lines(
                word,
                letter,
                base,
                vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
            ),
        }
    }
}
