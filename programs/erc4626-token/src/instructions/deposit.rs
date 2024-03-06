use crate::*;

#[derive(Accounts)]
pub struct CreateDeposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init_if_needed, payer = owner, space = 32)]
    pub deposit: Account<'info, Deposit>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn exec(ctx: Context<CreateDeposit>, assets: u64) -> Result<()> {
    let deposit: &mut Account<Deposit> = &mut ctx.accounts.deposit;
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
    deposit.author = *author.key;
    deposit.assets += assets;

    Ok(())
}

#[account]
pub struct Deposit {
    pub author: Pubkey,
    pub assets: u64,
}
