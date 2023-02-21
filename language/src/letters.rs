use crate::letter_parts::*;
use crate::math_util::{Degree, Polar, Drawing, arc3_d};
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
        let letter = 2.0;
        match self {
            GallifreyanLetter::A(word) => todo!(""),
            GallifreyanLetter::E(word) => todo!(""),
            GallifreyanLetter::I(word) => todo!(""),
            GallifreyanLetter::O(word) => todo!(""),
            GallifreyanLetter::U(word) => todo!(""),
            GallifreyanLetter::B(word) => LetterParts {
                base: Base::Crescent(*word, letter),
                modifiers: None,
            },
            GallifreyanLetter::CH(word) => LetterParts {
                base: Base::Crescent(*word, letter),
                modifiers: Some(Modifier::Dot2(*word, letter, CRESCENT_HEIGHT)),
            },
            GallifreyanLetter::D(word) => LetterParts {
                base: Base::Crescent(*word, letter),
                modifiers: Some(Modifier::Dot3(*word, letter, CRESCENT_HEIGHT)),
            },
            GallifreyanLetter::G(word) => LetterParts {
                base: Base::Crescent(*word, letter),
                modifiers: Some(Modifier::Line1(
                    *word,
                    letter,
                    CRESCENT_HEIGHT,
                    Degree(0.0),
                )),
            },
            GallifreyanLetter::H(word) => LetterParts {
                base: Base::Crescent(*word, letter),
                modifiers: Some(Modifier::Line2(*word, letter, CRESCENT_HEIGHT)),
            },
            GallifreyanLetter::F(word) => LetterParts {
                base: Base::Crescent(*word, letter),
                modifiers: Some(Modifier::Line3(*word, letter, CRESCENT_HEIGHT)),
            },
            GallifreyanLetter::J(word) => LetterParts {
                base: Base::Full(*word, letter),
                modifiers: None,
            },
            GallifreyanLetter::PH(word) => LetterParts {
                base: Base::Full(*word, letter),
                modifiers: Some(Modifier::Dot1(*word, letter, FULL_HEIGHT)),
            },
            GallifreyanLetter::K(word) => LetterParts {
                base: Base::Full(*word, letter),
                modifiers: Some(Modifier::Dot2(*word, letter, FULL_HEIGHT)),
            },
            GallifreyanLetter::L(word) => LetterParts {
                base: Base::Full(*word, letter),
                modifiers: Some(Modifier::Dot3(*word, letter, FULL_HEIGHT)),
            },
            GallifreyanLetter::C(word) => LetterParts {
                base: Base::Full(*word, letter),
                modifiers: Some(Modifier::Dot4(*word, letter, FULL_HEIGHT)),
            },
            GallifreyanLetter::N(word) => LetterParts {
                base: Base::Full(*word, letter),
                modifiers: Some(Modifier::Line1(
                    *word,
                    letter,
                    FULL_HEIGHT,
                    Degree(0.0),
                )),
            },
            GallifreyanLetter::P(word) => LetterParts {
                base: Base::Full(*word, letter),
                modifiers: Some(Modifier::Line2(*word, letter, FULL_HEIGHT)),
            },
            GallifreyanLetter::M(word) => LetterParts {
                base: Base::Full(*word, letter),
                modifiers: Some(Modifier::Line3(*word, letter, FULL_HEIGHT)),
            },
            GallifreyanLetter::T(word) => LetterParts {
                base: Base::Quarter(*word, letter),
                modifiers: None,
            },
            GallifreyanLetter::WH(word) => LetterParts {
                base: Base::Quarter(*word, letter),
                modifiers: Some(Modifier::Dot1(*word, letter, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::SH(word) => LetterParts {
                base: Base::Quarter(*word, letter),
                modifiers: Some(Modifier::Dot2(*word, letter, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::R(word) => LetterParts {
                base: Base::Quarter(*word, letter),
                modifiers: Some(Modifier::Dot3(*word, letter, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::V(word) => LetterParts {
                base: Base::Quarter(*word, letter),
                modifiers: Some(Modifier::Line1(
                    *word,
                    letter,
                    DEFAULT_BASE_HEIGHT,
                    Degree(0.0),
                )),
            },
            GallifreyanLetter::W(word) => LetterParts {
                base: Base::Quarter(*word, letter),
                modifiers: Some(Modifier::Line2(*word, letter, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::S(word) => LetterParts {
                base: Base::Quarter(*word, letter),
                modifiers: Some(Modifier::Line3(*word, letter, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::TH(word) => LetterParts {
                base: Base::New(*word, letter),
                modifiers: None,
            },
            GallifreyanLetter::GH(word) => LetterParts {
                base: Base::New(*word, letter),
                modifiers: Some(Modifier::Dot1(*word, letter, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::Y(word) => LetterParts {
                base: Base::New(*word, letter),
                modifiers: Some(Modifier::Dot2(*word, letter, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::Z(word) => LetterParts {
                base: Base::New(*word, letter),
                modifiers: Some(Modifier::Dot3(*word, letter, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::Q(word) => LetterParts {
                base: Base::New(*word, letter),
                modifiers: Some(Modifier::Dot4(*word, letter, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::QU(word) => LetterParts {
                base: Base::New(*word, letter),
                modifiers: Some(Modifier::Line1(
                    *word,
                    letter,
                    DEFAULT_BASE_HEIGHT,
                    Degree(0.0),
                )),
            },
            GallifreyanLetter::X(word) => LetterParts {
                base: Base::New(*word, letter),
                modifiers: Some(Modifier::Line2(*word, letter, DEFAULT_BASE_HEIGHT)),
            },
            GallifreyanLetter::NG(word) => LetterParts {
                base: Base::New(*word, letter),
                modifiers: Some(Modifier::Line3(*word, letter, DEFAULT_BASE_HEIGHT)),
            },
        }
    }

    fn is_vowel(&self) -> bool {
        let _word = Polar::new(0.0, Degree(0.0));
        matches!(
            self,
            GallifreyanLetter::A(_word)
                | GallifreyanLetter::E(_word)
                | GallifreyanLetter::I(_word)
                | GallifreyanLetter::O(_word)
                | GallifreyanLetter::U(_word)
        )
    }
}
