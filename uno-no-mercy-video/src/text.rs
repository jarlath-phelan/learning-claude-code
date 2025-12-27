//! Text rendering module
//!
//! Handles text overlays and animated text effects.

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;

/// Text style configuration
#[derive(Clone)]
pub struct TextStyle {
    pub color: Rgba<u8>,
    pub outline_color: Option<Rgba<u8>>,
    pub outline_width: u32,
    pub shadow: bool,
    pub shadow_offset: (i32, i32),
    pub shadow_color: Rgba<u8>,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: Rgba([255, 255, 255, 255]),
            outline_color: Some(Rgba([0, 0, 0, 255])),
            outline_width: 3,
            shadow: true,
            shadow_offset: (4, 4),
            shadow_color: Rgba([0, 0, 0, 180]),
        }
    }
}

impl TextStyle {
    pub fn white_with_black_outline() -> Self {
        Self::default()
    }

    pub fn red_bold() -> Self {
        Self {
            color: Rgba([255, 50, 50, 255]),
            outline_color: Some(Rgba([255, 255, 255, 255])),
            outline_width: 4,
            shadow: true,
            shadow_offset: (5, 5),
            shadow_color: Rgba([0, 0, 0, 200]),
        }
    }

    pub fn yellow_impact() -> Self {
        Self {
            color: Rgba([255, 230, 50, 255]),
            outline_color: Some(Rgba([0, 0, 0, 255])),
            outline_width: 5,
            shadow: true,
            shadow_offset: (6, 6),
            shadow_color: Rgba([0, 0, 0, 220]),
        }
    }

    pub fn blue_clean() -> Self {
        Self {
            color: Rgba([100, 180, 255, 255]),
            outline_color: Some(Rgba([0, 0, 0, 255])),
            outline_width: 3,
            shadow: false,
            shadow_offset: (0, 0),
            shadow_color: Rgba([0, 0, 0, 0]),
        }
    }
}

/// Text renderer with various effects
pub struct TextRenderer {
    font_data: &'static [u8],
}

impl TextRenderer {
    pub fn new() -> Self {
        let font_data: &'static [u8] = include_bytes!("../fonts/Roboto-Bold.ttf");
        Self { font_data }
    }

