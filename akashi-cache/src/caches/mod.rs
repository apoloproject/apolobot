use std::hash::RandomState;

use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use poise::serenity_prelude::ChannelId;

pub struct ImageCache {
    pub map: DashMap<ChannelId, String>,
}
impl ImageCache {
    pub fn new() -> ImageCache {
        let hasher = RandomState::new();
        ImageCache {
            map: DashMap::with_capacity_and_hasher(10, hasher),
        }
    }

    pub fn get(&self, channel_id: ChannelId) -> Option<Ref<'_, ChannelId, String>> {
        self.map.try_get(&channel_id).try_unwrap()
    }

    pub fn add(&self, channel_id: ChannelId, url: String) -> () {
        self.map.insert(channel_id, url);
    }

    pub fn flush(&self) -> bool {
        self.map.clear();
        self.map.is_empty()
    }

    pub fn capacity(&self) -> u64 {
        self.map.capacity() as u64
    }

    pub fn size(&self) -> u64 {
        self.map.len() as u64
    }
}
