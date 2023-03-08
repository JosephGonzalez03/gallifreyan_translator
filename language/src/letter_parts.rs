use geomath::{prelude::coordinates::Polar, vector::Vector2};
use std::f64::consts::{FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI};

pub const CRESCENT_BASE_RATIO: f64 = 0.9;
pub const FULL_BASE_RATIO: f64 = 1.2;
pub const DEFAULT_BASE_RATIO: f64 = 0.0;
pub const DOT_OFFSET_RATIO: f64 = 0.4;
pub const CRESCENT_BASE_OFFSET: f64 = FRAC_PI_6;
pub const QUARTER_BASE_OFFSET: f64 = 5.0 * PI / 9.0;

pub fn draw_base(position: Vector2, size: f64, range: (f64, f64), offset: f64) -> Vec<(f32, f32)> {
    let start_range = ((range.0 * 180.0) / PI).round() as i64;
    let end_range = ((range.1 * 180.0) / PI).round() as i64;

    match start_range < end_range {
        true => (start_range..=end_range),
        false => (start_range..=end_range + 360),
    }
    .map(|angle| position + Vector2::from_polar(size, (angle as f64 * PI) / 180.0 + offset))
    .map(|vector| (vector.x as f32, vector.y as f32))
    .collect::<Vec<(f32, f32)>>()
}

fn draw_dots(position: Vector2, size: f64, angles: Vec<f64>, offset: f64) -> Vec<Vec<(f32, f32)>> {
    angles
        .into_iter()
        .map(|angle| position - Vector2::from_polar(size + DOT_OFFSET_RATIO, angle + offset))
        .map(|vector| vec![(vector.x as f32, vector.y as f32)])
        .collect::<Vec<Vec<(f32, f32)>>>()
}

fn draw_lines(position: Vector2, size: f64, angles: Vec<f64>, offset: f64) -> Vec<Vec<(f32, f32)>> {
    angles
        .into_iter()
        .map(|angle| {
            let point1 = position - Vector2::from_polar(size, angle + offset);
            let point2 = position - Vector2::from_polar(1.5 * size, angle + offset);
            vec![
                (point1.x as f32, point1.y as f32),
                (point2.x as f32, point2.y as f32),
            ]
        })
        .collect::<Vec<Vec<(f32, f32)>>>()
}

#[derive(Clone, Copy)]
pub enum Base {
    Vowel,
    Crescent,
    Full,
    Quarter,
    New,
}

impl Base {
    pub fn to_drawing(&self, position: Vector2, letter_size: f64) -> Vec<(f32, f32)> {
        match self {
            /*
            Base::Vowel => draw_arc_3d(
                &word,
                &MyPolar::new(word.radius() / 3.0, vowel_position.angle()),
                &vowel_position.radius(),
                (Degree(0.0), Degree(360.0)),
            ),
            */
            Base::Vowel => todo!(),
            Base::Crescent => {
                let base = Vector2::from_polar(CRESCENT_BASE_RATIO * letter_size, position.phi());
                draw_base(
                    position - base,
                    letter_size,
                    (CRESCENT_BASE_OFFSET, -CRESCENT_BASE_OFFSET),
                    position.phi(),
                )
            }
            Base::Full => {
                let base = Vector2::from_polar(FULL_BASE_RATIO * letter_size, position.phi());
                draw_base(
                    position - base,
                    letter_size,
                    (0.0, 2.0 * PI),
                    position.phi(),
                )
            }
            Base::Quarter => {
                let base = Vector2::from_polar(DEFAULT_BASE_RATIO * letter_size, position.phi());
                draw_base(
                    position - base,
                    letter_size,
                    (QUARTER_BASE_OFFSET, -QUARTER_BASE_OFFSET),
                    position.phi(),
                )
            }
            Base::New => {
                let base = Vector2::from_polar(DEFAULT_BASE_RATIO * letter_size, position.phi());
                draw_base(
                    position - base,
                    letter_size,
                    (0.0, 2.0 * PI),
                    position.phi(),
                )
            }
        }
    }

    pub fn has_edge(&self) -> bool {
        match self {
            Base::Crescent | Base::Quarter => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
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
    pub fn to_drawings(
        &self,
        position: Vector2,
        base: Vector2,
        letter_size: f64,
    ) -> Vec<Vec<(f32, f32)>> {
        match self {
            Modifier::Dot1 => draw_dots(position - base, letter_size, vec![0.0], position.phi()),
            Modifier::Dot2 => draw_dots(
                position - base,
                letter_size,
                vec![-FRAC_PI_4, FRAC_PI_4],
                position.phi(),
            ),
            Modifier::Dot3 => draw_dots(
                position - base,
                letter_size,
                vec![-FRAC_PI_4, 0.0, FRAC_PI_4],
                position.phi(),
            ),
            Modifier::Dot4 => draw_dots(
                position - base,
                letter_size,
                vec![-FRAC_PI_4, -FRAC_PI_8, FRAC_PI_8, FRAC_PI_4],
                position.phi(),
            ),
            Modifier::Line1 => draw_lines(position - base, letter_size, vec![0.0], position.phi()),
            Modifier::Line2 => draw_lines(
                position - base,
                letter_size,
                vec![-FRAC_PI_4, FRAC_PI_4],
                position.phi(),
            ),
            Modifier::Line3 => draw_lines(
                position - base,
                letter_size,
                vec![-FRAC_PI_4, 0.0, FRAC_PI_4],
                position.phi(),
            ),
        }
    }
}
