use anchor_lang::prelude::*;

declare_id!("D9SsfPjbcNcKAWS8vBWBH9MCV7mcGaGK74KYx9umWAu9");

#[program]
pub mod hw13 {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing account");
        let account = &mut ctx.accounts.my_account;
        account.balance = 100;
        Ok(())
    }
    pub fn increment100(ctx: Context<Increment100>) -> Result<()> {
        msg!("Incrementing account by 100");
        let account = &mut ctx.accounts.my_account;
        if account.balance+100 > 1000{
            return err!(IncrementError::IncrementedBalanceOver1000);
        }
        else{
            account.balance += 100;
            Ok(())
        }
    }   
}

#[error_code]
pub enum IncrementError {
    #[msg("Cannot increment by 100")]
    IncrementedBalanceOver1000
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Increment100<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub balance: u64,
}
