//! UNO Card graphics generator
//!
//! Creates professional UNO card graphics for the video with
//! rounded corners, shadows, gradients, and authentic styling.

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_filled_ellipse_mut, draw_text_mut};
use imageproc::rect::Rect;

/// UNO card colors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardColor {
    Red,
    Blue,
    Green,
    Yellow,
    Wild,  // Black/multicolor
}

impl CardColor {
    pub fn to_rgba(&self) -> Rgba<u8> {
        match self {
            CardColor::Red => Rgba([237, 28, 36, 255]),      // Bright UNO red
            CardColor::Blue => Rgba([0, 114, 188, 255]),     // UNO blue
            CardColor::Green => Rgba([0, 166, 81, 255]),     // UNO green
            CardColor::Yellow => Rgba([255, 237, 0, 255]),   // UNO yellow
            CardColor::Wild => Rgba([35, 31, 32, 255]),      // UNO black
        }
    }

    pub fn to_dark(&self) -> Rgba<u8> {
        match self {
            CardColor::Red => Rgba([180, 20, 25, 255]),
            CardColor::Blue => Rgba([0, 80, 140, 255]),
            CardColor::Green => Rgba([0, 120, 60, 255]),
            CardColor::Yellow => Rgba([200, 180, 0, 255]),
            CardColor::Wild => Rgba([20, 18, 18, 255]),
        }
    }

    pub fn to_light(&self) -> Rgba<u8> {
        match self {
            CardColor::Red => Rgba([255, 80, 80, 255]),
            CardColor::Blue => Rgba([80, 160, 230, 255]),
            CardColor::Green => Rgba([80, 200, 120, 255]),
            CardColor::Yellow => Rgba([255, 250, 100, 255]),
            CardColor::Wild => Rgba([60, 55, 55, 255]),
        }
    }
}

/// UNO card types
#[derive(Debug, Clone, PartialEq)]
pub enum CardType {
    Number(u8),           // 0-9
    DrawTwo,              // +2
    DrawFour,             // +4 (colored in No Mercy)
    DrawSix,              // +6 Wild
    DrawTen,              // +10 Wild
    Skip,
    SkipEveryone,         // Skip all players
    Reverse,
    ReverseDrawFour,      // Reverse + Draw 4
    DiscardAll,           // Discard all of one color
    ColorRoulette,        // Wild color roulette
}

impl CardType {
    pub fn display_text(&self) -> String {
        match self {
            CardType::Number(n) => n.to_string(),
            CardType::DrawTwo => "+2".to_string(),
            CardType::DrawFour => "+4".to_string(),
            CardType::DrawSix => "+6".to_string(),
            CardType::DrawTen => "+10".to_string(),
            CardType::Skip => "O/".to_string(),
            CardType::SkipEveryone => "O//".to_string(),
            CardType::Reverse => "<>".to_string(),
            CardType::ReverseDrawFour => "<>4".to_string(),
            CardType::DiscardAll => "ALL".to_string(),
            CardType::ColorRoulette => "?".to_string(),
        }
    }

    pub fn corner_text(&self) -> String {
        match self {
            CardType::Number(n) => n.to_string(),
            CardType::DrawTwo => "+2".to_string(),
            CardType::DrawFour => "+4".to_string(),
            CardType::DrawSix => "+6".to_string(),
            CardType::DrawTen => "+10".to_string(),
            CardType::Skip => "X".to_string(),
            CardType::SkipEveryone => "XX".to_string(),
            CardType::Reverse => "<>".to_string(),
            CardType::ReverseDrawFour => "<>".to_string(),
            CardType::DiscardAll => "DA".to_string(),
            CardType::ColorRoulette => "?".to_string(),
        }
    }
}

/// Represents an UNO card
pub struct Card {
    pub color: CardColor,
    pub card_type: CardType,
}

impl Card {
    pub fn new(color: CardColor, card_type: CardType) -> Self {
        Self { color, card_type }
    }

    /// Render the card as an image with professional styling
    pub fn render(&self, width: u32, height: u32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);
        let corner_radius = (width.min(height) as f32 * 0.12) as i32;

