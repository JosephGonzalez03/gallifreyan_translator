use crate::math_util::*;

/// A enumeration for the shapes that compose a GallifreyanLetter.
pub enum LetterPart {
    Vowel(Polar, Polar),
    Crescent(Polar),
    Full(Polar),
    Quarter(Polar),
    New(Polar),
    Dot1(Polar),
    Dot2(Polar),
    Dot3(Polar),
    Dot4(Polar),
    Line1(Polar, Degree),
    Line2(Polar),
    Line3(Polar),
}

impl LetterPart {
    pub fn to_points(&self) -> Vec<Vec<(f32, f32)>> {
        match self {
            LetterPart::Vowel(position, vowel_position) => vec![arc3_d(
                &position,
                &Polar::new(position.radius() / 3.0, vowel_position.angle()),
                vowel_position.radius(),
                (Degree::new(0.0), Degree::new(360.0)),
            )],
            LetterPart::Crescent(position) => vec![arc3_d(
                &position,
                &Polar::new(
                    0.90 * position.radius() / 3.0,
                    position.angle() + Degree::new(180.0),
                ),
                position.radius() / 3.0,
                (
                    position.angle() + Degree::new(30.0),
                    position.angle() + Degree::new(330.0),
                ),
            )],
            LetterPart::Full(position) => vec![arc3_d(
                &position,
                &Polar::new(
                    1.2 * position.radius() / 3.0,
                    position.angle() + Degree::new(180.0),
                ),
                position.radius() / 3.0,
                (Degree::new(0.0), Degree::new(360.0)),
            )],
            LetterPart::Quarter(position) => vec![arc3_d(
                &position,
                &Polar::new(0.0, position.angle() + Degree::new(180.0)),
                position.radius() / 3.0,
                (
                    position.angle() + Degree::new(95.0),
                    position.angle() + Degree::new(265.0),
                ),
            )],
            LetterPart::New(position) => vec![arc3_d(
                &position,
                &Polar::new(0.0, position.angle() + Degree::new(180.0)),
                position.radius() / 3.0,
                (
                    position.angle() + Degree::new(0.0),
                    position.angle() + Degree::new(360.0),
                ),
            )],
            LetterPart::Dot1(position) => vec![dot(
                &position,
                position / &Polar::new(3.0, Degree::new(180.0)),
            )],
            LetterPart::Dot2(position) => vec![
                dot(&position, position / &Polar::new(3.0, Degree::new(135.0))),
                dot(&position, position / &Polar::new(3.0, Degree::new(225.0))),
            ],
            LetterPart::Dot3(position) => vec![
                dot(&position, position / &Polar::new(3.0, Degree::new(135.0))),
                dot(&position, position / &Polar::new(3.0, Degree::new(180.0))),
                dot(&position, position / &Polar::new(3.0, Degree::new(225.0))),
            ],
            LetterPart::Dot4(position) => vec![
                dot(&position, position / &Polar::new(3.0, Degree::new(150.0))),
                dot(&position, position / &Polar::new(3.0, Degree::new(165.0))),
                dot(&position, position / &Polar::new(3.0, Degree::new(195.0))),
                dot(&position, position / &Polar::new(3.0, Degree::new(210.0))),
            ],
            LetterPart::Line1(position, orientation) => vec![normal_line(
                &position,
                Polar::new(position.radius() / 3.0, &position.angle() + &orientation),
            )],
            LetterPart::Line2(position) => vec![
                normal_line(&position, position / &Polar::new(3.0, Degree::new(135.0))),
                normal_line(&position, position / &Polar::new(3.0, Degree::new(225.0))),
            ],
            LetterPart::Line3(position) => vec![
                normal_line(&position, position / &Polar::new(3.0, Degree::new(135.0))),
                normal_line(&position, position / &Polar::new(3.0, Degree::new(180.0))),
                normal_line(&position, position / &Polar::new(3.0, Degree::new(225.0))),
            ],
        }
    }

    fn starting_angle(&self) -> Option<Degree> {
        match self {
            LetterPart::Quarter(position) => Some(
                law_of_sines_angle(
                    &position.radius(),
                    &(&position.radius() / 3.0),
                    Degree::new(90.0),
                ) + position.angle(),
            ),
            LetterPart::Crescent(position) => Some(
                law_of_sines_angle(
                    &position.radius(),
                    &(&position.radius() / 3.0),
                    Degree::new(30.0),
                ) + position.angle(),
            ),
            _ => None,
        }
    }

