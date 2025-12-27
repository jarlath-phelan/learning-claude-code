//! UNO Card graphics generator
//!
//! Creates simplified UNO card graphics for the video.

use ab_glyph::{FontRef, PxScale};
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
            CardColor::Red => Rgba([220, 50, 50, 255]),
            CardColor::Blue => Rgba([50, 100, 200, 255]),
            CardColor::Green => Rgba([50, 180, 80, 255]),
            CardColor::Yellow => Rgba([250, 200, 50, 255]),
            CardColor::Wild => Rgba([30, 30, 30, 255]),
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
            CardType::Skip => "X".to_string(),
            CardType::SkipEveryone => "XX".to_string(),
            CardType::Reverse => "<>".to_string(),
            CardType::ReverseDrawFour => "<>+4".to_string(),
            CardType::DiscardAll => "ALL".to_string(),
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

    /// Render the card as an image
    pub fn render(&self, width: u32, height: u32) -> RgbaImage {
        let mut img = RgbaImage::new(width, height);

        // Card background with rounded corners effect (using rectangles)
        let bg_color = self.color.to_rgba();
        let border_color = Rgba([255, 255, 255, 255]);

        // Draw border (white outline)
        draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(width, height), border_color);

        // Draw main card body
        let border = 4;
        draw_filled_rect_mut(
            &mut img,
            Rect::at(border, border).of_size(width - border as u32 * 2, height - border as u32 * 2),
            bg_color,
        );

        // Draw white oval in center
        let oval_width = (width as f32 * 0.6) as i32;
        let oval_height = (height as f32 * 0.4) as i32;
        let cx = width as i32 / 2;
        let cy = height as i32 / 2;

        draw_filled_ellipse_mut(
            &mut img,
            (cx, cy),
            oval_width / 2,
            oval_height / 2,
            Rgba([255, 255, 255, 255]),
        );

        // Draw card text/symbol
        self.draw_card_text(&mut img, width, height);

        img
    }

    fn draw_card_text(&self, img: &mut RgbaImage, width: u32, height: u32) {
        let text = self.card_type.display_text();
        let text_color = match self.color {
            CardColor::Yellow => Rgba([30, 30, 30, 255]), // Dark text on yellow
            _ => self.color.to_rgba(),
        };

        // Use ab_glyph font rendering
        let font_data = include_bytes!("../fonts/Roboto-Bold.ttf");
        let font = match FontRef::try_from_slice(font_data) {
            Ok(f) => f,
            Err(_) => return, // Skip text if font fails
        };

        let scale = PxScale::from((height as f32 * 0.3).min(width as f32 * 0.4));
        let cx = width as i32 / 2;
        let cy = height as i32 / 2;

        // Calculate text position (centered)
        let text_width = text.chars().count() as i32 * (scale.x as i32 / 3);
        let x = cx - text_width / 2;
        let y = cy - (scale.y as i32 / 3);

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

    /// Render a card with a glow effect
    pub fn render_with_glow(&self, width: u32, height: u32, glow_color: Rgba<u8>, glow_size: u32) -> RgbaImage {
        let total_width = width + glow_size * 2;
        let total_height = height + glow_size * 2;
        let mut img = RgbaImage::new(total_width, total_height);

        // Draw glow (simple colored rectangle behind card)
        for i in 0..glow_size {
            let alpha = ((glow_size - i) as f32 / glow_size as f32 * 150.0) as u8;
            let glow = Rgba([glow_color[0], glow_color[1], glow_color[2], alpha]);
            draw_filled_rect_mut(
                &mut img,
                Rect::at(i as i32, i as i32).of_size(total_width - i * 2, total_height - i * 2),
                glow,
            );
        }

        // Draw card on top
        let card_img = self.render(width, height);
        for (x, y, pixel) in card_img.enumerate_pixels() {
            let dest_x = x + glow_size;
            let dest_y = y + glow_size;
            if dest_x < total_width && dest_y < total_height {
                img.put_pixel(dest_x, dest_y, *pixel);
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

    /// Composite one image onto another at given position
    fn composite_image(dest: &mut RgbaImage, src: &RgbaImage, x: i32, y: i32) {
        for (sx, sy, pixel) in src.enumerate_pixels() {
            let dx = x + sx as i32;
            let dy = y + sy as i32;

            if dx >= 0 && dy >= 0 && (dx as u32) < dest.width() && (dy as u32) < dest.height() {
                if pixel[3] > 0 {
                    dest.put_pixel(dx as u32, dy as u32, *pixel);
                }
            }
        }
    }

    /// Render cards flying/scattered
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

            let small_width = card_width / 2;
            let small_height = card_height / 2;
            let card_img = card.render(small_width, small_height);

            let x = rng.gen_range(0..(canvas_width - small_width)) as i32;
            let y = rng.gen_range(0..(canvas_height - small_height)) as i32;

            Self::composite_image(&mut img, &card_img, x, y);
        }

        img
    }
}
