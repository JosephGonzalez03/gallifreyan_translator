use crate::math_util::*;

enum LetterPart {
    Vowel(f32, f32, Degree),
    Crescent(f32),
    Full(f32),
    Quarter(f32),
    New(f32),
    Dot1(f32),
    Dot2(f32),
    Dot3(f32),
    Dot4(f32),
    Line1(f32, Degree),
    Line2(f32),
    Line3(f32),
}

impl LetterPart {
    fn to_points(self, position: &PolarCoordinate) -> Vec<Vec<(f32, f32)>> {
        match self {
            LetterPart::Vowel(letter_radius, vowel_radius, vowel_angle) => vec![arc3_d(
                &position,
                &PolarCoordinate::new(letter_radius, vowel_angle),
                vowel_radius,
                (Degree::new(0.0), Degree::new(360.0)),
            )],
            LetterPart::Crescent(letter_radius) => vec![arc3_d(
                &position,
                &PolarCoordinate::new(0.90 * letter_radius, position.angle() + Degree::new(180.0)),
                letter_radius,
                (
                    position.angle() + Degree::new(30.0),
                    position.angle() + Degree::new(330.0),
                ),
            )],
            LetterPart::Full(letter_radius) => vec![arc3_d(
                &position,
                &PolarCoordinate::new(1.2 * letter_radius, position.angle() + Degree::new(180.0)),
                letter_radius,
                (Degree::new(0.0), Degree::new(360.0)),
            )],
            LetterPart::Quarter(letter_radius) => vec![arc3_d(
                &position,
                &PolarCoordinate::new(0.0, position.angle() + Degree::new(180.0)),
                letter_radius,
                (
                    position.angle() + Degree::new(95.0),
                    position.angle() + Degree::new(265.0),
                ),
            )],
            LetterPart::New(letter_radius) => vec![arc3_d(
                &position,
                &PolarCoordinate::new(0.0, position.angle() + Degree::new(180.0)),
                letter_radius,
                (
                    position.angle() + Degree::new(0.0),
                    position.angle() + Degree::new(360.0),
                ),
            )],
            LetterPart::Dot1(letter_radius) => vec![dot(
                &position,
                PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(180.0)),
            )],
            LetterPart::Dot2(letter_radius) => vec![
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(135.0)),
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(225.0)),
                ),
            ],
            LetterPart::Dot3(letter_radius) => vec![
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(135.0)),
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(180.0)),
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(225.0)),
                ),
            ],
            LetterPart::Dot4(letter_radius) => vec![
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(150.0)),
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(165.0)),
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(195.0)),
                ),
                dot(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(210.0)),
                ),
            ],
            LetterPart::Line1(letter_radius, orientation) => vec![normal_line(
                &position,
                PolarCoordinate::new(letter_radius, &position.angle() + &orientation),
            )],
            LetterPart::Line2(letter_radius) => vec![
                normal_line(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(135.0)),
                ),
                normal_line(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(225.0)),
                ),
            ],
            LetterPart::Line3(letter_radius) => vec![
                normal_line(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(135.0)),
                ),
                normal_line(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(180.0)),
                ),
                normal_line(
                    &position,
                    PolarCoordinate::new(letter_radius, &position.angle() + &Degree::new(225.0)),
                ),
            ],
        }
    }
}

