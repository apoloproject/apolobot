use akashi_common::framework::akashi::{AkashiData, AkashiError};

mod caption;
mod convert;
mod globe;
mod invert;
mod opacity;
mod rembg;
mod speech;
mod spin;

pub fn register() -> Vec<poise::Command<AkashiData, AkashiError>> {
    vec![
        invert::invert(),
        caption::caption(),
        convert::convert(),
        speech::speech(),
        opacity::opacity(),
        rembg::rembg(),
        globe::globe(),
        spin::spin(),
    ]
}
