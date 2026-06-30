use anchor_lang::prelude::*;
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

// have to declare id otherwise instructions won't locate: Anchor.toml -> program_id
declare_id!("9bLFXWQAEm8GAkNX4NWashck88JJVtnFxyGAy9Gc5xe4");
