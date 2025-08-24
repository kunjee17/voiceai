use actix_web::{web, HttpResponse, Result};
use actix_web::web::Data;
use sqlx::sqlite::SqlitePool;
use tera::Tera;
use uuid::Uuid;
use tracing::{info, error};

use crate::models::*;
use crate::database;
use crate::services::AIService;
use crate::base64;

pub async fn index(tera: Data<Tera>) -> Result<HttpResponse> {
    let context = tera::Context::new();
    let body = tera.render("index.html", &context)
        .map_err(|e| {
            error!("Template error: {}", e);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub async fn record_audio(
    pool: Data<SqlitePool>,
    payload: web::Json<RecordAudioRequest>,
) -> Result<HttpResponse> {
    let id = Uuid::new_v4().to_string();
    
    // Decode base64 audio data
    let audio_data = match base64::decode(&payload.audio_data) {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to decode audio data: {}", e);
            return Ok(HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Invalid audio data format".to_string()),
            }));
        }
    };

    // Save recording to database
    if let Err(e) = database::save_recording(&pool, &id, &payload.title, &audio_data).await {
        error!("Failed to save recording: {}", e);
        return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Failed to save recording".to_string()),
        }));
    }

    // Process audio asynchronously
    let pool_clone = pool.clone();
    let id_clone = id.clone();
    let ai_service = match AIService::new() {
        Ok(service) => service,
        Err(e) => {
            error!("Failed to initialize AI service: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to initialize AI service".to_string()),
            }));
        }
    };

    // Spawn background task for transcription and summarization
    tokio::spawn(async move {
        info!("Starting transcription for recording: {}", id_clone);
        
        // Transcribe audio
        match ai_service.transcribe_audio(&audio_data).await {
            Ok(transcription) => {
                info!("Transcription completed for recording: {}", id_clone);
                
                if let Err(e) = database::update_transcription(&pool_clone, &id_clone, &transcription).await {
                    error!("Failed to save transcription: {}", e);
                    return;
                }

                // Generate summary
                match ai_service.summarize_text(&transcription).await {
                    Ok(summary) => {
                        info!("Summary completed for recording: {}", id_clone);
                        
                        if let Err(e) = database::update_summary(&pool_clone, &id_clone, &summary).await {
                            error!("Failed to save summary: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to generate summary: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to transcribe audio: {}", e);
            }
        }
    });

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(RecordAudioResponse {
            id,
            message: "Recording saved successfully. Processing transcription and summary...".to_string(),
        }),
        error: None,
    }))
}

pub async fn get_recordings(pool: Data<SqlitePool>) -> Result<HttpResponse> {
    match database::get_all_recordings(&pool).await {
        Ok(recordings) => {
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(recordings),
                error: None,
            }))
        }
        Err(e) => {
            error!("Failed to get recordings: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to get recordings".to_string()),
            }))
        }
    }
}

pub async fn get_recording(
    pool: Data<SqlitePool>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    
    match database::get_recording(&pool, &id).await {
        Ok(Some(recording)) => {
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(recording),
                error: None,
            }))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Recording not found".to_string()),
            }))
        }
        Err(e) => {
            error!("Failed to get recording: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Failed to get recording".to_string()),
            }))
        }
    }
}