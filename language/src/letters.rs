use core::fmt;
use std::f64::consts::PI;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGallifreyanTokenError;

/// An enumeration for the letters in the Gallifreyan alphabet.
#[derive(Clone, Copy, Debug)]
pub enum GallifreyanToken {
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

impl Display for GallifreyanToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GallifreyanToken::A => write!(f, "Gallifreyan A"),
            GallifreyanToken::E => write!(f, "Gallifreyan E"),
            GallifreyanToken::I => write!(f, "Gallifreyan I"),
            GallifreyanToken::O => write!(f, "Gallifreyan O"),
            GallifreyanToken::U => write!(f, "Gallifreyan U"),
            GallifreyanToken::B => write!(f, "Gallifreyan B"),
            GallifreyanToken::CH => write!(f, "Gallifreyan CH"),
            GallifreyanToken::D => write!(f, "Gallifreyan D"),
            GallifreyanToken::G => write!(f, "Gallifreyan G"),
            GallifreyanToken::H => write!(f, "Gallifreyan H"),
            GallifreyanToken::F => write!(f, "Gallifreyan F"),
            GallifreyanToken::J => write!(f, "Gallifreyan J"),
            GallifreyanToken::PH => write!(f, "Gallifreyan PH"),
            GallifreyanToken::K => write!(f, "Gallifreyan K"),
            GallifreyanToken::L => write!(f, "Gallifreyan L"),
            GallifreyanToken::C => write!(f, "Gallifreyan C"),
            GallifreyanToken::N => write!(f, "Gallifreyan N"),
            GallifreyanToken::P => write!(f, "Gallifreyan P"),
            GallifreyanToken::M => write!(f, "Gallifreyan M"),
            GallifreyanToken::T => write!(f, "Gallifreyan T"),
            GallifreyanToken::WH => write!(f, "Gallifreyan WH"),
            GallifreyanToken::SH => write!(f, "Gallifreyan SH"),
            GallifreyanToken::R => write!(f, "Gallifreyan R"),
            GallifreyanToken::V => write!(f, "Gallifreyan V"),
            GallifreyanToken::W => write!(f, "Gallifreyan W"),
            GallifreyanToken::S => write!(f, "Gallifreyan S"),
            GallifreyanToken::TH => write!(f, "Gallifreyan TH"),
            GallifreyanToken::GH => write!(f, "Gallifreyan GH"),
            GallifreyanToken::Y => write!(f, "Gallifreyan Y"),
            GallifreyanToken::Z => write!(f, "Gallifreyan Z"),
            GallifreyanToken::Q => write!(f, "Gallifreyan Q"),
            GallifreyanToken::QU => write!(f, "Gallifreyan QU"),
            GallifreyanToken::X => write!(f, "Gallifreyan X"),
            GallifreyanToken::NG => write!(f, "Gallifreyan NG"),
        }
    }
}

impl FromStr for GallifreyanToken {
    type Err = ParseGallifreyanTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Result::Ok(GallifreyanToken::A),
            "E" => Result::Ok(GallifreyanToken::E),
            "I" => Result::Ok(GallifreyanToken::I),
            "O" => Result::Ok(GallifreyanToken::O),
            "U" => Result::Ok(GallifreyanToken::U),
            "B" => Result::Ok(GallifreyanToken::B),
            "CH" => Result::Ok(GallifreyanToken::CH),
            "D" => Result::Ok(GallifreyanToken::D),
            "G" => Result::Ok(GallifreyanToken::G),
            "H" => Result::Ok(GallifreyanToken::H),
            "F" => Result::Ok(GallifreyanToken::F),
            "J" => Result::Ok(GallifreyanToken::J),
            "PH" => Result::Ok(GallifreyanToken::PH),
            "K" => Result::Ok(GallifreyanToken::K),
            "L" => Result::Ok(GallifreyanToken::L),
            "C" => Result::Ok(GallifreyanToken::C),
            "N" => Result::Ok(GallifreyanToken::N),
            "P" => Result::Ok(GallifreyanToken::P),
            "M" => Result::Ok(GallifreyanToken::M),
            "T" => Result::Ok(GallifreyanToken::T),
            "WH" => Result::Ok(GallifreyanToken::WH),
            "SH" => Result::Ok(GallifreyanToken::SH),
            "R" => Result::Ok(GallifreyanToken::R),
            "V" => Result::Ok(GallifreyanToken::V),
            "W" => Result::Ok(GallifreyanToken::W),
            "S" => Result::Ok(GallifreyanToken::S),
            "TH" => Result::Ok(GallifreyanToken::TH),
            "GH" => Result::Ok(GallifreyanToken::GH),
            "Y" => Result::Ok(GallifreyanToken::Y),
            "Z" => Result::Ok(GallifreyanToken::Z),
            "Q" => Result::Ok(GallifreyanToken::Q),
            "QU" => Result::Ok(GallifreyanToken::QU),
            "X" => Result::Ok(GallifreyanToken::X),
            "NG" => Result::Ok(GallifreyanToken::NG),
            _ => Result::Err(ParseGallifreyanTokenError),
        }
    }
}

