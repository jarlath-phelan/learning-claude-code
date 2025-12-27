#!/bin/bash
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
    -framerate 30 \
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
