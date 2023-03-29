#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod math {
    use super::*;

    pub fn operate(ctx: Context<EqnExists>) -> Result<()> {
        instructions::operate::operate(ctx)
    }

    pub fn new(ctx: Context<EqnSetup>) -> Result<()> {
        instructions::equation::new(ctx)
    }

    pub fn set_element_one(ctx: Context<EqnExists>, incoming: [f32; 4]) -> Result<()> {
        instructions::equation::set_element_1(ctx, incoming)
    }

    pub fn set_element_two(ctx: Context<EqnExists>, incoming: [f32; 4]) -> Result<()> {
        instructions::equation::set_element_2(ctx, incoming)
    }

    pub fn set_operation(ctx: Context<EqnExists>, incoming: String) -> Result<()> {
        instructions::equation::set_operation(ctx, &incoming)
    }
}
