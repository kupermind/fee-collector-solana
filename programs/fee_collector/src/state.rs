use anchor_lang::prelude::*;

#[account]
pub struct FeeCollector {
  // Fee Collector bump
  pub bump: [u8; 1],
  // Total SOL amount transferred
  pub total_sol_transferred: u64,
  // Total OLAS amount transferred
  pub total_olas_transferred: u64
}

impl FeeCollector {
  pub const LEN: usize = 8 + 1 + 8 * 2;

  pub fn seeds(&self) -> [&[u8]; 2] {
    [
      &b"fee_collector"[..],
      self.bump.as_ref()
    ]
  }

  pub fn initialize(
    &mut self,
    bump: u8
  ) -> Result<()> {
    self.bump = [bump];
    self.total_sol_transferred = 0;
    self.total_olas_transferred = 0;

    Ok(())
  }
}
