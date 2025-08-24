# Voice AI - Conversation Recorder POC

A web-based AI tool that records conversations, transcribes them using OpenAI's Whisper API, and generates AI-powered summaries using GPT-3.5-turbo.

## Features

- 🎤 **Voice Recording**: Record conversations directly in the browser using the Web Audio API
- 📝 **Automatic Transcription**: Convert speech to text using OpenAI's Whisper API
- 🧠 **AI Summarization**: Generate concise summaries using GPT-3.5-turbo
- 💾 **Local Storage**: Store recordings in SQLite database
- 🎨 **Modern UI**: Beautiful interface built with Bulma CSS and Alpine.js
- ⚡ **Real-time Processing**: Background processing with status indicators

## Tech Stack

- **Backend**: Rust with Actix-web
- **Frontend**: Tera templates with Bulma CSS and Alpine.js
- **Database**: SQLite with SQLx
- **AI Services**: OpenAI API (Whisper + GPT-3.5-turbo)
- **Audio**: Web Audio API for client-side recording

## Prerequisites

- Rust (latest stable version)
- OpenAI API key

## Setup

1. **Clone and navigate to the project**:
   ```bash
   cd voiceai
   ```

2. **Set up environment variables**:
   Create a `.env` file in the project root:
   ```bash
   OPENAI_API_KEY=your_openai_api_key_here
   DATABASE_URL=sqlite:voiceai.db
   ```

3. **Install dependencies and run**:
   ```bash
   cargo run
   ```

4. **Access the application**:
   Open your browser and go to `http://127.0.0.1:8080`

## Usage

1. **Start Recording**:
   - Enter a title for your recording
   - Click "Start Recording" and allow microphone access
   - Speak clearly into your microphone
   - Click "Stop Recording" when finished

2. **View Results**:
   - The recording will be automatically processed
   - Transcription and summary will appear in the recordings list
   - Click "View Details" to see the full transcription and summary

3. **Manage Recordings**:
   - All recordings are stored locally in the SQLite database
   - Use the "Refresh" button to reload the recordings list

## API Endpoints

- `GET /` - Main application page
- `POST /api/record` - Save a new recording
- `GET /api/recordings` - Get all recordings
- `GET /api/recordings/{id}` - Get specific recording details

## Project Structure

```
voiceai/
├── src/
│   ├── main.rs          # Application entry point
│   ├── handlers.rs      # HTTP request handlers
│   ├── models.rs        # Data structures
│   ├── services.rs      # AI service integration
│   └── database.rs      # Database operations
├── templates/
│   └── index.html       # Main application template
├── static/              # Static assets (if any)
├── Cargo.toml           # Rust dependencies
└── README.md           # This file
```

## Configuration

### Environment Variables

- `OPENAI_API_KEY`: Your OpenAI API key (required)
- `DATABASE_URL`: SQLite database URL (default: `sqlite:voiceai.db`)

### OpenAI API Setup

1. Sign up for an OpenAI account at https://platform.openai.com
2. Generate an API key in your account settings
3. Add the API key to your `.env` file

## Development

### Running in Development Mode

```bash
# Run with logging
RUST_LOG=info cargo run

# Run with debug logging
RUST_LOG=debug cargo run
```

### Building for Production

```bash
cargo build --release
```

## Limitations

This is a POC with the following limitations:

- Audio format is limited to WAV format
- Maximum recording length depends on browser memory limits
- OpenAI API costs apply for transcription and summarization
- No user authentication or multi-user support
- Local storage only (no cloud backup)

## Future Enhancements

- User authentication and multi-user support
- Cloud storage integration
- Multiple audio format support
- Real-time transcription
- Custom AI models
- Export functionality
- Mobile app support

## Troubleshooting

### Common Issues

1. **Microphone Access Denied**:
   - Ensure your browser has permission to access the microphone
   - Try refreshing the page and allowing microphone access

2. **OpenAI API Errors**:
   - Verify your API key is correct
   - Check your OpenAI account has sufficient credits
   - Ensure the API key has access to Whisper and GPT models

3. **Database Errors**:
   - Ensure the application has write permissions in the project directory
   - Delete the `voiceai.db` file to reset the database

### Logs

Check the console output for detailed error messages and processing status.

## License

This project is for educational and POC purposes. Feel free to modify and extend as needed.