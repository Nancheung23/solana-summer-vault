use anchor_lang::prelude::*;

use crate::{VAULT_SEED, VAULT_STATE_SEED, VaultState};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

}

pub fn withdraw_lamports(ctx: Context<Withdraw>, amount: u64) -> Result<()> {

    let signer_seeds = &[
        VAULT_SEED,
        &ctx.accounts.user.key().to_bytes(),
        &[ctx.accounts.vault_state.vault_bump],
    ];
    let binding = [&signer_seeds[..]];

    Ok(())
}