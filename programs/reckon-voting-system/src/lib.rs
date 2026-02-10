use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;


pub use instructions::*;
pub use state::*;
pub use errors::*;

declare_id!("A3TBFpv3KRKpSHowtstALX7PFPWQ1Kthp2wnHjgtuExP");


#[program]
pub mod reckon_voting_system {
    use super::*;

    pub fn init_contract(ctx: Context<Global>) -> Result<()> {
        ctx.accounts.init_contract(&ctx.bumps)
    }

    pub fn init_poll(ctx: Context<CreatePoll>, poll_creator_data: PollData) -> Result<()> {
        ctx.accounts.create_poll(poll_creator_data, &ctx.bumps)
    }

    pub fn press_vote(ctx: Context<Voter>, poll_id: u64, option_index: u8) -> Result<()> {
        ctx.accounts.vote(poll_id, option_index, &ctx.bumps)
    }
}

