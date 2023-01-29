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

pub fn arc3_d(
    position1: &Polar,
    position2: &Polar,
    radius: f32,
    range: (Degree, Degree),
) -> Vec<(f32, f32)> {
    let start_range = range.0 .0.round() as i32;
    let end_range = range.1 .0.round() as i32;

    match start_range < end_range {
        true => (start_range..=end_range),
        false => (start_range..=end_range + 360),
    }
    .map(|angle| {
        (
            position1.radius * position1.angle.value().to_radians().cos()
                + position2.radius * position2.angle.0.to_radians().cos()
                + radius * (angle as f32).to_radians().cos(),
            position1.radius * position1.angle.0.to_radians().sin()
                + position2.radius * position2.angle.0.to_radians().sin()
                + radius * (angle as f32).to_radians().sin(),
        )
    })
    .collect::<Vec<(f32, f32)>>()
}

pub fn dot(position: &Polar, orientation: Polar) -> Vec<(f32, f32)> {
    vec![(
        position.radius * position.angle.0.to_radians().cos()
            + 1.2 * orientation.radius * orientation.angle.0.to_radians().cos(),
        position.radius * position.angle.0.to_radians().sin()
            + 1.2 * orientation.radius * orientation.angle.0.to_radians().sin(),
    )]
}

pub fn normal_line(position: &Polar, orientation: Polar) -> Vec<(f32, f32)> {
    vec![
        (
            position.radius * position.angle.0.to_radians().cos()
                + orientation.radius * orientation.angle.0.to_radians().cos(),
            position.radius * position.angle.0.to_radians().sin()
                + orientation.radius * orientation.angle.0.to_radians().sin(),
        ),
        (
            position.radius * position.angle.0.to_radians().cos()
                + 1.5 * orientation.radius * orientation.angle.0.to_radians().cos(),
            position.radius * position.angle.0.to_radians().sin()
                + 1.5 * orientation.radius * orientation.angle.0.to_radians().sin(),
        ),
    ]
}

pub fn law_of_sines_angle(side_a: &f32, side_b: &f32, angle_b: Degree) -> Degree {
    Degree::new(
        ((side_b * (180.0 - angle_b.value().to_radians()).sin()) / side_a)
            .asin()
            .to_degrees(),
    )
}