impl GallifreyanToken {
    pub fn parts(&self) -> Vec<Part> {
        match self {
            GallifreyanToken::A => vec![Part::Moon(PI)],
            GallifreyanToken::E => vec![Part::Core],
            GallifreyanToken::I => vec![Part::Core, Part::VowelLine1(0.0)],
            GallifreyanToken::O => vec![Part::Moon(0.0)],
            GallifreyanToken::U => vec![Part::Core, Part::VowelLine1(PI)],
            GallifreyanToken::B => vec![Part::Crescent],
            GallifreyanToken::CH => vec![Part::Crescent, Part::Dot2],
            GallifreyanToken::D => vec![Part::Crescent, Part::Dot3],
            GallifreyanToken::G => vec![Part::Crescent, Part::Line1],
            GallifreyanToken::H => vec![Part::Crescent, Part::Line2],
            GallifreyanToken::F => vec![Part::Crescent, Part::Line3],
            GallifreyanToken::J => vec![Part::Full],
            GallifreyanToken::PH => vec![Part::Full, Part::Dot1],
            GallifreyanToken::K => vec![Part::Full, Part::Dot2],
            GallifreyanToken::L => vec![Part::Full, Part::Dot3],
            GallifreyanToken::C => vec![Part::Full, Part::Dot4],
            GallifreyanToken::N => vec![Part::Full, Part::Line1],
            GallifreyanToken::P => vec![Part::Full, Part::Line2],
            GallifreyanToken::M => vec![Part::Full, Part::Line3],
            GallifreyanToken::T => vec![Part::Quarter],
            GallifreyanToken::WH => vec![Part::Quarter, Part::Dot1],
            GallifreyanToken::SH => vec![Part::Quarter, Part::Dot2],
            GallifreyanToken::R => vec![Part::Quarter, Part::Dot3],
            GallifreyanToken::V => vec![Part::Quarter, Part::Line1],
            GallifreyanToken::W => vec![Part::Quarter, Part::Line2],
            GallifreyanToken::S => vec![Part::Quarter, Part::Line3],
            GallifreyanToken::TH => vec![Part::New],
            GallifreyanToken::GH => vec![Part::New, Part::Dot1],
            GallifreyanToken::Y => vec![Part::New, Part::Dot2],
            GallifreyanToken::Z => vec![Part::New, Part::Dot3],
            GallifreyanToken::Q => vec![Part::New, Part::Dot4],
            GallifreyanToken::QU => vec![Part::New, Part::Line1],
            GallifreyanToken::X => vec![Part::New, Part::Line2],
            GallifreyanToken::NG => vec![Part::New, Part::Line3],
        }
    }

    pub fn is_letter(&self) -> bool {
        matches!(
            self,
            GallifreyanToken::A
                | GallifreyanToken::E
                | GallifreyanToken::I
                | GallifreyanToken::O
                | GallifreyanToken::U
                | GallifreyanToken::B
                | GallifreyanToken::CH
                | GallifreyanToken::D
                | GallifreyanToken::G
                | GallifreyanToken::H
                | GallifreyanToken::F
                | GallifreyanToken::J
                | GallifreyanToken::PH
                | GallifreyanToken::K
                | GallifreyanToken::L
                | GallifreyanToken::C
                | GallifreyanToken::N
                | GallifreyanToken::P
                | GallifreyanToken::M
                | GallifreyanToken::T
                | GallifreyanToken::WH
                | GallifreyanToken::SH
                | GallifreyanToken::R
                | GallifreyanToken::V
                | GallifreyanToken::W
                | GallifreyanToken::S
                | GallifreyanToken::TH
                | GallifreyanToken::GH
                | GallifreyanToken::Y
                | GallifreyanToken::Z
                | GallifreyanToken::Q
                | GallifreyanToken::QU
                | GallifreyanToken::X
                | GallifreyanToken::NG
        )
    }

    pub fn is_vowel(&self) -> bool {
        matches!(
            self,
            GallifreyanToken::A
                | GallifreyanToken::E
                | GallifreyanToken::I
                | GallifreyanToken::O
                | GallifreyanToken::U
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Part {
    Moon(f64),
    Core,
    Crescent,
    Full,
    Quarter,
    New,
    Dot1,
    Dot2,
    Dot3,
    Dot4,
    VowelLine1(f64),
    Line1,
    Line2,
    Line3,
    Divot,
    Edge(f64, f64),
}

impl Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Part::Moon(_) => write!(f, "Moon"),
            Part::Core => write!(f, "Core"),
            Part::Crescent => write!(f, "Crescent"),
            Part::Full => write!(f, "Full"),
            Part::Quarter => write!(f, "Quarter"),
            Part::New => write!(f, "New"),
            Part::Dot1 => write!(f, "Dot1"),
            Part::Dot2 => write!(f, "Dot2"),
            Part::Dot3 => write!(f, "Dot3"),
            Part::Dot4 => write!(f, "Dot4"),
            Part::VowelLine1(_) => write!(f, "VowelLine1"),
            Part::Line1 => write!(f, "Line1"),
            Part::Line2 => write!(f, "Line2"),
            Part::Line3 => write!(f, "Line3"),
            Part::Divot => write!(f, "Divot"),
            Part::Edge(_, _) => write!(f, "Edge"),
        }
    }
}

impl Part {
    pub fn is_base(&self) -> bool {
        matches!(
            self,
            Part::Crescent
                | Part::Full
                | Part::Core
                | Part::Quarter
                | Part::New
                | Part::Moon(_)
                | Part::Divot
        )
    }

    pub fn is_modifier(&self) -> bool {
        matches!(
            self,
            Part::Dot1
                | Part::Dot2
                | Part::Dot3
                | Part::Dot4
                | Part::Line1
                | Part::Line2
                | Part::Line3
                | Part::VowelLine1(_)
        )
    }
}
