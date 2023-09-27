use anchor_lang::prelude::*;

declare_id!("D9SsfPjbcNcKAWS8vBWBH9MCV7mcGaGK74KYx9umWAu9");

#[program]
pub mod hw13 {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account = &mut ctx.accounts.my_account;
        account.balance = 100;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyAccount {
    pub balance: u64,
}