        // Draw drop shadow first
        self.draw_shadow(&mut img, width, height, corner_radius);

        // Draw card background with rounded corners
        self.draw_rounded_rect(&mut img, 4, 4, width - 8, height - 8, corner_radius,
            Rgba([255, 255, 255, 255])); // White border

        // Draw main card color with gradient
        let border = 8;
        self.draw_card_gradient(&mut img, border, border,
            width - border as u32 * 2, height - border as u32 * 2,
            corner_radius - 4);

        // Draw the diagonal white oval (UNO signature style)
        self.draw_diagonal_oval(&mut img, width, height);

        // Draw center symbol/text
        self.draw_center_symbol(&mut img, width, height);

        // Draw corner numbers
        self.draw_corner_numbers(&mut img, width, height);

        img
    }

    fn draw_shadow(&self, img: &mut RgbaImage, width: u32, height: u32, corner_radius: i32) {
        let shadow_offset = 6;
        let shadow_blur = 8;

        for blur in 0..shadow_blur {
            let alpha = ((shadow_blur - blur) as f32 / shadow_blur as f32 * 80.0) as u8;
            let expand = blur as i32;
            self.draw_rounded_rect(
                img,
                shadow_offset - expand,
                shadow_offset - expand,
                width + expand as u32 * 2 - 8,
                height + expand as u32 * 2 - 8,
                corner_radius + expand,
                Rgba([0, 0, 0, alpha]),
            );
        }
    }

    fn draw_rounded_rect(&self, img: &mut RgbaImage, x: i32, y: i32, w: u32, h: u32, radius: i32, color: Rgba<u8>) {
        let w = w as i32;
        let h = h as i32;

        for py in 0..h {
            for px in 0..w {
                let dx = x + px;
                let dy = y + py;

                if dx < 0 || dy < 0 || dx >= img.width() as i32 || dy >= img.height() as i32 {
                    continue;
                }

                // Check if point is within rounded rectangle
                let in_corner = self.point_in_rounded_rect(px, py, w, h, radius);
                if in_corner {
                    let dest = img.get_pixel(dx as u32, dy as u32);
                    let blended = Self::blend_pixels(color, *dest);
                    img.put_pixel(dx as u32, dy as u32, blended);
                }
            }
        }
    }

    fn point_in_rounded_rect(&self, px: i32, py: i32, w: i32, h: i32, radius: i32) -> bool {
        // Check corners
        let corners = [
            (radius, radius),                    // Top-left
            (w - radius - 1, radius),            // Top-right
            (radius, h - radius - 1),            // Bottom-left
            (w - radius - 1, h - radius - 1),    // Bottom-right
        ];

        for (cx, cy) in corners {
            let in_corner_zone = (px < radius && py < radius) ||
                                 (px >= w - radius && py < radius) ||
                                 (px < radius && py >= h - radius) ||
                                 (px >= w - radius && py >= h - radius);

            if in_corner_zone {
                let dx = (px - cx).abs();
                let dy = (py - cy).abs();
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                if dist > radius as f32 {
                    return false;
                }
            }
        }

        true
    }

    fn blend_pixels(src: Rgba<u8>, dest: Rgba<u8>) -> Rgba<u8> {
        let src_a = src[3] as f32 / 255.0;
        let dest_a = dest[3] as f32 / 255.0;
        let out_a = src_a + dest_a * (1.0 - src_a);

        if out_a == 0.0 {
            return Rgba([0, 0, 0, 0]);
        }

        let r = (src[0] as f32 * src_a + dest[0] as f32 * dest_a * (1.0 - src_a)) / out_a;
        let g = (src[1] as f32 * src_a + dest[1] as f32 * dest_a * (1.0 - src_a)) / out_a;
        let b = (src[2] as f32 * src_a + dest[2] as f32 * dest_a * (1.0 - src_a)) / out_a;

        Rgba([r as u8, g as u8, b as u8, (out_a * 255.0) as u8])
    }

    fn draw_card_gradient(&self, img: &mut RgbaImage, x: i32, y: i32, w: u32, h: u32, radius: i32) {
        let base = self.color.to_rgba();
        let dark = self.color.to_dark();
        let light = self.color.to_light();

        for py in 0..h as i32 {
            for px in 0..w as i32 {
                let dx = x + px;
                let dy = y + py;

                if dx < 0 || dy < 0 || dx >= img.width() as i32 || dy >= img.height() as i32 {
                    continue;
                }

                if !self.point_in_rounded_rect(px, py, w as i32, h as i32, radius) {
                    continue;
                }

                // Gradient from top-left (light) to bottom-right (dark)
                let t = (px as f32 / w as f32 + py as f32 / h as f32) / 2.0;

                // Add some shine at top
                let shine = if py < h as i32 / 4 {
                    (1.0 - py as f32 / (h as f32 / 4.0)) * 0.2
                } else {
                    0.0
                };

                let r = Self::lerp(light[0] as f32, dark[0] as f32, t) + shine * 50.0;
                let g = Self::lerp(light[1] as f32, dark[1] as f32, t) + shine * 50.0;
                let b = Self::lerp(light[2] as f32, dark[2] as f32, t) + shine * 50.0;

                img.put_pixel(dx as u32, dy as u32, Rgba([
                    r.min(255.0) as u8,
                    g.min(255.0) as u8,
                    b.min(255.0) as u8,
                    255
                ]));
            }
        }
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    fn draw_diagonal_oval(&self, img: &mut RgbaImage, width: u32, height: u32) {
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;
        let oval_a = width as f32 * 0.38;  // Semi-major axis
        let oval_b = height as f32 * 0.28; // Semi-minor axis
        let angle: f32 = -0.35; // Rotation angle (radians) - diagonal tilt

        let cos_a = angle.cos();
        let sin_a = angle.sin();

        for y in 0..height {
            for x in 0..width {
                // Transform to oval-centered coordinates
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;

                // Rotate
                let rx = dx * cos_a + dy * sin_a;
                let ry = -dx * sin_a + dy * cos_a;

                // Check if inside ellipse
                let dist = (rx / oval_a).powi(2) + (ry / oval_b).powi(2);

                if dist <= 1.0 {
                    // Gradient from center to edge for subtle 3D effect
                    let edge_dist = 1.0 - dist;
                    let brightness = 240 + (15.0 * edge_dist) as u8;
                    img.put_pixel(x, y, Rgba([brightness, brightness, brightness, 255]));
                } else if dist <= 1.08 {
                    // Subtle shadow at edge
                    let alpha = ((1.08 - dist) / 0.08 * 100.0) as u8;
                    let current = img.get_pixel(x, y);
                    let blended = Self::blend_pixels(Rgba([200, 200, 200, alpha]), *current);
                    img.put_pixel(x, y, blended);
                }
            }
        }
    }

    fn draw_center_symbol(&self, img: &mut RgbaImage, width: u32, height: u32) {
        let text = self.card_type.display_text();
        let text_color = match self.color {
            CardColor::Yellow => Rgba([35, 31, 32, 255]), // Dark text on yellow
            _ => self.color.to_rgba(),
        };

        let font_data = include_bytes!("../fonts/Roboto-Bold.ttf");
        let font = match FontRef::try_from_slice(font_data) {
            Ok(f) => f,
            Err(_) => return,
        };

        // Larger, bolder center text
        let scale = PxScale::from((height as f32 * 0.35).min(width as f32 * 0.5));
        let scaled_font = font.as_scaled(scale);

        // Calculate text dimensions for centering
        let mut text_width = 0.0f32;
        for c in text.chars() {
            text_width += scaled_font.h_advance(scaled_font.glyph_id(c));
        }
        let text_height = scaled_font.height();

        let x = (width as f32 / 2.0 - text_width / 2.0) as i32;
        let y = (height as f32 / 2.0 - text_height / 2.0) as i32;

        // Draw text shadow
        draw_text_mut(
            img,
            Rgba([0, 0, 0, 80]),
            x + 3,
            y + 3,
            scale,
            &font,
            &text,
        );

        // Draw main text
        draw_text_mut(
            img,
            text_color,
            x,
            y,
            scale,
            &font,
            &text,
        );
    }

    fn draw_corner_numbers(&self, img: &mut RgbaImage, width: u32, height: u32) {
        let text = self.card_type.corner_text();
        let text_color = Rgba([255, 255, 255, 255]);

        let font_data = include_bytes!("../fonts/Roboto-Bold.ttf");
        let font = match FontRef::try_from_slice(font_data) {
            Ok(f) => f,
            Err(_) => return,
        };

        let corner_size = (height as f32 * 0.12).min(width as f32 * 0.18);
        let scale = PxScale::from(corner_size);
        let margin = (width as f32 * 0.08) as i32;

        // Top-left corner
        draw_text_mut(img, Rgba([0, 0, 0, 100]), margin + 2, margin + 2, scale, &font, &text);
        draw_text_mut(img, text_color, margin, margin, scale, &font, &text);

        // Bottom-right corner (upside down effect - just draw normally for simplicity)
        let scaled_font = font.as_scaled(scale);
        let mut text_width = 0.0f32;
        for c in text.chars() {
            text_width += scaled_font.h_advance(scaled_font.glyph_id(c));
        }
        let br_x = width as i32 - margin - text_width as i32;
        let br_y = height as i32 - margin - corner_size as i32;

        draw_text_mut(img, Rgba([0, 0, 0, 100]), br_x + 2, br_y + 2, scale, &font, &text);
        draw_text_mut(img, text_color, br_x, br_y, scale, &font, &text);
    }

    /// Render a card with a glow effect
    pub fn render_with_glow(&self, width: u32, height: u32, glow_color: Rgba<u8>, glow_size: u32) -> RgbaImage {
        let total_width = width + glow_size * 2;
        let total_height = height + glow_size * 2;
        let mut img = RgbaImage::new(total_width, total_height);

        // Draw glow with gradient falloff
        let corner_radius = (width.min(height) as f32 * 0.12) as i32;
        for i in 0..glow_size {
            let alpha = ((glow_size - i) as f32 / glow_size as f32).powi(2) * 180.0;
            let glow = Rgba([glow_color[0], glow_color[1], glow_color[2], alpha as u8]);
            self.draw_rounded_rect(
                &mut img,
                i as i32,
                i as i32,
                total_width - i * 2,
                total_height - i * 2,
                corner_radius + (glow_size - i) as i32,
                glow,
            );
        }

        // Draw card on top
        let card_img = self.render(width, height);
        for (x, y, pixel) in card_img.enumerate_pixels() {
            if pixel[3] > 0 {
                let dest_x = x + glow_size;
                let dest_y = y + glow_size;
                if dest_x < total_width && dest_y < total_height {
                    let dest = img.get_pixel(dest_x, dest_y);
                    let blended = Self::blend_pixels(*pixel, *dest);
                    img.put_pixel(dest_x, dest_y, blended);
                }
            }
        }

        img
    }
}

