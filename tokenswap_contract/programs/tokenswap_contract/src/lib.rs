use anchor_lang::prelude::*;

declare_id!("AFPwsL4KdPowEBSbREcED8qboPYega1WZQ5xCrV2eqZK");

#[program]
pub mod tokenswap_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
