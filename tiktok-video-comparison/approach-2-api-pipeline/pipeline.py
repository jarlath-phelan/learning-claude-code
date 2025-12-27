#!/usr/bin/env python3
"""
TikTok Video Generation Pipeline

Chains multiple AI services to generate short-form explainer videos.
"""

import asyncio
import json
import os
import subprocess
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path
from typing import Optional

import httpx
from dotenv import load_dotenv
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn

load_dotenv()
console = Console()


# =============================================================================
# Data Models
# =============================================================================

@dataclass
class Scene:
    """A single scene in the video."""

    index: int
    start_time: float
    end_time: float
    narration: str
    visual_prompt: str
    text_overlay: Optional[str] = None


@dataclass
class VideoScript:
    """Complete video script with scenes."""

    title: str
    topic: str
    style: str
    total_duration: float
    scenes: list[Scene] = field(default_factory=list)

    def to_narration(self) -> str:
        """Get full narration text for TTS."""
        return " ".join(scene.narration for scene in self.scenes)


@dataclass
class GeneratedAsset:
    """A generated asset (video clip, audio, etc.)."""

    path: Path
    asset_type: str  # "video", "audio", "image"
    duration: float
    metadata: dict = field(default_factory=dict)


# =============================================================================
# Script Generation Providers
# =============================================================================

class ScriptProvider(ABC):
    """Base class for script generation providers."""

    @abstractmethod
    async def generate_script(
        self, topic: str, style: str, duration: int = 60
    ) -> VideoScript:
        pass


class ClaudeScriptProvider(ScriptProvider):
    """Generate scripts using Claude API."""

    def __init__(self):
        self.api_key = os.getenv("ANTHROPIC_API_KEY")
        self.base_url = "https://api.anthropic.com/v1/messages"

    async def generate_script(
        self, topic: str, style: str, duration: int = 60
    ) -> VideoScript:
        prompt = f"""Create a TikTok video script about: {topic}

Style: {style}
Duration: {duration} seconds

Return a JSON object with this structure:
{{
    "title": "Video title",
    "scenes": [
        {{
            "start_time": 0,
            "end_time": 5,
            "narration": "What the narrator says",
            "visual_prompt": "Description of visuals to generate",
            "text_overlay": "Bold text shown on screen (optional)"
        }}
    ]
}}

Requirements:
- Hook in first 3 seconds
- 5-7 scenes total
- Each scene 5-15 seconds
- Narration should be conversational, engaging
- Visual prompts should be specific for AI image/video generation
- Text overlays for key statistics or emphasis

Return ONLY the JSON, no other text."""

        async with httpx.AsyncClient() as client:
            response = await client.post(
                self.base_url,
                headers={
                    "x-api-key": self.api_key,
                    "content-type": "application/json",
                    "anthropic-version": "2023-06-01",
                },
                json={
                    "model": "claude-sonnet-4-20250514",
                    "max_tokens": 2000,
                    "messages": [{"role": "user", "content": prompt}],
                },
                timeout=60.0,
            )
            response.raise_for_status()

            content = response.json()["content"][0]["text"]
            # Extract JSON from response
            data = json.loads(content)

            scenes = [
                Scene(
                    index=i,
                    start_time=s["start_time"],
                    end_time=s["end_time"],
                    narration=s["narration"],
                    visual_prompt=s["visual_prompt"],
                    text_overlay=s.get("text_overlay"),
                )
                for i, s in enumerate(data["scenes"])
            ]

            return VideoScript(
                title=data["title"],
                topic=topic,
                style=style,
                total_duration=duration,
                scenes=scenes,
            )


# =============================================================================
# Video Generation Providers
# =============================================================================

class VideoProvider(ABC):
    """Base class for video generation providers."""

    @abstractmethod
    async def generate_clip(
        self, prompt: str, duration: float, output_path: Path
    ) -> GeneratedAsset:
        pass


