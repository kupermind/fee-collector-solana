use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use solana_program::{
    pubkey::Pubkey,
    program::invoke_signed,
    bpf_loader_upgradeable::set_upgrade_authority,
    bpf_loader_upgradeable::upgrade
};
use spl_token::instruction::{set_authority, AuthorityType};

pub use context::*;
pub use errors::*;
pub use events::*;
pub use message::*;
pub use state::*;

pub mod context;
pub mod errors;
pub mod events;
pub mod message;
pub mod state;

declare_id!("DWDGo2UkBUFZ3VitBfWRBMvRnHr7E2DSh57NK27xMYaB");

#[program]
pub mod lockbox_governor {
    use super::*;
    use solana_program::pubkey;
    use wormhole_anchor_sdk::wormhole;

    // SOL address
    const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
    // OLAS address
    const OLAS: Pubkey = pubkey!("Ez3nzG9ofodYCvEmw73XhQ87LWNYVRM2s7diB5tBZPyM");

    /// Initializes a Lockbox account that stores state data.
    pub fn initialize(
    ctx: Context<InitializeLockboxGovernor>,
      chain: u16,
      timelock: [u8; 32],
    ) -> Result<()> {
        // Foreign emitter cannot share the same Wormhole Chain ID as the
        // Solana Wormhole program's. And cannot register a zero address.
        require!(
            chain > 0 && chain != wormhole::CHAIN_ID_SOLANA && !timelock.iter().all(|&x| x == 0),
            GovernorError::InvalidForeignEmitter,
        );

        // Get the config account
        let config = &mut ctx.accounts.config;

        // Anchor IDL default coder cannot handle wormhole::Finality enum,
        // so this value is stored as u8.
        config.finality = wormhole::Finality::Confirmed as u8;

        // Assign initialization parameters
        config.bump = [ctx.bumps.config];
        config.chain = chain;
        config.foreign_emitter = timelock;

        // Set zero initial values
        config.total_sol_transferred = 0;
        config.total_olas_transferred = 0;

        Ok(())
    }

