use actix_web::http::header::{CacheControl, CacheDirective, ContentType};
use actix_web::{get, HttpResponse};

use akashi_common::structs::api::ExifResponse;

use crate::utils::http::{ErrorResponse, ImagePayload};

#[get("/exif")]
pub async fn exif(payload: ImagePayload) -> Result<HttpResponse, ErrorResponse> {
    let payload_data = payload.image.clone();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(CacheControl(vec![CacheDirective::MaxAge(360u32)]))
        .json(ExifResponse {
            width: payload_data.width(),
            height: payload_data.height(),
            size: payload_data.data.as_slice().len(),
        }))
}