class ReplicateVideoProvider(VideoProvider):
    """Generate videos using Replicate API."""

    def __init__(self, model: str = "stability-ai/stable-video-diffusion"):
        self.api_token = os.getenv("REPLICATE_API_TOKEN")
        self.model = model
        self.base_url = "https://api.replicate.com/v1/predictions"

    async def generate_clip(
        self, prompt: str, duration: float, output_path: Path
    ) -> GeneratedAsset:
        async with httpx.AsyncClient() as client:
            # Start prediction
            response = await client.post(
                self.base_url,
                headers={"Authorization": f"Token {self.api_token}"},
                json={
                    "version": self.model,
                    "input": {
                        "prompt": prompt,
                        "num_frames": int(duration * 24),  # 24fps
                    },
                },
                timeout=30.0,
            )
            response.raise_for_status()
            prediction = response.json()

            # Poll for completion
            prediction_url = prediction["urls"]["get"]
            while True:
                await asyncio.sleep(5)
                response = await client.get(
                    prediction_url,
                    headers={"Authorization": f"Token {self.api_token}"},
                )
                prediction = response.json()

                if prediction["status"] == "succeeded":
                    video_url = prediction["output"]
                    break
                elif prediction["status"] == "failed":
                    raise RuntimeError(f"Video generation failed: {prediction.get('error')}")

            # Download video
            video_response = await client.get(video_url)
            output_path.write_bytes(video_response.content)

            return GeneratedAsset(
                path=output_path,
                asset_type="video",
                duration=duration,
                metadata={"model": self.model, "prompt": prompt},
            )


class FallbackImageProvider(VideoProvider):
    """Generate static images as fallback (cheaper, always available)."""

    def __init__(self):
        self.api_token = os.getenv("REPLICATE_API_TOKEN")

    async def generate_clip(
        self, prompt: str, duration: float, output_path: Path
    ) -> GeneratedAsset:
        """Generate an image and convert to video with Ken Burns effect."""

        # For demo: create a placeholder
        # In production, use SDXL or similar via Replicate
        console.print(f"[yellow]Generating image for: {prompt[:50]}...[/yellow]")

        # Create placeholder image using PIL
        from PIL import Image, ImageDraw, ImageFont

        img = Image.new("RGB", (1080, 1920), color=(30, 30, 40))
        draw = ImageDraw.Draw(img)

        # Add prompt text
        try:
            font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 40)
        except OSError:
            font = ImageFont.load_default()

        # Wrap text
        words = prompt.split()
        lines = []
        current_line = []
        for word in words:
            current_line.append(word)
            if len(" ".join(current_line)) > 30:
                lines.append(" ".join(current_line[:-1]))
                current_line = [word]
        if current_line:
            lines.append(" ".join(current_line))

        y_offset = 800
        for line in lines[:5]:
            draw.text((100, y_offset), line, fill=(255, 255, 255), font=font)
            y_offset += 50

        image_path = output_path.with_suffix(".png")
        img.save(image_path)

        # Convert to video with FFmpeg
        subprocess.run([
            "ffmpeg", "-y",
            "-loop", "1",
            "-i", str(image_path),
            "-c:v", "libx264",
            "-t", str(duration),
            "-pix_fmt", "yuv420p",
            "-vf", "scale=1080:1920,zoompan=z='min(zoom+0.001,1.2)':d=1:s=1080x1920",
            str(output_path),
        ], capture_output=True, check=True)

        image_path.unlink()  # Clean up image

        return GeneratedAsset(
            path=output_path,
            asset_type="video",
            duration=duration,
            metadata={"type": "image_fallback", "prompt": prompt},
        )


# =============================================================================
# TTS Providers
# =============================================================================

class TTSProvider(ABC):
    """Base class for text-to-speech providers."""

    @abstractmethod
    async def generate_audio(
        self, text: str, output_path: Path, voice: str = "default"
    ) -> GeneratedAsset:
        pass


