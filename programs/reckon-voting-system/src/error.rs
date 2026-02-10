use anchor_lang::prelude::*;





#[error_code]
pub enum PollingErrors {
    #[msg("voting optins needs to be atleast two")]
    NotEnoughOptions,

    #[msg("end time and start time can't be same")]
    InvalidTimeRange,

    #[msg("title can't be empty")]
    EmptyTitle,

    #[msg("i think poll has expired ! ")]
    PollNotActive,

    #[msg("no poll exists with this id")]
    InvalidPollId,

    #[msg("option doesn't exist")]
    InvalidOptionIndex,

    #[msg("voting has not started yet ")]
    VotingNotStarted,


    #[msg("Voting has closed")]
    VotingEnded
}