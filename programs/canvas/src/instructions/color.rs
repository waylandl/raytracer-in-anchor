use anchor_lang::prelude::*;
use math::instructions::{elements, Tuple};
use std::ops::{Add, Mul, Sub};
pub static EPSILON: f32 = 0.00001;

pub fn equal_floats(a: f32, b: f32) -> bool {
    if (a.abs() - b.abs()).abs() < EPSILON {
        true
    } else {
        false
    }
}
#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn colorize(element: Tuple) -> Color {
        let data = element.data;
        Color::new([data[0], data[1], data[2]])
    }

    pub fn new(incoming: [f32; 3]) -> Color {
        Color {
            r: incoming[0],
            g: incoming[1],
            b: incoming[2],
        }
    }
    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
    pub fn equal(self, o: Color) -> bool {
        let l = vec![
            equal_floats(self.r, o.r),
            equal_floats(self.g, o.g),
            equal_floats(self.b, o.b),
        ];
        if l.contains(&false) {
            false
        } else {
            true
        }
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color {
            r: (self.r - other.r).clamp(-1.0, 1.0),
            g: (self.g - other.g).clamp(-1.0, 1.0),
            b: (self.b - other.b).clamp(-1.0, 1.0),
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            r: (self.r + other.r).clamp(-1.0, 1.0),
            g: (self.g + other.g).clamp(-1.0, 1.0),
            b: (self.b + other.b).clamp(-1.0, 1.0),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Color {
        Color {
            r: (self.r * other).clamp(-1.0, 1.0),
            g: (self.g * other).clamp(-1.0, 1.0),
            b: (self.b * other).clamp(-1.0, 1.0),
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {
            r: (self.r * other.r).clamp(-1.0, 1.0),
            g: (self.g * other.g).clamp(-1.0, 1.0),
            b: (self.b * other.b).clamp(-1.0, 1.0),
        }
    }
}
