use std::sync::Arc;

use actix_web::{get, HttpResponse};
use ril::{Dynamic, Frame, Image, ImageSequence, Rgba};

use crate::utils::http::{AnimatedResponse, ErrorResponse, ImagePayload, IntoHttpResponse};

#[get("/spin")]
pub async fn spin(payload: ImagePayload) -> Result<HttpResponse, ErrorResponse> {
    let mut sequence = ImageSequence::<Dynamic>::new();
    let frame_count = 3;

    // for every frame, rotate the image by 22.5 degrees
    for i in 0..frame_count {
        let angle = 90 * i;
        let rotated = payload.image.clone().rotated(angle);
        sequence.push_frame(Frame::from_image(Image::<Rgba>::from(rotated).into()))
    }

    let response = Arc::new(AnimatedResponse::new(sequence));

    response.into_http_response().await
}
