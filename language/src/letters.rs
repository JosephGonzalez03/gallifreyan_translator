use crate::math_util::*;
use std::mem::discriminant;

pub enum Base {
    Vowel(Polar, Polar),
    Crescent(Polar, Polar),
    Full(Polar, Polar),
    Quarter(Polar, Polar),
    New(Polar, Polar),
}

impl Base {
    fn position(&self) -> &Polar {
        match self {
            Base::Vowel(position, _) => position,
            Base::Crescent(position, _) => position,
            Base::Full(position, _) => position,
            Base::Quarter(position, _) => position,
            Base::New(position, _) => position,
        }
    }

    pub fn to_points(&self) -> Vec<(f32, f32)> {
        match self {
            Base::Vowel(position, vowel_position) => arc3_d(
                &position,
                &Polar::new(position.radius() / 3.0, vowel_position.angle()),
                vowel_position.radius(),
                (Degree(0.0), Degree(360.0)),
            ),
            Base::Crescent(letter, base) => arc3_d(
                &letter,
                &(base * &Polar::new(0.9, Degree(180.0))),
                base.radius(),
                (
                    letter.angle() + Degree(30.0),
                    letter.angle() + Degree(330.0),
                ),
            ),
            Base::Full(letter, base) => arc3_d(
                &letter,
                &(base * &Polar::new(1.2, Degree(180.0))),
                base.radius(),
                (Degree(0.0), Degree(360.0)),
            ),
            Base::Quarter(letter, base) => arc3_d(
                &letter,
                &Polar::new(0.0, Degree(180.0)),
                base.radius(),
                (
                    letter.angle() + Degree(95.0),
                    letter.angle() + Degree(265.0),
                ),
            ),
            Base::New(letter, base) => arc3_d(
                &letter,
                &Polar::new(0.0, Degree(0.0)),
                base.radius(),
                (letter.angle() + Degree(0.0), letter.angle() + Degree(360.0)),
            ),
        }
    }

    fn starting_angle(&self) -> Option<Degree> {
        match self {
            Base::Crescent(position, _) => Some(
                law_of_sines_angle(
                    &position.radius(),
                    &(&position.radius() / 3.0),
                    Degree(30.0),
                ) + position.angle(),
            ),
            Base::Quarter(position, _) => Some(
                law_of_sines_angle(
                    &position.radius(),
                    &(&position.radius() / 3.0),
                    Degree(90.0),
                ) + position.angle(),
            ),
            _ => None,
        }
    }

    fn ending_angle(&self) -> Option<Degree> {
        match self {
            Base::Crescent(position, _) => Some(
                law_of_sines_angle(
                    &position.radius(),
                    &(&position.radius() / 3.0),
                    Degree(-30.0),
                ) + position.angle(),
            ),
            Base::Quarter(position, _) => Some(
                law_of_sines_angle(
                    &position.radius(),
                    &(&position.radius() / 3.0),
                    Degree(-90.0),
                ) + position.angle(),
            ),
            _ => None,
        }
    }
}

pub enum Modifier {
    Blank(Polar, Polar),
    Dot1(Polar, Polar),
    Dot2(Polar, Polar),
    Dot3(Polar, Polar),
    Dot4(Polar, Polar),
    Line1(Polar, Polar, Degree),
    Line2(Polar, Polar),
    Line3(Polar, Polar),
}

impl Modifier {
    pub fn to_points(&self) -> Vec<Vec<(f32, f32)>> {
        match self {
            Modifier::Dot1(position) => {
                vec![dot(&position, position / &Polar::new(3.0, Degree(180.0)))]
            }
            Modifier::Dot2(position) => vec![
                dot(&position, position / &Polar::new(3.0, Degree(135.0))),
                dot(&position, position / &Polar::new(3.0, Degree(225.0))),
            ],
            Modifier::Dot3(position) => vec![
                dot(&position, position / &Polar::new(3.0, Degree(135.0))),
                dot(&position, position / &Polar::new(3.0, Degree(180.0))),
                dot(&position, position / &Polar::new(3.0, Degree(225.0))),
            ],
            Modifier::Dot4(position) => vec![
                dot(&position, position / &Polar::new(3.0, Degree(150.0))),
                dot(&position, position / &Polar::new(3.0, Degree(165.0))),
                dot(&position, position / &Polar::new(3.0, Degree(195.0))),
                dot(&position, position / &Polar::new(3.0, Degree(210.0))),
            ],
            Modifier::Line1(position, orientation) => vec![normal_line(
                &position,
                Polar::new(position.radius(), &position.angle() + &orientation),
            )],
            Modifier::Line2(position) => vec![
                normal_line(&position, position / &Polar::new(3.0, Degree(135.0))),
                normal_line(&position, position / &Polar::new(3.0, Degree(225.0))),
            ],
            Modifier::Line3(position) => vec![
                normal_line(&position, position / &Polar::new(3.0, Degree(135.0))),
                normal_line(&position, position / &Polar::new(3.0, Degree(180.0))),
                normal_line(&position, position / &Polar::new(3.0, Degree(225.0))),
            ],
        }
    }
}

pub struct Letter {
    base: Base,
    modifiers: Option<Modifier>,
}

impl Letter {
    pub fn base(&self) -> &Base {
        &self.base
    }

    pub fn modifier(&self) -> Option<&Modifier> {
        self.modifiers.as_ref()
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
    pub fn letter(&self) -> Letter {
        match self {
            GallifreyanLetter::A(position) => todo!(""),
            GallifreyanLetter::E(position) => todo!(""),
            GallifreyanLetter::I(position) => todo!(""),
            GallifreyanLetter::O(position) => todo!(""),
            GallifreyanLetter::U(position) => todo!(""),
            GallifreyanLetter::B(position) => todo!(""),
            GallifreyanLetter::CH(position) => todo!(""),
            GallifreyanLetter::D(position) => todo!(""),
            GallifreyanLetter::G(position) => todo!(""),
            GallifreyanLetter::H(position) => todo!(""),
            GallifreyanLetter::F(position) => todo!(""),
            GallifreyanLetter::J(position) => todo!(""),
            GallifreyanLetter::PH(position) => todo!(""),
            GallifreyanLetter::K(position) => todo!(""),
            GallifreyanLetter::L(position) => todo!(""),
            GallifreyanLetter::C(position) => todo!(""),
            GallifreyanLetter::N(position) => todo!(""),
            GallifreyanLetter::P(position) => todo!(""),
            GallifreyanLetter::M(position) => todo!(""),
            GallifreyanLetter::T(position) => todo!(""),
            GallifreyanLetter::WH(position) => todo!(""),
            GallifreyanLetter::SH(position) => todo!(""),
            GallifreyanLetter::R(position) => todo!(""),
            GallifreyanLetter::V(position) => todo!(""),
            GallifreyanLetter::W(position) => todo!(""),
            GallifreyanLetter::S(position) => todo!(""),
            GallifreyanLetter::TH(position) => todo!(""),
            GallifreyanLetter::GH(position) => todo!(""),
            GallifreyanLetter::Y(position) => todo!(""),
            GallifreyanLetter::Z(position) => todo!(""),
            GallifreyanLetter::Q(position) => todo!(""),
            GallifreyanLetter::QU(position) => todo!(""),
            GallifreyanLetter::X(position) => todo!(""),
            GallifreyanLetter::NG(position) => todo!(""),
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
}
