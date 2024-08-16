use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use ril::{Image, TrueColor};
use serde::Deserialize;

use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse, IntoHttpResponse};

#[derive(Deserialize)]
pub struct OpacityPayload {
    pub value: f32,
}

#[get("/opacity/{value}")]
pub async fn opacity(
    payload: ImagePayload,
    opacity_payload: web::Path<OpacityPayload>,
) -> Result<HttpResponse, ErrorResponse> {
    let value = opacity_payload.value;

    if !(0.0..=1.0).contains(&value) {
        return Err(ErrorResponse {
            message: "opacity must be between 0 and 1".to_string(),
        });
    }

    let image = payload.image.clone();
    let alpha_clamped = opacity_payload.value.clamp(0.0, 1.0);

    let result = image.map_pixels(|pixel| {
        let mut rgba = pixel.into_rgba();
        rgba.a = (alpha_clamped * 255.0).round() as u8;
        rgba
    });

    let response = Arc::new(ImageResponse::new(Image::from(result), payload.format));

    response.into_http_response().await
}