class EdgeTTSProvider(TTSProvider):
    """Free TTS using Microsoft Edge."""

    async def generate_audio(
        self, text: str, output_path: Path, voice: str = "en-US-GuyNeural"
    ) -> GeneratedAsset:
        import edge_tts

        communicate = edge_tts.Communicate(text, voice, rate="+10%")
        await communicate.save(str(output_path))

        # Get duration using ffprobe
        result = subprocess.run(
            ["ffprobe", "-v", "quiet", "-show_entries", "format=duration",
             "-of", "csv=p=0", str(output_path)],
            capture_output=True, text=True
        )
        duration = float(result.stdout.strip()) if result.stdout.strip() else 0

        return GeneratedAsset(
            path=output_path,
            asset_type="audio",
            duration=duration,
            metadata={"voice": voice},
        )


class OpenAITTSProvider(TTSProvider):
    """TTS using OpenAI API."""

    def __init__(self):
        self.api_key = os.getenv("OPENAI_API_KEY")

    async def generate_audio(
        self, text: str, output_path: Path, voice: str = "onyx"
    ) -> GeneratedAsset:
        async with httpx.AsyncClient() as client:
            response = await client.post(
                "https://api.openai.com/v1/audio/speech",
                headers={"Authorization": f"Bearer {self.api_key}"},
                json={
                    "model": "tts-1",
                    "input": text,
                    "voice": voice,
                },
                timeout=60.0,
            )
            response.raise_for_status()
            output_path.write_bytes(response.content)

        # Get duration
        result = subprocess.run(
            ["ffprobe", "-v", "quiet", "-show_entries", "format=duration",
             "-of", "csv=p=0", str(output_path)],
            capture_output=True, text=True
        )
        duration = float(result.stdout.strip()) if result.stdout.strip() else 0

        return GeneratedAsset(
            path=output_path,
            asset_type="audio",
            duration=duration,
            metadata={"voice": voice, "provider": "openai"},
        )


# =============================================================================
# Video Compiler
# =============================================================================

class VideoCompiler:
    """Compile generated assets into final video."""

    def __init__(self, output_dir: Path):
        self.output_dir = output_dir
        self.output_dir.mkdir(parents=True, exist_ok=True)

    def compile(
        self,
        video_clips: list[GeneratedAsset],
        audio: GeneratedAsset,
        script: VideoScript,
    ) -> Path:
        """Compile clips and audio into final video."""

        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        output_path = self.output_dir / f"video_{timestamp}.mp4"

        # Create concat file
        concat_file = self.output_dir / "concat.txt"
        with open(concat_file, "w") as f:
            for clip in video_clips:
                f.write(f"file '{clip.path.absolute()}'\n")

        # Concatenate clips
        temp_video = self.output_dir / "temp_concat.mp4"
        subprocess.run([
            "ffmpeg", "-y",
            "-f", "concat",
            "-safe", "0",
            "-i", str(concat_file),
            "-c", "copy",
            str(temp_video),
        ], capture_output=True, check=True)

        # Add audio
        subprocess.run([
            "ffmpeg", "-y",
            "-i", str(temp_video),
            "-i", str(audio.path),
            "-c:v", "copy",
            "-c:a", "aac",
            "-shortest",
            str(output_path),
        ], capture_output=True, check=True)

        # Cleanup
        concat_file.unlink()
        temp_video.unlink()

        return output_path


# =============================================================================
# Main Pipeline
# =============================================================================

