use anchor_lang::prelude::*;









#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub owner: Pubkey,
    pub active_poll_count: u64,   
    #[max_len(1,000)]       
    pub active_poll_addresses: Vec<Pubkey>, 
    pub total_polls_created: u64,       
    pub bump: u8,
}


