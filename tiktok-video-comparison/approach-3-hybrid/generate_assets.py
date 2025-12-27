#!/usr/bin/env python3
"""
AI Asset Generator for Hybrid Video Pipeline

Generates backgrounds, characters, and props using AI image generation,
then the Rust engine uses these assets for frame-by-frame video creation.
"""

import asyncio
import os
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

import httpx
from dotenv import load_dotenv
from PIL import Image
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn

load_dotenv()
console = Console()


# =============================================================================
# Configuration
# =============================================================================

@dataclass
class AssetConfig:
    """Configuration for a single asset to generate."""

    name: str
    prompt: str
    width: int
    height: int
    style_suffix: str = ""
    remove_background: bool = False


# Theme definitions with asset prompts
THEMES = {
    "uno-no-mercy": {
        "description": "UNO No Mercy card game explainer",
        "backgrounds": [
            AssetConfig(
                name="bg_intro",
                prompt="Dark dramatic background with subtle UNO card colors (red, blue, green, yellow) as accent lighting, abstract geometric shapes, cinematic mood",
                width=1080,
                height=1920,
            ),
            AssetConfig(
                name="bg_dramatic",
                prompt="Intense dark background with spotlight effect, red and black color scheme, dramatic shadows, tension atmosphere",
                width=1080,
                height=1920,
            ),
            AssetConfig(
                name="bg_chaos",
                prompt="Chaotic abstract background with scattered playing cards, glitch effects, neon UNO colors on dark background, motion blur",
                width=1080,
                height=1920,
            ),
            AssetConfig(
                name="bg_outro",
                prompt="Ominous dark background with subtle evil red glow, vignette effect, minimal and foreboding atmosphere",
                width=1080,
                height=1920,
            ),
        ],
        "characters": [
            AssetConfig(
                name="character_neutral",
                prompt="Cartoon-style young adult character, casual confident pose, neutral expression, colorful modern clothing, facing forward, clean lines",
                width=512,
                height=768,
                remove_background=True,
            ),
            AssetConfig(
                name="character_shocked",
                prompt="Cartoon-style young adult character, shocked surprised expression, wide eyes, hands up in disbelief, expressive pose",
                width=512,
                height=768,
                remove_background=True,
            ),
            AssetConfig(
                name="character_mischievous",
                prompt="Cartoon-style young adult character, mischievous evil grin, raised eyebrow, scheming pose, hands together like plotting",
                width=512,
                height=768,
                remove_background=True,
            ),
            AssetConfig(
                name="character_serious",
                prompt="Cartoon-style young adult character, dead serious intense expression, arms crossed, stern look, authoritative pose",
                width=512,
                height=768,
                remove_background=True,
            ),
        ],
        "props": [
            AssetConfig(
                name="card_stack",
                prompt="Stack of colorful UNO cards, red blue green yellow, fanned out arrangement, dramatic lighting, isolated on transparent background",
                width=400,
                height=300,
                remove_background=True,
            ),
            AssetConfig(
                name="plus_ten_card",
                prompt="Single UNO card with +10 text, dramatic glow effect, rainbow wild card colors, menacing appearance",
                width=200,
                height=300,
                remove_background=True,
            ),
        ],
    },
    "financial-basics": {
        "description": "Personal finance fundamentals explainer",
        "backgrounds": [
            AssetConfig(
                name="bg_intro",
                prompt="Clean modern gradient background, soft blue and green tones, professional financial aesthetic, subtle geometric patterns",
                width=1080,
                height=1920,
            ),
            AssetConfig(
                name="bg_growth",
                prompt="Abstract upward trending graph background, green glow, wealth and prosperity feeling, clean minimal design",
                width=1080,
                height=1920,
            ),
            AssetConfig(
                name="bg_warning",
                prompt="Subtle red gradient background, warning atmosphere, credit card silhouette faded in background, cautionary mood",
                width=1080,
                height=1920,
            ),
            AssetConfig(
                name="bg_success",
                prompt="Triumphant golden hour gradient background, warm orange and yellow tones, achievement and success feeling",
                width=1080,
                height=1920,
            ),
        ],
        "characters": [
            AssetConfig(
                name="character_friendly",
                prompt="Friendly professional cartoon character, business casual attire, welcoming smile, approachable pose, clean modern style",
                width=512,
                height=768,
                remove_background=True,
            ),
            AssetConfig(
                name="character_explaining",
                prompt="Professional cartoon character pointing and explaining, hand gesture toward side, teaching pose, confident expression",
                width=512,
                height=768,
                remove_background=True,
            ),
        ],
        "props": [
            AssetConfig(
                name="pie_chart",
                prompt="Clean 3D pie chart with three sections in blue, green, and orange, modern infographic style, transparent background",
                width=400,
                height=400,
                remove_background=True,
            ),
            AssetConfig(
                name="money_stack",
                prompt="Stack of dollar bills with coins, clean illustration style, growth arrow, transparent background",
                width=300,
                height=300,
                remove_background=True,
            ),
        ],
    },
}

