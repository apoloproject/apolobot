use poise::serenity_prelude::Attachment;

use akashi_common::framework::akashi::{AkashiContext, AkashiError};
use akashi_common::utils::commands::{get_command_media, process_api_image};

/// LOL LOL LOL LOL
#[poise::command(slash_command, prefix_command, track_edits, category = "Image")]
pub async fn speech(
    ctx: AkashiContext<'_>,
    #[description = "Image url"] url: Option<String>,
    #[description = "Image attachment"] attachment: Option<Attachment>,
) -> Result<(), AkashiError> {
    let media = get_command_media(ctx, url, attachment)
        .await
        .map_err(AkashiError::from)?;

    process_api_image(ctx, "speech".to_string(), media, None).await?;
    Ok(())
}
