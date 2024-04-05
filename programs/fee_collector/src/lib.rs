pub mod state;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Approve, Transfer};
use liquidity_lockbox::{
  self,
  state::{LiquidityLockbox},
  cpi::accounts::InitializeLiquidityLockbox
};
use whirlpool::{
  self,
  state::{Whirlpool, Position},
};
use solana_program::{
  pubkey::Pubkey,
  program::invoke_signed,
  bpf_loader_upgradeable::set_upgrade_authority,
  bpf_loader_upgradeable::upgrade,
  loader_upgradeable_instruction::UpgradeableLoaderInstruction
};
use spl_token::instruction::{transfer, set_authority, AuthorityType};
pub use state::*;

declare_id!("DWDGo2UkBUFZ3VitBfWRBMvRnHr7E2DSh57NK27xMYaB");

#[program]
pub mod fee_collector {
  use super::*;
  use solana_program::pubkey;

  // SOL address
  const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
  // OLAS address
  const OLAS: Pubkey = pubkey!("Ez3nzG9ofodYCvEmw73XhQ87LWNYVRM2s7diB5tBZPyM");


  /// Initializes a Lockbox account that stores state data.
  pub fn initialize(
    ctx: Context<InitializeFeeCollector>
  ) -> Result<()> {
    // Get the fee collector account
    let collector = &mut ctx.accounts.collector;

    // Get the anchor-derived bump
    let bump = *ctx.bumps.get("collector").unwrap();

    // Initialize lockbox account
    collector.initialize(
      bump
    )?;

    Ok(())
  }

  /// Transfer token funds.
  pub fn transfer(
    ctx: Context<TransferFeeCollector>,
    amount: u64
  ) -> Result<()> {
    // Transfer the amount
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.collector_account.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.collector.to_account_info(),
            },
            &[&ctx.accounts.collector.seeds()],
        ),
        amount,
    )?;

    Ok(())
  }

  /// Transfer token account.
  pub fn transfer_token_account(
    ctx: Context<TransferTokenAccountFeeCollector>
  ) -> Result<()> {
    // Transfer the token associated account
    invoke_signed(
        &set_authority(
            ctx.accounts.token_program.key,
            ctx.accounts.collector_account.to_account_info().key,
            Some(ctx.accounts.destination.to_account_info().key),
            AuthorityType::AccountOwner,
            ctx.accounts.collector.to_account_info().key,
            &[ctx.accounts.collector.to_account_info().key],
        )?,
        &[
            ctx.accounts.collector_account.to_account_info(),
            ctx.accounts.collector.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
        &[&ctx.accounts.collector.seeds()],
    )?;

    Ok(())
  }

  /// Transfer token account.
  pub fn change_upgrade_authority(
    ctx: Context<ChangeUpgradeAuthorityFeeCollector>
  ) -> Result<()> {
    // Transfer the token associated account
    invoke_signed(
        &set_upgrade_authority(
            ctx.accounts.program_to_update_authority.to_account_info().key,
            ctx.accounts.collector.to_account_info().key,
            Some(ctx.accounts.destination.to_account_info().key)
        ),
        &[
            ctx.accounts.program_data_to_update_authority.to_account_info(),
            ctx.accounts.collector.to_account_info(),
            ctx.accounts.destination.to_account_info()
        ],
        &[&ctx.accounts.collector.seeds()]
    )?;

    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeFeeCollector<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(init,
    seeds = [
      b"fee_collector".as_ref()
    ],
    bump,
    payer = signer,
    space = FeeCollector::LEN
  )]
  pub collector: Box<Account<'info, FeeCollector>>,

  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct TransferFeeCollector<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(mut)]
  pub collector: Box<Account<'info, FeeCollector>>,

  #[account(mut)]
  pub collector_account: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub destination: Box<Account<'info, TokenAccount>>,

  #[account(address = token::ID)]
  pub token_program: Program<'info, Token>
}

#[derive(Accounts)]
pub struct TransferTokenAccountFeeCollector<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(mut)]
  pub collector: Box<Account<'info, FeeCollector>>,

  #[account(mut)]
  pub collector_account: Box<Account<'info, TokenAccount>>,

  /// CHECK: Check later
  #[account(mut)]
  pub destination: UncheckedAccount<'info>,

  #[account(address = token::ID)]
  pub token_program: Program<'info, Token>
}

#[derive(Accounts)]
pub struct ChangeUpgradeAuthorityFeeCollector<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  /// CHECK: Check later
  #[account(mut)]
  pub program_to_update_authority: UncheckedAccount<'info>,

  /// CHECK: Check later
  #[account(mut)]
  pub program_data_to_update_authority: UncheckedAccount<'info>,

  #[account(mut)]
  pub collector: Box<Account<'info, FeeCollector>>,

  /// CHECK: Check later
  #[account(mut)]
  pub destination: UncheckedAccount<'info>,

  /// CHECK: Check later
  pub bpf_loader: UncheckedAccount<'info>
}


#[error_code]
pub enum ErrorCode {
  #[msg("Wrong token mint")]
  WrongTokenMint,
}


#[event]
pub struct TransferEvent {
  // Signer (user)
  #[index]
  pub signer: Pubkey,
  // SOL amount transferred
  pub sol_transferred: u64,
  // OLAS amount transferred
  pub olas_transferred: u64
}
