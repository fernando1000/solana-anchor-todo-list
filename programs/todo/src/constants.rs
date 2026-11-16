use anchor_lang::prelude::*;

#[constant]
pub const TASK_SEED: &[u8] = b"task";

pub const MAX_DESCRIPTION_LEN: usize = 280;
