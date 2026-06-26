use crate::VaultError;
use anchor_lang::{prelude::*, system_program::Transfer};

use crate::{VaultState, VAULT_SEED, VAULT_STATE_SEED};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [VAULT_STATE_SEED, user.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [VAULT_SEED, user.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_lamports(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user_key = ctx.accounts.user.key();
    let seeds = &[VAULT_SEED, user_key.as_ref(), &[ctx.bumps.vault]];
    let signer_seeds = &[&seeds[..]];
    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
    };
    require!(
        amount <= ctx.accounts.vault_state.max_withdraw,
        VaultError::WithdrawalTooLarge
    );
    let cpi_ctx = CpiContext::new_with_signer(cpi_program.key(), cpi_accounts, signer_seeds);
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;
    Ok(())
}
