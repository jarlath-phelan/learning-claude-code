//! Video composition module
//!
//! Handles frame composition and image layering.

use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

/// Frame composer for layering multiple elements
pub struct FrameComposer {
    width: u32,
    height: u32,
}

impl FrameComposer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Create a new empty frame with background color
    pub fn create_frame(&self, bg_color: Rgba<u8>) -> RgbaImage {
        let mut frame = RgbaImage::new(self.width, self.height);
        draw_filled_rect_mut(
            &mut frame,
            Rect::at(0, 0).of_size(self.width, self.height),
            bg_color,
        );
        frame
    }

    /// Create a frame with gradient background
    pub fn create_gradient_frame(&self, top_color: Rgba<u8>, bottom_color: Rgba<u8>) -> RgbaImage {
        let mut frame = RgbaImage::new(self.width, self.height);

        for y in 0..self.height {
            let t = y as f32 / self.height as f32;
            let color = Self::lerp_color(top_color, bottom_color, t);
            for x in 0..self.width {
                frame.put_pixel(x, y, color);
            }
        }

        frame
    }

    /// Composite an image onto the frame at given position
    pub fn composite(&self, frame: &mut RgbaImage, layer: &RgbaImage, x: i32, y: i32) {
        self.composite_with_alpha(frame, layer, x, y, 1.0);
    }

    /// Composite with alpha multiplier
    pub fn composite_with_alpha(&self, frame: &mut RgbaImage, layer: &RgbaImage, x: i32, y: i32, alpha: f32) {
        for (lx, ly, pixel) in layer.enumerate_pixels() {
            let fx = x + lx as i32;
            let fy = y + ly as i32;

            if fx >= 0 && fy >= 0 && (fx as u32) < frame.width() && (fy as u32) < frame.height() {
                let src_alpha = (pixel[3] as f32 / 255.0) * alpha;
                if src_alpha > 0.0 {
                    let dest_pixel = frame.get_pixel(fx as u32, fy as u32);
                    let dest_alpha = dest_pixel[3] as f32 / 255.0;

                    let out_alpha = src_alpha + dest_alpha * (1.0 - src_alpha);
                    if out_alpha > 0.0 {
                        let r = (pixel[0] as f32 * src_alpha + dest_pixel[0] as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;
                        let g = (pixel[1] as f32 * src_alpha + dest_pixel[1] as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;
                        let b = (pixel[2] as f32 * src_alpha + dest_pixel[2] as f32 * dest_alpha * (1.0 - src_alpha)) / out_alpha;

                        frame.put_pixel(fx as u32, fy as u32, Rgba([r as u8, g as u8, b as u8, (out_alpha * 255.0) as u8]));
                    }
                }
            }
        }
    }

    /// Composite centered on frame
    pub fn composite_centered(&self, frame: &mut RgbaImage, layer: &RgbaImage) {
        let x = (self.width as i32 - layer.width() as i32) / 2;
        let y = (self.height as i32 - layer.height() as i32) / 2;
        self.composite(frame, layer, x, y);
    }

    /// Composite at relative position (0.0-1.0 coordinates)
    pub fn composite_relative(&self, frame: &mut RgbaImage, layer: &RgbaImage, rel_x: f32, rel_y: f32) {
        let x = (self.width as f32 * rel_x) as i32 - layer.width() as i32 / 2;
        let y = (self.height as f32 * rel_y) as i32 - layer.height() as i32 / 2;
        self.composite(frame, layer, x, y);
    }

    /// Scale an image
    pub fn scale_image(img: &RgbaImage, scale: f32) -> RgbaImage {
        if (scale - 1.0).abs() < 0.001 {
            return img.clone();
        }

        let new_width = ((img.width() as f32 * scale).round() as u32).max(1);
        let new_height = ((img.height() as f32 * scale).round() as u32).max(1);

        image::imageops::resize(img, new_width, new_height, image::imageops::FilterType::Triangle)
    }

    /// Crop an image to fit within bounds
    pub fn crop_to_fit(img: &RgbaImage, max_width: u32, max_height: u32) -> RgbaImage {
        if img.width() <= max_width && img.height() <= max_height {
            return img.clone();
        }

        let scale_w = max_width as f32 / img.width() as f32;
        let scale_h = max_height as f32 / img.height() as f32;
        let scale = scale_w.min(scale_h);

        Self::scale_image(img, scale)
    }

    fn lerp_color(c1: Rgba<u8>, c2: Rgba<u8>, t: f32) -> Rgba<u8> {
        Rgba([
            (c1[0] as f32 + (c2[0] as f32 - c1[0] as f32) * t) as u8,
            (c1[1] as f32 + (c2[1] as f32 - c1[1] as f32) * t) as u8,
            (c1[2] as f32 + (c2[2] as f32 - c1[2] as f32) * t) as u8,
            (c1[3] as f32 + (c2[3] as f32 - c1[3] as f32) * t) as u8,
        ])
    }
}

/// Background patterns and effects
pub struct Backgrounds;

impl Backgrounds {
    /// Create UNO-themed background with color segments
    pub fn uno_theme(width: u32, height: u32, time: f32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);

        // Base dark color
        draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(width, height), Rgba([20, 20, 30, 255]));

        // Animated color bars on sides
        let colors = [
            Rgba([220, 50, 50, 80]),   // Red
            Rgba([50, 100, 200, 80]),  // Blue
            Rgba([50, 180, 80, 80]),   // Green
            Rgba([250, 200, 50, 80]),  // Yellow
        ];

        let bar_height = height / 4;
        for (i, color) in colors.iter().enumerate() {
            let y = (i as u32 * bar_height) as i32;
            let offset = ((time * 2.0 + i as f32 * 0.5).sin() * 20.0) as i32;

            // Left bar
            draw_filled_rect_mut(
                &mut img,
                Rect::at(offset, y).of_size(50, bar_height),
                *color,
            );

            // Right bar
            draw_filled_rect_mut(
                &mut img,
                Rect::at(width as i32 - 50 - offset, y).of_size(50, bar_height),
                *color,
            );
        }

        img
    }

    /// Create dramatic dark background with spotlight
    pub fn spotlight(width: u32, height: u32, spotlight_x: f32, spotlight_y: f32, intensity: f32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);

        let cx = width as f32 * spotlight_x;
        let cy = height as f32 * spotlight_y;
        let max_dist = (width.max(height) as f32) * 0.7;

        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                let brightness = (1.0 - (dist / max_dist).min(1.0)) * intensity;
                let base = 10;
                let lit = (base as f32 + 60.0 * brightness) as u8;

                img.put_pixel(x, y, Rgba([lit, lit / 2, lit / 2, 255]));
            }
        }

        img
    }

    /// Create chaotic/glitch-style background
    pub fn chaos(width: u32, height: u32, time: f32, seed: u64) -> RgbaImage {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;

        let mut img = RgbaImage::new(width, height);
        let mut rng = StdRng::seed_from_u64(seed + (time * 10.0) as u64);

        // Dark base
        draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(width, height), Rgba([15, 10, 20, 255]));

        // Random colored rectangles
        let colors = [
            Rgba([220, 50, 50, 100]),
            Rgba([50, 100, 200, 100]),
            Rgba([50, 180, 80, 100]),
            Rgba([250, 200, 50, 100]),
        ];

        for _ in 0..20 {
            let color = colors[rng.gen_range(0..4)];
            let x = rng.gen_range(0..width as i32);
            let y = rng.gen_range(0..height as i32);
            let w = rng.gen_range(20..200) as u32;
            let h = rng.gen_range(10..100) as u32;

            draw_filled_rect_mut(&mut img, Rect::at(x, y).of_size(w, h), color);
        }

        img
    }

    /// Simple solid color with vignette
    pub fn solid_with_vignette(width: u32, height: u32, base_color: Rgba<u8>, vignette_strength: f32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);

        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;
        let max_dist = (cx * cx + cy * cy).sqrt();

        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt() / max_dist;

                let vignette = 1.0 - (dist * dist * vignette_strength).min(0.7);
                let r = (base_color[0] as f32 * vignette) as u8;
                let g = (base_color[1] as f32 * vignette) as u8;
                let b = (base_color[2] as f32 * vignette) as u8;

                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }

        img
    }
}
