# Approach 2: Automated API Pipeline

A Python-based pipeline that chains multiple AI services to generate TikTok videos programmatically.

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Claude    │────▶│  AI Video   │────▶│    TTS      │────▶│   FFmpeg    │
│  (Script)   │     │ Generation  │     │  (Voice)    │     │  (Compile)  │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
                           │
                    ┌──────┴──────┐
                    ▼             ▼
              [Runway API]  [Replicate]
              [Pika/Fal]    [Open-Sora]
```

## Setup

1. Install dependencies:
```bash
pip install -r requirements.txt
```

2. Create `.env` file:
```bash
cp .env.example .env
# Add your API keys
```

3. Run the pipeline:
```bash
python pipeline.py generate --topic "UNO No Mercy rules" --style dramatic
```

## Available Providers

| Stage | Provider | Status | Cost |
|-------|----------|--------|------|
| Script | Claude API | Ready | ~$0.01/script |
| Script | OpenAI GPT-4 | Ready | ~$0.03/script |
| Video | Replicate models | Ready | ~$0.10-0.50/clip |
| Video | Runway API | Limited | ~$0.05/second |
| Video | Fal.ai (Pika) | Ready | ~$0.05/clip |
| TTS | Edge-TTS | Ready | Free |
| TTS | ElevenLabs | Ready | ~$0.30/minute |
| TTS | OpenAI TTS | Ready | ~$0.015/1K chars |

## Output

Videos are saved to `output/` with timestamps:
- `output/video_20241227_143052.mp4`
- `output/script_20241227_143052.txt`
- `output/metadata_20241227_143052.json`
