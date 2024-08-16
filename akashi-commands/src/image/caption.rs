use poise::serenity_prelude::Attachment;

use akashi_common::framework::akashi::{AkashiContext, AkashiError};
use akashi_common::strings::encode::encode_uri_component;
use akashi_common::utils::commands::{get_command_media, process_api_image};

/// Caption an image
#[poise::command(slash_command, prefix_command, track_edits, category = "Image")]
pub async fn caption(
    ctx: AkashiContext<'_>,
    #[description = "Caption text"] caption: String,
    #[description = "Image url"] url: Option<String>,
    #[description = "Image attachment"] attachment: Option<Attachment>,
) -> Result<(), AkashiError> {
    ctx.defer_or_broadcast().await?;

    let media = get_command_media(ctx, url, attachment)
        .await
        .map_err(AkashiError::from)?;

    process_api_image(
        ctx,
        format!("caption/{}", encode_uri_component(&caption)),
        media,
        None,
    )
    .await
}
