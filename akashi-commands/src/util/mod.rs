use akashi_common::framework::akashi::{AkashiData, AkashiError};

mod help;
mod ping;
mod stats;

pub fn register() -> Vec<poise::Command<AkashiData, AkashiError>> {
    vec![help::help(), stats::stats(), ping::ping()]
}