# Style modifiers
STYLES = {
    "cartoon": ", cartoon style, 2D animated, clean lines, vibrant colors, Pixar-like quality",
    "anime": ", anime style, Japanese animation, expressive, detailed shading",
    "realistic": ", photorealistic, high detail, cinematic lighting, 8K quality",
    "minimal": ", minimalist design, flat colors, clean geometric shapes, modern aesthetic",
    "retro": ", retro 80s style, neon colors, synthwave aesthetic, VHS texture",
}


# =============================================================================
# Image Generation Providers
# =============================================================================

class ImageGenerator:
    """Generate images using AI APIs."""

    def __init__(self, provider: str = "replicate"):
        self.provider = provider
        self.replicate_token = os.getenv("REPLICATE_API_TOKEN")
        self.openai_key = os.getenv("OPENAI_API_KEY")

    async def generate(
        self,
        prompt: str,
        width: int,
        height: int,
        output_path: Path,
    ) -> Path:
        """Generate an image from prompt."""

        if self.provider == "replicate" and self.replicate_token:
            return await self._generate_replicate(prompt, width, height, output_path)
        elif self.provider == "openai" and self.openai_key:
            return await self._generate_openai(prompt, width, height, output_path)
        else:
            return await self._generate_placeholder(prompt, width, height, output_path)

    async def _generate_replicate(
        self, prompt: str, width: int, height: int, output_path: Path
    ) -> Path:
        """Generate using Replicate (SDXL)."""

        async with httpx.AsyncClient(timeout=120.0) as client:
            # Start prediction
            response = await client.post(
                "https://api.replicate.com/v1/predictions",
                headers={"Authorization": f"Token {self.replicate_token}"},
                json={
                    "version": "39ed52f2a78e934b3ba6e2a89f5b1c712de7dfea535525255b1aa35c5565e08b",  # SDXL
                    "input": {
                        "prompt": prompt,
                        "width": min(width, 1024),
                        "height": min(height, 1024),
                        "num_inference_steps": 30,
                    },
                },
            )
            response.raise_for_status()
            prediction = response.json()

            # Poll for completion
            while True:
                await asyncio.sleep(3)
                status_response = await client.get(
                    prediction["urls"]["get"],
                    headers={"Authorization": f"Token {self.replicate_token}"},
                )
                result = status_response.json()

                if result["status"] == "succeeded":
                    image_url = result["output"][0]
                    break
                elif result["status"] == "failed":
                    raise RuntimeError(f"Generation failed: {result.get('error')}")

            # Download and resize
            img_response = await client.get(image_url)
            output_path.write_bytes(img_response.content)

            # Resize if needed
            if width > 1024 or height > 1024:
                img = Image.open(output_path)
                img = img.resize((width, height), Image.Resampling.LANCZOS)
                img.save(output_path)

            return output_path

    async def _generate_openai(
        self, prompt: str, width: int, height: int, output_path: Path
    ) -> Path:
        """Generate using OpenAI DALL-E 3."""

        # DALL-E 3 supports specific sizes
        size = "1024x1792" if height > width else "1792x1024"
        if width == height:
            size = "1024x1024"

        async with httpx.AsyncClient(timeout=120.0) as client:
            response = await client.post(
                "https://api.openai.com/v1/images/generations",
                headers={"Authorization": f"Bearer {self.openai_key}"},
                json={
                    "model": "dall-e-3",
                    "prompt": prompt,
                    "size": size,
                    "quality": "standard",
                    "n": 1,
                },
            )
            response.raise_for_status()
            result = response.json()

            image_url = result["data"][0]["url"]
            img_response = await client.get(image_url)
            output_path.write_bytes(img_response.content)

            # Resize to exact dimensions
            img = Image.open(output_path)
            img = img.resize((width, height), Image.Resampling.LANCZOS)
            img.save(output_path)

            return output_path

    async def _generate_placeholder(
        self, prompt: str, width: int, height: int, output_path: Path
    ) -> Path:
        """Generate a placeholder image (no API key)."""

        from PIL import ImageDraw, ImageFont

        # Create gradient background
        img = Image.new("RGB", (width, height))
        for y in range(height):
            r = int(30 + (y / height) * 20)
            g = int(30 + (y / height) * 10)
            b = int(40 + (y / height) * 30)
            for x in range(width):
                img.putpixel((x, y), (r, g, b))

        draw = ImageDraw.Draw(img)

        # Add prompt text
        try:
            font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 24)
        except OSError:
            font = ImageFont.load_default()

        # Wrap text
        words = prompt.split()
        lines = []
        current = []
        for word in words:
            current.append(word)
            if len(" ".join(current)) > 40:
                lines.append(" ".join(current[:-1]))
                current = [word]
        if current:
            lines.append(" ".join(current))

        y_offset = height // 2 - len(lines) * 15
        for line in lines[:8]:
            bbox = draw.textbbox((0, 0), line, font=font)
            text_width = bbox[2] - bbox[0]
            x = (width - text_width) // 2
            draw.text((x, y_offset), line, fill=(200, 200, 200), font=font)
            y_offset += 30

        # Add "PLACEHOLDER" watermark
        draw.text((10, 10), "PLACEHOLDER - Add API key for real images", fill=(100, 100, 100), font=font)

        img.save(output_path)
        return output_path


