use geomath::{prelude::coordinates::Polar, vector::Vector2};
use std::f64::consts::{FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI};

const CRESCENT_BASE_RATIO: f64 = 0.9;
const FULL_BASE_RATIO: f64 = 1.2;
const MOON_BASE_RATIO: f64 = 1.0;
const DEFAULT_BASE_RATIO: f64 = 0.0;
const DOT_OFFSET: f64 = 0.4;
const CRESCENT_BASE_OFFSET: f64 = FRAC_PI_6;
const QUARTER_BASE_OFFSET: f64 = 5.0 * PI / 9.0;

pub fn draw_base(origin: Vector2, size: f64, range: (f64, f64), offset: f64) -> Vec<(f32, f32)> {
    let start_range = ((range.0 * 180.0) / PI).round() as i64;
    let end_range = ((range.1 * 180.0) / PI).round() as i64;

    match start_range < end_range {
        true => (start_range..=end_range),
        false => (start_range..=end_range + 360),
    }
    .map(|angle| origin + Vector2::from_polar(size, (angle as f64 * PI) / 180.0 + offset))
    .map(|vector| (vector.x as f32, vector.y as f32))
    .collect::<Vec<(f32, f32)>>()
}

fn draw_dots(origin: Vector2, size: f64, angles: Vec<f64>, offset: f64) -> Vec<Vec<(f32, f32)>> {
    angles
        .into_iter()
        .map(|angle| origin - Vector2::from_polar(size + DOT_OFFSET, angle + offset))
        .map(|vector| vec![(vector.x as f32, vector.y as f32)])
        .collect::<Vec<Vec<(f32, f32)>>>()
}

