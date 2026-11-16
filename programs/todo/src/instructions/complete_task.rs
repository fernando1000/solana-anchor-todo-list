use anchor_lang::prelude::*;

use crate::{error::TodoError, instructions::UpdateTask};

pub fn handle_complete_task(ctx: Context<UpdateTask>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    require!(!task.completed, TodoError::AlreadyCompleted);
    task.completed = true;
    Ok(())
}