/// Card factory for creating common cards
pub struct CardFactory;

impl CardFactory {
    pub fn plus_two(color: CardColor) -> Card {
        Card::new(color, CardType::DrawTwo)
    }

    pub fn plus_four(color: CardColor) -> Card {
        Card::new(color, CardType::DrawFour)
    }

    pub fn plus_six() -> Card {
        Card::new(CardColor::Wild, CardType::DrawSix)
    }

    pub fn plus_ten() -> Card {
        Card::new(CardColor::Wild, CardType::DrawTen)
    }

    pub fn skip(color: CardColor) -> Card {
        Card::new(color, CardType::Skip)
    }

    pub fn skip_everyone() -> Card {
        Card::new(CardColor::Wild, CardType::SkipEveryone)
    }

    pub fn reverse(color: CardColor) -> Card {
        Card::new(color, CardType::Reverse)
    }

    pub fn reverse_draw_four() -> Card {
        Card::new(CardColor::Wild, CardType::ReverseDrawFour)
    }

    pub fn discard_all(color: CardColor) -> Card {
        Card::new(color, CardType::DiscardAll)
    }

    pub fn color_roulette() -> Card {
        Card::new(CardColor::Wild, CardType::ColorRoulette)
    }

    pub fn number(color: CardColor, num: u8) -> Card {
        Card::new(color, CardType::Number(num.min(9)))
    }
}

