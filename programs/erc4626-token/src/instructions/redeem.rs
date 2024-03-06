use crate::*;

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // #[account(init_if_needed, payer = owner, space = 8)]
    // pub deposit: Account<'info, Deposit>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_program: Account<'info, TokenAccount>,
    pub token: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn exec(ctx: Context<Redeem>, assets: u64) -> Result<()> {

    let author: &Signer = &ctx.accounts.owner;
    // if (assets > maxAssets) {
        // revert ERC4626ExceededMaxDeposit(receiver, assets, maxAssets);
    // }
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: author.to_account_info(),
            },
        ),
        assets,
    )?;

    Ok(())
}