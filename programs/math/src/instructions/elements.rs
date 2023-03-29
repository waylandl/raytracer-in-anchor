use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TupleType {
    Vector,
    Point,
    Color,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Tuple {
    pub data: [f32; 4],
}

impl Default for Tuple {
    fn default() -> Self {
        Tuple {
            data: [0 as f32; 4],
        }
    }
}
impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.data == other.data
    }
}

impl Tuple {
    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn tt(&self) -> f32 {
        self.data[3]
    }

    pub fn new(incoming: [f32; 4]) -> Self {
        Tuple { data: incoming }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    pub fn normal(&self) -> Self {
        let mag: f32 = self.magnitude();
        Tuple {
            data: [
                self.x() * (1.0 / mag),
                self.y() * (1.0 / mag),
                self.z() * (1.0 / mag),
                (self.tt() * (1.0 / mag)).clamp(0.0, 1.0),
            ],
        }
    }
}
