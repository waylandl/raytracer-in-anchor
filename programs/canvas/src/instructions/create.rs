use crate::color::Color;
use anchor_lang::prelude::*;

pub fn initialize(ctx: Context<Initialize>, w: u8, h: u8) -> Result<()> {
    let canvas = &mut ctx.accounts.canvas;
    canvas.new(w, h)
}

pub fn fill(ctx: Context<Fill>, _seed: String, color: Color) -> Result<()> {
    let pda = &mut ctx.accounts.pda;
    pda.bump = *ctx.bumps.get("pda").unwrap();
    pda.color = color;
    Ok(())
}

pub fn modify_color(ctx: Context<PdaExisting>, color: Color) -> Result<()> {
    let pda = &mut ctx.accounts.pda;
    pda.color = color;
    Ok(())
}

#[account]
pub struct Canvas {
    pub width: u8,
    pub height: u8,
    pub pixels: String,
    pub helper: u8,
}

impl Canvas {
    pub fn new(&mut self, w: u8, h: u8) -> Result<()> {
        self.width = w;
        self.height = h;
        self.pixels = "P3\n".to_owned()
            + &self.width.to_string()
            + " "
            + &self.height.to_string()
            + "\n255\n";
        self.helper = 0;
        Ok(())
    }

    pub fn draw(&mut self, color: Color) -> Result<()> {
        if self.helper >= 4 {
            self.pixels.push_str("\n");
            self.helper = 0;
        }
        self.pixels.push_str(
            &(((color.r * 255.0).round() as u8).to_string()
                + " "
                + &((color.g * 255.0).round() as u8).to_string()
                + " "
                + &((color.b * 255.0).round() as u8).to_string()
                + " "),
        );
        self.helper = self.helper + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = operator, space = 8 + 10200)]
    pub canvas: Account<'info, Canvas>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Coloring<'info> {
    #[account(mut)]
    pub canvas: Account<'info, Canvas>,
}

#[derive(Accounts)]
pub struct PdaExisting<'info> {
    #[account(mut)]
    pub pda: Account<'info, Pda>,
}
#[derive(Accounts)]
#[instruction(seed: String, bump: u8)]
pub struct Fill<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,
    pub canvas: Account<'info, Canvas>,
    #[account(
        init,
        payer = operator,
        space = 100,
        seeds = [
            canvas.key().as_ref(),
            seed.as_bytes()
        ],
        bump,
    )]
    pub pda: Account<'info, Pda>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pda {
    pub color: Color,
    pub bump: u8,
}
