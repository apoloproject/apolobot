use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use ril::{Image, ImageFormat, Rgba};
use serde::Deserialize;

use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse, IntoHttpResponse};

#[derive(Deserialize)]
pub struct RmbgPayload {
    pub t: Option<u8>,
}

#[get("/rmbg")]
pub async fn rmbg(
    payload: ImagePayload,
    rmbg_payload: web::Query<RmbgPayload>,
) -> Result<HttpResponse, ErrorResponse> {
    let mut image = payload.image.clone();

    image.set_format(ImageFormat::Png);

    let background_color = detect_background_color(&image.clone().into());
    let tolerance = if let Some(tolerance) = rmbg_payload.t {
        match tolerance {
            10..=30 => tolerance,
            _ => 30,
        }
    } else {
        30
    };

    let image = remove_background(&image.into(), background_color, tolerance);

    let response = Arc::new(ImageResponse::new(image.into(), payload.format));

    response.into_http_response().await
}

fn detect_background_color(image: &Image<Rgba>) -> Rgba {
    let (width, height) = image.dimensions();

    let mut edge_pixels = Vec::new();

    for x in 0..width {
        edge_pixels.push(image.get_pixel(x, 0));
        edge_pixels.push(image.get_pixel(x, height - 1));
    }

    for y in 0..height {
        edge_pixels.push(image.get_pixel(0, y));
        edge_pixels.push(image.get_pixel(width - 1, y));
    }

    let mut color_count = HashMap::new();
    for pixel in edge_pixels {
        *color_count.entry(pixel).or_insert(0) += 1;
    }

    *color_count
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0
        .unwrap()
}

fn remove_background(img: &Image<Rgba>, background_color: Rgba, tolerance: u8) -> Image<Rgba> {
    img.clone().map_pixels(|pixel| {
        if is_color_within_tolerance(&pixel, &background_color, tolerance) {
            Rgba::transparent()
        } else {
            pixel
        }
    })
}

fn is_color_within_tolerance(pixel: &Rgba, background_color: &Rgba, tolerance: u8) -> bool {
    let dr = (pixel.r as i16 - background_color.r as i16).unsigned_abs() as u8;
    let dg = (pixel.g as i16 - background_color.g as i16).unsigned_abs() as u8;
    let db = (pixel.b as i16 - background_color.b as i16).unsigned_abs() as u8;

    dr <= tolerance && dg <= tolerance && db <= tolerance
}
