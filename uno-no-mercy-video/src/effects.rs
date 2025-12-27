//! Animation effects module
//!
//! Provides various visual effects for the video.

use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut};
use imageproc::rect::Rect;

/// Easing functions for animations
pub struct Easing;

impl Easing {
    /// Linear interpolation
    pub fn linear(t: f32) -> f32 {
        t.clamp(0.0, 1.0)
    }

    /// Ease in (slow start)
    pub fn ease_in(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        t * t
    }

    /// Ease out (slow end)
    pub fn ease_out(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        1.0 - (1.0 - t) * (1.0 - t)
    }

    /// Ease in-out
    pub fn ease_in_out(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
        }
    }

    /// Bounce effect
    pub fn bounce(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        if t < 1.0 / 2.75 {
            7.5625 * t * t
        } else if t < 2.0 / 2.75 {
            let t = t - 1.5 / 2.75;
            7.5625 * t * t + 0.75
        } else if t < 2.5 / 2.75 {
            let t = t - 2.25 / 2.75;
            7.5625 * t * t + 0.9375
        } else {
            let t = t - 2.625 / 2.75;
            7.5625 * t * t + 0.984375
        }
    }

    /// Elastic effect
    pub fn elastic(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        if t == 0.0 || t == 1.0 {
            return t;
        }
        let p = 0.3;
        let s = p / 4.0;
        let t = t - 1.0;
        -(2.0_f32.powf(10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin())
    }
}

/// Screen shake effect
pub struct ScreenShake {
    intensity: f32,
    frequency: f32,
}

impl ScreenShake {
    pub fn new(intensity: f32, frequency: f32) -> Self {
        Self { intensity, frequency }
    }

    /// Get shake offset for a given time
    pub fn get_offset(&self, time: f32) -> (i32, i32) {
        let x = (time * self.frequency * 17.0).sin() * self.intensity;
        let y = (time * self.frequency * 23.0).cos() * self.intensity;
        (x as i32, y as i32)
    }
}

/// Zoom effect
pub struct Zoom;

impl Zoom {
    /// Calculate scale factor for zoom animation
    pub fn calculate_scale(start_scale: f32, end_scale: f32, progress: f32, easing: fn(f32) -> f32) -> f32 {
        let t = easing(progress);
        start_scale + (end_scale - start_scale) * t
    }
}

/// Fade effect
pub struct Fade;

impl Fade {
    /// Apply fade to an image
    pub fn apply(img: &mut RgbaImage, alpha: f32) {
        let alpha = (alpha.clamp(0.0, 1.0) * 255.0) as u8;
        for pixel in img.pixels_mut() {
            pixel[3] = ((pixel[3] as f32 * alpha as f32) / 255.0) as u8;
        }
    }

    /// Fade from black
    pub fn from_black(width: u32, height: u32, progress: f32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);
        let alpha = ((1.0 - progress.clamp(0.0, 1.0)) * 255.0) as u8;
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, 0).of_size(width, height),
            Rgba([0, 0, 0, alpha]),
        );
        img
    }

    /// Fade to black
    pub fn to_black(width: u32, height: u32, progress: f32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);
        let alpha = (progress.clamp(0.0, 1.0) * 255.0) as u8;
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, 0).of_size(width, height),
            Rgba([0, 0, 0, alpha]),
        );
        img
    }
}

/// Flash effect
pub struct Flash;

impl Flash {
    /// Create a white flash overlay
    pub fn white(width: u32, height: u32, intensity: f32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);
        let alpha = (intensity.clamp(0.0, 1.0) * 255.0) as u8;
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, 0).of_size(width, height),
            Rgba([255, 255, 255, alpha]),
        );
        img
    }

    /// Create a colored flash overlay
    pub fn colored(width: u32, height: u32, color: Rgba<u8>, intensity: f32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);
        let alpha = (intensity.clamp(0.0, 1.0) * color[3] as f32) as u8;
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, 0).of_size(width, height),
            Rgba([color[0], color[1], color[2], alpha]),
        );
        img
    }
}

/// Pulse effect for text/elements
pub struct Pulse;

impl Pulse {
    /// Get scale factor for pulsing animation
    pub fn get_scale(time: f32, base_scale: f32, amplitude: f32, frequency: f32) -> f32 {
        base_scale + amplitude * (time * frequency * 2.0 * std::f32::consts::PI).sin()
    }
}

/// Slide animation
pub struct Slide;

