use anchor_lang::prelude::*;

use crate::{constants::*, instructions::check_description, state::Task};

#[derive(Accounts)]
pub struct UpdateTask<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
        seeds = [TASK_SEED, owner.key().as_ref(), &task.task_id.to_le_bytes()],
        bump = task.bump
    )]
    pub task: Account<'info, Task>,
}

pub fn handle_update_task(ctx: Context<UpdateTask>, description: String) -> Result<()> {
    check_description(&description)?;
    ctx.accounts.task.description = description;
    Ok(())
}
