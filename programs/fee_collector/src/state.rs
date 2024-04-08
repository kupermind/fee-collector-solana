use anchor_lang::prelude::*;

#[account]
pub struct FeeCollector {
  // Fee Collector bump
  pub bump: [u8; 1],
  // Foreign emitter address in bytes array
  pub foreign_emitter: [u8; 32],
  // Total SOL amount transferred
  pub total_sol_transferred: u64,
  // Total OLAS amount transferred
  pub total_olas_transferred: u64
}

impl FeeCollector {
  pub const LEN: usize = 8 + 1 + 32 + 8 * 2;

  // TODO: Make a constant
  pub fn seeds(&self) -> [&[u8]; 2] {
    [
      &b"fee_collector"[..],
      self.bump.as_ref()
    ]
  }

  // Initialize chain Id and foreign emitter address
  pub fn initialize(
    &mut self,
    bump: u8,
    chain: u16,
    address: [u8; 32],
  ) -> Result<()> {
    self.bump = [bump];
    self.total_sol_transferred = 0;
    self.total_olas_transferred = 0;

    Ok(())
  }

    /// Convenience method to check whether an address equals the one saved in this account.
    pub fn verify(&self, address: &[u8; 32]) -> bool {
        *address == self.foreign_emitter;
    }
}
