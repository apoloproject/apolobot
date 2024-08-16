use std::f32::consts::PI;
use std::sync::Arc;

use actix_web::{get, HttpResponse};
use ril::{Dynamic, Image, Rgba, TrueColor};

use crate::utils::http::{ErrorResponse, ImagePayload, ImageResponse, IntoHttpResponse};

#[get("/globe")]
pub async fn globe(payload: ImagePayload) -> Result<HttpResponse, ErrorResponse> {
    let image = payload.image.clone();

    let response = Arc::new(ImageResponse::new(image_to_globe(&image)?, payload.format));

    response.into_http_response().await
}

fn image_to_globe(image: &Image<Dynamic>) -> Result<Image<Dynamic>, ril::Error> {
    let (width, height) = image.dimensions();
    let mut new_image = Image::new(width, height, Rgba::new(0, 0, 0, 0));

    for y in 0..height {
        for x in 0..width {
            // Normalize x and y to the range -1 to 1
            let nx = 2.0 * (x as f32 / width as f32) - 1.0;
            let ny = 2.0 * (y as f32 / height as f32) - 1.0;

            // Calculate the spherical coordinates
            let r = (nx * nx + ny * ny).sqrt();
            if r <= 1.0 {
                let theta = r * PI;
                let phi = nx.atan2(ny);

                // Map back to 2D coordinates
                let u = (theta.sin() * phi.cos() + 1.0) * 0.5;
                let v = (theta.sin() * phi.sin() + 1.0) * 0.5;

                let src_x = (u * width as f32) as u32;
                let src_y = (v * height as f32) as u32;

                if src_x < width && src_y < height {
                    let pixel = image.get_pixel(src_x, src_y).unwrap();
                    new_image.set_pixel(x, y, pixel.into_rgba());
                }
            }
        }
    }

    Ok(new_image.into())
}
