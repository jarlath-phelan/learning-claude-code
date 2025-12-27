//! Video composition module
//!
//! Handles frame composition, backgrounds, and visual effects
//! with professional quality rendering.

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

    pub fn lerp_color(c1: Rgba<u8>, c2: Rgba<u8>, t: f32) -> Rgba<u8> {
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
    /// Create professional UNO-themed background with radial gradient and animated elements
    pub fn uno_theme(width: u32, height: u32, time: f32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);

        let cx = width as f32 / 2.0;
        let cy = height as f32 * 0.4;
        let max_dist = (width.max(height) as f32) * 0.8;

        // Radial gradient base
        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt() / max_dist;

                // Deep blue to dark gradient
                let t = dist.min(1.0);
                let r = (30.0 - 20.0 * t) as u8;
                let g = (40.0 - 30.0 * t) as u8;
                let b = (70.0 - 50.0 * t) as u8;

                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }

        // Animated diagonal light beams
        let beam_colors = [
            Rgba([237, 28, 36, 35]),   // UNO Red
            Rgba([0, 114, 188, 35]),   // UNO Blue
            Rgba([0, 166, 81, 35]),    // UNO Green
            Rgba([255, 237, 0, 35]),   // UNO Yellow
        ];

        for (i, color) in beam_colors.iter().enumerate() {
            let phase = time * 0.5 + i as f32 * 1.5;
            let beam_x = ((phase.sin() * 0.5 + 0.5) * width as f32 * 1.5 - width as f32 * 0.25) as i32;

            Self::draw_diagonal_beam(&mut img, beam_x, *color, 80, 0.3);
        }

        // Subtle animated particles/dots
        Self::add_floating_particles(&mut img, time, 20);

        img
    }

    fn draw_diagonal_beam(img: &mut RgbaImage, x_offset: i32, color: Rgba<u8>, width: i32, angle: f32) {
        let img_width = img.width() as i32;
        let img_height = img.height() as i32;

        for y in 0..img_height {
            let beam_center = x_offset + (y as f32 * angle) as i32;

            for x in (beam_center - width).max(0)..(beam_center + width).min(img_width) {
                let dist = ((x - beam_center) as f32).abs() / width as f32;
                let alpha = ((1.0 - dist) * color[3] as f32) as u8;

                if alpha > 0 {
                    let current = img.get_pixel(x as u32, y as u32);
                    let blended = Self::blend_additive(*current, Rgba([color[0], color[1], color[2], alpha]));
                    img.put_pixel(x as u32, y as u32, blended);
                }
            }
        }
    }

    fn add_floating_particles(img: &mut RgbaImage, time: f32, count: usize) {
        use std::f32::consts::PI;

        let width = img.width() as f32;
        let height = img.height() as f32;

        for i in 0..count {
            let seed = i as f32 * 7.3;
            let x = ((seed * 13.7 + time * 0.1).sin() * 0.5 + 0.5) * width;
            let y = ((seed * 17.3 + time * 0.15).cos() * 0.5 + 0.5) * height;
            let size = 2.0 + (seed * 3.1).sin().abs() * 4.0;
            let alpha = ((time * 2.0 + seed).sin() * 0.5 + 0.5) * 80.0;

            Self::draw_soft_circle(img, x, y, size, Rgba([255, 255, 255, alpha as u8]));
        }
    }

    fn draw_soft_circle(img: &mut RgbaImage, cx: f32, cy: f32, radius: f32, color: Rgba<u8>) {
        let x_start = (cx - radius - 2.0).max(0.0) as u32;
        let x_end = ((cx + radius + 2.0) as u32).min(img.width());
        let y_start = (cy - radius - 2.0).max(0.0) as u32;
        let y_end = ((cy + radius + 2.0) as u32).min(img.height());

        for y in y_start..y_end {
            for x in x_start..x_end {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < radius {
                    let falloff = 1.0 - (dist / radius);
                    let alpha = (falloff * falloff * color[3] as f32) as u8;
                    let current = img.get_pixel(x, y);
                    let blended = Self::blend_additive(*current, Rgba([color[0], color[1], color[2], alpha]));
                    img.put_pixel(x, y, blended);
                }
            }
        }
    }

    fn blend_additive(base: Rgba<u8>, add: Rgba<u8>) -> Rgba<u8> {
        let add_factor = add[3] as f32 / 255.0;
        Rgba([
            (base[0] as f32 + add[0] as f32 * add_factor).min(255.0) as u8,
            (base[1] as f32 + add[1] as f32 * add_factor).min(255.0) as u8,
            (base[2] as f32 + add[2] as f32 * add_factor).min(255.0) as u8,
            255,
        ])
    }

    /// Create dramatic dark background with animated spotlight
    pub fn spotlight(width: u32, height: u32, spotlight_x: f32, spotlight_y: f32, intensity: f32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);

        let cx = width as f32 * spotlight_x;
        let cy = height as f32 * spotlight_y;
        let max_dist = (width.max(height) as f32) * 0.6;

        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                // Smooth spotlight falloff
                let t = (dist / max_dist).min(1.0);
                let brightness = (1.0 - t * t) * intensity;

                // Warm spotlight color
                let base_r = 15.0 + 80.0 * brightness;
                let base_g = 12.0 + 50.0 * brightness;
                let base_b = 20.0 + 30.0 * brightness;

                img.put_pixel(x, y, Rgba([base_r as u8, base_g as u8, base_b as u8, 255]));
            }
        }

        // Add subtle rim lighting
        Self::add_rim_light(&mut img, 0.7);

        img
    }

    fn add_rim_light(img: &mut RgbaImage, intensity: f32) {
        let width = img.width();
        let height = img.height();

        for y in 0..height {
            for x in 0..width {
                let edge_dist_x = (x as f32).min((width - x) as f32) / 50.0;
                let edge_dist_y = (y as f32).min((height - y) as f32) / 50.0;
                let edge_factor = (1.0 - edge_dist_x.min(edge_dist_y).min(1.0)) * intensity;

                if edge_factor > 0.01 {
                    let current = img.get_pixel(x, y);
                    let rim = Rgba([
                        (current[0] as f32 + 40.0 * edge_factor).min(255.0) as u8,
                        (current[1] as f32 + 20.0 * edge_factor).min(255.0) as u8,
                        (current[2] as f32 + 50.0 * edge_factor).min(255.0) as u8,
                        255,
                    ]);
                    img.put_pixel(x, y, rim);
                }
            }
        }
    }

    /// Create chaotic/energy background with glitch effects
    pub fn chaos(width: u32, height: u32, time: f32, seed: u64) -> RgbaImage {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;

        let mut img = RgbaImage::new(width, height);
        let frame_seed = seed + (time * 5.0) as u64;
        let mut rng = StdRng::seed_from_u64(frame_seed);

        // Animated gradient base
        for y in 0..height {
            for x in 0..width {
                let noise = ((x as f32 * 0.01 + time).sin() * (y as f32 * 0.01 + time * 0.7).cos()) * 0.5 + 0.5;
                let r = (20.0 + noise * 25.0) as u8;
                let g = (15.0 + noise * 15.0) as u8;
                let b = (30.0 + noise * 20.0) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }

        // Glitch lines
        let line_count = 8 + (time * 3.0).sin().abs() as usize * 5;
        for _ in 0..line_count {
            let y = rng.gen_range(0..height);
            let line_height = rng.gen_range(2..15);
            let color_idx = rng.gen_range(0..4);

            let colors = [
                Rgba([237, 28, 36, 120]),
                Rgba([0, 114, 188, 120]),
                Rgba([0, 166, 81, 120]),
                Rgba([255, 237, 0, 120]),
            ];

            let offset = rng.gen_range(-30..30);

            for ly in y..(y + line_height).min(height) {
                for x in 0..width {
                    let shifted_x = ((x as i32 + offset) as u32) % width;
                    let current = img.get_pixel(shifted_x, ly);
                    let color = colors[color_idx];
                    let blended = Self::blend_additive(*current, color);
                    img.put_pixel(shifted_x, ly, blended);
                }
            }
        }

        // Energy bursts
        let burst_count = 3 + rng.gen_range(0..4);
        for _ in 0..burst_count {
            let bx = rng.gen_range(0.0..width as f32);
            let by = rng.gen_range(0.0..height as f32);
            let size = rng.gen_range(30.0..100.0);
            let color_idx = rng.gen_range(0..4);

            let colors = [
                Rgba([255, 100, 100, 60]),
                Rgba([100, 150, 255, 60]),
                Rgba([100, 255, 150, 60]),
                Rgba([255, 255, 100, 60]),
            ];

            Self::draw_soft_circle(&mut img, bx, by, size, colors[color_idx]);
        }

        img
    }

    /// Professional solid color with vignette and subtle texture
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

                // Smooth vignette
                let vignette = 1.0 - (dist * dist * vignette_strength).min(0.75);

                // Subtle noise texture
                let noise = ((x as f32 * 0.5).sin() * (y as f32 * 0.5).cos() * 0.02 + 1.0);

                let r = (base_color[0] as f32 * vignette * noise).min(255.0) as u8;
                let g = (base_color[1] as f32 * vignette * noise).min(255.0) as u8;
                let b = (base_color[2] as f32 * vignette * noise).min(255.0) as u8;

                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }

        img
    }

    /// Create epic reveal background with radial wipe
    pub fn epic_reveal(width: u32, height: u32, progress: f32, primary: Rgba<u8>, secondary: Rgba<u8>) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);

        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;
        let max_radius = ((cx * cx + cy * cy).sqrt()) * 1.2;
        let current_radius = max_radius * progress;

        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                let color = if dist < current_radius - 50.0 {
                    primary
                } else if dist < current_radius {
                    // Blend zone
                    let t = (current_radius - dist) / 50.0;
                    FrameComposer::lerp_color(secondary, primary, t)
                } else {
                    secondary
                };

                img.put_pixel(x, y, color);
            }
        }

        // Add glow at edge
        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();

                let edge_dist = (dist - current_radius).abs();
                if edge_dist < 30.0 {
                    let glow_intensity = (1.0 - edge_dist / 30.0) * 100.0;
                    let current = img.get_pixel(x, y);
                    let glowed = Rgba([
                        (current[0] as f32 + glow_intensity).min(255.0) as u8,
                        (current[1] as f32 + glow_intensity * 0.8).min(255.0) as u8,
                        (current[2] as f32 + glow_intensity * 0.6).min(255.0) as u8,
                        255,
                    ]);
                    img.put_pixel(x, y, glowed);
                }
            }
        }

        img
    }

    /// Dramatic dark scene with moving light sources
    pub fn dramatic_dark(width: u32, height: u32, time: f32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);

        // Deep dark base with subtle gradient
        for y in 0..height {
            for x in 0..width {
                let gradient = y as f32 / height as f32;
                let r = (10.0 + gradient * 15.0) as u8;
                let g = (8.0 + gradient * 10.0) as u8;
                let b = (15.0 + gradient * 20.0) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }

        // Multiple moving light sources
        let lights = [
            (0.3 + (time * 0.2).sin() * 0.15, 0.4, Rgba([255, 100, 80, 50]), 400.0),
            (0.7 + (time * 0.15).cos() * 0.1, 0.6, Rgba([80, 100, 255, 40]), 350.0),
            (0.5, 0.3 + (time * 0.1).sin() * 0.1, Rgba([255, 200, 80, 35]), 300.0),
        ];

        for (lx, ly, color, radius) in lights.iter() {
            let cx = width as f32 * lx;
            let cy = height as f32 * ly;

            for y in 0..height {
                for x in 0..width {
                    let dx = x as f32 - cx;
                    let dy = y as f32 - cy;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < *radius {
                        let falloff = 1.0 - (dist / radius);
                        let intensity = falloff * falloff;
                        let current = img.get_pixel(x, y);
                        let lit = Rgba([
                            (current[0] as f32 + color[0] as f32 * intensity * (color[3] as f32 / 255.0)).min(255.0) as u8,
                            (current[1] as f32 + color[1] as f32 * intensity * (color[3] as f32 / 255.0)).min(255.0) as u8,
                            (current[2] as f32 + color[2] as f32 * intensity * (color[3] as f32 / 255.0)).min(255.0) as u8,
                            255,
                        ]);
                        img.put_pixel(x, y, lit);
                    }
                }
            }
        }

        img
    }
}
