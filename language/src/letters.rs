use crate::glyphs::*;
use core::fmt;
use geomath::prelude::coordinates::Polar;
use geomath::vector::Vector2;
use std::f64::consts::{FRAC_PI_2, PI};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGallifreyanLetterError;

/// An enumeration for the letters in the Gallifreyan alphabet.
#[derive(Clone, Copy, Debug)]
pub enum GallifreyanLetter {
    A,
    E,
    I,
    O,
    U,
    B,
    CH,
    D,
    G,
    H,
    F,
    J,
    PH,
    K,
    L,
    C,
    N,
    P,
    M,
    T,
    WH,
    SH,
    R,
    V,
    W,
    S,
    TH,
    GH,
    Y,
    Z,
    Q,
    QU,
    X,
    NG,
}

impl Display for GallifreyanLetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GallifreyanLetter::A => write!(f, "Gallifreyan A"),
            GallifreyanLetter::E => write!(f, "Gallifreyan E"),
            GallifreyanLetter::I => write!(f, "Gallifreyan I"),
            GallifreyanLetter::O => write!(f, "Gallifreyan O"),
            GallifreyanLetter::U => write!(f, "Gallifreyan U"),
            GallifreyanLetter::B => write!(f, "Gallifreyan B"),
            GallifreyanLetter::CH => write!(f, "Gallifreyan CH"),
            GallifreyanLetter::D => write!(f, "Gallifreyan D"),
            GallifreyanLetter::G => write!(f, "Gallifreyan G"),
            GallifreyanLetter::H => write!(f, "Gallifreyan H"),
            GallifreyanLetter::F => write!(f, "Gallifreyan F"),
            GallifreyanLetter::J => write!(f, "Gallifreyan J"),
            GallifreyanLetter::PH => write!(f, "Gallifreyan PH"),
            GallifreyanLetter::K => write!(f, "Gallifreyan K"),
            GallifreyanLetter::L => write!(f, "Gallifreyan L"),
            GallifreyanLetter::C => write!(f, "Gallifreyan C"),
            GallifreyanLetter::N => write!(f, "Gallifreyan N"),
            GallifreyanLetter::P => write!(f, "Gallifreyan P"),
            GallifreyanLetter::M => write!(f, "Gallifreyan M"),
            GallifreyanLetter::T => write!(f, "Gallifreyan T"),
            GallifreyanLetter::WH => write!(f, "Gallifreyan WH"),
            GallifreyanLetter::SH => write!(f, "Gallifreyan SH"),
            GallifreyanLetter::R => write!(f, "Gallifreyan R"),
            GallifreyanLetter::V => write!(f, "Gallifreyan V"),
            GallifreyanLetter::W => write!(f, "Gallifreyan W"),
            GallifreyanLetter::S => write!(f, "Gallifreyan S"),
            GallifreyanLetter::TH => write!(f, "Gallifreyan TH"),
            GallifreyanLetter::GH => write!(f, "Gallifreyan GH"),
            GallifreyanLetter::Y => write!(f, "Gallifreyan Y"),
            GallifreyanLetter::Z => write!(f, "Gallifreyan Z"),
            GallifreyanLetter::Q => write!(f, "Gallifreyan Q"),
            GallifreyanLetter::QU => write!(f, "Gallifreyan QU"),
            GallifreyanLetter::X => write!(f, "Gallifreyan X"),
            GallifreyanLetter::NG => write!(f, "Gallifreyan NG"),
        }
    }
}

impl FromStr for GallifreyanLetter {
    type Err = ParseGallifreyanLetterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Result::Ok(GallifreyanLetter::A),
            "E" => Result::Ok(GallifreyanLetter::E),
            "I" => Result::Ok(GallifreyanLetter::I),
            "O" => Result::Ok(GallifreyanLetter::O),
            "U" => Result::Ok(GallifreyanLetter::U),
            "B" => Result::Ok(GallifreyanLetter::B),
            "CH" => Result::Ok(GallifreyanLetter::CH),
            "D" => Result::Ok(GallifreyanLetter::D),
            "G" => Result::Ok(GallifreyanLetter::G),
            "H" => Result::Ok(GallifreyanLetter::H),
            "F" => Result::Ok(GallifreyanLetter::F),
            "J" => Result::Ok(GallifreyanLetter::J),
            "PH" => Result::Ok(GallifreyanLetter::PH),
            "K" => Result::Ok(GallifreyanLetter::K),
            "L" => Result::Ok(GallifreyanLetter::L),
            "C" => Result::Ok(GallifreyanLetter::C),
            "N" => Result::Ok(GallifreyanLetter::N),
            "P" => Result::Ok(GallifreyanLetter::P),
            "M" => Result::Ok(GallifreyanLetter::M),
            "T" => Result::Ok(GallifreyanLetter::T),
            "WH" => Result::Ok(GallifreyanLetter::WH),
            "SH" => Result::Ok(GallifreyanLetter::SH),
            "R" => Result::Ok(GallifreyanLetter::R),
            "V" => Result::Ok(GallifreyanLetter::V),
            "W" => Result::Ok(GallifreyanLetter::W),
            "S" => Result::Ok(GallifreyanLetter::S),
            "TH" => Result::Ok(GallifreyanLetter::TH),
            "GH" => Result::Ok(GallifreyanLetter::GH),
            "Y" => Result::Ok(GallifreyanLetter::Y),
            "Z" => Result::Ok(GallifreyanLetter::Z),
            "Q" => Result::Ok(GallifreyanLetter::Q),
            "QU" => Result::Ok(GallifreyanLetter::QU),
            "X" => Result::Ok(GallifreyanLetter::X),
            "NG" => Result::Ok(GallifreyanLetter::NG),
            _ => Result::Err(ParseGallifreyanLetterError),
        }
    }
}

