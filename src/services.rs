use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAITranscriptionResponse {
    text: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

pub struct AIService {
    client: Client,
    api_key: String,
}

impl AIService {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY environment variable must be set");
        
        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    pub async fn transcribe_audio(&self, audio_data: &[u8]) -> Result<String> {
        // For this POC, we'll use a simple approach with OpenAI's API
        // In a real implementation, you'd want to handle different audio formats properly
        
        // Create a multipart form with the audio data
        let response = self.client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(reqwest::multipart::Form::new()
                .part("file", reqwest::multipart::Part::bytes(audio_data.to_vec())
                    .file_name("audio.wav")
                    .mime_str("audio/wav")?)
                .text("model", "whisper-1"))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }

        let transcription_response: OpenAITranscriptionResponse = response.json().await?;
        Ok(transcription_response.text)
    }

    pub async fn summarize_text(&self, text: &str) -> Result<String> {
        let request = OpenAIChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: "You are a helpful assistant that creates concise summaries of conversations. Focus on the key points and main topics discussed.".to_string(),
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: format!("Please summarize this conversation:\n\n{}", text),
                },
            ],
            max_tokens: 500,
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }

        let chat_response: OpenAIChatResponse = response.json().await?;
        
        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("No response from OpenAI"))
        }
    }
}