use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Degree(pub f32);

impl Degree {
    pub fn new(value: f32) -> Self {
        Degree(value)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl From<f32> for Degree {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl Add for Degree {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for Degree {
    type Output = Degree;

    fn sub(self, rhs: Self) -> Self::Output {
        Degree(self.0 - rhs.0)
    }
}

impl Add for &Degree {
    type Output = Degree;

    fn add(self, rhs: Self) -> Self::Output {
        Degree(self.0 + rhs.0)
    }
}

impl Sub for &Degree {
    type Output = Degree;

    fn sub(self, rhs: Self) -> Self::Output {
        Degree(self.0 - rhs.0)
    }
}

#[derive(Copy, Clone)]
pub struct Polar {
    radius: f32,
    angle: Degree,
}

impl Polar {
    pub fn new(radius: f32, angle: Degree) -> Self {
        Self { radius, angle }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn angle(&self) -> Degree {
        self.angle
    }
}

impl Mul for Polar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Polar::new(self.radius() * rhs.radius(), self.angle() + rhs.angle())
    }
}

impl Div for Polar {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Polar::new(self.radius() / rhs.radius(), self.angle() - rhs.angle())
    }
}

impl Mul for &Polar {
    type Output = Polar;

    fn mul(self, rhs: Self) -> Self::Output {
        Polar::new(self.radius() * rhs.radius(), self.angle() + rhs.angle())
    }
}

impl Div for &Polar {
    type Output = Polar;

    fn div(self, rhs: Self) -> Self::Output {
        Polar::new(self.radius() / rhs.radius(), self.angle() - rhs.angle())
    }
}

#[derive(Clone)]
pub struct Drawing(Vec<(f32, f32)>);

impl Drawing {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, value: (f32, f32)) {
        self.0.push(value);
    }

    pub fn to_points(&self) -> &Vec<(f32, f32)> {
        &self.0
    }
}

impl FromIterator<(f32, f32)> for Drawing {
    fn from_iter<T: IntoIterator<Item = (f32, f32)>>(iter: T) -> Self {
        let mut drawing = Drawing::new();

        for i in iter {
            drawing.push(i)
        }

        drawing
    }
}

pub fn draw_arc(radius: f32, range: (Degree, Degree)) -> Drawing {
    let start_range = range.0 .0.round() as i32;
    let end_range = range.1 .0.round() as i32;

    match start_range < end_range {
        true => (start_range..=end_range),
        false => (start_range..=end_range + 360),
    }
    .map(|angle| {
        (
            radius * (angle as f32).to_radians().cos(),
            radius * (angle as f32).to_radians().sin(),
        )
    })
    .collect::<Drawing>()
}

pub fn draw_arc_3d(word: &Polar, letter: &Polar, base: &f32, range: (Degree, Degree)) -> Drawing {
    let start_range = range.0 .0.round() as i32;
    let end_range = range.1 .0.round() as i32;

    match start_range < end_range {
        true => (start_range..=end_range),
        false => (start_range..=end_range + 360),
    }
    .map(|angle| {
        (
            word.radius * word.angle.0.to_radians().cos()
                + letter.radius * letter.angle.0.to_radians().cos()
                + base * (angle as f32).to_radians().cos(),
            word.radius * word.angle.0.to_radians().sin()
                + letter.radius * letter.angle.0.to_radians().sin()
                + base * (angle as f32).to_radians().sin(),
        )
    })
    .collect::<Drawing>()
}

pub fn draw_dot_3d(word: &Polar, letter: &Polar, modifier: &Polar) -> Drawing {
    Drawing(vec![(
        word.radius * word.angle.0.to_radians().cos()
            + letter.radius * letter.angle.0.to_radians().cos()
            + modifier.radius * modifier.angle.0.to_radians().cos(),
        word.radius * word.angle.0.to_radians().sin()
            + letter.radius * letter.angle.0.to_radians().sin()
            + modifier.radius * modifier.angle.0.to_radians().sin(),
    )])
}

pub fn draw_line_3d(word: &Polar, letter: &Polar, modifier: &Polar) -> Drawing {
    Drawing(vec![
        (
            word.radius * word.angle.0.to_radians().cos()
                + letter.radius * letter.angle.0.to_radians().cos()
                + modifier.radius * modifier.angle.0.to_radians().cos(),
            word.radius * word.angle.0.to_radians().sin()
                + letter.radius * letter.angle.0.to_radians().sin()
                + modifier.radius * modifier.angle.0.to_radians().sin(),
        ),
        (
            word.radius * word.angle.0.to_radians().cos()
                + letter.radius * letter.angle.0.to_radians().cos()
                + 1.5 * modifier.radius * modifier.angle.0.to_radians().cos(),
            word.radius * word.angle.0.to_radians().sin()
                + letter.radius * letter.angle.0.to_radians().sin()
                + 1.5 * modifier.radius * modifier.angle.0.to_radians().sin(),
        ),
    ])
}

pub fn law_of_sines_angle(side_a: &f32, side_b: &f32, angle_b: Degree) -> Degree {
    Degree::new(
        ((side_b * (180.0 - angle_b.value()).to_radians().sin()) / side_a)
            .asin()
            .to_degrees(),
    )
}
