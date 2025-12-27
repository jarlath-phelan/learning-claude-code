# UNO No Mercy - Animated Explainer Video Generator

A Rust application that generates a TikTok-style animated explainer video (9:16 vertical format, ~75 seconds) featuring a cartoon character explaining UNO No Mercy rules.

## Features

- **Cartoon Character Animation**: 2D cartoon character with 6 different expressions (Neutral, Shocked, Serious, Mischievous, Mind-Blown, Whispering)
- **UNO Card Graphics**: Procedurally generated UNO card graphics with all No Mercy special cards
- **Scene Composition**: 7 scenes with animated text, transitions, and effects
- **Text Effects**: Shadow, outline, shake, pop-in animations
- **Background Effects**: Gradient, spotlight, chaos/glitch, vignette

## Video Specifications

- **Aspect Ratio**: 9:16 (1080x1920 pixels)
- **Duration**: 75 seconds
- **Frame Rate**: 30 fps
- **Total Frames**: 2250
- **Output Format**: PNG frames (for FFmpeg compilation to MP4)

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

This will generate:
- `output/frames/` - 2250 PNG frames
- `output/voiceover_script.txt` - Script for TTS voiceover
- `output/compile_video.sh` - FFmpeg compilation script

## Creating the Final Video

### Step 1: Generate Voiceover

Using edge-tts (free):
```bash
pip install edge-tts
edge-tts --text "$(cat output/voiceover_script.txt)" \
         --voice en-US-GuyNeural \
         --rate "+10%" \
         --write-media output/audio/voiceover.mp3
```

Or use any TTS service with the script in `output/voiceover_script.txt`.

### Step 2: Compile Video

```bash
bash output/compile_video.sh
```

This requires FFmpeg to be installed. The final video will be `output/uno_no_mercy.mp4`.

## Project Structure

```
src/
├── main.rs        # Entry point and orchestration
├── character.rs   # Character sprite generation
├── cards.rs       # UNO card graphics
├── effects.rs     # Animation effects (easing, shake, fade, etc.)
├── scenes.rs      # Scene composition and timing
├── text.rs        # Text rendering with effects
└── video.rs       # Frame composition and backgrounds
fonts/
└── Roboto-Bold.ttf
output/
├── frames/        # Generated PNG frames
├── audio/         # Voiceover audio (after TTS)
├── voiceover_script.txt
└── compile_video.sh
```

## Scene Breakdown

1. **Hook (0-3s)**: Title reveal with "UNO NO MERCY" text
2. **Basics (3-15s)**: 168 cards, 6 players max, mercy rule explanation
3. **Draw Cards (15-30s)**: +2, +4, +10 stacking demonstration
4. **Plot Twist (30-45s)**: +4 is NOT wild - color-specific cards
5. **Chaos Cards (45-60s)**: 7 swap, 0 pass, Skip Everyone, Discard All, Color Roulette
6. **Golden Rule (60-70s)**: Draw until you can play
7. **Outro (70-75s)**: "Who wants to play?" with evil smile

## Dependencies

- `image` - Image processing
- `imageproc` - Drawing primitives
- `ab_glyph` - Font rendering
- `rand` - Random number generation
- `anyhow` - Error handling
- `indicatif` - Progress bars

## License

MIT