fn draw_lines(origin: Vector2, size: f64, angles: Vec<f64>, offset: f64) -> Vec<Vec<(f32, f32)>> {
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

pub enum Base {
    Moon(f64),
    Core,
    Crescent,
    Full,
    Quarter,
    New,
}

impl Base {
    pub fn base_vector(&self, letter_size: f64, phi: f64) -> Vector2 {
        match self {
            Base::Crescent => Vector2::from_polar(CRESCENT_BASE_RATIO * letter_size, phi),
            Base::Full => Vector2::from_polar(FULL_BASE_RATIO * letter_size, phi),
            Base::Moon(offset) => Vector2::from_polar(MOON_BASE_RATIO * letter_size, phi + offset),
            _ => Vector2::from_polar(DEFAULT_BASE_RATIO * letter_size, phi),
        }
    }

    pub fn to_drawing(&self, origin: Vector2, letter_size: f64) -> Vec<(f32, f32)> {
        match self {
            Base::Moon(_) => draw_base(
                origin + self.base_vector(letter_size, origin.phi()),
                letter_size / 3.0,
                (0.0, 2.0 * PI),
                origin.phi(),
            ),
            Base::Core => draw_base(
                origin - self.base_vector(letter_size, origin.phi()),
                letter_size / 3.0,
                (0.0, 2.0 * PI),
                origin.phi(),
            ),
            Base::Crescent => draw_base(
                origin - self.base_vector(letter_size, origin.phi()),
                letter_size,
                (CRESCENT_BASE_OFFSET, -CRESCENT_BASE_OFFSET),
                origin.phi(),
            ),
            Base::Full => draw_base(
                origin - self.base_vector(letter_size, origin.phi()),
                letter_size,
                (0.0, 2.0 * PI),
                origin.phi(),
            ),
            Base::Quarter => draw_base(
                origin - self.base_vector(letter_size, origin.phi()),
                letter_size,
                (QUARTER_BASE_OFFSET, -QUARTER_BASE_OFFSET),
                origin.phi(),
            ),
            Base::New => {
                let base = Vector2::from_polar(DEFAULT_BASE_RATIO * letter_size, origin.phi());
                draw_base(origin - base, letter_size, (0.0, 2.0 * PI), origin.phi())
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

pub enum Modifier {
    Dot1,
    Dot2,
    Dot3,
    Dot4,
    VowelLine1(f64),
    Line1,
    Line2,
    Line3,
}

impl Modifier {
    pub fn to_drawings(&self, origin: Vector2, letter_size: f64) -> Vec<Vec<(f32, f32)>> {
        match self {
            Modifier::Dot1 => draw_dots(origin, letter_size, vec![0.0], origin.phi()),
            Modifier::Dot2 => draw_dots(
                origin,
                letter_size,
                vec![-FRAC_PI_4, FRAC_PI_4],
                origin.phi(),
            ),
            Modifier::Dot3 => draw_dots(
                origin,
                letter_size,
                vec![-FRAC_PI_4, 0.0, FRAC_PI_4],
                origin.phi(),
            ),
            Modifier::Dot4 => draw_dots(
                origin,
                letter_size,
                vec![-FRAC_PI_4, -FRAC_PI_8, FRAC_PI_8, FRAC_PI_4],
                origin.phi(),
            ),
            Modifier::VowelLine1(offset) => {
                draw_lines(origin, letter_size / 3.0, vec![0.0], origin.phi() + offset)
            }
            Modifier::Line1 => draw_lines(origin, letter_size, vec![0.0], origin.phi()),
            Modifier::Line2 => draw_lines(
                origin,
                letter_size,
                vec![-FRAC_PI_4, FRAC_PI_4],
                origin.phi(),
            ),
            Modifier::Line3 => draw_lines(
                origin,
                letter_size,
                vec![-FRAC_PI_4, 0.0, FRAC_PI_4],
                origin.phi(),
            ),
        }
    }
}

pub struct GallifreyanCharacter {
    pub base: Base,
    pub modifier: Option<Modifier>,
    pub origin: Vector2,
    pub size: f64,
}

impl GallifreyanCharacter {
    pub fn base_vector(&self) -> Vector2 {
        self.base.base_vector(self.size, self.origin.phi())
    }

    pub fn draw_base(&self) -> Vec<(f32, f32)> {
        self.base.to_drawing(self.origin, self.size).to_owned()
    }

    pub fn draw_modifier(&self) -> Option<Vec<Vec<(f32, f32)>>> {
        if let Some(modifier) = &self.modifier {
            Some(modifier.to_drawings(self.origin - self.base_vector(), self.size))
        } else {
            None
        }
    }

    pub fn has_edge(&self) -> bool {
        matches!(&self.base, Base::Crescent | Base::Quarter)
    }

    fn find_edge_wrt_word(&self, angle: f64) -> f64 {
        ((self.size * angle.sin()) / self.origin.rho()).asin()
    }

    pub fn starting_angle(&self) -> Option<f64> {
        match self.base {
            Base::Crescent => {
                Some(self.origin.phi() - self.find_edge_wrt_word(CRESCENT_BASE_OFFSET))
            }
            Base::Quarter => Some(self.origin.phi() - self.find_edge_wrt_word(QUARTER_BASE_OFFSET)),
            _ => None,
        }
    }

    pub fn ending_angle(&self) -> Option<f64> {
        match self.base {
            Base::Crescent => {
                Some(self.origin.phi() + self.find_edge_wrt_word(CRESCENT_BASE_OFFSET))
            }
            Base::Quarter => Some(self.origin.phi() + self.find_edge_wrt_word(QUARTER_BASE_OFFSET)),
            _ => None,
        }
    }
}

pub struct GallifreyanCharacterCollection(pub Vec<GallifreyanCharacter>);

impl FromIterator<GallifreyanCharacter> for GallifreyanCharacterCollection {
    fn from_iter<T: IntoIterator<Item = GallifreyanCharacter>>(iter: T) -> Self {
        let mut gallifreyan_characters = GallifreyanCharacterCollection::new();

        for i in iter {
            gallifreyan_characters.0.push(i)
        }

        gallifreyan_characters
    }
}

impl GallifreyanCharacterCollection {
    fn new() -> Self {
        Self(Vec::new())
    }
}