impl Slide {
    /// Calculate position for slide-in from left
    pub fn from_left(start_x: i32, end_x: i32, progress: f32, easing: fn(f32) -> f32) -> i32 {
        let t = easing(progress);
        (start_x as f32 + (end_x - start_x) as f32 * t) as i32
    }

    /// Calculate position for slide-in from right
    pub fn from_right(canvas_width: i32, element_width: i32, progress: f32, easing: fn(f32) -> f32) -> i32 {
        let start_x = canvas_width;
        let end_x = (canvas_width - element_width) / 2;
        Self::from_left(start_x, end_x, progress, easing)
    }

    /// Calculate position for slide-in from top
    pub fn from_top(start_y: i32, end_y: i32, progress: f32, easing: fn(f32) -> f32) -> i32 {
        let t = easing(progress);
        (start_y as f32 + (end_y - start_y) as f32 * t) as i32
    }

    /// Calculate position for slide-in from bottom
    pub fn from_bottom(canvas_height: i32, element_height: i32, progress: f32, easing: fn(f32) -> f32) -> i32 {
        let start_y = canvas_height;
        let end_y = (canvas_height - element_height) / 2;
        Self::from_top(start_y, end_y, progress, easing)
    }
}

/// Pop-in animation (scale from 0 to 1 with overshoot)
pub struct PopIn;

impl PopIn {
    /// Get scale for pop-in effect with optional overshoot
    pub fn get_scale(progress: f32, overshoot: f32) -> f32 {
        let t = progress.clamp(0.0, 1.0);
        if t == 0.0 {
            return 0.0;
        }
        if t == 1.0 {
            return 1.0;
        }

        // Overshoot formula
        let s = 1.70158 * overshoot;
        let t = t - 1.0;
        t * t * ((s + 1.0) * t + s) + 1.0
    }
}

/// Glow effect for images and text
pub struct Glow;

