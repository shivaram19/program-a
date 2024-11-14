use anchor_lang::prelude::*;
use program_b::program::ProgramB;

declare_id!("3UGGfoHvLugF1NkGd7g3b5WCYtyfuD7Cn8MZYWBTXfZw");

#[program]
pub mod program_a {
    use anchor_lang::solana_program::{program:: invoke_signed,  system_instruction};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from program A");
        let pda_address = ctx.accounts.pda_account.key();
        let signer_address = &ctx.accounts.signer.key();
        let instruction = &system_instruction::transfer(&pda_address, &signer_address, 1_000_000_000);
        let account_infos = [
            ctx.accounts.pda_account.to_account_info(), 
            ctx.accounts.signer.to_account_info(), 
            ctx.accounts.system_program.to_account_info()
        ];

        let signer_seeds : &[&[&[u8]]] = &[&[b"ackee",signer_address.as_ref(), &[ctx.bumps.pda_account]]];
        invoke_signed(instruction, &account_infos, signer_seeds)?;

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.program_b.to_account_info(),
            program_b::cpi::accounts::Initialize{ pda_account : ctx.accounts.pda_account.to_account_info() },
            signer_seeds
        );

        program_b::cpi::initialize(cpi_context)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK : ackee
    // AccountInfo is a commontype it doesn't have any checks 
    // if it's of type Account<'info> then it'll check and deserialize the data and put in the structure 
    #[account(
        mut,
        // payer = signer,
        seeds=[b"ackee",signer.key().as_ref()],
        //  we can use bump = 5, or directly put bump, what if program derived address will endup on curve for bump = 5 , then it'll be a problem , even though signer keeps on changing but we can't gaurantee that for bump = 5 it'll create a PDA that won't be in the curve , bump starts from 255 and cam go till 0
        bump
    )]
    pub pda_account : AccountInfo<'info>,
    //  we made the acccount of signer mutabkle , when system program calls function transfer it'll update the contents present in signer as well as PDA
    #[account(
        mut
    )]
    pub signer : Signer<'info>,
    pub system_program : Program<'info, System>,
    // we say anchor that pub program_b is of type Program , and precisely it's going to be ProgramB
    pub program_b : Program<'info, ProgramB>
}
