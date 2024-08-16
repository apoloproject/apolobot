#![feature(duration_millis_float)]
#![forbid(unsafe_code)]
use akashi_common::framework::akashi::AkashiData;

pub mod dev;
pub mod image;
pub mod util;

/// From [KonkonBot](https://github.com/KonkonBot/Konkon/blob/main/src/commands/mod.rs)
macro_rules! register_commands {
    ($($module:ident), *) => {
        {
            let mut cmds = Vec::new();
            $(
                cmds.extend($module::register());
            )*
            cmds
        }
    };
}

pub fn register_all_commands(
) -> Vec<poise::Command<AkashiData, Box<dyn std::error::Error + Send + Sync>>> {
    register_commands!(util, image, dev)
}
