//! TipVault — BagsTip MVP Solana program (stub).
//!
//! Replace the placeholder `initialize` with create tip / release instructions per `TASKLIST.md`.

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bagstip_tipvault {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
