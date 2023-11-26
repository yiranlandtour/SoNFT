use anchor_lang::prelude::*;

declare_id!("2gJwfKpPfhn1NUL4iHw5y4Di5rDtVuQgoVmAP7zN9oo5");

#[program]
pub mod contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
