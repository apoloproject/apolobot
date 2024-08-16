use std::sync::Arc;

pub mod caches;

pub struct AkashiCustomCache {
    pub images: Arc<caches::ImageCache>,
}

pub fn initialize_custom_cache() -> AkashiCustomCache {
    AkashiCustomCache {
        images: Arc::new(caches::ImageCache::new()),
    }
}
