use poise::serenity_prelude::Attachment;

use akashi_common::framework::akashi::{AkashiContext, AkashiError};
use akashi_common::utils::commands::{get_command_media, process_api_image};

/// Change an image ALPHA channel
#[poise::command(slash_command, prefix_command, track_edits, category = "Image")]
pub async fn opacity(
    ctx: AkashiContext<'_>,
    #[description = "New opacity value"] value: f32,
    #[description = "Image url"] url: Option<String>,
    #[description = "Image attachment"] attachment: Option<Attachment>,
) -> Result<(), AkashiError> {
    let media = get_command_media(ctx, url, attachment)
        .await
        .map_err(AkashiError::from)?;

    process_api_image(ctx, format!("opacity/{value}"), media, None).await
}
