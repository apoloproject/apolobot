use akashi_common::framework::akashi::{AkashiContext, AkashiError};

/// Flushes custom cache
#[poise::command(prefix_command, category = "Dev", owners_only, hide_in_help)]
pub async fn cflush(ctx: AkashiContext<'_>) -> Result<(), AkashiError> {
    let cache = ctx.data().custom_cache.lock().await;
    let images = cache.images.clone();

    images.flush();

    ctx.reply("Flushed cache").await?;
    Ok(())
}
