use crate::glyphs::*;
use geomath::prelude::coordinates::Polar;
use geomath::vector::Vector2;
use std::f64::consts::{FRAC_PI_2, PI};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGallifreyanLetterError;

/// An enumeration for the letters in the Gallifreyan alphabet.
pub enum GallifreyanLetter {
    E,
    A,
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
    pub fn to_gallifreyan_character(&self, position: Vector2) -> GallifreyanCharacter {
        let size = 2.0;
        match self {
            GallifreyanLetter::A => todo!(""),
            GallifreyanLetter::E => todo!(""),
            GallifreyanLetter::I => todo!(""),
            GallifreyanLetter::O => todo!(""),
            GallifreyanLetter::U => todo!(""),
            GallifreyanLetter::B => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: None,
                position,
                size,
            },
            GallifreyanLetter::CH => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Dot2),
                position,
                size,
            },
            GallifreyanLetter::D => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Dot3),
                position,
                size,
            },
            GallifreyanLetter::G => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Line1),
                position,
                size,
            },
            GallifreyanLetter::H => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Line2),
                position,
                size,
            },
            GallifreyanLetter::F => GallifreyanCharacter {
                base: Base::Crescent,
                modifier: Some(Modifier::Line3),
                position,
                size,
            },
            GallifreyanLetter::J => GallifreyanCharacter {
                base: Base::Full,
                modifier: None,
                position,
                size,
            },
            GallifreyanLetter::PH => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot1),
                position,
                size,
            },
            GallifreyanLetter::K => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot2),
                position,
                size,
            },
            GallifreyanLetter::L => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot3),
                position,
                size,
            },
            GallifreyanLetter::C => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Dot4),
                position,
                size,
            },
            GallifreyanLetter::N => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Line1),
                position,
                size,
            },
            GallifreyanLetter::P => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Line2),
                position,
                size,
            },
            GallifreyanLetter::M => GallifreyanCharacter {
                base: Base::Full,
                modifier: Some(Modifier::Line3),
                position,
                size,
            },
            GallifreyanLetter::T => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: None,
                position,
                size,
            },
            GallifreyanLetter::WH => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Dot1),
                position,
                size,
            },
            GallifreyanLetter::SH => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Dot2),
                position,
                size,
            },
            GallifreyanLetter::R => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Dot3),
                position,
                size,
            },
            GallifreyanLetter::V => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Line1),
                position,
                size,
            },
            GallifreyanLetter::W => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Line2),
                position,
                size,
            },
            GallifreyanLetter::S => GallifreyanCharacter {
                base: Base::Quarter,
                modifier: Some(Modifier::Line3),
                position,
                size,
            },
            GallifreyanLetter::TH => GallifreyanCharacter {
                base: Base::New,
                modifier: None,
                position,
                size,
            },
            GallifreyanLetter::GH => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot1),
                position,
                size,
            },
            GallifreyanLetter::Y => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot2),
                position,
                size,
            },
            GallifreyanLetter::Z => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot3),
                position,
                size,
            },
            GallifreyanLetter::Q => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Dot4),
                position,
                size,
            },
            GallifreyanLetter::QU => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Line1),
                position,
                size,
            },
            GallifreyanLetter::X => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Line2),
                position,
                size,
            },
            GallifreyanLetter::NG => GallifreyanCharacter {
                base: Base::New,
                modifier: Some(Modifier::Line3),
                position,
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

pub struct GallifreyanWord(Vec<GallifreyanLetter>);

impl FromIterator<GallifreyanLetter> for GallifreyanWord {
    fn from_iter<T: IntoIterator<Item = GallifreyanLetter>>(iter: T) -> Self {
        let mut word = GallifreyanWord::new();

        for i in iter {
            word.push(i);
        }

        word
    }
}

impl GallifreyanWord {
    pub fn new() -> GallifreyanWord {
        GallifreyanWord(Vec::new())
    }

    fn push(&mut self, letter: GallifreyanLetter) {
        self.0.push(letter);
    }

    pub fn from(word: &str) -> Result<GallifreyanWord, ParseGallifreyanLetterError> {
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
        grouped_letters
            .iter()
            .map(|letter| letter.parse::<GallifreyanLetter>())
            .collect::<Result<GallifreyanWord, ParseGallifreyanLetterError>>()
    }

    pub fn to_gallifreyan_characters(&self, word_size: f64) -> Vec<GallifreyanCharacter> {
        let mut current_position: f64 = -1.0;
        let step_size: f64 = 2.0 * PI
            / self
                .0
                .iter()
                .filter(|gallifreyan_letter| !gallifreyan_letter.is_vowel())
                .map(|_| 1.0)
                .sum::<f64>();

        self.0
            .iter()
            .map(|gallifreyan_letter| match gallifreyan_letter.is_vowel() {
                true => current_position,
                false => {
                    current_position += 1.0;
                    current_position
                }
            })
            .map(|position| (position * step_size) - FRAC_PI_2)
            .zip(&mut self.0.iter())
            .map(|(position, gallifreyan_letter)| {
                gallifreyan_letter
                    .to_gallifreyan_character(Vector2::from_polar(word_size, position))
            })
            .collect::<GallifreyanCharacterCollection>()
            .0
    }
}
