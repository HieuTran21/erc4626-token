use crate::*;

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // #[account(
    //     init,
    //      seeds = [
    //         b"spl-token-mint".as_ref(),
    //      ],
    //     bump,
    //     payer = signer,
    //     mint::authority = signer,
    //     mint::decimals = 0,
    //     mint::freeze_authority = signer
    // )]
    // pub spl_token_mint: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn exec(ctx: Context<CreateMint>, shares: u64) -> Result<()> {

    let author: &Signer = &ctx.accounts.signer;
    // if (shares > maxShares) {
    //     revert ERC4626ExceededMaxMint(ctx.accounts.token_account, shares, maxShares);
    // }
    let assets = shares;
    anchor_spl::token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: author.to_account_info(),
            },
        ),
        assets,
    )?;

    Ok(())
}