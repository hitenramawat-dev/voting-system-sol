use anchor_lang::{Bumps, prelude::*};

use crate::state::GlobalState;




#[derive(Accounts)]
pub struct Global<'info> {
    
    #[account(mut)]
    pub owner: Signer<'info>,


    #[account(
        init,
        payer = owner,
        seeds = [b"info_vault",owner.key().as_ref()],
        bump,
        space = 8 + GlobalState::INIT_SPACE,
    )]
    pub info_vault:Account<'info,GlobalState>,

    pub system_program:Program<'info,System>,

}


impl<'info>  Global<'info> {
    
    pub fn init_contract(&mut self,bumps:GlobalBumps) -> Result<()> {

        self.info_vault.set_inner(
            GlobalState { 
                owner:self.owner.key(), 
                active_poll_count: 0, 
                active_poll_addresses: Vec::new(), 
                total_polls_created:0, 
                bump:bumps.info_vault 
            });

        Ok(())
    }

    
}