pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5o3NtP3cm4YbGw6oFXg7fZK2TiZqHnuDGfVxdZrv4CwB");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        initialize_config_handler(ctx)
    }
}
