# Approach 3: Hybrid (Rust Engine + AI Assets)

Combines the precision of programmatic frame generation with AI-generated visual assets.

## Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│                         Hybrid Pipeline                                 │
├─────────────────┬─────────────────┬─────────────────┬─────────────────┤
│   AI Assets     │   Rust Engine   │   TTS           │   FFmpeg        │
│   Generation    │   (Frames)      │   (Voice)       │   (Compile)     │
├─────────────────┼─────────────────┼─────────────────┼─────────────────┤
│ - Backgrounds   │ - Scene timing  │ - edge-tts      │ - Frames→Video  │
│ - Characters    │ - Animations    │ - OpenAI TTS    │ - Add audio     │
│ - Card designs  │ - Text overlays │                 │ - Encode MP4    │
│ - Style images  │ - Compositing   │                 │                 │
└─────────────────┴─────────────────┴─────────────────┴─────────────────┘
```

## Why Hybrid?

| Aspect | Pure Programmatic | Pure AI | Hybrid |
|--------|-------------------|---------|--------|
| Visual quality | Limited | High | High |
| Consistency | Perfect | Variable | Controlled |
| Animation control | Full | Limited | Full |
| Cost | Free | ~$1-5/video | ~$0.10-0.50 |
| Speed | Fast | Slow | Medium |
| Customization | Code changes | Prompts | Both |

## Setup

1. Install Python dependencies:
```bash
pip install -r requirements.txt
```

2. Configure API keys in `.env`:
```bash
cp .env.example .env
```

3. Generate AI assets:
```bash
python generate_assets.py --theme "uno-no-mercy" --style "cartoon"
```

4. Run Rust engine with AI assets:
```bash
cd ../uno-no-mercy-video
cargo run --release -- --assets-dir ../approach-3-hybrid/assets
```

## Asset Types

### Backgrounds (1080x1920)
- `bg_intro.png` - Hook scene background
- `bg_dramatic.png` - Dramatic reveal scenes
- `bg_chaos.png` - Chaotic scene background
- `bg_outro.png` - Closing scene background

### Characters (transparent PNG)
- `character_neutral.png`
- `character_shocked.png`
- `character_mischievous.png`
- `character_serious.png`

### Props
- `card_stack.png` - Stack of UNO cards
- `effects_overlay.png` - Particle/glow effects

## Workflow

```bash
# Step 1: Generate AI assets
python generate_assets.py --theme "financial-basics" --style "modern-minimal"

# Step 2: Preview assets
python preview_assets.py

# Step 3: Generate video frames (uses Rust engine)
cargo run --release --manifest-path ../uno-no-mercy-video/Cargo.toml

# Step 4: Generate voiceover
edge-tts --text "$(cat output/script.txt)" --voice en-US-GuyNeural -o output/voiceover.mp3

# Step 5: Compile final video
ffmpeg -framerate 30 -i output/frames/frame_%05d.png -i output/voiceover.mp3 \
       -c:v libx264 -c:a aac -shortest output/final.mp4
```