  /// Transfer token funds.
  pub fn transfer(
    ctx: Context<TransferLockboxGovernor>,
    vaa_hash: [u8; 32]
  ) -> Result<()> {
        let posted_message = &ctx.accounts.posted;

        msg!(
            "Foreign emitter {:?}",
            ctx.accounts.posted.emitter_address()
        );

        msg!(
            "Emitter chain {:?}",
            ctx.accounts.posted.emitter_chain()
        );

        msg!(
            "Sequence {:?}",
            ctx.accounts.posted.sequence()
        );

        let TransferMessage { token, destination, amount } = posted_message.data();
        let token_account = Pubkey::try_from(*token).unwrap();
        let destination_account = Pubkey::try_from(*destination).unwrap();

        // Check token mint
        require!(
            token_account == ctx.accounts.destination_account.mint,
            GovernorError::InvalidMessage,
        );
        // Check destination account
        require!(
            destination_account == ctx.accounts.destination_account.key(),
            GovernorError::InvalidMessage,
        );

        msg!(
            "Token mint {:?}",
            *token
        );

        msg!(
            "Destination {:?}",
            *destination
        );

        msg!(
            "Amount {:?}",
            *amount
        );

    // Check that the token mint is SOL or OLAS
    if ctx.accounts.source_account.mint == SOL && ctx.accounts.destination_account.mint == SOL {
      ctx.accounts.config.total_sol_transferred += amount;
    } else if ctx.accounts.source_account.mint == OLAS && ctx.accounts.destination_account.mint == OLAS {
      ctx.accounts.config.total_olas_transferred += amount;
    } else {
      return Err(GovernorError::WrongTokenMint.into());
    }

        // Save batch ID, keccak256 hash and message payload.
        let received = &mut ctx.accounts.received;
        received.batch_id = posted_message.batch_id();
        received.wormhole_message_hash = vaa_hash;
        received.sequence = posted_message.sequence();

    // TODO: verifications

    // Transfer the amount of SOL
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.source_account.to_account_info(),
                to: ctx.accounts.destination_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[&ctx.accounts.config.seeds()],
        ),
        *amount
    )?;

    emit!(TransferEvent {
        signer: ctx.accounts.signer.key(),
        token: ctx.accounts.source_account.mint,
        destination: destination_account,
        amount: *amount
    });

    Ok(())
  }

  /// Transfer token funds.
  pub fn transfer_all(
    ctx: Context<TransferAllLockboxGovernor>
  ) -> Result<()> {
    // Check that the first token mint is SOL
    if ctx.accounts.source_account_sol.mint != SOL || ctx.accounts.destination_account_sol.mint != SOL {
      return Err(GovernorError::WrongTokenMint.into());
    }

    // Check that the second token mint is OLAS
    if ctx.accounts.source_account_olas.mint != OLAS || ctx.accounts.destination_account_olas.mint != OLAS {
      return Err(GovernorError::WrongTokenMint.into());
    }

    // Get all amounts
    let amount_sol = ctx.accounts.source_account_sol.amount;
    let amount_olas = ctx.accounts.source_account_olas.amount;
    ctx.accounts.config.total_sol_transferred += amount_sol;
    ctx.accounts.config.total_olas_transferred += amount_olas;

    // TODO optimize with creating context and calling transfer one by one
    // Transfer the amount of SOL
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.source_account_sol.to_account_info(),
                to: ctx.accounts.destination_account_sol.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[&ctx.accounts.config.seeds()],
        ),
        amount_sol,
    )?;

    // Transfer the amount of OLAS
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.source_account_olas.to_account_info(),
                to: ctx.accounts.destination_account_olas.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[&ctx.accounts.config.seeds()],
        ),
        amount_olas,
    )?;

    emit!(TransferAllEvent {
        signer: ctx.accounts.signer.key(),
        amount_sol,
        amount_olas
    });

    Ok(())
  }

  /// Transfer token account.
  pub fn transfer_token_accounts(
    ctx: Context<TransferTokenAccountsLockboxGovernor>
  ) -> Result<()> {
    // Check that the first token mint is SOL
    if ctx.accounts.source_account_sol.mint != SOL {
      return Err(GovernorError::WrongTokenMint.into());
    }

    // Check that the second token mint is OLAS
    if ctx.accounts.source_account_olas.mint != OLAS {
      return Err(GovernorError::WrongTokenMint.into());
    }

    // Transfer SOL token associated account
    invoke_signed(
        &set_authority(
            ctx.accounts.token_program.key,
            ctx.accounts.source_account_sol.to_account_info().key,
            Some(ctx.accounts.destination.to_account_info().key),
            AuthorityType::AccountOwner,
            ctx.accounts.config.to_account_info().key,
            &[],
        )?,
        &[
            ctx.accounts.source_account_sol.to_account_info(),
            ctx.accounts.config.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
        &[&ctx.accounts.config.seeds()],
    )?;

    // Transfer OLAS token associated account
    invoke_signed(
        &set_authority(
            ctx.accounts.token_program.key,
            ctx.accounts.source_account_olas.to_account_info().key,
            Some(ctx.accounts.destination.to_account_info().key),
            AuthorityType::AccountOwner,
            ctx.accounts.config.to_account_info().key,
            &[],
        )?,
        &[
            ctx.accounts.source_account_olas.to_account_info(),
            ctx.accounts.config.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
        &[&ctx.accounts.config.seeds()],
    )?;

    Ok(())
  }

  /// Change upgrade authority.
  pub fn change_upgrade_authority(
    ctx: Context<ChangeUpgradeAuthorityLockboxGovernor>
  ) -> Result<()> {
    // Change upgrade authority
    invoke_signed(
        &set_upgrade_authority(
            ctx.accounts.program_to_update_authority.to_account_info().key,
            ctx.accounts.config.to_account_info().key,
            Some(ctx.accounts.destination.to_account_info().key)
        ),
        &[
            ctx.accounts.program_data_to_update_authority.to_account_info(),
            ctx.accounts.config.to_account_info(),
            ctx.accounts.destination.to_account_info()
        ],
        &[&ctx.accounts.config.seeds()]
    )?;

    Ok(())
  }

  /// Upgrade the program.
  pub fn upgrade_program(
    ctx: Context<UpgradeProgramLockboxGovernor>
  ) -> Result<()> {
    // Transfer the token associated account
    invoke_signed(
        &upgrade(
            ctx.accounts.program_address.to_account_info().key,
            ctx.accounts.buffer_address.to_account_info().key,
            ctx.accounts.config.to_account_info().key,
            ctx.accounts.spill_address.to_account_info().key
        ),
        &[
            ctx.accounts.program_data_address.to_account_info(),
            ctx.accounts.program_address.to_account_info(),
            ctx.accounts.buffer_address.to_account_info(),
            ctx.accounts.spill_address.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.clock.to_account_info(),
            ctx.accounts.config.to_account_info()
        ],
        &[&ctx.accounts.config.seeds()]
    )?;

    Ok(())
  }

//     /// This instruction reads a posted verified Wormhole message and verifies
//     /// that the payload is of type [HelloWorldMessage::Hello] (payload ID == 1). HelloWorldMessage
//     /// data is stored in a [Received] account.
//     ///
//     /// See [HelloWorldMessage] enum for deserialization implementation.
//     ///
//     /// # Arguments
//     ///
//     /// * `vaa_hash` - Keccak256 hash of verified Wormhole message
//     pub fn receive_message(ctx: Context<ReceiveMessage>, vaa_hash: [u8; 32]) -> Result<()> {
//         let posted_message = &ctx.accounts.posted;
//
//         msg!(
//             "Foreign emitter {:?}",
//             ctx.accounts.posted.emitter_address()
//         );
//
//         msg!(
//             "Emitter chain {:?}",
//             ctx.accounts.posted.emitter_chain()
//         );
//
//         msg!(
//             "Sequence {:?}",
//             ctx.accounts.posted.sequence()
//         );
//
//         let GovernorMessage { message } = posted_message.data();
//         // GovernorMessage cannot be larger than the maximum size of the account.
//         require!(
//             message.len() <= MESSAGE_MAX_LENGTH,
//             GovernorError::InvalidMessage,
//         );
//
//
//         msg!(
//             "Message {:?}",
//             message
//         );
//
//         // Save batch ID, keccak256 hash and message payload.
//         let received = &mut ctx.accounts.received;
//         received.batch_id = posted_message.batch_id();
//         received.wormhole_message_hash = vaa_hash;
//         received.message = message.clone();
//
//         // Done
//         Ok(())
//     }
}
