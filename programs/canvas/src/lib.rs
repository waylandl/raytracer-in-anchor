#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;

declare_id!("9ZpEywiXTgkHMAxZgjgBffGbyJBBu6pekFmhu6iFezp4");

#[program]
pub mod canvas {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, w: u8, h: u8) -> Result<()> {
        instructions::create::initialize(ctx, w, h)
    }

    pub fn fill(ctx: Context<Fill>, seed: String, color: [f32; 3]) -> Result<()> {
        let color = Color::new(color);
        instructions::create::fill(ctx, seed, color)
    }

    pub fn modify_color(ctx: Context<PdaExisting>, color: [f32; 3]) -> Result<()> {
        let color = Color::new(color);
        instructions::create::modify_color(ctx, color)
    }

    pub fn draw(ctx: Context<Coloring>, color: [f32; 3]) -> Result<()> {
        let color = Color::new(color);
        ctx.accounts.canvas.draw(color)
    }
}
