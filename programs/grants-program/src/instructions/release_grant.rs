use anchor_lang::prelude::*;

use crate::state::{ProgramInfo, Grant, GrantState};

#[derive(Accounts)]
pub struct ReleaseGrant<'info> {
    admin: Signer<'info>,

    #[account(
        has_one = admin, 
        seeds = [ProgramInfo::SEED.as_bytes().as_ref()],
        bump = program_info.bump,
    )]
    program_info: Account<'info, ProgramInfo>,

    #[account(
        mut,
        has_one = author,
        seeds = [b"grant", grant.grant_num.to_be_bytes().as_ref()],
        bump = grant.bump,
    )]
    grant: Account<'info, Grant>,

    #[account(mut)]
    /// CHECK: We check that the grant has this author
    author: AccountInfo<'info>,
}

pub fn release_grant(ctx: Context<ReleaseGrant>) -> Result<()> {
    // Check that it is releasable
    ctx.accounts.grant.is_active()?;

    // transfer lamports from grant to creator
    **ctx.accounts.grant
        .to_account_info().try_borrow_mut_lamports()? -= ctx.accounts.grant.lamports_raised;

    **ctx.accounts.author.try_borrow_mut_lamports()? += ctx.accounts.grant.lamports_raised;

    // update grant state
    ctx.accounts.grant.state = GrantState::Released;

    Ok(())
}
