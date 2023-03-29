use crate::elements::Tuple;
use crate::operate::Operation;
use anchor_lang::prelude::*;
use std::ops::{Add, Mul, Neg, Sub};

pub fn new(ctx: Context<EqnSetup>) -> Result<()> {
    let eqn = &mut ctx.accounts.eqn;
    eqn.new_eqn()
}

pub fn set_element_1(ctx: Context<EqnExists>, coords: [f32; 4]) -> Result<()> {
    ctx.accounts.eqn.set_element_1(coords)
}

pub fn set_element_2(ctx: Context<EqnExists>, coords: [f32; 4]) -> Result<()> {
    let eqn = &mut ctx.accounts.eqn;
    eqn.set_element_2(coords)
}

pub fn set_operation(ctx: Context<EqnExists>, incoming: &str) -> Result<()> {
    let eqn = &mut ctx.accounts.eqn;
    eqn.set_operation(incoming)
}

#[derive(Accounts)]
pub struct EqnExists<'info> {
    #[account(mut)]
    pub eqn: Account<'info, Eqn>,
}

#[derive(Accounts)]
pub struct EqnSetup<'info> {
    #[account(init, payer = operator, space = 8 + 52)]
    pub eqn: Account<'info, Eqn>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Debug)]
#[account]
pub struct Eqn {
    pub element_1: Option<Tuple>, // 1 + 16
    pub element_2: Option<Tuple>, // 17
    pub operation: Operation,     // 1
    pub result: Option<Tuple>,    // 1 + 16
}

impl Eqn {
    pub fn new_eqn(&mut self) -> Result<()> {
        self.element_1 = None;
        self.element_2 = None;
        self.operation = crate::Operation::Addition;
        self.result = None;
        Ok(())
    }
    pub fn set_element_1(&mut self, incoming: [f32; 4]) -> Result<()> {
        self.element_1 = Some(Tuple::new(incoming));
        Ok(())
    }

    pub fn set_element_2(&mut self, incoming: [f32; 4]) -> Result<()> {
        self.element_2 = Some(Tuple::new(incoming));
        Ok(())
    }
    pub fn set_operation(&mut self, incoming: &str) -> Result<()> {
        let new_op = match incoming {
            "+" => Operation::Addition,
            "-" => Operation::Subtraction,
            _ => panic!("unknown operation"),
        };
        self.operation = new_op;
        Ok(())
    }
}

impl Tuple {
    pub fn dot(self, other: Tuple) -> f32 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(x, y)| x * y)
            .sum()
    }

    pub fn cross(self, other: Tuple) -> Self {
        Tuple {
            data: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
                0.0,
            ],
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;
    fn sub(self, other: Tuple) -> Self {
        Tuple {
            data: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
                self.tt() - other.tt().clamp(0.0, 1.0),
            ],
        }
    }
}

impl Add for Tuple {
    type Output = Tuple;
    fn add(self, other: Tuple) -> Self {
        Tuple {
            data: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
                self.tt() + other.tt().clamp(0.0, 1.0),
            ],
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Self {
        Tuple {
            data: [self.x() * -1.0, self.y() * -1.0, self.z() * -1.0, self.tt()],
        }
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;
    fn mul(self, other: f32) -> Self {
        Tuple {
            data: [
                self.x() * other,
                self.y() * other,
                self.z() * other,
                self.tt() * other.clamp(0.0, 1.0),
            ],
        }
    }
}

impl Mul<Tuple> for f32 {
    type Output = Tuple;
    fn mul(self, other: Tuple) -> Tuple {
        Tuple {
            data: [
                self * other.x(),
                self * other.y(),
                self * other.z(),
                self * other.tt().clamp(0.0, 1.0),
            ],
        }
    }
}
