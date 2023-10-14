pub mod errors;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("2Ta8JBMNnWoddVeHBP3xZbZMgyPbbV4wvCn9r8EsmQbm");

#[program]
pub mod webhook_program {
    pub use super::*;

    pub fn webhook_create<'info>(
        ctx: Context<WebhookCreate>,
        body: Vec<u8>,
        headers: std::collections::HashMap<String, String>,
        id: Vec<u8>,
        method: HttpMethod,
        url: String,
    ) -> Result<()> {
        webhook_create::handler(ctx, body, headers, id, method, url)
    }

    pub fn webhook_respond<'info>(ctx: Context<WebhookRespond>) -> Result<()> {
        webhook_respond::handler(ctx)
    }
}