impl Glow {
    /// Add glow effect to an image
    pub fn apply(img: &RgbaImage, glow_color: Rgba<u8>, radius: u32, intensity: f32) -> RgbaImage {
        let width = img.width() + radius * 2;
        let height = img.height() + radius * 2;
        let mut result = RgbaImage::new(width, height);

        // Create glow layers
        for layer in 0..radius {
            let alpha = ((radius - layer) as f32 / radius as f32 * intensity * glow_color[3] as f32) as u8;
            let expand = layer as i32;

            for (x, y, pixel) in img.enumerate_pixels() {
                if pixel[3] > 128 {
                    let gx = (x as i32 + radius as i32) as u32;
                    let gy = (y as i32 + radius as i32) as u32;

                    // Draw glow around each opaque pixel
                    for dy in -expand..=expand {
                        for dx in -expand..=expand {
                            let dist = ((dx * dx + dy * dy) as f32).sqrt();
                            if dist <= expand as f32 {
                                let px = (gx as i32 + dx) as u32;
                                let py = (gy as i32 + dy) as u32;
                                if px < width && py < height {
                                    let falloff = 1.0 - (dist / expand as f32);
                                    let a = (alpha as f32 * falloff) as u8;
                                    let current = result.get_pixel(px, py);
                                    let blended = Self::blend_additive(*current, Rgba([glow_color[0], glow_color[1], glow_color[2], a]));
                                    result.put_pixel(px, py, blended);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Composite original image on top
        for (x, y, pixel) in img.enumerate_pixels() {
            if pixel[3] > 0 {
                let px = x + radius;
                let py = y + radius;
                let current = result.get_pixel(px, py);
                let blended = Self::blend_over(*pixel, *current);
                result.put_pixel(px, py, blended);
            }
        }

        result
    }

    fn blend_additive(base: Rgba<u8>, add: Rgba<u8>) -> Rgba<u8> {
        let add_factor = add[3] as f32 / 255.0;
        Rgba([
            (base[0] as f32 + add[0] as f32 * add_factor).min(255.0) as u8,
            (base[1] as f32 + add[1] as f32 * add_factor).min(255.0) as u8,
            (base[2] as f32 + add[2] as f32 * add_factor).min(255.0) as u8,
            base[3].max((add[3] as f32 * 0.5) as u8),
        ])
    }

    fn blend_over(src: Rgba<u8>, dest: Rgba<u8>) -> Rgba<u8> {
        let src_a = src[3] as f32 / 255.0;
        let dest_a = dest[3] as f32 / 255.0;
        let out_a = src_a + dest_a * (1.0 - src_a);

        if out_a < 0.001 {
            return Rgba([0, 0, 0, 0]);
        }

        Rgba([
            ((src[0] as f32 * src_a + dest[0] as f32 * dest_a * (1.0 - src_a)) / out_a) as u8,
            ((src[1] as f32 * src_a + dest[1] as f32 * dest_a * (1.0 - src_a)) / out_a) as u8,
            ((src[2] as f32 * src_a + dest[2] as f32 * dest_a * (1.0 - src_a)) / out_a) as u8,
            (out_a * 255.0) as u8,
        ])
    }
}

/// Particle system for floating effects
pub struct Particles;

impl Particles {
    /// Generate sparkle particles
    pub fn sparkles(width: u32, height: u32, count: usize, time: f32, seed: u64) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);

        for i in 0..count {
            let phase = seed as f32 + i as f32 * 7.3;
            let x = ((phase * 13.7 + time * 0.3).sin() * 0.5 + 0.5) * width as f32;
            let y = ((phase * 17.3 + time * 0.2).cos() * 0.5 + 0.5) * height as f32;

            // Twinkle effect
            let twinkle = ((time * 5.0 + phase).sin() * 0.5 + 0.5);
            let size = 2.0 + twinkle * 3.0;
            let alpha = (twinkle * 200.0) as u8;

            Self::draw_star(&mut img, x, y, size, Rgba([255, 255, 220, alpha]));
        }

        img
    }

    fn draw_star(img: &mut RgbaImage, cx: f32, cy: f32, size: f32, color: Rgba<u8>) {
        // Simple cross-shaped star
        let x = cx as i32;
        let y = cy as i32;
        let s = size as i32;

        for d in -s..=s {
            // Horizontal
            let px = (x + d).max(0) as u32;
            let py = y.max(0) as u32;
            if px < img.width() && py < img.height() {
                let dist = d.abs() as f32 / s as f32;
                let a = ((1.0 - dist) * color[3] as f32) as u8;
                let current = img.get_pixel(px, py);
                let blended = Glow::blend_additive(*current, Rgba([color[0], color[1], color[2], a]));
                img.put_pixel(px, py, blended);
            }

            // Vertical
            let px = x.max(0) as u32;
            let py = (y + d).max(0) as u32;
            if px < img.width() && py < img.height() {
                let dist = d.abs() as f32 / s as f32;
                let a = ((1.0 - dist) * color[3] as f32) as u8;
                let current = img.get_pixel(px, py);
                let blended = Glow::blend_additive(*current, Rgba([color[0], color[1], color[2], a]));
                img.put_pixel(px, py, blended);
            }
        }
    }

    /// Generate energy wave effect
    pub fn energy_wave(width: u32, height: u32, time: f32, color: Rgba<u8>) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;

        let wave_radius = ((time % 2.0) / 2.0) * (width.max(height) as f32);
        let wave_width = 30.0;

        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                let dist_from_wave = (dist - wave_radius).abs();
                if dist_from_wave < wave_width {
                    let intensity = 1.0 - (dist_from_wave / wave_width);
                    let a = (intensity * intensity * color[3] as f32) as u8;
                    img.put_pixel(x, y, Rgba([color[0], color[1], color[2], a]));
                }
            }
        }

        img
    }
}

/// Color utilities
pub struct ColorUtils;

impl ColorUtils {
    /// Interpolate between two colors
    pub fn lerp(color1: Rgba<u8>, color2: Rgba<u8>, t: f32) -> Rgba<u8> {
        let t = t.clamp(0.0, 1.0);
        Rgba([
            (color1[0] as f32 + (color2[0] as f32 - color1[0] as f32) * t) as u8,
            (color1[1] as f32 + (color2[1] as f32 - color1[1] as f32) * t) as u8,
            (color1[2] as f32 + (color2[2] as f32 - color1[2] as f32) * t) as u8,
            (color1[3] as f32 + (color2[3] as f32 - color1[3] as f32) * t) as u8,
        ])
    }

    /// Create a gradient
    pub fn gradient(width: u32, height: u32, top: Rgba<u8>, bottom: Rgba<u8>) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);
        for y in 0..height {
            let t = y as f32 / height as f32;
            let color = Self::lerp(top, bottom, t);
            for x in 0..width {
                img.put_pixel(x, y, color);
            }
        }
        img
    }

    /// Create a radial vignette
    pub fn vignette(width: u32, height: u32, strength: f32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;
        let max_dist = (cx * cx + cy * cy).sqrt();

        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt() / max_dist;
                let alpha = (dist * dist * strength * 255.0).min(255.0) as u8;
                img.put_pixel(x, y, Rgba([0, 0, 0, alpha]));
            }
        }
        img
    }
}
