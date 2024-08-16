use poise::futures_util::lock::MutexGuard;
use poise::serenity_prelude::{ChannelId, Message};

use akashi_cache::AkashiCustomCache;

pub fn parse_message_media(msg: &Message) -> Option<String> {
    if let Some(attachment) = msg.attachments.first() {
        return Some(attachment.proxy_url.clone());
    }

    if let Some(embed) = msg.embeds.first() {
        if let Some(image) = &embed.image {
            return Some(image.proxy_url.clone().unwrap());
        } else if let Some(thumbnail) = &embed.thumbnail {
            return Some(thumbnail.proxy_url.clone().unwrap());
        }
    }

    if let Some(sticker) = msg.sticker_items.first() {
        return sticker.image_url().clone();
    }

    None
}

pub fn get_cached_media(
    cache: MutexGuard<AkashiCustomCache>,
    channel_id: ChannelId,
) -> Option<String> {
    if let Some(url) = cache.images.get(channel_id) {
        return Some(url.to_string());
    }

    None
}
