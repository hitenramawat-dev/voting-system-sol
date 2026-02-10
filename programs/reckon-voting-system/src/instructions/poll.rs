use anchor_lang::prelude::*;

use crate::{
    errors::PollingErrors,
    state::{GlobalState, Poll, PollData, PollStatus},
};

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"info_vault",global.owner.as_ref(),],
        bump  = global.bump,
    )]
    pub global: Account<'info, GlobalState>,

    #[account(
        init,
        payer = owner,
        seeds = [b"poll_state",owner.key().as_ref(),global.total_polls_created.to_le_bytes().as_ref()],
        space = 8 + Poll::INIT_SPACE,
        bump,
    )]
    pub poll_pda: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

impl CreatePoll<'_> {
    pub fn create_poll(
        &mut self,
        poll_creator_data: PollData,
        bumps: &CreatePollBumps,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        require!(
            poll_creator_data.start_time >= current_time,
            PollingErrors::StartTimeInPast
        );

        require!(
            poll_creator_data.end_time > current_time,
            PollingErrors::EndTimeInPast
        );

        require!(
            poll_creator_data.options.len() >= 2,
            PollingErrors::NotEnoughOptions
        );

        require!(
            poll_creator_data.end_time > poll_creator_data.start_time,
            PollingErrors::InvalidTimeRange
        );

        require!(
            !poll_creator_data.title.is_empty(),
            PollingErrors::EmptyTitle
        );

        require!(
            !poll_creator_data.description.is_empty(),
            PollingErrors::EmptyDescription
        );


        require!(
            poll_creator_data.options.iter().all(|opt| !opt.name.is_empty()),
            PollingErrors::EmptyOptionName
        );

        require!(
            self.global.active_poll_addresses.len() < 1000, // or whatever your max_len is
            PollingErrors::TooManyActivePolls
        );

        let poll_id = self.global.total_polls_created;

        self.poll_pda.set_inner(Poll {
            poll_id: poll_id,
            creator: self.owner.key(),
            title: poll_creator_data.title,
            description: poll_creator_data.description,
            options: poll_creator_data.options,
            status: PollStatus::BufferTime,
            start_time: poll_creator_data.start_time,
            end_time: poll_creator_data.end_time,
            total_votes: 0,
            bump: bumps.poll_pda,
        });

        self.global.total_polls_created += 1;
        self.global.active_poll_count += 1;

        require!(
            self.global.active_poll_addresses.len() < 1000, // or whatever your max_len is
            PollingErrors::TooManyActivePolls
        );

        self.global.active_poll_addresses.push(self.poll_pda.key());

        Ok(())
    }
}
