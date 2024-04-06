pub mod state;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use solana_program::{
  pubkey::Pubkey,
  program::invoke_signed,
  bpf_loader_upgradeable::set_upgrade_authority,
  bpf_loader_upgradeable::upgrade,
  system_program,
  sysvar
};
use spl_token::instruction::{set_authority, AuthorityType};
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
    // Check that the token mint is SOL or OLAS
    if ctx.accounts.collector_account.mint == SOL && ctx.accounts.destination_account.mint == SOL {
      ctx.accounts.collector.total_sol_transferred += amount;
    } else if ctx.accounts.collector_account.mint == OLAS && ctx.accounts.destination_account.mint == OLAS {
      ctx.accounts.collector.total_olas_transferred += amount;
    } else {
      return Err(ErrorCode::WrongTokenMint.into());
    }

    // Transfer the amount of SOL
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.collector_account.to_account_info(),
                to: ctx.accounts.destination_account.to_account_info(),
                authority: ctx.accounts.collector.to_account_info(),
            },
            &[&ctx.accounts.collector.seeds()],
        ),
        amount
    )?;

    Ok(())
  }

  /// Transfer token funds.
  pub fn transfer_all(
    ctx: Context<TransferAllFeeCollector>
  ) -> Result<()> {
    // Check that the first token mint is SOL
    if ctx.accounts.collector_account_sol.mint != SOL || ctx.accounts.destination_account_sol.mint != SOL {
      return Err(ErrorCode::WrongTokenMint.into());
    }

    // Check that the second token mint is OLAS
    if ctx.accounts.collector_account_olas.mint != OLAS || ctx.accounts.destination_account_olas.mint != OLAS {
      return Err(ErrorCode::WrongTokenMint.into());
    }

    // Get all amounts
    let amount_sol = ctx.accounts.collector_account_sol.amount;
    let amount_olas = ctx.accounts.collector_account_olas.amount;

    // TODO optimize with creating context and calling transfer one by one
    // Transfer the amount of SOL
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.collector_account_sol.to_account_info(),
                to: ctx.accounts.destination_account_sol.to_account_info(),
                authority: ctx.accounts.collector.to_account_info(),
            },
            &[&ctx.accounts.collector.seeds()],
        ),
        amount_sol,
    )?;

    // Transfer the amount of OLAS
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.collector_account_olas.to_account_info(),
                to: ctx.accounts.destination_account_olas.to_account_info(),
                authority: ctx.accounts.collector.to_account_info(),
            },
            &[&ctx.accounts.collector.seeds()],
        ),
        amount_olas,
    )?;

    Ok(())
  }

  /// Transfer token account.
  pub fn transfer_token_accounts(
    ctx: Context<TransferTokenAccountsFeeCollector>
  ) -> Result<()> {
    // Check that the first token mint is SOL
    if ctx.accounts.collector_account_sol.mint != SOL {
      return Err(ErrorCode::WrongTokenMint.into());
    }

    // Check that the second token mint is OLAS
    if ctx.accounts.collector_account_olas.mint != OLAS {
      return Err(ErrorCode::WrongTokenMint.into());
    }

    // Transfer SOL token associated account
    invoke_signed(
        &set_authority(
            ctx.accounts.token_program.key,
            ctx.accounts.collector_account_sol.to_account_info().key,
            Some(ctx.accounts.destination.to_account_info().key),
            AuthorityType::AccountOwner,
            ctx.accounts.collector.to_account_info().key,
            &[ctx.accounts.collector.to_account_info().key],
        )?,
        &[
            ctx.accounts.collector_account_sol.to_account_info(),
            ctx.accounts.collector.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
        &[&ctx.accounts.collector.seeds()],
    )?;

    // Transfer OLAS token associated account
    invoke_signed(
        &set_authority(
            ctx.accounts.token_program.key,
            ctx.accounts.collector_account_olas.to_account_info().key,
            Some(ctx.accounts.destination.to_account_info().key),
            AuthorityType::AccountOwner,
            ctx.accounts.collector.to_account_info().key,
            &[ctx.accounts.collector.to_account_info().key],
        )?,
        &[
            ctx.accounts.collector_account_olas.to_account_info(),
            ctx.accounts.collector.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
        &[&ctx.accounts.collector.seeds()],
    )?;

    Ok(())
  }

  /// Change upgrade authority.
  pub fn change_upgrade_authority(
    ctx: Context<ChangeUpgradeAuthorityFeeCollector>
  ) -> Result<()> {
    // Change upgrade authority
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

  /// Upgrade the program.
  pub fn upgrade_program(
    ctx: Context<UpgradeProgramFeeCollector>
  ) -> Result<()> {
    // Transfer the token associated account
    invoke_signed(
        &upgrade(
            ctx.accounts.program_address.to_account_info().key,
            ctx.accounts.buffer_address.to_account_info().key,
            ctx.accounts.collector.to_account_info().key,
            ctx.accounts.spill_address.to_account_info().key
        ),
        &[
            ctx.accounts.program_data_address.to_account_info(),
            ctx.accounts.program_address.to_account_info(),
            ctx.accounts.buffer_address.to_account_info(),
            ctx.accounts.spill_address.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.clock.to_account_info(),
            ctx.accounts.collector.to_account_info()
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

  #[account(address = system_program::ID)]
  pub system_program: Program<'info, System>,
  #[account(address = sysvar::rent::ID)]
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
  pub destination_account: Box<Account<'info, TokenAccount>>,

  #[account(address = token::ID)]
  pub token_program: Program<'info, Token>
}

#[derive(Accounts)]
pub struct TransferAllFeeCollector<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(mut)]
  pub collector: Box<Account<'info, FeeCollector>>,

  #[account(mut)]
  pub collector_account_sol: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub collector_account_olas: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub destination_account_sol: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub destination_account_olas: Box<Account<'info, TokenAccount>>,

  #[account(address = token::ID)]
  pub token_program: Program<'info, Token>
}

#[derive(Accounts)]
pub struct TransferTokenAccountsFeeCollector<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(mut)]
  pub collector: Box<Account<'info, FeeCollector>>,

  #[account(mut)]
  pub collector_account_sol: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub collector_account_olas: Box<Account<'info, TokenAccount>>,

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
}

#[derive(Accounts)]
pub struct UpgradeProgramFeeCollector<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,

  /// CHECK: Check later
  #[account(mut)]
  pub program_address: UncheckedAccount<'info>,

  /// CHECK: Check later
  #[account(mut)]
  pub program_data_address: UncheckedAccount<'info>,

  /// CHECK: Check later
  #[account(mut)]
  pub buffer_address: UncheckedAccount<'info>,

  #[account(mut)]
  pub spill_address: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub collector: Box<Account<'info, FeeCollector>>,

  #[account(address = sysvar::rent::ID)]
  pub rent: Sysvar<'info, Rent>,
  #[account(address = sysvar::clock::ID)]
  pub clock: Sysvar<'info, Clock>
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
