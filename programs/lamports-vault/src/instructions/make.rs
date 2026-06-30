use crate::constants::*;
use crate::state::EscrowState;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked};

#[derive(Accounts)]
#[instruction(seed: u16)]
pub struct Make<'info> {
    // maker: mut, exist always, just Signer in this lifecircle
    #[account(mut)]
    pub maker: Signer<'info>,

    // is a mint
    pub mint_deposit: InterfaceAccount<'info, Mint>,
    pub mint_receive: InterfaceAccount<'info, Mint>,

    // temporary vault: a box of interface:token account
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_deposit,
        associated_token::authority = escrow_state,
    )]
    pub temporary_vault: InterfaceAccount<'info, TokenAccount>,

    // escrow pda: is a Escrow State, combine to Account
    #[account(init,
        payer = maker,
        space = 8 + EscrowState::INIT_SPACE,
        // use the seed from beginning instruction
        seeds = [ESCROW_SEED, seed.to_le_bytes().as_ref(), maker.key().as_ref()],
        bump
    )]
    pub escrow_state: Account<'info, EscrowState>,
    // make ata, an Interface account
    #[account(mut,
associated_token::mint = mint_deposit, associated_token::authority = maker)]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(mut,
    address = ADMIN_ADDR.parse::<Pubkey>().unwrap())]
    // convert admin address to Pubkey
    pub fee_vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    // because we used InterfaceAccount (Interface has ID trait)
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn make_handler(
    ctx: Context<Make>,
    seed: u16,
    deposit_amount: u64,
    receive_amount: u64,
    fees_bps: u16,
) -> Result<()> {
    let cpi_program = ctx.accounts.token_program.to_account_info();
    // calculate fee amount
    let fee_amount = (deposit_amount as u128 * fees_bps as u128 / 10000) as u64;
    // remain amount
    let remain_amount = deposit_amount - fee_amount;
    // initialize escrow state
    ctx.accounts.escrow_state.set_inner(EscrowState {
        maker: ctx.accounts.maker.key(),
        mint_deposit: ctx.accounts.mint_deposit.key(),
        deposit_amount: remain_amount,
        mint_receive: ctx.accounts.mint_receive.key(),
        receive_amount: receive_amount,
        created_at: Clock::get()?.unix_timestamp,
        fees_bps,
        seed,
        bump: ctx.bumps.escrow_state,
    });
    // transfer fees
    let transfer_fees = TransferChecked {
        from: ctx.accounts.maker_ata_a.to_account_info(),
        mint: ctx.accounts.mint_deposit.to_account_info(),
        to: ctx.accounts.fee_vault.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };

    // escrow transfer
    let transfer_accounts = TransferChecked {
        from: ctx.accounts.maker_ata_a.to_account_info(),
        mint: ctx.accounts.mint_deposit.to_account_info(),
        to: ctx.accounts.temporary_vault.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };
    // contexts
    let cpi_ctx_fees = CpiContext::new(cpi_program.key(), transfer_fees);
    let cpi_ctx = CpiContext::new(cpi_program.key(), transfer_accounts);

    anchor_spl::token_interface::transfer_checked(
        cpi_ctx,
        remain_amount,
        ctx.accounts.mint_deposit.decimals,
    )?;
    anchor_spl::token_interface::transfer_checked(
        cpi_ctx_fees,
        fee_amount,
        ctx.accounts.mint_deposit.decimals,
    )?;
    Ok(())
}
