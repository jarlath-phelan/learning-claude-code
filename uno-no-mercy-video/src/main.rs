//! UNO No Mercy Animated Explainer Video Generator
//!
//! Generates a TikTok-style vertical video (9:16, 1080x1920) explaining UNO No Mercy rules.

mod character;
mod cards;
mod effects;
mod scenes;
mod text;
mod video;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

/// Video configuration
pub const VIDEO_WIDTH: u32 = 1080;
pub const VIDEO_HEIGHT: u32 = 1920;
pub const FRAME_RATE: u32 = 30;
pub const TOTAL_DURATION_SECS: f32 = 75.0;

fn main() -> Result<()> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║       UNO NO MERCY - Animated Explainer Video Generator   ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    let output_dir = Path::new("output");
    let frames_dir = output_dir.join("frames");

    // Create output directories
    std::fs::create_dir_all(&frames_dir)?;
    std::fs::create_dir_all(output_dir.join("audio"))?;

    let total_frames = (TOTAL_DURATION_SECS * FRAME_RATE as f32) as u64;

    println!("📊 Video Specifications:");
    println!("   Resolution: {}x{} (9:16 vertical)", VIDEO_WIDTH, VIDEO_HEIGHT);
    println!("   Frame Rate: {} fps", FRAME_RATE);
    println!("   Duration: {:.1} seconds", TOTAL_DURATION_SECS);
    println!("   Total Frames: {}", total_frames);
    println!();

    // Generate the video script for TTS
    println!("📝 Generating voiceover script...");
    generate_script_file(output_dir)?;

    // Generate all frames
    println!("🎬 Generating video frames...");
    let pb = ProgressBar::new(total_frames);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("#>-"));

    let scene_manager = scenes::SceneManager::new();

    for frame_num in 0..total_frames {
        let time = frame_num as f32 / FRAME_RATE as f32;
        let frame = scene_manager.render_frame(time, frame_num as u32)?;

        let frame_path = frames_dir.join(format!("frame_{:05}.png", frame_num));
        frame.save(&frame_path)?;

        pb.inc(1);
    }

    pb.finish_with_message("Frames generated!");
    println!();

    // Generate FFmpeg command
    println!("🎥 Generating FFmpeg compilation script...");
    generate_ffmpeg_script(output_dir)?;

    println!();
    println!("✅ Frame generation complete!");
    println!();
    println!("📁 Output files:");
    println!("   Frames: output/frames/frame_*.png");
    println!("   Script: output/voiceover_script.txt");
    println!("   FFmpeg: output/compile_video.sh");
    println!();
    println!("🎙️ Next steps:");
    println!("   1. Generate voiceover audio from output/voiceover_script.txt");
    println!("      Use: edge-tts --text \"$(cat output/voiceover_script.txt)\" --voice en-US-GuyNeural --write-media output/audio/voiceover.mp3");
    println!("   2. Run: bash output/compile_video.sh");
    println!("   3. Final video: output/uno_no_mercy.mp4");

    Ok(())
}

fn generate_script_file(output_dir: &Path) -> Result<()> {
    let script = r#"So you think you know UNO? Nah. Let me tell you about NO MERCY.

168 cards. SIX players max. And if you get 25 cards in your hand? You're DEAD. Eliminated. Gone. That's the Mercy Rule and there IS no mercy.

Plus 2? That's cute. Plus 4? Getting warmer. PLUS 10. And guess what? You can STACK them. Someone hits you with a plus 4? Throw down a plus 6. Now THEY draw 10. Unless they stack higher. It keeps going until someone CAN'T match it and draws EVERYTHING.

But here's what NO ONE tells you. That plus 4? It's NOT a wild card anymore. It has a COLOR. Red plus 4 only plays on RED. The wilds are Draw 6, Draw 10, Reverse Draw 4, and Color Roulette. THOSE play anytime.

Oh you thought we were done? Play a 7, you SWAP your entire hand with someone. Play a 0, EVERYONE passes their hand to the next person. Skip Everyone? You skip THE WHOLE TABLE and go again. Discard All? Dump every card of that color at once. Color Roulette? They flip cards until they hit the color they call. Could be 2 cards. Could be 15.

And if you can't play? You don't just draw one card like a NORMAL person. You draw until you CAN play. No stopping. No passing. Just pain.

This game has ended friendships. Ruined holidays. Created villains. Anyway, who wants to play?"#;

    std::fs::write(output_dir.join("voiceover_script.txt"), script)?;
    Ok(())
}

fn generate_ffmpeg_script(output_dir: &Path) -> Result<()> {
    let script = format!(r#"#!/bin/bash
# UNO No Mercy Video Compilation Script

# Check if voiceover exists
if [ ! -f "output/audio/voiceover.mp3" ]; then
    echo "⚠️  No voiceover found. Generating with edge-tts..."
    edge-tts --text "$(cat output/voiceover_script.txt)" \
             --voice en-US-GuyNeural \
             --rate "+10%" \
             --write-media output/audio/voiceover.mp3
fi

# Compile frames to video with audio
ffmpeg -y \
    -framerate {fps} \
    -i output/frames/frame_%05d.png \
    -i output/audio/voiceover.mp3 \
    -c:v libx264 \
    -preset medium \
    -crf 23 \
    -pix_fmt yuv420p \
    -c:a aac \
    -b:a 192k \
    -shortest \
    -movflags +faststart \
    output/uno_no_mercy.mp4

echo ""
echo "✅ Video compiled successfully!"
echo "📹 Output: output/uno_no_mercy.mp4"
"#, fps = FRAME_RATE);

    let script_path = output_dir.join("compile_video.sh");
    std::fs::write(&script_path, script)?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&script_path, std::fs::Permissions::from_mode(0o755))?;
    }

    Ok(())
}