enum GallifreyanLetter {
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

impl GallifreyanLetter {
    fn to_letter_parts(&self, position: PolarCoordinate) -> Vec<LetterPart> {
        let letter_radius = position.radius() / 3.0;
        match self {
            GallifreyanLetter::A => vec![LetterPart::Vowel(
                letter_radius,
                letter_radius / 3.0,
                Degree::new(0.0),
            )],
            GallifreyanLetter::E => vec![LetterPart::Vowel(
                0.0,
                letter_radius / 3.0,
                Degree::new(0.0),
            )],
            GallifreyanLetter::I => vec![
                LetterPart::Vowel(0.0, letter_radius / 3.0, Degree::new(0.0)),
                LetterPart::Line1(letter_radius, Degree::new(180.0)),
            ],
            GallifreyanLetter::O => vec![LetterPart::Vowel(
                letter_radius,
                letter_radius / 3.0,
                Degree::new(180.0),
            )],
            GallifreyanLetter::U => vec![
                LetterPart::Vowel(0.0, letter_radius / 3.0, Degree::new(0.0)),
                LetterPart::Line1(letter_radius, Degree::new(0.0)),
            ],
            GallifreyanLetter::B => vec![LetterPart::Crescent(letter_radius)],
            GallifreyanLetter::CH => vec![
                LetterPart::Crescent(letter_radius),
                LetterPart::Dot2(letter_radius),
            ],
            GallifreyanLetter::D => vec![
                LetterPart::Crescent(letter_radius),
                LetterPart::Dot3(letter_radius),
            ],
            GallifreyanLetter::G => vec![
                LetterPart::Crescent(letter_radius),
                LetterPart::Line1(letter_radius, Degree::new(180.0)),
            ],
            GallifreyanLetter::H => vec![
                LetterPart::Crescent(letter_radius),
                LetterPart::Line2(letter_radius),
            ],
            GallifreyanLetter::F => vec![
                LetterPart::Crescent(letter_radius),
                LetterPart::Line3(letter_radius),
            ],
            GallifreyanLetter::J => vec![LetterPart::Full(letter_radius)],
            GallifreyanLetter::PH => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Dot1(letter_radius),
            ],
            GallifreyanLetter::K => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Dot2(letter_radius),
            ],
            GallifreyanLetter::L => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Dot3(letter_radius),
            ],
            GallifreyanLetter::C => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Dot4(letter_radius),
            ],
            GallifreyanLetter::N => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Line1(letter_radius, Degree::new(180.0)),
            ],
            GallifreyanLetter::P => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Line2(letter_radius),
            ],
            GallifreyanLetter::M => vec![
                LetterPart::Full(letter_radius),
                LetterPart::Line3(letter_radius),
            ],
            GallifreyanLetter::T => vec![LetterPart::Quarter(letter_radius)],
            GallifreyanLetter::WH => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Dot1(letter_radius),
            ],
            GallifreyanLetter::SH => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Dot2(letter_radius),
            ],
            GallifreyanLetter::R => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Dot3(letter_radius),
            ],
            GallifreyanLetter::V => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Line1(letter_radius, Degree::new(180.0)),
            ],
            GallifreyanLetter::W => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Line2(letter_radius),
            ],
            GallifreyanLetter::S => vec![
                LetterPart::Quarter(letter_radius),
                LetterPart::Line3(letter_radius),
            ],
            GallifreyanLetter::TH => vec![LetterPart::New(letter_radius)],
            GallifreyanLetter::GH => vec![
                LetterPart::New(letter_radius),
                LetterPart::Dot1(letter_radius),
            ],
            GallifreyanLetter::Y => vec![
                LetterPart::New(letter_radius),
                LetterPart::Dot2(letter_radius),
            ],
            GallifreyanLetter::Z => vec![
                LetterPart::New(letter_radius),
                LetterPart::Dot3(letter_radius),
            ],
            GallifreyanLetter::Q => vec![
                LetterPart::New(letter_radius),
                LetterPart::Dot4(letter_radius),
            ],
            GallifreyanLetter::QU => vec![
                LetterPart::New(letter_radius),
                LetterPart::Line1(letter_radius, Degree::new(180.0)),
            ],
            GallifreyanLetter::X => vec![
                LetterPart::New(letter_radius),
                LetterPart::Line2(letter_radius),
            ],
            GallifreyanLetter::NG => vec![
                LetterPart::New(letter_radius),
                LetterPart::Line3(letter_radius),
            ],
        }
    }
}
