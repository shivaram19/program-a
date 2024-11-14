use anchor_lang::prelude::*;

declare_id!("3LPhBzNyxtXy4E6pJzzzsz2nTEv6frwwPVwBQBbuQxmn");

#[program]
pub mod program_b {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from Program B");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub pda_account : Signer<'info>
}
