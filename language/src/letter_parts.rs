use crate::math_util::*;

const CRESCENT_HEIGHT: f32 = 0.9;
const FULL_HEIGHT: f32 = 1.2;
const DEFAULT_BASE_HEIGHT: f32 = 0.0;
const DOT_OFFSET_HEIGHT: f32 = 0.2;

fn draw_dots(letter: &Polar, base: &Polar, base_height: f32, angles: Vec<Degree>) -> Vec<Drawing> {
    angles
        .into_iter()
        .map(|angle| {
            dot(
                letter,
                &(base * &Polar::new(base_height + DOT_OFFSET_HEIGHT, Degree(0.0))),
                &(base * &Polar::new(1.0, angle)),
            )
        })
        .collect()
}

fn draw_normal_lines(
    letter: &Polar,
    base: &Polar,
    base_height: f32,
    angles: Vec<Degree>,
) -> Vec<Drawing> {
    angles
        .into_iter()
        .map(|angle| {
            normal_line(
                letter,
                &(base * &Polar::new(base_height + DOT_OFFSET_HEIGHT, Degree(0.0))),
                &(base * &Polar::new(1.0, angle)),
            )
        })
        .collect()
}

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

    pub fn to_drawing(&self) -> Drawing {
        match self {
            Base::Vowel(position, vowel_position) => arc3_d(
                &position,
                &Polar::new(position.radius() / 3.0, vowel_position.angle()),
                vowel_position.radius(),
                (Degree(0.0), Degree(360.0)),
            ),
            Base::Crescent(letter, base) => arc3_d(
                &letter,
                &(base * &Polar::new(CRESCENT_HEIGHT, Degree(0.0))),
                base.radius(),
                (
                    letter.angle() + Degree(30.0),
                    letter.angle() + Degree(330.0),
                ),
            ),
            Base::Full(letter, base) => arc3_d(
                &letter,
                &(base * &Polar::new(FULL_HEIGHT, Degree(0.0))),
                base.radius(),
                (Degree(0.0), Degree(360.0)),
            ),
            Base::Quarter(letter, base) => arc3_d(
                &letter,
                &Polar::new(DEFAULT_BASE_HEIGHT, Degree(180.0)),
                base.radius(),
                (
                    letter.angle() + Degree(95.0),
                    letter.angle() + Degree(265.0),
                ),
            ),
            Base::New(letter, base) => arc3_d(
                &letter,
                &Polar::new(DEFAULT_BASE_HEIGHT, Degree(0.0)),
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
    Blank(Base),
    Dot1(Base),
    Dot2(Base),
    Dot3(Base),
    Dot4(Base),
    Line1(Base, Degree),
    Line2(Base),
    Line3(Base),
}

impl Modifier {
    pub fn base(&self) -> &Base {
        match self {
            Modifier::Blank(base_type) => base_type,
            Modifier::Dot1(base_type) => base_type,
            Modifier::Dot2(base_type) => base_type,
            Modifier::Dot3(base_type) => base_type,
            Modifier::Dot4(base_type) => base_type,
            Modifier::Line1(base_type, _) => base_type,
            Modifier::Line2(base_type) => base_type,
            Modifier::Line3(base_type) => base_type,
        }
    }

    pub fn to_drawings(&self) -> Vec<Drawing> {
        match self {
            Modifier::Dot1(base_type) => match base_type {
                Base::Crescent(letter, base) => {
                    draw_dots(letter, base, CRESCENT_HEIGHT, vec![Degree(0.0)])
                }
                Base::Full(letter, base) => draw_dots(letter, base, FULL_HEIGHT, vec![Degree(0.0)]),
                Base::Quarter(letter, base) => {
                    draw_dots(letter, base, DEFAULT_BASE_HEIGHT, vec![Degree(0.0)])
                }
                Base::New(letter, base) => {
                    draw_dots(letter, base, DEFAULT_BASE_HEIGHT, vec![Degree(0.0)])
                }
                _ => todo!("Vowel"),
            },
            Modifier::Dot2(base_type) => match base_type {
                Base::Crescent(letter, base) => draw_dots(
                    letter,
                    base,
                    CRESCENT_HEIGHT,
                    vec![Degree(-45.0), Degree(45.0)],
                ),
                Base::Full(letter, base) => {
                    draw_dots(letter, base, FULL_HEIGHT, vec![Degree(-45.0), Degree(45.0)])
                }
                Base::Quarter(letter, base) => draw_dots(
                    letter,
                    base,
                    DEFAULT_BASE_HEIGHT,
                    vec![Degree(-45.0), Degree(45.0)],
                ),
                Base::New(letter, base) => draw_dots(
                    letter,
                    base,
                    DEFAULT_BASE_HEIGHT,
                    vec![Degree(-45.0), Degree(45.0)],
                ),
                _ => todo!("Vowel"),
            },
            Modifier::Dot3(base_type) => match base_type {
                Base::Crescent(letter, base) => draw_dots(
                    letter,
                    base,
                    CRESCENT_HEIGHT,
                    vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
                ),
                Base::Full(letter, base) => draw_dots(
                    letter,
                    base,
                    FULL_HEIGHT,
                    vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
                ),
                Base::Quarter(letter, base) => draw_dots(
                    letter,
                    base,
                    DEFAULT_BASE_HEIGHT,
                    vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
                ),
                Base::New(letter, base) => draw_dots(
                    letter,
                    base,
                    DEFAULT_BASE_HEIGHT,
                    vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
                ),
                _ => todo!("Vowel"),
            },
            Modifier::Dot4(base_type) => match base_type {
                Base::Crescent(letter, base) => draw_dots(
                    letter,
                    base,
                    CRESCENT_HEIGHT,
                    vec![Degree(-30.0), Degree(-15.0), Degree(15.0), Degree(30.0)],
                ),
                Base::Full(letter, base) => draw_dots(
                    letter,
                    base,
                    FULL_HEIGHT,
                    vec![Degree(-30.0), Degree(-15.0), Degree(15.0), Degree(30.0)],
                ),
                Base::Quarter(letter, base) => draw_dots(
                    letter,
                    base,
                    DEFAULT_BASE_HEIGHT,
                    vec![Degree(-30.0), Degree(-15.0), Degree(15.0), Degree(30.0)],
                ),
                Base::New(letter, base) => draw_dots(
                    letter,
                    base,
                    DEFAULT_BASE_HEIGHT,
                    vec![Degree(-30.0), Degree(-15.0), Degree(15.0), Degree(30.0)],
                ),
                _ => todo!("Vowel"),
            },
            Modifier::Line1(base_type, angle) => match base_type {
                Base::Crescent(letter, base) => {
                    draw_normal_lines(letter, base, CRESCENT_HEIGHT, vec![*angle])
                }
                Base::Full(letter, base) => {
                    draw_normal_lines(letter, base, FULL_HEIGHT, vec![*angle])
                }
                Base::Quarter(letter, base) => {
                    draw_normal_lines(letter, base, DEFAULT_BASE_HEIGHT, vec![*angle])
                }
                Base::New(letter, base) => {
                    draw_normal_lines(letter, base, DEFAULT_BASE_HEIGHT, vec![*angle])
                }
                _ => todo!("Vowel"),
            },
            Modifier::Line2(base_type) => match base_type {
                Base::Crescent(letter, base) => draw_normal_lines(
                    letter,
                    base,
                    CRESCENT_HEIGHT,
                    vec![Degree(-45.0), Degree(45.0)],
                ),
                Base::Full(letter, base) => {
                    draw_normal_lines(letter, base, FULL_HEIGHT, vec![Degree(-45.0), Degree(45.0)])
                }
                Base::Quarter(letter, base) => draw_normal_lines(
                    letter,
                    base,
                    DEFAULT_BASE_HEIGHT,
                    vec![Degree(-45.0), Degree(45.0)],
                ),
                Base::New(letter, base) => draw_normal_lines(
                    letter,
                    base,
                    DEFAULT_BASE_HEIGHT,
                    vec![Degree(-45.0), Degree(45.0)],
                ),
                _ => todo!("Vowel"),
            },
            Modifier::Line3(base_type) => match base_type {
                Base::Crescent(letter, base) => draw_normal_lines(
                    letter,
                    base,
                    CRESCENT_HEIGHT,
                    vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
                ),
                Base::Full(letter, base) => draw_normal_lines(
                    letter,
                    base,
                    FULL_HEIGHT,
                    vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
                ),
                Base::Quarter(letter, base) => draw_normal_lines(
                    letter,
                    base,
                    DEFAULT_BASE_HEIGHT,
                    vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
                ),
                Base::New(letter, base) => draw_normal_lines(
                    letter,
                    base,
                    DEFAULT_BASE_HEIGHT,
                    vec![Degree(-45.0), Degree(0.0), Degree(45.0)],
                ),
                _ => todo!("Vowel"),
            },
            _ => todo!(""),
        }
    }
}