/// Renders multiple cards in a fan or stack arrangement
pub struct CardRenderer;

impl CardRenderer {
    /// Create a stack of cards with offset
    pub fn render_stack(cards: &[Card], card_width: u32, card_height: u32, offset: i32) -> RgbaImage {
        let num_cards = cards.len();
        if num_cards == 0 {
            return RgbaImage::new(card_width, card_height);
        }

        let total_width = card_width + (offset.unsigned_abs() * (num_cards as u32 - 1));
        let total_height = card_height + (offset.unsigned_abs() * (num_cards as u32 - 1));

        let mut img = RgbaImage::new(total_width, total_height);

        for (i, card) in cards.iter().enumerate() {
            let card_img = card.render(card_width, card_height);
            let x = (i as i32 * offset.abs()) as u32;
            let y = (i as i32 * offset.abs()) as u32;

            Self::composite_image(&mut img, &card_img, x as i32, y as i32);
        }

        img
    }

    /// Render cards in a fan arrangement
    pub fn render_fan(cards: &[Card], card_width: u32, card_height: u32, spread_angle: f32) -> RgbaImage {
        let num_cards = cards.len();
        if num_cards == 0 {
            return RgbaImage::new(card_width, card_height);
        }

        // Canvas size to accommodate fanned cards
        let canvas_size = (card_width.max(card_height) as f32 * 2.5) as u32;
        let mut img = RgbaImage::new(canvas_size, canvas_size);

        let center_x = canvas_size as f32 / 2.0;
        let center_y = canvas_size as f32 * 0.8;

        let start_angle = -spread_angle / 2.0;
        let angle_step = if num_cards > 1 { spread_angle / (num_cards - 1) as f32 } else { 0.0 };

        for (i, card) in cards.iter().enumerate() {
            let angle = start_angle + angle_step * i as f32;
            let card_img = card.render(card_width, card_height);

            // Simple positioning (rotation would need more complex implementation)
            let offset_x = (angle.sin() * card_width as f32 * 0.8) as i32;
            let x = center_x as i32 - card_width as i32 / 2 + offset_x;
            let y = center_y as i32 - card_height as i32;

            Self::composite_image(&mut img, &card_img, x, y);
        }

        img
    }

