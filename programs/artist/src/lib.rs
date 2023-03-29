#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use anchor_lang::prelude::*;
use math::cpi::accounts::EqnExists;
use math::program::Math;
use math::{
    self,
    instructions::{elements::Tuple, equation::Eqn},
};

declare_id!("CT72WNsv3kjdnesJzh5Ma9CebSyYCJJ2uUDxfjvUPjnK");

#[program]
pub mod artist {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        gravity: [f32; 4],
        wind: [f32; 4],
        position: [f32; 4],
        velocity: [f32; 4],
    ) -> Result<()> {
        let variables = &mut ctx.accounts.variables;
        variables.env.gravity = gravity;
        variables.env.wind = wind;
        variables.proj.position = position;
        variables.proj.velocity = velocity;
        variables.bump = *ctx.bumps.get("variables").unwrap();
        Ok(())
    }

    pub fn update_vars(
        ctx: Context<Vars>,
        gravity: [f32; 4],
        wind: [f32; 4],
        position: [f32; 4],
        velocity: [f32; 4],
    ) -> Result<()> {
        let variables = &mut ctx.accounts.var;
        variables.env.gravity = gravity;
        variables.env.wind = wind;
        variables.proj.position = position;
        variables.proj.velocity = velocity;
        Ok(())
    }
    pub fn tick(ctx: Context<Tick>, counter: String) -> Result<()> {
        msg!("canvas coords: {:?}", counter);
        let variables = &ctx.accounts.var;
        let position = variables.proj.position.clone();
        let velocity = variables.proj.velocity.clone();
        let gravity = variables.env.gravity.clone();
        let wind = variables.env.wind.clone();

        msg!("ok varibles {:?} {:?}", position, velocity);
        math::cpi::set_element_one(ctx.accounts.eqn_exists_ctx(), position)?;
        math::cpi::set_element_two(ctx.accounts.eqn_exists_ctx(), velocity)?;
        math::cpi::set_operation(ctx.accounts.eqn_exists_ctx(), "+".to_string())?;
        math::cpi::operate(ctx.accounts.eqn_exists_ctx())?;

        ctx.accounts.eqn.reload()?;
        let eqn_state = (&ctx.accounts.eqn.result).clone();
        let new_position = eqn_state.expect("first operation failed").data;

        math::cpi::set_element_one(ctx.accounts.eqn_exists_ctx(), velocity)?;
        math::cpi::set_element_two(ctx.accounts.eqn_exists_ctx(), gravity)?;
        math::cpi::set_operation(ctx.accounts.eqn_exists_ctx(), "+".to_string())?;
        math::cpi::operate(ctx.accounts.eqn_exists_ctx())?;

        ctx.accounts.eqn.reload()?;
        let eqn_state = (&ctx.accounts.eqn.result).clone();
        let new_proj_temp = eqn_state.expect("second operation failed").data;

        math::cpi::set_element_one(ctx.accounts.eqn_exists_ctx(), new_proj_temp)?;
        math::cpi::set_element_two(ctx.accounts.eqn_exists_ctx(), wind)?;
        math::cpi::set_operation(ctx.accounts.eqn_exists_ctx(), "+".to_string())?;
        math::cpi::operate(ctx.accounts.eqn_exists_ctx())?;

        ctx.accounts.eqn.reload()?;
        let eqn_state = (&ctx.accounts.eqn.result).clone();
        let new_velo = eqn_state.expect("third operation failed").data;

        let variables = &mut ctx.accounts.var;
        variables.proj.position = new_position;
        variables.proj.velocity = new_velo;

        Ok(())
    }
}

impl<'info> Tick<'info> {
    pub fn eqn_exists_ctx(&self) -> CpiContext<'_, '_, '_, 'info, EqnExists<'info>> {
        let cpi_program = self.math_program.to_account_info();
        let cpi_accounts = EqnExists {
            eqn: self.eqn.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct Vars<'info> {
    #[account(mut)]
    pub var: Account<'info, Variables>,
}

#[derive(Accounts)]
pub struct Tick<'info> {
    #[account(mut)]
    pub eqn: Account<'info, Eqn>,
    #[account(mut)]
    pub var: Account<'info, Variables>,
    pub math_program: Program<'info, Math>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub operator: Signer<'info>,
    #[account(
        init,
        payer = operator,
        space = 73,
        seeds = [
            b"variables",
            operator.key().as_ref(),
        ],
        bump,
    )]
    pub variables: Account<'info, Variables>,
    pub system_program: Program<'info, System>,
}

#[derive(Debug)]
#[account]
pub struct Variables {
    pub env: Env,
    pub proj: Proj,
    pub bump: u8, // 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Env {
    pub gravity: [f32; 4],
    pub wind: [f32; 4],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Proj {
    pub position: [f32; 4],
    pub velocity: [f32; 4],
}
