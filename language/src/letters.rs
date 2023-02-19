use crate::letter_parts::*;
use crate::math_util::{Degree, Polar};
use std::mem::discriminant;

pub struct LetterParts {
    base: Base,
    modifiers: Option<Modifier>,
}

impl LetterParts {
    pub fn base(&self) -> &Base {
        &self.base
    }

    pub fn modifiers(&self) -> &Option<Modifier> {
        &self.modifiers
    }
}

/// An enumeration for the letters in the Gallifreyan alphabet.
pub enum GallifreyanLetter {
    A(Polar),
    E(Polar),
    I(Polar),
    O(Polar),
    U(Polar),
    B(Polar),
    CH(Polar),
    D(Polar),
    G(Polar),
    H(Polar),
    F(Polar),
    J(Polar),
    PH(Polar),
    K(Polar),
    L(Polar),
    C(Polar),
    N(Polar),
    P(Polar),
    M(Polar),
    T(Polar),
    WH(Polar),
    SH(Polar),
    R(Polar),
    V(Polar),
    W(Polar),
    S(Polar),
    TH(Polar),
    GH(Polar),
    Y(Polar),
    Z(Polar),
    Q(Polar),
    QU(Polar),
    X(Polar),
    NG(Polar),
}

impl PartialEq for GallifreyanLetter {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl GallifreyanLetter {
    pub fn letter(&self) -> LetterParts {
        let letter_size = Polar::new(2.0, Degree(180.0));
        match self {
            GallifreyanLetter::A(position) => todo!(""),
            GallifreyanLetter::E(position) => todo!(""),
            GallifreyanLetter::I(position) => todo!(""),
            GallifreyanLetter::O(position) => todo!(""),
            GallifreyanLetter::U(position) => todo!(""),
            GallifreyanLetter::B(position) => LetterParts {
                base: Base::Crescent(*position, letter_size),
                modifiers: None,
            },
            GallifreyanLetter::CH(position) => LetterParts {
                base: Base::Crescent(*position, letter_size),
                modifiers: Some(Modifier::Dot2(*position, letter_size, CRESCENT_HEIGHT)),
            },
            GallifreyanLetter::D(position) => LetterParts {
                base: Base::Crescent(*position, letter_size),
                modifiers: Some(Modifier::Dot3(*position, letter_size, CRESCENT_HEIGHT)),
            },
            GallifreyanLetter::G(position) => LetterParts {
                base: Base::Crescent(*position, letter_size),
                modifiers: Some(Modifier::Line1(
                    *position,
                    letter_size,
                    CRESCENT_HEIGHT,
                    Degree(0.0),
                )),
            },
            GallifreyanLetter::H(position) => LetterParts {
                base: Base::Crescent(*position, letter_size),
                modifiers: Some(Modifier::Line2(*position, letter_size, CRESCENT_HEIGHT)),
            },
            GallifreyanLetter::F(position) => LetterParts {
                base: Base::Crescent(*position, letter_size),
                modifiers: Some(Modifier::Line3(*position, letter_size, CRESCENT_HEIGHT)),
            },
            GallifreyanLetter::J(position) => LetterParts {
                base: Base::Full(*position, letter_size),
                modifiers: None,
            },
            GallifreyanLetter::PH(position) => LetterParts {
                base: Base::Full(*position, letter_size),
                modifiers: Some(Modifier::Dot1(*position, letter_size, FULL_HEIGHT)),
            },
            GallifreyanLetter::K(position) => LetterParts {
                base: Base::Full(*position, letter_size),
                modifiers: Some(Modifier::Dot2(*position, letter_size, FULL_HEIGHT)),
            },
            GallifreyanLetter::L(position) => LetterParts {
                base: Base::Full(*position, letter_size),
                modifiers: Some(Modifier::Dot3(*position, letter_size, FULL_HEIGHT)),
            },
            GallifreyanLetter::C(position) => LetterParts {
                base: Base::Full(*position, letter_size),
                modifiers: Some(Modifier::Dot4(*position, letter_size, FULL_HEIGHT)),
            },
            GallifreyanLetter::N(position) => LetterParts {
                base: Base::Full(*position, letter_size),
                modifiers: Some(Modifier::Line1(
                    *position,
                    letter_size,
                    FULL_HEIGHT,
                    Degree(0.0),
                )),
            },
            GallifreyanLetter::P(position) => LetterParts {
                base: Base::Full(*position, letter_size),
                modifiers: Some(Modifier::Line2(*position, letter_size, FULL_HEIGHT)),
            },
            GallifreyanLetter::M(position) => LetterParts {
                base: Base::Full(*position, letter_size),
                modifiers: Some(Modifier::Line3(*position, letter_size, FULL_HEIGHT)),
            },
            GallifreyanLetter::T(position) => LetterParts {
                base: Base::Quarter(*position, letter_size),
                modifiers: None,
            },
            GallifreyanLetter::WH(position) => LetterParts {
                base: Base::Quarter(*position, letter_size),
                modifiers: Some(Modifier::Dot1(*position, letter_size, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::SH(position) => LetterParts {
                base: Base::Quarter(*position, letter_size),
                modifiers: Some(Modifier::Dot2(*position, letter_size, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::R(position) => LetterParts {
                base: Base::Quarter(*position, letter_size),
                modifiers: Some(Modifier::Dot3(*position, letter_size, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::V(position) => LetterParts {
                base: Base::Quarter(*position, letter_size),
                modifiers: Some(Modifier::Line1(
                    *position,
                    letter_size,
                    DEFAULT_BASE_HEIGHT,
                    Degree(0.0),
                )),
            },
            GallifreyanLetter::W(position) => LetterParts {
                base: Base::Quarter(*position, letter_size),
                modifiers: Some(Modifier::Line2(*position, letter_size, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::S(position) => LetterParts {
                base: Base::Quarter(*position, letter_size),
                modifiers: Some(Modifier::Line3(*position, letter_size, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::TH(position) => LetterParts {
                base: Base::New(*position, letter_size),
                modifiers: None,
            },
            GallifreyanLetter::GH(position) => LetterParts {
                base: Base::New(*position, letter_size),
                modifiers: Some(Modifier::Dot1(*position, letter_size, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::Y(position) => LetterParts {
                base: Base::New(*position, letter_size),
                modifiers: Some(Modifier::Dot2(*position, letter_size, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::Z(position) => LetterParts {
                base: Base::New(*position, letter_size),
                modifiers: Some(Modifier::Dot3(*position, letter_size, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::Q(position) => LetterParts {
                base: Base::New(*position, letter_size),
                modifiers: Some(Modifier::Dot4(*position, letter_size, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::QU(position) => LetterParts {
                base: Base::New(*position, letter_size),
                modifiers: Some(Modifier::Line1(
                    *position,
                    letter_size,
                    DEFAULT_BASE_HEIGHT,
                    Degree(0.0),
                )),
            },
            GallifreyanLetter::X(position) => LetterParts {
                base: Base::New(*position, letter_size),
                modifiers: Some(Modifier::Line2(*position, letter_size, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::NG(position) => LetterParts {
                base: Base::New(*position, letter_size),
                modifiers: Some(Modifier::Line3(*position, letter_size, DEFAULT_BASE_HEIGHT)),
            },
        }
    }

    fn is_vowel(&self) -> bool {
        let _position = Polar::new(0.0, Degree(0.0));
        matches!(
            self,
            GallifreyanLetter::A(_position)
                | GallifreyanLetter::E(_position)
                | GallifreyanLetter::I(_position)
                | GallifreyanLetter::O(_position)
                | GallifreyanLetter::U(_position)
        )
    }

    /*
    pub fn buffer(&self, next: GallifreyanLetter) -> Option<Vec<(f32, f32)>> {
        if self.is_vowel() || next.is_vowel() {
            return None;
        }

        let edge1 = self
            .letter()
            .base()
            .ending_angle()
            .expect("Vowels don't have a letter part with an edge. There is an early return if self is a vowel.");
        let edge2 = next
            .letter()
            .base()
            .starting_angle()
            .expect("Vowels don't have a letter part with an edge. There is an early return if self is a vowel.");

        Some(arc3_d(
            self.letter().base().position(),
            &Polar::new(0.0, Degree(0.0)),
            0.0,
            (edge1, edge2),
        ))
    }
    */
}
