pub mod state;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Approve};
use liquidity_lockbox::{
  self,
  state::{LiquidityLockbox},
  cpi::accounts::InitializeLiquidityLockbox
};
use whirlpool::{
  self,
  state::{Whirlpool, Position},
};
use solana_program::{pubkey::Pubkey, program::invoke_signed};
use spl_token::instruction::{burn_checked, mint_to};
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
    // Check that the first token mint is SOL
    if ctx.accounts.token_sol_mint.key() != SOL {
      return Err(ErrorCode::WrongTokenMint.into());
    }

    // Check that the second token mint is OLAS
    if ctx.accounts.token_olas_mint.key() != OLAS {
      return Err(ErrorCode::WrongTokenMint.into());
    }

    // Get the fee collector account
    let collector = &mut ctx.accounts.collector;

    // Get the anchor-derived bump
    let bump = *ctx.bumps.get("collector").unwrap();

    // Initialize lockbox account
    collector.initialize(
      bump
    )?;

    // Get fee collector signer seeds
    let signer_seeds = &[&ctx.accounts.collector.seeds()[..]];

    // CPI call to increase liquidity
    let cpi_program_lockbox_initialize = ctx.accounts.lockbox_program.to_account_info();
    let cpi_accounts_lockbox_initialize = InitializeLiquidityLockbox {
      signer: ctx.accounts.collector.to_account_info(),
      lockbox: ctx.accounts.lockbox.to_account_info(),
      bridged_token_mint: ctx.accounts.bridged_token_mint.to_account_info(),
      fee_collector_token_owner_account_a: ctx.accounts.token_sol_account.to_account_info(),
      fee_collector_token_owner_account_b: ctx.accounts.token_olas_account.to_account_info(),
      position: ctx.accounts.position.to_account_info(),
      position_mint: ctx.accounts.position_mint.to_account_info(),
      pda_position_account: ctx.accounts.pda_position_account.to_account_info(),
      whirlpool: ctx.accounts.whirlpool.to_account_info(),
      token_program: ctx.accounts.token_program.to_account_info(),
      system_program: ctx.accounts.system_program.to_account_info(),
      rent: ctx.accounts.rent.to_account_info(),
    };

    let cpi_ctx_lockbox_initialize = CpiContext::new_with_signer(
      cpi_program_lockbox_initialize,
      cpi_accounts_lockbox_initialize,
      signer_seeds
    );
    liquidity_lockbox::cpi::initialize(cpi_ctx_lockbox_initialize)?;

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
    space = FeeCollector::LEN)]
  pub collector: Box<Account<'info, FeeCollector>>,

//   #[account(constraint = bridged_token_mint.mint_authority.unwrap() == lockbox.key())]
//   pub bridged_token_mint: Box<Account<'info, Mint>>,

  #[account(constraint = token_sol_mint.key() != token_olas_mint.key())]
  pub token_sol_mint: Box<Account<'info, Mint>>,
  pub token_olas_mint: Box<Account<'info, Mint>>,

  #[account(init,
    token::mint = token_sol_mint,
    token::authority = collector,
    payer = signer,
    )]
  pub token_sol_account: Box<Account<'info, TokenAccount>>,

  #[account(init,
    token::mint = token_olas_mint,
    token::authority = collector,
    payer = signer,
    )]
  pub token_olas_account: Box<Account<'info, TokenAccount>>,

  #[account(mut, constraint = lockbox.to_account_info().owner == &lockbox_program.key())]
  pub lockbox: Box<Account<'info, LiquidityLockbox>>,
  // All of the following account are checked in the Liquidity Lockbox program initialization
  pub bridged_token_mint: Box<Account<'info, Mint>>,
  pub position: Box<Account<'info, Position>>,
  pub position_mint: Box<Account<'info, Mint>>,
  pub pda_position_account: Box<Account<'info, TokenAccount>>,
  pub whirlpool: Box<Account<'info, Whirlpool>>,
  pub lockbox_program: Program<'info, liquidity_lockbox::program::LiquidityLockbox>,

  #[account(address = token::ID)]
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>
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
