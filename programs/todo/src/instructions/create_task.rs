use anchor_lang::prelude::*;

use crate::{constants::*, error::TodoError, state::Task};

#[derive(Accounts)]
#[instruction(task_id: u64)]
pub struct CreateTask<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Task::INIT_SPACE,
        seeds = [TASK_SEED, owner.key().as_ref(), &task_id.to_le_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_task(
    ctx: Context<CreateTask>,
    task_id: u64,
    description: String,
) -> Result<()> {
    check_description(&description)?;

    let task = &mut ctx.accounts.task;
    task.owner = ctx.accounts.owner.key();
    task.task_id = task_id;
    task.description = description;
    task.completed = false;
    task.bump = ctx.bumps.task;
    Ok(())
}

pub fn check_description(description: &str) -> Result<()> {
    require!(!description.is_empty(), TodoError::DescriptionEmpty);
    require!(
        description.len() <= MAX_DESCRIPTION_LEN,
        TodoError::DescriptionTooLong
    );
    Ok(())
}
