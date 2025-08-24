use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recording {
    pub id: String,
    pub title: String,
    pub audio_data: Vec<u8>,
    pub transcription: Option<String>,
    pub summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordingListItem {
    pub id: String,
    pub title: String,
    pub transcription: Option<String>,
    pub summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RecordAudioRequest {
    pub title: String,
    pub audio_data: String, // Base64 encoded audio
}

#[derive(Debug, Serialize)]
pub struct RecordAudioResponse {
    pub id: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

