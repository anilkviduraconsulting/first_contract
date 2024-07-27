use anchor_lang::prelude::*;

declare_id!("AsrEFubxdx6Nvdft6nyvdVtzKb2YtVSCUiYDSpUAJvWA");

#[program]
pub mod first_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
