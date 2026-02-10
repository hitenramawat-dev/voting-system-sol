use anchor_lang::prelude::*;

mod instructions;
mod state;
mod error;


declare_id!("A3TBFpv3KRKpSHowtstALX7PFPWQ1Kthp2wnHjgtuExP");






#[program]
pub mod reckon_voting_system {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }


}



#[derive(Accounts)]
pub struct Initialize {}
