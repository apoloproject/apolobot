use akashi_common::framework::akashi::{AkashiData, AkashiError};

mod cflush;
mod servers;

pub fn register() -> Vec<poise::Command<AkashiData, AkashiError>> {
    vec![cflush::cflush(), servers::servers()]
}
