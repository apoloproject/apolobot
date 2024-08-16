use poise::serenity_prelude::Attachment;

use akashi_common::framework::akashi::{AkashiContext, AkashiError};
use akashi_common::utils::commands::{get_command_media, process_api_image};

/// Spins an image (3 frames since I can't do more idk why)
#[poise::command(slash_command, prefix_command, track_edits, category = "Image")]
pub async fn spin(
    ctx: AkashiContext<'_>,
    #[description = "Image url"] url: Option<String>,
    #[description = "Image attachment"] attachment: Option<Attachment>,
) -> Result<(), AkashiError> {
    let media = get_command_media(ctx, url, attachment)
        .await
        .map_err(AkashiError::from)?;

    process_api_image(ctx, "spin".to_string(), media, None).await?;
    Ok(())
}