class VideoPipeline:
    """Main pipeline orchestrator."""

    def __init__(
        self,
        script_provider: ScriptProvider,
        video_provider: VideoProvider,
        tts_provider: TTSProvider,
        output_dir: Path = Path("./output"),
    ):
        self.script_provider = script_provider
        self.video_provider = video_provider
        self.tts_provider = tts_provider
        self.output_dir = output_dir
        self.output_dir.mkdir(parents=True, exist_ok=True)
        self.compiler = VideoCompiler(output_dir)

    async def generate(self, topic: str, style: str, duration: int = 60) -> Path:
        """Generate a complete video."""

        with Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            console=console,
        ) as progress:
            # Step 1: Generate script
            task = progress.add_task("Generating script...", total=None)
            script = await self.script_provider.generate_script(topic, style, duration)
            progress.update(task, description="[green]Script generated!")

            console.print(f"\n[bold]Title:[/bold] {script.title}")
            console.print(f"[bold]Scenes:[/bold] {len(script.scenes)}\n")

            # Save script
            script_path = self.output_dir / f"script_{datetime.now():%Y%m%d_%H%M%S}.json"
            with open(script_path, "w") as f:
                json.dump({
                    "title": script.title,
                    "topic": script.topic,
                    "style": script.style,
                    "scenes": [
                        {
                            "narration": s.narration,
                            "visual_prompt": s.visual_prompt,
                            "text_overlay": s.text_overlay,
                            "start_time": s.start_time,
                            "end_time": s.end_time,
                        }
                        for s in script.scenes
                    ],
                }, f, indent=2)

            # Step 2: Generate TTS
            task = progress.add_task("Generating voiceover...", total=None)
            audio_path = self.output_dir / "voiceover.mp3"
            audio = await self.tts_provider.generate_audio(
                script.to_narration(), audio_path
            )
            progress.update(task, description=f"[green]Voiceover generated! ({audio.duration:.1f}s)")

            # Step 3: Generate video clips for each scene
            clips = []
            for i, scene in enumerate(script.scenes):
                task = progress.add_task(
                    f"Generating scene {i+1}/{len(script.scenes)}...", total=None
                )
                clip_path = self.output_dir / f"clip_{i:02d}.mp4"
                clip_duration = scene.end_time - scene.start_time

                clip = await self.video_provider.generate_clip(
                    scene.visual_prompt, clip_duration, clip_path
                )
                clips.append(clip)
                progress.update(task, description=f"[green]Scene {i+1} complete!")

            # Step 4: Compile final video
            task = progress.add_task("Compiling final video...", total=None)
            final_path = self.compiler.compile(clips, audio, script)
            progress.update(task, description="[green]Video compiled!")

            # Cleanup clips
            for clip in clips:
                clip.path.unlink()
            audio.path.unlink()

        console.print(f"\n[bold green]Video saved to:[/bold green] {final_path}")
        return final_path


# =============================================================================
# CLI Interface
# =============================================================================

def main():
    """CLI entry point."""
    import typer

    app = typer.Typer(help="TikTok Video Generation Pipeline")

    @app.command()
    def generate(
        topic: str = typer.Argument(..., help="Topic to create video about"),
        style: str = typer.Option("dramatic", help="Video style (dramatic, educational, humorous)"),
        duration: int = typer.Option(60, help="Target duration in seconds"),
        use_openai_tts: bool = typer.Option(False, help="Use OpenAI TTS instead of Edge-TTS"),
    ):
        """Generate a TikTok-style explainer video."""

        console.print("[bold]TikTok Video Generation Pipeline[/bold]\n")
        console.print(f"Topic: {topic}")
        console.print(f"Style: {style}")
        console.print(f"Duration: {duration}s\n")

        # Initialize providers
        script_provider = ClaudeScriptProvider()
        video_provider = FallbackImageProvider()  # Use image fallback by default
        tts_provider = OpenAITTSProvider() if use_openai_tts else EdgeTTSProvider()

        # Run pipeline
        pipeline = VideoPipeline(
            script_provider=script_provider,
            video_provider=video_provider,
            tts_provider=tts_provider,
        )

        asyncio.run(pipeline.generate(topic, style, duration))

    @app.command()
    def list_providers():
        """List available providers and their status."""

        console.print("\n[bold]Script Providers:[/bold]")
        console.print("  - Claude API: " + ("[green]Ready[/green]" if os.getenv("ANTHROPIC_API_KEY") else "[red]No API key[/red]"))

        console.print("\n[bold]Video Providers:[/bold]")
        console.print("  - Replicate: " + ("[green]Ready[/green]" if os.getenv("REPLICATE_API_TOKEN") else "[red]No API key[/red]"))
        console.print("  - Image Fallback: [green]Always available[/green]")

        console.print("\n[bold]TTS Providers:[/bold]")
        console.print("  - Edge-TTS: [green]Always available (free)[/green]")
        console.print("  - OpenAI TTS: " + ("[green]Ready[/green]" if os.getenv("OPENAI_API_KEY") else "[red]No API key[/red]"))

    app()


if __name__ == "__main__":
    main()
