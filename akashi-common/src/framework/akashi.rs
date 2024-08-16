use std::collections::HashSet;
use std::sync::Arc;

use poise::futures_util::lock::Mutex;
use poise::serenity_prelude::UserId;
use poise::Context;

use akashi_cache::{initialize_custom_cache, AkashiCustomCache};

pub type AkashiError = Box<dyn std::error::Error + Send + Sync>;
pub type AkashiContext<'a> = Context<'a, AkashiData, AkashiError>;

pub struct AkashiData {
    pub sysinfo: Mutex<sysinfo::System>,
    pub custom_cache: Mutex<AkashiCustomCache>,
    pub reqwest: Arc<reqwest::Client>,
    pub api: String,
}

pub fn initialize_owners() -> HashSet<UserId> {
    HashSet::<UserId>::from_iter(vec![
        UserId::new(1076700780175831100), // Simxnet
        UserId::new(462780441594822687),  // Chiko
    ])
}
pub fn initialize_data() -> AkashiData {
    AkashiData {
        sysinfo: Mutex::new(sysinfo::System::new()),
        custom_cache: Mutex::new(initialize_custom_cache()),
        reqwest: Arc::new(reqwest::Client::new()),
        api: std::env::var("API_HOST").expect("API_HOST not set"),
    }
}
