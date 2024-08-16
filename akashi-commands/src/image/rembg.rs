use dashmap::DashMap;
use poise::serenity_prelude::Attachment;

use akashi_common::framework::akashi::{AkashiContext, AkashiError};
use akashi_common::utils::commands::{get_command_media, process_api_image};

/// Remove the background from an image
#[poise::command(slash_command, prefix_command, track_edits, category = "Image")]
pub async fn rembg(
    ctx: AkashiContext<'_>,
    #[description = "Background color tolerance"] tolerance: Option<u8>,
    #[description = "Image attachment"] attachment: Option<Attachment>,
    #[description = "Image url"] url: Option<String>,
) -> Result<(), AkashiError> {
    ctx.defer_or_broadcast().await?;

    let media = get_command_media(ctx, url, attachment)
        .await
        .map_err(AkashiError::from)?;

    let query = DashMap::new();

    query.insert("t".to_string(), tolerance.unwrap_or(30).to_string());

    process_api_image(ctx, "rmbg".to_string(), media, Some(query)).await?;
    Ok(())
}
