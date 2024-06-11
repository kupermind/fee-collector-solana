use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_program::{
    system_program,
    sysvar
};

use crate::{
    errors::GovernorError,
    message::{TransferMessage},
    state::{Config, Received},
};

#[derive(Accounts)]
/// Context used to initialize program data (i.e. config).
pub struct Initialize<'info> {
    #[account(mut)]
    /// Whoever initializes the config will be the payer of the program. Signer
    /// for creating the [`Config`] account and posting a Wormhole message
    /// indicating that the program is alive.
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [Config::SEED_PREFIX],
        bump,
        space = Config::LEN,

    )]
    /// Config account, which saves program data useful for other instructions.
    pub config: Account<'info, Config>,

    /// Clock sysvar.
    pub clock: Sysvar<'info, Clock>,

    /// Rent sysvar.
    pub rent: Sysvar<'info, Rent>,

    /// System program.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(vaa_hash: [u8; 32])]
pub struct ReceiveMessage<'info> {
    #[account(mut)]
    /// Payer will initialize an account that tracks his own message IDs.
    pub signer: Signer<'info>,

    #[account(
        seeds = [Config::SEED_PREFIX],
        bump,
        constraint = config.verify(posted.emitter_address()) @ GovernorError::InvalidForeignEmitter,
        constraint = posted.emitter_chain() == config.chain
    )]
    /// Config account. Wormhole PDAs specified in the config are checked
    /// against the Wormhole accounts in this context. Read-only.
    pub config: Account<'info, Config>,

    // Wormhole program.
    //pub wormhole_program: Program<'info, wormhole::program::Wormhole>,
    /// CHECK: testing
    pub wormhole_program: UncheckedAccount<'info>,

    #[account(
        seeds = [
            wormhole::SEED_PREFIX_POSTED_VAA,
            &vaa_hash
        ],
        bump,
        seeds::program = wormhole_program
    )]
    /// Verified Wormhole message account. The Wormhole program verified
    /// signatures and posted the account data here. Read-only.
    pub posted: Account<'info, wormhole::PostedVaa<TransferMessage>>,

    #[account(
        init,
        payer = signer,
        seeds = [
            Received::SEED_PREFIX,
            &posted.emitter_chain().to_le_bytes()[..],
            &posted.sequence().to_le_bytes()[..]
        ],
        bump,
        space = Received::LEN
    )]
    /// Received account. [`receive_message`](crate::receive_message) will
    /// deserialize the Wormhole message's payload and save it to this account.
    /// This account cannot be overwritten, and will prevent Wormhole message
    /// replay with the same sequence.
    pub received: Account<'info, Received>,

    /// System program.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeLockboxGovernor<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [Config::SEED_PREFIX],
        bump,
        space = Config::LEN
    )]
    /// Config account, which saves program data useful for other instructions.
    pub config: Account<'info, Config>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
#[instruction(vaa_hash: [u8; 32])]
pub struct TransferLockboxGovernor<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [Config::SEED_PREFIX],
        bump,
        constraint = config.verify(posted.emitter_address()) @ GovernorError::InvalidForeignEmitter,
        constraint = posted.emitter_chain() == config.chain
    )]
    /// Config account. Wormhole PDAs specified in the config are checked
    /// against the Wormhole accounts in this context. Read-only.
    pub config: Account<'info, Config>,

    // Wormhole program.
    //pub wormhole_program: Program<'info, wormhole::program::Wormhole>,
    /// CHECK: testing
    pub wormhole_program: UncheckedAccount<'info>,

    #[account(
        seeds = [
            wormhole::SEED_PREFIX_POSTED_VAA,
            &vaa_hash
        ],
        bump,
        seeds::program = wormhole_program
    )]
    /// Verified Wormhole message account. The Wormhole program verified
    /// signatures and posted the account data here. Read-only.
    pub posted: Account<'info, wormhole::PostedVaa<TransferMessage>>,

    #[account(
        init,
        payer = signer,
        seeds = [
            Received::SEED_PREFIX,
            &posted.emitter_chain().to_le_bytes()[..],
            &posted.sequence().to_le_bytes()[..]
        ],
        bump,
        space = Received::LEN
    )]
    /// Received account. [`receive_message`](crate::receive_message) will
    /// deserialize the Wormhole message's payload and save it to this account.
    /// This account cannot be overwritten, and will prevent Wormhole message
    /// replay with the same sequence.
    pub received: Account<'info, Received>,

    #[account(mut)]
    pub source_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub destination_account: Box<Account<'info, TokenAccount>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    /// System program.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferAllLockboxGovernor<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub source_account_sol: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub source_account_olas: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub destination_account_sol: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub destination_account_olas: Box<Account<'info, TokenAccount>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>
}

#[derive(Accounts)]
pub struct TransferTokenAccountsLockboxGovernor<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub source_account_sol: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub source_account_olas: Box<Account<'info, TokenAccount>>,

    /// CHECK: Check later
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>
}

#[derive(Accounts)]
pub struct ChangeUpgradeAuthorityLockboxGovernor<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: Check later
    #[account(mut)]
    pub program_to_update_authority: UncheckedAccount<'info>,

    /// CHECK: Check later
    #[account(mut)]
    pub program_data_to_update_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub config: Box<Account<'info, Config>>,

    /// CHECK: Check later
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UpgradeProgramLockboxGovernor<'info> {
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
    pub config: Box<Account<'info, Config>>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>
}