def remove_background(image_path: Path) -> Path:
    """Remove background from image using rembg."""

    try:
        from rembg import remove

        img = Image.open(image_path)
        output = remove(img)
        output.save(image_path)
        return image_path
    except ImportError:
        console.print("[yellow]rembg not installed, skipping background removal[/yellow]")
        return image_path


# =============================================================================
# Asset Generation Pipeline
# =============================================================================

async def generate_theme_assets(
    theme_name: str,
    style_name: str = "cartoon",
    output_dir: Path = Path("./assets"),
    provider: str = "replicate",
):
    """Generate all assets for a theme."""

    if theme_name not in THEMES:
        console.print(f"[red]Unknown theme: {theme_name}[/red]")
        console.print(f"Available themes: {', '.join(THEMES.keys())}")
        return

    theme = THEMES[theme_name]
    style_suffix = STYLES.get(style_name, "")

    console.print(f"\n[bold]Generating assets for: {theme['description']}[/bold]")
    console.print(f"Style: {style_name}")
    console.print(f"Provider: {provider}\n")

    # Create output directories
    dirs = {
        "backgrounds": output_dir / "backgrounds",
        "characters": output_dir / "characters",
        "props": output_dir / "props",
    }
    for d in dirs.values():
        d.mkdir(parents=True, exist_ok=True)

    generator = ImageGenerator(provider)

    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        console=console,
    ) as progress:
        # Generate backgrounds
        for asset in theme.get("backgrounds", []):
            task = progress.add_task(f"Generating {asset.name}...", total=None)
            output_path = dirs["backgrounds"] / f"{asset.name}.png"

            await generator.generate(
                asset.prompt + style_suffix,
                asset.width,
                asset.height,
                output_path,
            )
            progress.update(task, description=f"[green]{asset.name} complete!")

        # Generate characters
        for asset in theme.get("characters", []):
            task = progress.add_task(f"Generating {asset.name}...", total=None)
            output_path = dirs["characters"] / f"{asset.name}.png"

            await generator.generate(
                asset.prompt + style_suffix,
                asset.width,
                asset.height,
                output_path,
            )

            if asset.remove_background:
                remove_background(output_path)

            progress.update(task, description=f"[green]{asset.name} complete!")

        # Generate props
        for asset in theme.get("props", []):
            task = progress.add_task(f"Generating {asset.name}...", total=None)
            output_path = dirs["props"] / f"{asset.name}.png"

            await generator.generate(
                asset.prompt + style_suffix,
                asset.width,
                asset.height,
                output_path,
            )

            if asset.remove_background:
                remove_background(output_path)

            progress.update(task, description=f"[green]{asset.name} complete!")

    # Generate manifest
    manifest = {
        "theme": theme_name,
        "style": style_name,
        "description": theme["description"],
        "assets": {
            "backgrounds": [f"backgrounds/{a.name}.png" for a in theme.get("backgrounds", [])],
            "characters": [f"characters/{a.name}.png" for a in theme.get("characters", [])],
            "props": [f"props/{a.name}.png" for a in theme.get("props", [])],
        },
    }

    import json
    with open(output_dir / "manifest.json", "w") as f:
        json.dump(manifest, f, indent=2)

    console.print(f"\n[bold green]Assets generated in: {output_dir}[/bold green]")
    console.print(f"Manifest: {output_dir}/manifest.json")


