use poise::serenity_prelude::Attachment;
use poise::ChoiceParameter;

use akashi_common::framework::akashi::{AkashiContext, AkashiError};
use akashi_common::strings::encode::encode_uri_component;
use akashi_common::utils::commands::{get_command_media, process_api_image};

#[derive(ChoiceParameter)]
enum Mimetypes {
    #[name = "png"]
    Png,
    #[name = "jpeg"]
    Jpeg,
    #[name = "webp"]
    Webp,
    #[name = "gif"]
    Gif,
}

/// Change an image mime-type
#[poise::command(slash_command, prefix_command, track_edits, category = "Image")]
pub async fn convert(
    ctx: AkashiContext<'_>,
    #[description = "New mime-type"] mime: Mimetypes,
    #[description = "Image url"] url: Option<String>,
    #[description = "Image attachment"] attachment: Option<Attachment>,
) -> Result<(), AkashiError> {
    let media = get_command_media(ctx, url, attachment)
        .await
        .map_err(AkashiError::from)?;

    process_api_image(
        ctx,
        format!("convert/{}", encode_uri_component(mime.name())),
        media,
        None,
    )
    .await
}
