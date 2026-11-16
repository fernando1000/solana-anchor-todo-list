use anchor_lang::prelude::*;

use crate::{constants::*, state::Task};

#[derive(Accounts)]
pub struct DeleteTask<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
        close = owner,
        seeds = [TASK_SEED, owner.key().as_ref(), &task.task_id.to_le_bytes()],
        bump = task.bump
    )]
    pub task: Account<'info, Task>,
}

pub fn handle_delete_task(_ctx: Context<DeleteTask>) -> Result<()> {
    Ok(())
}