    fn get_font(&self) -> FontRef<'_> {
        FontRef::try_from_slice(self.font_data).expect("Failed to load font")
    }

    /// Calculate text dimensions
    fn text_dimensions(&self, text: &str, size: f32) -> (u32, u32) {
        let font = self.get_font();
        let scale = PxScale::from(size);
        let scaled_font = font.as_scaled(scale);

        let mut width = 0.0f32;
        for c in text.chars() {
            width += scaled_font.h_advance(scaled_font.glyph_id(c));
        }

        let height = scaled_font.height();
        (width.ceil() as u32, height.ceil() as u32)
    }

    /// Render text with style to a new image
    pub fn render(&self, text: &str, size: f32, style: &TextStyle) -> RgbaImage {
        let font = self.get_font();
        let scale = PxScale::from(size);

        // Calculate text dimensions
        let (width, height) = self.text_dimensions(text, size);

        // Add padding for outline and shadow
        let padding = style.outline_width + style.shadow_offset.0.unsigned_abs().max(style.shadow_offset.1.unsigned_abs());
        let img_width = width + padding * 2 + 10; // Extra padding for safety
        let img_height = height + padding * 2 + 10;

        let mut img = RgbaImage::new(img_width, img_height);

        // Draw shadow
        if style.shadow {
            draw_text_mut(
                &mut img,
                style.shadow_color,
                padding as i32 + style.shadow_offset.0,
                padding as i32 + style.shadow_offset.1,
                scale,
                &font,
                text,
            );
        }

        // Draw outline (by drawing text multiple times offset)
        if let Some(outline_color) = style.outline_color {
            let offsets: Vec<(i32, i32)> = (-1..=1)
                .flat_map(|x| (-1..=1).map(move |y| (x, y)))
                .filter(|&(x, y)| x != 0 || y != 0)
                .collect();

            for w in 1..=style.outline_width as i32 {
                for (ox, oy) in &offsets {
                    draw_text_mut(
                        &mut img,
                        outline_color,
                        padding as i32 + ox * w,
                        padding as i32 + oy * w,
                        scale,
                        &font,
                        text,
                    );
                }
            }
        }

        // Draw main text
        draw_text_mut(
            &mut img,
            style.color,
            padding as i32,
            padding as i32,
            scale,
            &font,
            text,
        );

        img
    }

    /// Render text centered on a canvas
    pub fn render_centered(&self, text: &str, size: f32, style: &TextStyle, canvas_width: u32, canvas_height: u32) -> RgbaImage {
        let text_img = self.render(text, size, style);
        let mut canvas = RgbaImage::new(canvas_width, canvas_height);

        let x = ((canvas_width as i32 - text_img.width() as i32) / 2).max(0);
        let y = ((canvas_height as i32 - text_img.height() as i32) / 2).max(0);

        Self::composite(&mut canvas, &text_img, x, y);
        canvas
    }

    /// Composite one image onto another
    pub fn composite(dest: &mut RgbaImage, src: &RgbaImage, x: i32, y: i32) {
        for (sx, sy, pixel) in src.enumerate_pixels() {
            let dx = x + sx as i32;
            let dy = y + sy as i32;

            if dx >= 0 && dy >= 0 && (dx as u32) < dest.width() && (dy as u32) < dest.height() {
                let src_alpha = pixel[3] as f32 / 255.0;
                if src_alpha > 0.0 {
                    let dest_pixel = dest.get_pixel(dx as u32, dy as u32);
                    let dest_alpha = dest_pixel[3] as f32 / 255.0;

                    // Alpha blending
                    let out_alpha = src_alpha + dest_alpha * (1.0 - src_alpha);
                    if out_alpha > 0.0 {
                        let r = (pixel[0] as f32 * src_alpha + dest_pixel[0] as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;
                        let g = (pixel[1] as f32 * src_alpha + dest_pixel[1] as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;
                        let b = (pixel[2] as f32 * src_alpha + dest_pixel[2] as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;

                        dest.put_pixel(dx as u32, dy as u32, Rgba([r as u8, g as u8, b as u8, (out_alpha * 255.0) as u8]));
                    }
                }
            }
        }
    }
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Animated text effects
pub struct AnimatedText;

impl AnimatedText {
    /// Create shaking text effect by rendering at offset positions
    pub fn shake_offset(time: f32, intensity: f32) -> (i32, i32) {
        let x = (time * 50.0).sin() * intensity;
        let y = (time * 47.0).cos() * intensity;
        (x as i32, y as i32)
    }

    /// Create typewriter reveal progress (0.0 to 1.0 = full text visible)
    pub fn typewriter_progress(text: &str, progress: f32) -> &str {
        let char_count = text.chars().count();
        let visible_chars = (char_count as f32 * progress.clamp(0.0, 1.0)).ceil() as usize;
        let mut end_byte = 0;
        for (i, (byte_idx, _)) in text.char_indices().enumerate() {
            if i >= visible_chars {
                break;
            }
            end_byte = byte_idx + text[byte_idx..].chars().next().unwrap().len_utf8();
        }
        &text[..end_byte]
    }

    /// Wave effect - get y offset for each character based on position
    pub fn wave_offset(char_index: usize, time: f32, amplitude: f32, frequency: f32) -> i32 {
        let phase = char_index as f32 * 0.5;
        (((time * frequency + phase).sin()) * amplitude) as i32
    }
}

/// Title card renderer
pub struct TitleCard;

impl TitleCard {
    /// Create a dramatic title card
    pub fn render(title: &str, subtitle: Option<&str>, width: u32, height: u32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);

        // Dark gradient background
        for y in 0..height {
            let t = y as f32 / height as f32;
            let gray = (20.0 + 30.0 * t) as u8;
            for x in 0..width {
                img.put_pixel(x, y, Rgba([gray, gray / 2, gray / 2, 255]));
            }
        }

        let text_renderer = TextRenderer::new();

        // Main title
        let title_style = TextStyle::red_bold();
        let title_size = (width as f32 * 0.12).min(height as f32 * 0.08);
        let title_img = text_renderer.render(title, title_size, &title_style);

        let title_x = ((width as i32 - title_img.width() as i32) / 2).max(0);
        let title_y = if subtitle.is_some() {
            (height as f32 * 0.35) as i32
        } else {
            (height as f32 * 0.45) as i32
        };
        TextRenderer::composite(&mut img, &title_img, title_x, title_y);

        // Subtitle
        if let Some(sub) = subtitle {
            let sub_style = TextStyle::white_with_black_outline();
            let sub_size = title_size * 0.5;
            let sub_img = text_renderer.render(sub, sub_size, &sub_style);

            let sub_x = ((width as i32 - sub_img.width() as i32) / 2).max(0);
            let sub_y = title_y + title_img.height() as i32 + 20;
            TextRenderer::composite(&mut img, &sub_img, sub_x, sub_y);
        }

        img
    }
}

/// Lower third (caption bar)
pub struct LowerThird;

impl LowerThird {
    /// Create a lower third caption bar
    pub fn render(text: &str, width: u32, bar_height: u32) -> RgbaImage {
        let mut img = RgbaImage::new(width, bar_height);

        // Semi-transparent dark background
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, 0).of_size(width, bar_height),
            Rgba([20, 20, 20, 200]),
        );

        // Red accent bar at top
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, 0).of_size(width, 4),
            Rgba([220, 50, 50, 255]),
        );

        // Text
        let text_renderer = TextRenderer::new();
        let style = TextStyle {
            color: Rgba([255, 255, 255, 255]),
            outline_color: None,
            outline_width: 0,
            shadow: false,
            shadow_offset: (0, 0),
            shadow_color: Rgba([0, 0, 0, 0]),
        };

        let text_size = (bar_height as f32 * 0.5).min(width as f32 * 0.04);
        let text_img = text_renderer.render(text, text_size, &style);

        let x = ((width as i32 - text_img.width() as i32) / 2).max(0);
        let y = ((bar_height as i32 - text_img.height() as i32) / 2).max(0);
        TextRenderer::composite(&mut img, &text_img, x, y);

        img
    }
}
