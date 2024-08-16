use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExifResponse {
    pub width: u32,
    pub height: u32,
    pub size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub memory: u64,
    pub cpu: f32,
    pub uptime: u128,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}