    /// Composite one image onto another at given position with alpha blending
    fn composite_image(dest: &mut RgbaImage, src: &RgbaImage, x: i32, y: i32) {
        for (sx, sy, pixel) in src.enumerate_pixels() {
            let dx = x + sx as i32;
            let dy = y + sy as i32;

            if dx >= 0 && dy >= 0 && (dx as u32) < dest.width() && (dy as u32) < dest.height() {
                if pixel[3] > 0 {
                    let dest_pixel = dest.get_pixel(dx as u32, dy as u32);
                    let blended = Card::blend_pixels(*pixel, *dest_pixel);
                    dest.put_pixel(dx as u32, dy as u32, blended);
                }
            }
        }
    }

    /// Render cards flying/scattered with more variety
    pub fn render_flying_cards(card_width: u32, card_height: u32, num_cards: usize, seed: u64) -> RgbaImage {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;

        let mut rng = StdRng::seed_from_u64(seed);
        let canvas_width = card_width * 4;
        let canvas_height = card_height * 4;

        let mut img = RgbaImage::new(canvas_width, canvas_height);

        let colors = [CardColor::Red, CardColor::Blue, CardColor::Green, CardColor::Yellow];

        for _ in 0..num_cards {
            let color = colors[rng.gen_range(0..4)];
            let num = rng.gen_range(0..10);
            let card = CardFactory::number(color, num);

            // Varying sizes for depth effect
            let scale = rng.gen_range(0.3..0.7);
            let small_width = (card_width as f32 * scale) as u32;
            let small_height = (card_height as f32 * scale) as u32;
            let card_img = card.render(small_width, small_height);

            let x = rng.gen_range(0..(canvas_width.saturating_sub(small_width)).max(1)) as i32;
            let y = rng.gen_range(0..(canvas_height.saturating_sub(small_height)).max(1)) as i32;

            Self::composite_image(&mut img, &card_img, x, y);
        }

        img
    }
}
