use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
declare_id!("ZtmCJQpJFxDKJ2g6tU8LrJeqgRNm8NjFyUnHgWjnzuD");

pub mod instructions;
pub use instructions::*;

#[program]
pub mod erc4626_token {
    use super::*;

    pub fn deposit(ctx: Context<CreateDeposit>, assets: u64) -> Result<()> {
        deposit::exec(ctx, assets)
    }

    pub fn mint(ctx: Context<CreateMint>, assets: u64) -> Result<()> {
        mint::exec(ctx, assets)
    }

    pub fn withdraw(ctx: Context<CreateWithdraw>, bump: u8, assets: u64) -> Result<()> {
        withdraw::exec(ctx, bump, assets)
    }

    pub fn redeem(ctx: Context<Redeem>, assets: u64) -> Result<()> {
        redeem::exec(ctx, assets)
    }
}