    fn ending_angle(&self) -> Option<Degree> {
        match self {
            LetterPart::Quarter(position) => Some(
                law_of_sines_angle(
                    &position.radius(),
                    &(&position.radius() / 3.0),
                    Degree::new(-90.0),
                ) + position.angle(),
            ),
            LetterPart::Crescent(position) => Some(
                law_of_sines_angle(
                    &position.radius(),
                    &(&position.radius() / 3.0),
                    Degree::new(-30.0),
                ) + position.angle(),
            ),
            _ => None,
        }
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

impl GallifreyanLetter {
    /// Decompose a GallifreyanLetter into its parts.
    ///
    ///```
    /// let parts: Vec<LetterPart> = GallifreyanLetter::D
    ///     .to_letter_parts(Polar::new(1, Degree::new(270.0));
    ///```
    pub fn to_letter_parts(&self) -> Vec<LetterPart> {
        match &self {
            GallifreyanLetter::A(position) => vec![LetterPart::Vowel(
                position / &Polar::new(3.0, Degree::new(0.0)),
                position / &Polar::new(6.0, position.angle()),
            )],
            GallifreyanLetter::E(position) => vec![LetterPart::Vowel(
                position * &Polar::new(0.0, Degree::new(0.0)),
                position / &Polar::new(6.0, position.angle()),
            )],
            GallifreyanLetter::I(position) => vec![
                LetterPart::Vowel(
                    position * &Polar::new(0.0, Degree::new(0.0)),
                    position / &Polar::new(6.0, position.angle()),
                ),
                LetterPart::Line1(
                    position / &Polar::new(3.0, Degree::new(0.0)),
                    Degree::new(180.0),
                ),
            ],
            GallifreyanLetter::O(position) => vec![LetterPart::Vowel(
                position / &Polar::new(3.0, Degree::new(0.0)),
                position / &Polar::new(6.0, Degree::new(180.0)),
            )],
            GallifreyanLetter::U(position) => vec![
                LetterPart::Vowel(
                    position * &Polar::new(0.0, Degree::new(0.0)),
                    position / &Polar::new(6.0, position.angle()),
                ),
                LetterPart::Line1(
                    position / &Polar::new(3.0, Degree::new(0.0)),
                    Degree::new(0.0),
                ),
            ],
            GallifreyanLetter::B(position) => vec![LetterPart::Crescent(
                position / &Polar::new(3.0, Degree::new(0.0)),
            )],
            GallifreyanLetter::CH(position) => vec![
                LetterPart::Crescent(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot2(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::D(position) => vec![
                LetterPart::Crescent(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot3(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::G(position) => vec![
                LetterPart::Crescent(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line1(
                    position / &Polar::new(3.0, Degree::new(0.0)),
                    Degree::new(180.0),
                ),
            ],
            GallifreyanLetter::H(position) => vec![
                LetterPart::Crescent(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line2(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::F(position) => vec![
                LetterPart::Crescent(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line3(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::J(position) => vec![LetterPart::Full(
                position / &Polar::new(3.0, Degree::new(0.0)),
            )],
            GallifreyanLetter::PH(position) => vec![
                LetterPart::Full(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot1(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::K(position) => vec![
                LetterPart::Full(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot2(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::L(position) => vec![
                LetterPart::Full(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot3(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::C(position) => vec![
                LetterPart::Full(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot4(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::N(position) => vec![
                LetterPart::Full(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line1(
                    position / &Polar::new(3.0, Degree::new(0.0)),
                    Degree::new(180.0),
                ),
            ],
            GallifreyanLetter::P(position) => vec![
                LetterPart::Full(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line2(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::M(position) => vec![
                LetterPart::Full(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line3(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::T(position) => vec![LetterPart::Quarter(
                position / &Polar::new(3.0, Degree::new(0.0)),
            )],
            GallifreyanLetter::WH(position) => vec![
                LetterPart::Quarter(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot1(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::SH(position) => vec![
                LetterPart::Quarter(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot2(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::R(position) => vec![
                LetterPart::Quarter(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot3(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::V(position) => vec![
                LetterPart::Quarter(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line1(
                    position / &Polar::new(3.0, Degree::new(0.0)),
                    Degree::new(180.0),
                ),
            ],
            GallifreyanLetter::W(position) => vec![
                LetterPart::Quarter(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line2(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::S(position) => vec![
                LetterPart::Quarter(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line3(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::TH(position) => vec![LetterPart::New(
                position / &Polar::new(3.0, Degree::new(0.0)),
            )],
            GallifreyanLetter::GH(position) => vec![
                LetterPart::New(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot1(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::Y(position) => vec![
                LetterPart::New(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot2(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::Z(position) => vec![
                LetterPart::New(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot3(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::Q(position) => vec![
                LetterPart::New(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Dot4(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::QU(position) => vec![
                LetterPart::New(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line1(
                    position / &Polar::new(3.0, Degree::new(0.0)),
                    Degree::new(180.0),
                ),
            ],
            GallifreyanLetter::X(position) => vec![
                LetterPart::New(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line2(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
            GallifreyanLetter::NG(position) => vec![
                LetterPart::New(position / &Polar::new(3.0, Degree::new(0.0))),
                LetterPart::Line3(position / &Polar::new(3.0, Degree::new(0.0))),
            ],
        }
    }

    pub fn buffer(
        &self,
        position: &Polar,
        next: Option<GallifreyanLetter>,
    ) -> Option<Vec<(f32, f32)>> {
        let starting_angle = self
            .to_letter_parts()
            .first()
            .expect("All letters have parts.")
            .starting_angle();
        let ending_angle = match next {
            Some(letter) => letter
                .to_letter_parts()
                .first()
                .expect("All letters have parts.")
                .ending_angle(),
            None => self
                .to_letter_parts()
                .first()
                .expect("All letters have parts.")
                .ending_angle(),
        };

        if let Some(start) = starting_angle {
            if let Some(end) = ending_angle {
                Some(arc3_d(
                    position,
                    &Polar::new(0.0, Degree::new(0.0)),
                    0.0,
                    (start, end),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}
