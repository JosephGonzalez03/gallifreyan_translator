use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Degree(f32);

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

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Add for &Degree {
    type Output = Degree;

    fn add(self, other: Self) -> Degree {
        Degree(self.0 + other.0)
    }
}

pub struct PolarCoordinate {
    radius: f32,
    angle: Degree,
}

impl PolarCoordinate {
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

pub fn arc3_d(
    position1: &PolarCoordinate,
    position2: &PolarCoordinate,
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

pub fn dot(position: &PolarCoordinate, orientation: PolarCoordinate) -> Vec<(f32, f32)> {
    vec![(
        position.radius * position.angle.0.to_radians().cos()
            + 1.2 * orientation.radius * orientation.angle.0.to_radians().cos(),
        position.radius * position.angle.0.to_radians().sin()
            + 1.2 * orientation.radius * orientation.angle.0.to_radians().sin(),
    )]
}

pub fn normal_line(position: &PolarCoordinate, orientation: PolarCoordinate) -> Vec<(f32, f32)> {
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
