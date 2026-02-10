use anchor_lang::prelude::*;

use crate::{error::PollingErrors, state::{Poll, PollStatus, VoteReceipt}};






#[derive(Accounts)]
#[instruction(poll_id: u64, option_index: u8)] 
pub struct Voter<'info>{

    #[account(mut)]
    pub voter:Signer<'info>,


    #[account(
        mut,
        seeds = [b"poll_state", poll.creator.as_ref(), poll_id.to_le_bytes().as_ref()],
        bump = poll.bump,
        constraint = poll.status == PollStatus::Voting @ PollingErrors::PollNotActive,
        constraint = poll.poll_id == poll_id @ PollingErrors::InvalidPollId,
    )]
    pub poll: Account<'info, Poll>,


    #[account(
        init,
        payer = voter,
        seeds = [b"vote_receipt", poll_id.to_le_bytes().as_ref(), voter.key().as_ref()],
        space = 8 + VoteReceipt::INIT_SPACE,
        bump,
    )]
    pub vote_receipt: Account<'info, VoteReceipt>,

    pub system_program:Program<'info,System>

}



impl Voter<'_> {
    
    pub fn vote(&mut self,poll_id:u64,option_index: u8,bumps:VoterBumps) -> Result<()> {


        require!(
            (option_index as usize) < self.poll.options.len(),
            PollingErrors::InvalidOptionIndex
        );

        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;


        require!(
            current_timestamp >= self.poll.start_time,
            PollingErrors::VotingNotStarted
        );
        
        require!(
            current_timestamp <= self.poll.end_time,
            PollingErrors::VotingEnded
        );

        self.vote_receipt.set_inner(
            VoteReceipt { 
                poll_id, 
                voter:self.voter.key(), 
                timestamp:current_timestamp, 
                bump:bumps.vote_receipt 
            });

            self.poll.options[option_index as usize].vote_count += 1;
            self.poll.total_votes += 1;

        Ok(())
    }
}
