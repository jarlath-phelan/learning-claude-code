# TikTok Video Generation Comparison

Compare three approaches for programmatically generating TikTok-style explainer videos.

## Approaches

| # | Approach | Description | Effort | Cost |
|---|----------|-------------|--------|------|
| 1 | [SaaS Tools](./approach-1-saas/) | Use InVideo, Synthesia, Canva with optimized prompts | Low | $20-50/mo |
| 2 | [API Pipeline](./approach-2-api-pipeline/) | Python automation with Claude + Replicate + TTS | Medium | $0.15-0.50/video |
| 3 | [Hybrid](./approach-3-hybrid/) | AI-generated assets + Rust frame engine | High | $0.10-0.20/video |

## Quick Start

### Try Approach 1 (Fastest)
```bash
cat approach-1-saas/prompts/uno-no-mercy.txt
# Copy output to invideo.io or canva.com
```

### Try Approach 2 (Automated)
```bash
cd approach-2-api-pipeline
pip install -r requirements.txt
cp .env.example .env  # Add API keys
python pipeline.py generate "UNO No Mercy rules"
```

### Try Approach 3 (Premium)
```bash
cd approach-3-hybrid
pip install -r requirements.txt
cp .env.example .env  # Add API keys
python orchestrate.py run uno-no-mercy
```

## Full Comparison

See [COMPARISON.md](./COMPARISON.md) for detailed analysis including:
- Cost breakdowns
- Quality benchmarks
- Decision matrix
- Evolution path recommendations

## Example Content

Both UNO No Mercy and Financial Basics explainer prompts/scripts are included.

## Requirements

- Python 3.10+
- FFmpeg (for video compilation)
- API keys (Anthropic, Replicate, OpenAI - depending on approach)
- Optional: Rust toolchain for Approach 3

## Project Structure

```
tiktok-video-comparison/
├── README.md              # This file
├── COMPARISON.md          # Detailed comparison
├── approach-1-saas/       # SaaS prompts and templates
├── approach-2-api-pipeline/   # Python automation pipeline
└── approach-3-hybrid/     # Hybrid AI + Rust approach
```
