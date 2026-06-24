use anchor_lang::prelude::*;

use crate::{VAULT_SEED, VAULT_STATE_SEED, VaultState};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}

pub fn close_vault(ctx: Context<Close>) -> Result<()> {

    Ok(())
}