use core::fmt;
use std::f64::consts::PI;
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
    pub fn parts(&self) -> Vec<Part> {
        match self {
            GallifreyanLetter::A => vec![Part::Moon(PI)],
            GallifreyanLetter::E => vec![Part::Core],
            GallifreyanLetter::I => vec![Part::Core, Part::VowelLine1(0.0)],
            GallifreyanLetter::O => vec![Part::Moon(0.0)],
            GallifreyanLetter::U => vec![Part::Core, Part::VowelLine1(PI)],
            GallifreyanLetter::B => vec![Part::Crescent],
            GallifreyanLetter::CH => vec![Part::Crescent, Part::Dot2],
            GallifreyanLetter::D => vec![Part::Crescent, Part::Dot3],
            GallifreyanLetter::G => vec![Part::Crescent, Part::Line1],
            GallifreyanLetter::H => vec![Part::Crescent, Part::Line2],
            GallifreyanLetter::F => vec![Part::Crescent, Part::Line3],
            GallifreyanLetter::J => vec![Part::Full],
            GallifreyanLetter::PH => vec![Part::Full, Part::Dot1],
            GallifreyanLetter::K => vec![Part::Full, Part::Dot2],
            GallifreyanLetter::L => vec![Part::Full, Part::Dot3],
            GallifreyanLetter::C => vec![Part::Full, Part::Dot4],
            GallifreyanLetter::N => vec![Part::Full, Part::Line1],
            GallifreyanLetter::P => vec![Part::Full, Part::Line2],
            GallifreyanLetter::M => vec![Part::Full, Part::Line3],
            GallifreyanLetter::T => vec![Part::Quarter],
            GallifreyanLetter::WH => vec![Part::Quarter, Part::Dot1],
            GallifreyanLetter::SH => vec![Part::Quarter, Part::Dot2],
            GallifreyanLetter::R => vec![Part::Quarter, Part::Dot3],
            GallifreyanLetter::V => vec![Part::Quarter, Part::Line1],
            GallifreyanLetter::W => vec![Part::Quarter, Part::Line2],
            GallifreyanLetter::S => vec![Part::Quarter, Part::Line3],
            GallifreyanLetter::TH => vec![Part::New],
            GallifreyanLetter::GH => vec![Part::New, Part::Dot1],
            GallifreyanLetter::Y => vec![Part::New, Part::Dot2],
            GallifreyanLetter::Z => vec![Part::New, Part::Dot3],
            GallifreyanLetter::Q => vec![Part::New, Part::Dot4],
            GallifreyanLetter::QU => vec![Part::New, Part::Line1],
            GallifreyanLetter::X => vec![Part::New, Part::Line2],
            GallifreyanLetter::NG => vec![Part::New, Part::Line3],
        }
    }

    pub fn is_vowel(&self) -> bool {
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
            Part::Edge(_, _) => write!(f, "Edge"),
        }
    }
}

impl Part {
    pub fn is_base(&self) -> bool {
        matches!(
            self,
            Part::Crescent | Part::Full | Part::Core | Part::Quarter | Part::New | Part::Moon(_)
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
