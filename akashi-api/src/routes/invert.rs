use std::sync::Arc;

use actix_web::{get, HttpResponse};
use ril::{Dynamic, Rgb};

use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse, IntoHttpResponse};

#[get("/invert")]
pub async fn invert(payload: ImagePayload) -> Result<HttpResponse, ErrorResponse> {
    let response = Arc::new(ImageResponse::new(
        (!ril::Image::<Rgb>::from(payload.image.clone())).convert::<Dynamic>(),
        payload.format,
    ));

    response.into_http_response().await
}
