use crate::*;

#[derive(Accounts)]
pub struct CreateWithdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = 32)]
    pub withdraw: Account<'info, Withdraw>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn exec(ctx: Context<CreateWithdraw>, bump: u8, assets: u64) -> Result<()> {

    let author: &Signer = &ctx.accounts.owner;
    // if (assets > maxAssets) {
        // revert ERC4626ExceededMaxDeposit(receiver, assets, maxAssets);
    // }
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: author.to_account_info(),
            }
            ,&[&[&[bump][..]]]
        ),
        assets,
    )?;

    Ok(())
}

#[account]
pub struct Withdraw {
    pub author: Pubkey,
    pub timestamp: i64,
    pub amount: u64,
}