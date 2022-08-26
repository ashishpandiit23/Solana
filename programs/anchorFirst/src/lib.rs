use anchor_lang::prelude::*;

declare_id!("nAAoDaKw9XQF9QgpyT6RyTvHPhdwd1fqkPZTM8LiW7H");

#[program]
pub mod anchor_first {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,_bump:u8) -> Result<()> {
        msg!("PDA initialised successfully");
        

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_bump : u8)]
pub struct Initialize<'info>{
    #[account(init,seeds=[owner.key.as_ref()],bump,payer=owner,space=100)]
    pub account_user:Account<'info,AccountUser>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[account]
pub struct AccountUser {
    pub user: String,
        }
