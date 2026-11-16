pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("21PCzXqHSYU8njPER9o7FQfT38512wrSrPpdy8fMbSa9");

#[program]
pub mod todo {
    use super::*;

    pub fn create_task(ctx: Context<CreateTask>, task_id: u64, description: String) -> Result<()> {
        handle_create_task(ctx, task_id, description)
    }

    pub fn update_task(ctx: Context<UpdateTask>, description: String) -> Result<()> {
        handle_update_task(ctx, description)
    }

    pub fn complete_task(ctx: Context<UpdateTask>) -> Result<()> {
        handle_complete_task(ctx)
    }

    pub fn delete_task(ctx: Context<DeleteTask>) -> Result<()> {
        handle_delete_task(ctx)
    }
}
