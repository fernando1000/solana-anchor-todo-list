use anchor_lang::prelude::*;

#[error_code]
pub enum TodoError {
    #[msg("The description is longer than 280 bytes")]
    DescriptionTooLong,
    #[msg("The description is empty")]
    DescriptionEmpty,
    #[msg("The task is already completed")]
    AlreadyCompleted,
}
