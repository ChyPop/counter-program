use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const COUNTER_SEED: &str = "user";

#[program]
pub mod counter_program {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let acc = &mut ctx.accounts.counter_account;
        acc.state += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let acc = &mut ctx.accounts.counter_account;
        acc.state -= 1;
        Ok(())
    }
}

#[account]
pub struct UserCounter {
    pub state: u32,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space = 8 + 4, seeds = [
      signer.key().as_ref(),
      COUNTER_SEED.as_bytes()  
    ], bump)]
    pub counter_account: Account<'info, UserCounter>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds = [
        signer.key().as_ref(),
        COUNTER_SEED.as_bytes(),
    ], bump)]
    pub counter_account: Account<'info, UserCounter>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds = [
        signer.key().as_ref(),
        COUNTER_SEED.as_bytes(),
    ], bump)]
    pub counter_account: Account<'info, UserCounter>,
}