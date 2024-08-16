use akashi_common::framework::akashi::{AkashiContext, AkashiError};

/// Shows servers
#[poise::command(slash_command, category = "Dev", owners_only, hide_in_help)]
pub async fn servers(ctx: AkashiContext<'_>) -> Result<(), AkashiError> {
    poise::builtins::servers(ctx).await?;

    Ok(())
}
