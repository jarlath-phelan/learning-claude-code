#!/usr/bin/env python3
"""
Hybrid Pipeline Orchestrator

Coordinates AI asset generation, Rust frame rendering, TTS, and final compilation.
"""

import asyncio
import json
import os
import subprocess
import sys
from datetime import datetime
from pathlib import Path

from dotenv import load_dotenv
from rich.console import Console
from rich.panel import Panel
from rich.progress import Progress, SpinnerColumn, TextColumn

load_dotenv()
console = Console()


class HybridPipeline:
    """Orchestrates the complete hybrid video pipeline."""

    def __init__(
        self,
        theme: str,
        style: str = "cartoon",
        output_dir: Path = Path("./output"),
        rust_project: Path = Path("../uno-no-mercy-video"),
    ):
        self.theme = theme
        self.style = style
        self.output_dir = output_dir
        self.rust_project = rust_project
        self.assets_dir = output_dir / "assets"
        self.frames_dir = output_dir / "frames"

        self.output_dir.mkdir(parents=True, exist_ok=True)
        self.frames_dir.mkdir(parents=True, exist_ok=True)

    async def run(self) -> Path:
        """Execute the complete pipeline."""

        console.print(Panel.fit(
            f"[bold]Hybrid Video Pipeline[/bold]\n"
            f"Theme: {self.theme}\n"
            f"Style: {self.style}",
            title="Starting",
        ))

        with Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            console=console,
        ) as progress:

            # Step 1: Generate AI assets
            task = progress.add_task("Step 1/4: Generating AI assets...", total=None)
            await self._generate_assets()
            progress.update(task, description="[green]Step 1/4: AI assets generated!")

            # Step 2: Generate frames with Rust engine
            task = progress.add_task("Step 2/4: Rendering frames with Rust engine...", total=None)
            await self._render_frames()
            progress.update(task, description="[green]Step 2/4: Frames rendered!")

            # Step 3: Generate voiceover
            task = progress.add_task("Step 3/4: Generating voiceover...", total=None)
            audio_path = await self._generate_voiceover()
            progress.update(task, description="[green]Step 3/4: Voiceover generated!")

            # Step 4: Compile final video
            task = progress.add_task("Step 4/4: Compiling final video...", total=None)
            video_path = await self._compile_video(audio_path)
            progress.update(task, description="[green]Step 4/4: Video compiled!")

        console.print(f"\n[bold green]Video saved to:[/bold green] {video_path}")
        return video_path

    async def _generate_assets(self):
        """Generate AI assets using the asset generator."""

        # Import and run the asset generator
        from generate_assets import generate_theme_assets

        await generate_theme_assets(
            theme_name=self.theme,
            style_name=self.style,
            output_dir=self.assets_dir,
            provider="replicate" if os.getenv("REPLICATE_API_TOKEN") else "placeholder",
        )

    async def _render_frames(self):
        """Render video frames using the Rust engine."""

        # Check if Rust project exists
        if not self.rust_project.exists():
            console.print("[yellow]Rust project not found, using Python fallback renderer[/yellow]")
            await self._render_frames_python()
            return

        # Build and run Rust project
        result = subprocess.run(
            ["cargo", "build", "--release"],
            cwd=self.rust_project,
            capture_output=True,
            text=True,
        )

        if result.returncode != 0:
            console.print(f"[yellow]Rust build failed, using Python fallback[/yellow]")
            await self._render_frames_python()
            return

        # Run with assets directory
        env = os.environ.copy()
        env["ASSETS_DIR"] = str(self.assets_dir.absolute())
        env["OUTPUT_DIR"] = str(self.frames_dir.absolute())

        result = subprocess.run(
            ["cargo", "run", "--release"],
            cwd=self.rust_project,
            env=env,
            capture_output=True,
            text=True,
        )

        if result.returncode != 0:
            console.print(f"[red]Rust execution failed: {result.stderr}[/red]")
            await self._render_frames_python()

    async def _render_frames_python(self):
        """Fallback Python frame renderer."""

        from PIL import Image, ImageDraw, ImageFont

        # Load manifest
        manifest_path = self.assets_dir / "manifest.json"
        if manifest_path.exists():
            with open(manifest_path) as f:
                manifest = json.load(f)
        else:
            manifest = {"assets": {"backgrounds": [], "characters": []}}

        # Video settings
        width, height = 1080, 1920
        fps = 30
        duration = 75  # seconds
        total_frames = fps * duration

        # Load background if available
        bg_files = manifest.get("assets", {}).get("backgrounds", [])
        backgrounds = []
        for bg_file in bg_files:
            bg_path = self.assets_dir / bg_file
            if bg_path.exists():
                backgrounds.append(Image.open(bg_path).resize((width, height)))

        if not backgrounds:
            # Create default gradient background
            bg = Image.new("RGB", (width, height))
            for y in range(height):
                r = int(20 + (y / height) * 20)
                g = int(15 + (y / height) * 15)
                b = int(30 + (y / height) * 20)
                for x in range(width):
                    bg.putpixel((x, y), (r, g, b))
            backgrounds = [bg]

        # Load character if available
        char_files = manifest.get("assets", {}).get("characters", [])
        characters = []
        for char_file in char_files:
            char_path = self.assets_dir / char_file
            if char_path.exists():
                characters.append(Image.open(char_path).convert("RGBA"))

        # Scene definitions (simplified)
        scenes = [
            {"start": 0, "end": 3, "text": "UNO NO MERCY", "bg_idx": 0},
            {"start": 3, "end": 15, "text": "168 CARDS\n6 PLAYERS\n25 = ELIMINATED", "bg_idx": 0},
            {"start": 15, "end": 30, "text": "+2  +4  +10\nSTACK THEM!", "bg_idx": 1 if len(backgrounds) > 1 else 0},
            {"start": 30, "end": 45, "text": "PLOT TWIST\n+4 HAS A COLOR!", "bg_idx": 1 if len(backgrounds) > 1 else 0},
            {"start": 45, "end": 60, "text": "7 = SWAP\n0 = PASS ALL\nSKIP EVERYONE", "bg_idx": 2 if len(backgrounds) > 2 else 0},
            {"start": 60, "end": 70, "text": "DRAW UNTIL\nYOU CAN PLAY", "bg_idx": 1 if len(backgrounds) > 1 else 0},
            {"start": 70, "end": 75, "text": "Who wants to play?", "bg_idx": 3 if len(backgrounds) > 3 else 0},
        ]

        try:
            font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 80)
        except OSError:
            font = ImageFont.load_default()

        console.print(f"Rendering {total_frames} frames...")

        for frame_num in range(total_frames):
            time = frame_num / fps

            # Find current scene
            current_scene = scenes[0]
            for scene in scenes:
                if scene["start"] <= time < scene["end"]:
                    current_scene = scene
                    break

            # Create frame
            bg_idx = current_scene["bg_idx"] % len(backgrounds)
            frame = backgrounds[bg_idx].copy()
            draw = ImageDraw.Draw(frame)

            # Add text
            text = current_scene["text"]
            lines = text.split("\n")
            y_offset = height // 3

            for line in lines:
                bbox = draw.textbbox((0, 0), line, font=font)
                text_width = bbox[2] - bbox[0]
                x = (width - text_width) // 2

                # Draw shadow
                draw.text((x + 4, y_offset + 4), line, fill=(0, 0, 0), font=font)
                # Draw text
                draw.text((x, y_offset), line, fill=(255, 255, 255), font=font)
                y_offset += 100

            # Add character if available
            if characters:
                char_idx = frame_num // (total_frames // len(characters)) if len(characters) > 1 else 0
                char_idx = min(char_idx, len(characters) - 1)
                char = characters[char_idx]

                # Scale character
                char_height = int(height * 0.5)
                char_width = int(char.width * (char_height / char.height))
                char_resized = char.resize((char_width, char_height))

                # Position at bottom center
                char_x = (width - char_width) // 2
                char_y = height - char_height + 50

                frame.paste(char_resized, (char_x, char_y), char_resized)

            # Save frame
            frame_path = self.frames_dir / f"frame_{frame_num:05d}.png"
            frame.save(frame_path)

            if frame_num % 100 == 0:
                console.print(f"  Frame {frame_num}/{total_frames}")

    async def _generate_voiceover(self) -> Path:
        """Generate TTS voiceover."""

        script = """So you think you know UNO? Nah. Let me tell you about NO MERCY.

168 cards. SIX players max. And if you get 25 cards in your hand? You're DEAD. Eliminated. Gone. That's the Mercy Rule and there IS no mercy.

Plus 2? That's cute. Plus 4? Getting warmer. PLUS 10. And guess what? You can STACK them. Someone hits you with a plus 4? Throw down a plus 6. Now THEY draw 10.

But here's what NO ONE tells you. That plus 4? It's NOT a wild card anymore. It has a COLOR. Red plus 4 only plays on RED.

Play a 7, you SWAP your entire hand with someone. Play a 0, EVERYONE passes their hand. Skip Everyone? You skip THE WHOLE TABLE.

And if you can't play? You draw until you CAN play. No stopping. Just pain.

This game has ended friendships. Who wants to play?"""

        audio_path = self.output_dir / "voiceover.mp3"

        # Save script
        script_path = self.output_dir / "script.txt"
        script_path.write_text(script)

        # Generate TTS using edge-tts
        import edge_tts

        communicate = edge_tts.Communicate(script, "en-US-GuyNeural", rate="+10%")
        await communicate.save(str(audio_path))

        return audio_path

    async def _compile_video(self, audio_path: Path) -> Path:
        """Compile frames and audio into final video."""

        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        output_path = self.output_dir / f"hybrid_video_{timestamp}.mp4"

        # Use FFmpeg to compile
        subprocess.run([
            "ffmpeg", "-y",
            "-framerate", "30",
            "-i", str(self.frames_dir / "frame_%05d.png"),
            "-i", str(audio_path),
            "-c:v", "libx264",
            "-preset", "medium",
            "-crf", "23",
            "-pix_fmt", "yuv420p",
            "-c:a", "aac",
            "-b:a", "192k",
            "-shortest",
            "-movflags", "+faststart",
            str(output_path),
        ], capture_output=True, check=True)

        return output_path


def main():
    import typer

    app = typer.Typer(help="Hybrid video generation pipeline")

    @app.command()
    def run(
        theme: str = typer.Argument("uno-no-mercy", help="Theme to generate"),
        style: str = typer.Option("cartoon", help="Visual style"),
        output_dir: Path = typer.Option(Path("./output"), help="Output directory"),
    ):
        """Run the complete hybrid pipeline."""

        pipeline = HybridPipeline(
            theme=theme,
            style=style,
            output_dir=output_dir,
        )
        asyncio.run(pipeline.run())

    @app.command()
    def quick_test():
        """Run a quick test with minimal frames."""
        console.print("[yellow]Quick test mode - generating 10 frames only[/yellow]")

        pipeline = HybridPipeline(
            theme="uno-no-mercy",
            style="cartoon",
            output_dir=Path("./output_test"),
        )
        asyncio.run(pipeline.run())

    app()


if __name__ == "__main__":
    main()
