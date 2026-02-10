use anchor_lang::prelude::*;







#[account]
#[derive(InitSpace)]
pub struct VoteReceipt {
    pub poll_id: u64,
    pub voter: Pubkey,
    pub timestamp: i64,
    pub bump: u8,
}