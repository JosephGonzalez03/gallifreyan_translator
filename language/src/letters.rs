use crate::math_util::*;
use crate::letter_parts::*;
use std::mem::discriminant;

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
    pub fn letter(&self) -> Modifier {
        match self {
            GallifreyanLetter::A(position) => todo!(""),
            GallifreyanLetter::E(position) => todo!(""),
            GallifreyanLetter::I(position) => todo!(""),
            GallifreyanLetter::O(position) => todo!(""),
            GallifreyanLetter::U(position) => todo!(""),
            GallifreyanLetter::B(position) => todo!(""),
            GallifreyanLetter::CH(position) => Modifier::Dot2(Base::Crescent(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::D(position) => Modifier::Dot3(Base::Crescent(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::G(position) => Modifier::Line1(
                Base::Crescent(*position, *position / Polar::new(3.0, Degree(180.0))),
                Degree(0.0),
            ),
            GallifreyanLetter::H(position) => Modifier::Line2(Base::Crescent(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::F(position) => Modifier::Line3(Base::Crescent(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::J(position) => todo!(""),
            GallifreyanLetter::PH(position) => Modifier::Dot1(Base::Full(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::K(position) => Modifier::Dot2(Base::Full(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::L(position) => Modifier::Dot3(Base::Full(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::C(position) => Modifier::Dot4(Base::Full(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::N(position) => Modifier::Line1(
                Base::Full(*position, *position / Polar::new(3.0, Degree(180.0))),
                Degree(0.0),
            ),
            GallifreyanLetter::P(position) => Modifier::Line2(Base::Full(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::M(position) => Modifier::Line3(Base::Full(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::T(position) => todo!(""),
            GallifreyanLetter::WH(position) => Modifier::Dot1(Base::Quarter(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::SH(position) => Modifier::Dot2(Base::Quarter(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::R(position) => Modifier::Dot3(Base::Quarter(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::V(position) => Modifier::Line1(
                Base::Quarter(*position, *position / Polar::new(3.0, Degree(180.0))),
                Degree(0.0),
            ),
            GallifreyanLetter::W(position) => Modifier::Line2(Base::Quarter(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::S(position) => Modifier::Line3(Base::Quarter(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::TH(position) => todo!(""),
            GallifreyanLetter::GH(position) => Modifier::Dot1(Base::New(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::Y(position) => Modifier::Dot2(Base::New(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::Z(position) => Modifier::Dot3(Base::New(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::Q(position) => Modifier::Dot4(Base::New(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::QU(position) => Modifier::Line1(
                Base::New(*position, *position / Polar::new(3.0, Degree(180.0))),
                Degree(0.0),
            ),
            GallifreyanLetter::X(position) => Modifier::Line2(Base::New(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
            GallifreyanLetter::NG(position) => Modifier::Line3(Base::New(
                *position,
                *position / Polar::new(3.0, Degree(180.0)),
            )),
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
