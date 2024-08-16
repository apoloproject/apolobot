use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use ril::{Dynamic, Font, Image, Rgba, TextAlign, TextLayout, TextSegment, WrapStyle};
use serde::Deserialize;

use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse, IntoHttpResponse};

#[derive(Deserialize)]
pub struct CaptionPayload {
    pub text: String,
}

#[get("/caption/{text}")]
pub async fn caption(
    payload: ImagePayload,
    caption_payload: web::Path<CaptionPayload>,
) -> Result<HttpResponse, ErrorResponse> {
    let text = caption_payload.into_inner().text;
    let (payload_width, payload_height) = payload.image.clone().dimensions();

    let mut caption_box =
        Image::<Dynamic>::new(payload_width, payload_height, Dynamic::from(Rgba::white()));
    let font_size = determine_font_size(&text, payload_width);
    let font = Font::open("akashi-api/src/routes/fonts/caption.otf", font_size)?;

    let mut base = Image::<Dynamic>::new(
        payload_width,
        payload_height * 2,
        Dynamic::from(Rgba::transparent()),
    );

    let (caption_x, caption_y) = caption_box.center();
    let caption_layout = TextLayout::<Dynamic>::new()
        .centered()
        .with_wrap(WrapStyle::Word)
        .with_width((payload_width as f32 / 1.5) as u32)
        .with_position(caption_x, caption_y)
        .with_align(TextAlign::Center)
        .with_segment(&TextSegment::new(&font, text, Dynamic::from(Rgba::black())));

    caption_box.draw(&caption_layout);

    base.paste(0, 0, &caption_box);
    base.paste(0, payload_height, &payload.image.clone());

    let response = Arc::new(ImageResponse::new(base, payload.format));

    response.into_http_response().await
}

pub fn determine_font_size(text: &str, image_width: u32) -> f32 {
    let text_length = text.chars().count();

    // Minimum readable font size
    let min_font_size = 20.0;

    // Calculate the maximum font size that fits the image width
    let max_font_size = image_width as f32 / text_length as f32;

    // Ensure the font size is at least the minimum readable size
    let font_size = f32::max(min_font_size, max_font_size);

    font_size.round()
}