impl GallifreyanLetter {
    pub fn to_gallifreyan_character(&self, origin: Vector2, size: f64) -> GallifreyanCharacter {
        match self {
            GallifreyanLetter::A => GallifreyanCharacter {
                base: Base::Moon(0.0),
                modifier: None,
                origin,
                size,
            },
            GallifreyanLetter::E => GallifreyanCharacter {
                base: Base::Core,
                modifier: None,
                origin,
                size,
            },
            GallifreyanLetter::I => GallifreyanCharacter {
                base: Base::Core,
                modifier: Some(Modifier::VowelLine1(0.0)),
                origin,
                size,
            },
            GallifreyanLetter::O => GallifreyanCharacter {
                base: Base::Moon(PI),
                modifier: None,
                origin,
                size,
            },
            GallifreyanLetter::U => GallifreyanCharacter {
                base: Base::Core,
                modifier: Some(Modifier::VowelLine1(PI)),
                origin,
                size,
            },
            GallifreyanLetter::B => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: None,
                origin,
                size,
            },
            GallifreyanLetter::CH => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Dot2),
                origin,
                size,
            },
            GallifreyanLetter::D => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Dot3),
                origin,
                size,
            },
            GallifreyanLetter::G => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Line1),
                origin,
                size,
            },
            GallifreyanLetter::H => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Line2),
                origin,
                size,
            },
            GallifreyanLetter::F => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Line3),
                origin,
                size,
            },
            GallifreyanLetter::J => GallifreyanCharacter {
                base: Base::Full,
                modifier: None,
                origin,
                size,
            },
            GallifreyanLetter::PH => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot1),
                origin,
                size,
            },
            GallifreyanLetter::K => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot2),
                origin,
                size,
            },
            GallifreyanLetter::L => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot3),
                origin,
                size,
            },
            GallifreyanLetter::C => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot4),
                origin,
                size,
            },
            GallifreyanLetter::N => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Line1),
                origin,
                size,
            },
            GallifreyanLetter::P => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Line2),
                origin,
                size,
            },
            GallifreyanLetter::M => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Line3),
                origin,
                size,
            },
            GallifreyanLetter::T => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: None,
                origin,
                size,
            },
            GallifreyanLetter::WH => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Dot1),
                origin,
                size,
            },
            GallifreyanLetter::SH => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Dot2),
                origin,
                size,
            },
            GallifreyanLetter::R => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Dot3),
                origin,
                size,
            },
            GallifreyanLetter::V => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Line1),
                origin,
                size,
            },
            GallifreyanLetter::W => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Line2),
                origin,
                size,
            },
            GallifreyanLetter::S => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Line3),
                origin,
                size,
            },
            GallifreyanLetter::TH => GallifreyanCharacter {
                base: Base::New,
                modifier: None,
                origin,
                size,
            },
            GallifreyanLetter::GH => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot1),
                origin,
                size,
            },
            GallifreyanLetter::Y => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot2),
                origin,
                size,
            },
            GallifreyanLetter::Z => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot3),
                origin,
                size,
            },
            GallifreyanLetter::Q => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot4),
                origin,
                size,
            },
            GallifreyanLetter::QU => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Line1),
                origin,
                size,
            },
            GallifreyanLetter::X => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Line2),
                origin,
                size,
            },
            GallifreyanLetter::NG => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Line3),
                origin,
                size,
            },
        }
    }

    fn is_vowel(&self) -> bool {
        matches!(
            self,
            GallifreyanLetter::A
                | GallifreyanLetter::E
                | GallifreyanLetter::I
                | GallifreyanLetter::O
                | GallifreyanLetter::U
        )
    }
}

pub struct GallifreyanWord {
    letters: Vec<GallifreyanLetter>,
    size: f64,
}

impl GallifreyanWord {
    const LETTER_SIZE: f64 = 2.0;

