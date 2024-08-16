use poise::serenity_prelude;

use akashi_img::parse_message_media;

use crate::framework::akashi::{AkashiData, AkashiError};

pub async fn event_handler(
    framework: poise::FrameworkContext<'_, AkashiData, AkashiError>,
    event: &serenity_prelude::FullEvent,
) -> Result<(), AkashiError> {
    let data = framework.user_data;

    match event {
        serenity_prelude::FullEvent::Ready { data_about_bot, .. } => {
            #[cfg(debug_assertions)]
            println!("{:?}", data_about_bot);

            #[cfg(not(debug_assertions))]
            println!("Logged in {}!", data_about_bot.user.name);
        }
        serenity_prelude::FullEvent::Message { new_message } => {
            if let Some(media) = parse_message_media(new_message) {
                let cache = data.custom_cache.lock().await;
                cache.images.add(new_message.channel_id, media)
            }
        }
        _ => {}
    };

    Ok(())
}
