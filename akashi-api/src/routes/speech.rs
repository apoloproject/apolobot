use std::sync::Arc;

use actix_web::{get, HttpResponse};
use ril::{Image, Rgba};

use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse, IntoHttpResponse};

#[get("/speech")]
pub async fn speech(payload: ImagePayload) -> Result<HttpResponse, ErrorResponse> {
    let mut balloon = Image::<Rgba>::open("akashi-api/src/routes/images/speech.png")
        .map_err(ErrorResponse::from)?;

    let balloon_height = 150;

    let (payload_width, payload_height) = payload.image.clone().dimensions();

    balloon.resize(payload_width, balloon_height, ril::ResizeAlgorithm::Nearest);

    let mut base = Image::<Rgba>::new(
        payload_width,
        payload_height + balloon_height,
        Rgba::black(),
    );

    base.paste(0, 0, &balloon);
    base.paste(0, balloon_height, &payload.image.clone().into());

    let response = Arc::new(ImageResponse::new(base.into(), payload.format));

    response.into_http_response().await
}
