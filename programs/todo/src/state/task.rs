use anchor_lang::prelude::*;

use crate::constants::MAX_DESCRIPTION_LEN;

#[account]
#[derive(InitSpace)]
pub struct Task {
    pub owner: Pubkey,
    pub task_id: u64,
    #[max_len(MAX_DESCRIPTION_LEN)]
    pub description: String,
    pub completed: bool,
    pub bump: u8,
}