    pub fn from(word: &str) -> GallifreyanWord {
        let mut grouped_letters = Vec::<String>::new();
        let mut char_iter = word.chars().into_iter().peekable();

        while let Some(current_letter) = char_iter.next() {
            let entry = match current_letter.to_ascii_uppercase() {
                'C' | 'P' | 'W' | 'S' | 'T' | 'G' => match char_iter.next_if_eq(&'H') {
                    Some(next_letter) => current_letter.to_string() + &next_letter.to_string(),
                    None => current_letter.to_string(),
                },
                'Q' => match char_iter.next_if_eq(&'U') {
                    Some(next_letter) => current_letter.to_string() + &next_letter.to_string(),
                    None => current_letter.to_string(),
                },
                'N' => match char_iter.next_if_eq(&'G') {
                    Some(next_letter) => current_letter.to_string() + &next_letter.to_string(),
                    None => current_letter.to_string(),
                },
                _ => current_letter.to_string(),
            };

            grouped_letters.push(entry);
        }

        grouped_letters.iter().for_each(|gl| println!("{}", gl));
        let parsed_letters = grouped_letters
            .iter()
            .map(|letter| letter.parse::<GallifreyanLetter>())
            .collect::<Result<Vec<GallifreyanLetter>, ParseGallifreyanLetterError>>();

        match parsed_letters {
            Ok(letters) => {
                let mut grouped_letters = Vec::<Vec<&GallifreyanLetter>>::new();
                let mut letter_iter = letters.iter().peekable();

                while let Some(current_letter) = letter_iter.next() {
                    let entry = match current_letter.is_vowel() {
                        true => vec![current_letter],
                        false => match letter_iter.next_if(|next| next.is_vowel()) {
                            Some(next_letter) => vec![current_letter, next_letter],
                            None => vec![current_letter],
                        },
                    };

                    grouped_letters.push(entry);
                }

                let size = match grouped_letters.len() {
                    0..=1 => 1.5 * Self::LETTER_SIZE,
                    2 => 2.8 * Self::LETTER_SIZE,
                    3..=4 => 3.0 * Self::LETTER_SIZE,
                    _ => {
                        (2.0 * (1.9 * Self::LETTER_SIZE))
                            / (2.0 * (PI / grouped_letters.len() as f64).sin())
                    }
                };

                GallifreyanWord {
                    letters,
                    size,
                }
            }
            Err(_) => panic!("The word could not be parsed to Gallifreyan!"),
        }
    }

    pub fn to_gallifreyan_characters(&self) -> Vec<GallifreyanCharacter> {
        let mut grouped_letters = Vec::new();
        let mut letter_iter = self.letters.iter().peekable();

        while let Some(current_letter) = letter_iter.next() {
            let entry = match current_letter.is_vowel() {
                true => vec![current_letter],
                false => match letter_iter.next_if(|next| next.is_vowel()) {
                    Some(next_letter) => vec![current_letter, next_letter],
                    None => vec![current_letter],
                },
            };

            grouped_letters.push(entry);
        }

        let step_size: f64 = 2.0 * PI / grouped_letters.iter().map(|_| 1.0).sum::<f64>();

        grouped_letters
            .iter()
            .enumerate()
            .flat_map(|(index, group)| {
                let position = (index as f64 * step_size) - FRAC_PI_2;
                let mut characters = Vec::new();
                let first_character = group
                    .get(0)
                    .expect("There should be at least one letter in each group.")
                    .to_gallifreyan_character(
                        Vector2::from_polar(self.size, position),
                        Self::LETTER_SIZE,
                    );

                if let Some(letter) = group.get(1) {
                    characters.push(letter.to_gallifreyan_character(
                        Vector2::from_polar(self.size, position) - first_character.base_vector(),
                        Self::LETTER_SIZE,
                    ));
                }
                characters.push(first_character);

                characters
            })
            .collect::<Vec<GallifreyanCharacter>>()
    }

    pub fn draw_edges(&self) -> Vec<Vec<(f32, f32)>> {
        let characters_with_edges = self
            .to_gallifreyan_characters()
            .into_iter()
            .filter(|gallifreyan_character| gallifreyan_character.has_edge())
            .collect::<Vec<GallifreyanCharacter>>();

        if characters_with_edges.len() == 0 {
            return vec![draw_base(
                Vector2::from_polar(0.0, 0.0),
                self.size,
                (0.0, 2.0 * PI),
                0.0,
            )];
        }

        let mut edges: Vec<Vec<(f32, f32)>> = characters_with_edges
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
            .collect();

        let edge1 = characters_with_edges
            .last()
            .expect("The character should exist.")
            .ending_angle()
            .expect("The Gallifreyan character should have an edge.");
        let edge2 = match characters_with_edges.first() {
            Some(character) => character
                .starting_angle()
                .expect("The Gallifreyan character should have an edge."),
            None => characters_with_edges
                .last()
                .expect("The character should exist.")
                .starting_angle()
                .expect("The Gallifreyan character should have an edge."),
        };

        edges.push(draw_base(
            Vector2::from_polar(0.0, 0.0),
            self.size,
            (edge1, edge2),
            0.0,
        ));

        edges
    }
}
