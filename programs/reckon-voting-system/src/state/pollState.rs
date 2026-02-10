use anchor_lang::prelude::*;



#[derive(AnchorSerialize, AnchorDeserialize, Clone,InitSpace)]
pub struct PollOption {
    pub id:i16,
    #[max_len(50)] 
    pub name: String,
    pub vote_count: u64,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum PollStatus {
    BufferTime,
    Voting,    
    Closed,
}



#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,                    
    pub creator: Pubkey,                 
    #[max_len(25)]
    pub title: String,                   
    #[max_len(100)]
    pub description: String,             
    #[max_len(10)]  
    pub options: Vec<PollOption>,        
    pub status: PollStatus,              
    pub start_time: i64,                 
    pub end_time: i64,                   
    pub total_votes: u64,                
    pub bump: u8,                        
}




#[derive(AnchorDeserialize,AnchorSerialize)]
pub struct PollData {                                 
    //#[max_len(25)]
    pub title: String,                   
    //#[max_len(100)]
    pub description: String,             
    //#[max_len(10)]  
    pub options: Vec<PollOption>,                      
    pub start_time: i64,                 
    pub end_time: i64,                                   
}