use akashi_common::framework::akashi::{AkashiContext, AkashiError};

/// Show Akashi's ping
///
/// If the bot has just connected, the value is zero.
#[poise::command(slash_command, prefix_command, category = "Util")]
pub async fn ping(ctx: AkashiContext<'_>) -> Result<(), AkashiError> {
    ctx.reply(format!(
        "From Shard #{}: {:.1}ms",
        ctx.serenity_context().shard_id,
        ctx.ping().await.as_millis_f32()
    ))
    .await?;
    Ok(())
}