# =============================================================================
# CLI Interface
# =============================================================================

def main():
    import typer

    app = typer.Typer(help="Generate AI assets for hybrid video pipeline")

    @app.command()
    def generate(
        theme: str = typer.Argument(..., help="Theme name (uno-no-mercy, financial-basics)"),
        style: str = typer.Option("cartoon", help="Visual style (cartoon, anime, realistic, minimal, retro)"),
        output_dir: Path = typer.Option(Path("./assets"), help="Output directory"),
        provider: str = typer.Option("replicate", help="AI provider (replicate, openai)"),
    ):
        """Generate all assets for a theme."""
        asyncio.run(generate_theme_assets(theme, style, output_dir, provider))

    @app.command()
    def list_themes():
        """List available themes."""
        console.print("\n[bold]Available Themes:[/bold]")
        for name, config in THEMES.items():
            console.print(f"  - {name}: {config['description']}")

        console.print("\n[bold]Available Styles:[/bold]")
        for name in STYLES.keys():
            console.print(f"  - {name}")

    @app.command()
    def preview(assets_dir: Path = typer.Argument(Path("./assets"))):
        """Preview generated assets."""
        import json

        manifest_path = assets_dir / "manifest.json"
        if not manifest_path.exists():
            console.print("[red]No manifest.json found. Generate assets first.[/red]")
            return

        with open(manifest_path) as f:
            manifest = json.load(f)

        console.print(f"\n[bold]Theme: {manifest['theme']}[/bold]")
        console.print(f"Style: {manifest['style']}")
        console.print(f"Description: {manifest['description']}\n")

        for category, files in manifest["assets"].items():
            console.print(f"[bold]{category.title()}:[/bold]")
            for f in files:
                path = assets_dir / f
                status = "[green]exists[/green]" if path.exists() else "[red]missing[/red]"
                console.print(f"  - {f} {status}")
            console.print()

    app()


if __name__ == "__main__":
    main()
