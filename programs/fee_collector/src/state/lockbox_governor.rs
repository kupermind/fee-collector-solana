use anchor_lang::prelude::*;

#[account]
pub struct LockboxGovernor {
  // Fee Collector bump
  pub bump: [u8; 1],
  // Foreign chain Id
  pub chain: u16,
  // Foreign emitter address in bytes array
  pub foreign_emitter: [u8; 32],
  // Total SOL amount transferred
  pub total_sol_transferred: u64,
  // Total OLAS amount transferred
  pub total_olas_transferred: u64
}

impl LockboxGovernor {
  pub const LEN: usize = 8 + 1 + 2 + 32 + 8 * 2;

  // TODO: Make a constant
  pub fn seeds(&self) -> [&[u8]; 2] {
    [
      &b"lockbox_governor"[..],
      self.bump.as_ref()
    ]
  }

  // Initialize chain Id and foreign emitter address
  pub fn initialize(
    &mut self,
    bump: u8,
    chain: u16,
    foreign_governor: [u8; 32],
  ) -> Result<()> {
    self.bump = [bump];
    self.chain = chain;
    self.foreign_emitter = foreign_governor;
    self.total_sol_transferred = 0;
    self.total_olas_transferred = 0;

    Ok(())
  }

    /// Convenience method to check whether an address equals the one saved in this account.
    pub fn verify(&self, check_address: &[u8; 32]) -> bool {
        return *check_address == self.foreign_emitter;
    }
}
