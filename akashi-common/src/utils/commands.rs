use dashmap::DashMap;
use poise::serenity_prelude::{Attachment, CreateAttachment};
use poise::{CreateReply, ReplyHandle};
use reqwest::header::CONTENT_TYPE;
use reqwest::Response;

use akashi_img::{get_cached_media, parse_message_media};

use crate::framework::akashi::{AkashiContext, AkashiError};
use crate::strings::encode::encode_uri_component;

/// Get first available media for a command
///
/// Order: Url option, attachment option, referenced message, cached media
pub async fn get_command_media(
    ctx: AkashiContext<'_>,
    url: Option<String>,
    attachment: Option<Attachment>,
) -> Result<String, AkashiError> {
    if let Some(url) = url {
        return Ok(url);
    }

    if let Some(attachment) = attachment {
        return Ok(attachment.url);
    }

    if let AkashiContext::Prefix(prefix) = ctx {
        if let Some(referenced) = prefix.msg.clone().referenced_message {
            if let Some(media) = parse_message_media(&referenced) {
                return Ok(media);
            }
        }
    }

    let cache = ctx.data().custom_cache.lock().await;
    if let Some(cached) = get_cached_media(cache, ctx.channel_id()) {
        return Ok(cached);
    }

    Err(AkashiError::from("No media found"))
}

pub async fn fetch_media(reqwest: &reqwest::Client, url: String) -> Result<Response, AkashiError> {
    reqwest.get(url).send().await.map_err(AkashiError::from)
}

pub async fn parse_media_response(response: Response) -> Result<(Vec<u8>, String), AkashiError> {
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .ok_or_else(|| AkashiError::from("Content-Type header missing"))?
        .to_str()
        .map_err(AkashiError::from)?
        .to_string();

    let bytes = response
        .bytes()
        .await
        .map_err(|e| AkashiError::from(e.to_string()))?
        .to_vec();

    Ok((bytes, content_type))
}

pub async fn send_image_with_time(
    ctx: AkashiContext<'_>,
    file: CreateAttachment,
    time: f32,
) -> Result<ReplyHandle, AkashiError> {
    ctx.send(
        CreateReply::default()
            .attachment(file.clone())
            .content(format!("-# *{time:.2}ms*")),
    )
    .await
    .map_err(AkashiError::from)
}

pub async fn process_api_image(
    ctx: AkashiContext<'_>,
    endpoint: String,
    url: String,
    extra_queries: Option<DashMap<String, String>>,
) -> Result<(), AkashiError> {
    let start_time = std::time::Instant::now();
    let reqwest = ctx.data().reqwest.clone();

    let base_url = format!("{}/api/{}", ctx.data().api, endpoint);
    let req_url = format!(
        "{}?url={}{}",
        base_url,
        encode_uri_component(&url),
        if let Some(extra_queries) = extra_queries {
            format!(
                "&{}",
                extra_queries
                    .iter()
                    .map(|v| format!("{}={}", v.key(), v.value()))
                    .collect::<Vec<String>>()
                    .join("&")
            )
        } else {
            "".to_string()
        }
    );

    let response = fetch_media(&reqwest, req_url).await?;
    let (bytes, content_type) = parse_media_response(response).await?;

    println!("{}", size_of_val(&bytes));

    let format = if let Some(format) = content_type.split("/").last() {
        format.to_string()
    } else {
        return Err(AkashiError::from("Invalid content type"));
    };

    let attachment = CreateAttachment::bytes(
        bytes,
        format!("akashi_{}.{:?}", ctx.invoked_command_name(), format),
    );

    send_image_with_time(ctx, attachment, start_time.elapsed().as_millis_f32()).await?;
    Ok(())
}
