//! Character sprite generation module
//!
//! Creates a simple 2D cartoon character with various expressions.

use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_ellipse_mut, draw_line_segment_mut};

/// Character expressions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Expression {
    Neutral,
    Shocked,
    Serious,
    Mischievous,
    MindBlown,
    Whispering,
}

/// Colors for the character
pub struct CharacterColors {
    pub skin: Rgba<u8>,
    pub outline: Rgba<u8>,
    pub hair: Rgba<u8>,
    pub eye_white: Rgba<u8>,
    pub eye_pupil: Rgba<u8>,
    pub mouth: Rgba<u8>,
    pub shirt: Rgba<u8>,
}

impl Default for CharacterColors {
    fn default() -> Self {
        Self {
            skin: Rgba([255, 213, 170, 255]),      // Warm skin tone
            outline: Rgba([50, 50, 50, 255]),       // Dark outline
            hair: Rgba([60, 40, 30, 255]),          // Dark brown hair
            eye_white: Rgba([255, 255, 255, 255]),  // White
            eye_pupil: Rgba([30, 30, 30, 255]),     // Nearly black
            mouth: Rgba([180, 80, 80, 255]),        // Reddish mouth
            shirt: Rgba([220, 50, 50, 255]),        // UNO red shirt
        }
    }
}

/// Character renderer
pub struct Character {
    colors: CharacterColors,
}

impl Character {
    pub fn new() -> Self {
        Self {
            colors: CharacterColors::default(),
        }
    }

    /// Render the character at the given position with the specified expression
    /// Returns the character as an image that can be composited onto the frame
    pub fn render(&self, expression: Expression, scale: f32) -> RgbaImage {
        let base_width = 400;
        let base_height = 500;
        let width = (base_width as f32 * scale) as u32;
        let height = (base_height as f32 * scale) as u32;

        let mut img = RgbaImage::new(width, height);

        // Scale factor for drawing
        let s = scale;

        // Center of character
        let cx = width as i32 / 2;
        let cy = height as i32 / 2;

        // Draw body/shirt (visible from chest up)
        self.draw_body(&mut img, cx, cy + (150.0 * s) as i32, s);

        // Draw neck
        self.draw_neck(&mut img, cx, cy + (80.0 * s) as i32, s);

        // Draw head
        self.draw_head(&mut img, cx, cy - (20.0 * s) as i32, s);

        // Draw hair
        self.draw_hair(&mut img, cx, cy - (80.0 * s) as i32, s);

        // Draw face features based on expression
        self.draw_face(&mut img, cx, cy - (20.0 * s) as i32, s, expression);

        img
    }

    fn draw_body(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32) {
        // Shirt body (trapezoid shape using ellipse)
        draw_filled_ellipse_mut(
            img,
            (cx, cy),
            (120.0 * s) as i32,
            (100.0 * s) as i32,
            self.colors.shirt,
        );

        // Outline
        let outline_thickness = (3.0 * s) as i32;
        for i in 0..outline_thickness {
            draw_filled_ellipse_mut(
                img,
                (cx, cy),
                (120.0 * s) as i32 + i,
                (100.0 * s) as i32 + i,
                self.colors.outline,
            );
        }
        draw_filled_ellipse_mut(
            img,
            (cx, cy),
            (117.0 * s) as i32,
            (97.0 * s) as i32,
            self.colors.shirt,
        );
    }

    fn draw_neck(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32) {
        // Neck
        draw_filled_ellipse_mut(
            img,
            (cx, cy),
            (30.0 * s) as i32,
            (40.0 * s) as i32,
            self.colors.skin,
        );
    }

    fn draw_head(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32) {
        // Head outline
        draw_filled_ellipse_mut(
            img,
            (cx, cy),
            (100.0 * s) as i32,
            (120.0 * s) as i32,
            self.colors.outline,
        );

        // Head fill
        draw_filled_ellipse_mut(
            img,
            (cx, cy),
            (96.0 * s) as i32,
            (116.0 * s) as i32,
            self.colors.skin,
        );
    }

    fn draw_hair(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32) {
        // Simple spiky hair on top
        draw_filled_ellipse_mut(
            img,
            (cx, cy - (20.0 * s) as i32),
            (80.0 * s) as i32,
            (50.0 * s) as i32,
            self.colors.hair,
        );

        // Hair spikes
        for offset in [-40, -20, 0, 20, 40].iter() {
            let spike_x = cx + (*offset as f32 * s) as i32;
            let spike_y = cy - (40.0 * s) as i32;
            draw_filled_circle_mut(
                img,
                (spike_x, spike_y),
                (15.0 * s) as i32,
                self.colors.hair,
            );
        }
    }

    fn draw_face(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32, expression: Expression) {
        match expression {
            Expression::Neutral => self.draw_neutral_face(img, cx, cy, s),
            Expression::Shocked => self.draw_shocked_face(img, cx, cy, s),
            Expression::Serious => self.draw_serious_face(img, cx, cy, s),
            Expression::Mischievous => self.draw_mischievous_face(img, cx, cy, s),
            Expression::MindBlown => self.draw_mind_blown_face(img, cx, cy, s),
            Expression::Whispering => self.draw_whispering_face(img, cx, cy, s),
        }
    }

    fn draw_neutral_face(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32) {
        // Eyes
        let eye_y = cy - (10.0 * s) as i32;
        let eye_offset = (35.0 * s) as i32;

        // Left eye
        draw_filled_ellipse_mut(img, (cx - eye_offset, eye_y), (20.0 * s) as i32, (25.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx - eye_offset, eye_y), (12.0 * s) as i32, self.colors.eye_pupil);

        // Right eye
        draw_filled_ellipse_mut(img, (cx + eye_offset, eye_y), (20.0 * s) as i32, (25.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx + eye_offset, eye_y), (12.0 * s) as i32, self.colors.eye_pupil);

        // Mouth - small open
        let mouth_y = cy + (40.0 * s) as i32;
        draw_filled_ellipse_mut(img, (cx, mouth_y), (15.0 * s) as i32, (10.0 * s) as i32, self.colors.mouth);

        // Eyebrows
        self.draw_eyebrow(img, cx - eye_offset, eye_y - (30.0 * s) as i32, s, 0.0);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - (30.0 * s) as i32, s, 0.0);
    }

    fn draw_shocked_face(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32) {
        // Wide eyes
        let eye_y = cy - (10.0 * s) as i32;
        let eye_offset = (35.0 * s) as i32;

        // Left eye - extra wide
        draw_filled_ellipse_mut(img, (cx - eye_offset, eye_y), (28.0 * s) as i32, (35.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx - eye_offset, eye_y - (5.0 * s) as i32), (10.0 * s) as i32, self.colors.eye_pupil);

        // Right eye - extra wide
        draw_filled_ellipse_mut(img, (cx + eye_offset, eye_y), (28.0 * s) as i32, (35.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx + eye_offset, eye_y - (5.0 * s) as i32), (10.0 * s) as i32, self.colors.eye_pupil);

        // Open mouth - big O
        let mouth_y = cy + (45.0 * s) as i32;
        draw_filled_ellipse_mut(img, (cx, mouth_y), (25.0 * s) as i32, (30.0 * s) as i32, self.colors.outline);
        draw_filled_ellipse_mut(img, (cx, mouth_y), (20.0 * s) as i32, (25.0 * s) as i32, self.colors.mouth);

        // Raised eyebrows
        self.draw_eyebrow(img, cx - eye_offset, eye_y - (45.0 * s) as i32, s, -0.3);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - (45.0 * s) as i32, s, 0.3);
    }

    fn draw_serious_face(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32) {
        // Narrowed eyes
        let eye_y = cy - (10.0 * s) as i32;
        let eye_offset = (35.0 * s) as i32;

        // Left eye - narrowed
        draw_filled_ellipse_mut(img, (cx - eye_offset, eye_y), (22.0 * s) as i32, (12.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx - eye_offset, eye_y), (8.0 * s) as i32, self.colors.eye_pupil);

        // Right eye - narrowed
        draw_filled_ellipse_mut(img, (cx + eye_offset, eye_y), (22.0 * s) as i32, (12.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx + eye_offset, eye_y), (8.0 * s) as i32, self.colors.eye_pupil);

        // Flat mouth
        let mouth_y = cy + (40.0 * s) as i32;
        draw_line_segment_mut(
            img,
            ((cx - (25.0 * s) as i32) as f32, mouth_y as f32),
            ((cx + (25.0 * s) as i32) as f32, mouth_y as f32),
            self.colors.outline,
        );

        // Furrowed eyebrows
        self.draw_eyebrow(img, cx - eye_offset, eye_y - (25.0 * s) as i32, s, 0.4);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - (25.0 * s) as i32, s, -0.4);
    }

    fn draw_mischievous_face(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32) {
        // Sly eyes
        let eye_y = cy - (10.0 * s) as i32;
        let eye_offset = (35.0 * s) as i32;

        // Left eye - slightly narrowed
        draw_filled_ellipse_mut(img, (cx - eye_offset, eye_y), (20.0 * s) as i32, (18.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx - eye_offset + (3.0 * s) as i32, eye_y), (10.0 * s) as i32, self.colors.eye_pupil);

        // Right eye - slightly narrowed
        draw_filled_ellipse_mut(img, (cx + eye_offset, eye_y), (20.0 * s) as i32, (18.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx + eye_offset + (3.0 * s) as i32, eye_y), (10.0 * s) as i32, self.colors.eye_pupil);

        // Smirking mouth - curved up on one side
        let mouth_y = cy + (40.0 * s) as i32;
        draw_filled_ellipse_mut(img, (cx + (10.0 * s) as i32, mouth_y), (25.0 * s) as i32, (12.0 * s) as i32, self.colors.mouth);

        // One raised eyebrow
        self.draw_eyebrow(img, cx - eye_offset, eye_y - (28.0 * s) as i32, s, 0.2);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - (35.0 * s) as i32, s, -0.3);
    }

    fn draw_mind_blown_face(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32) {
        // Extra wide shocked eyes with spiral pupils
        let eye_y = cy - (10.0 * s) as i32;
        let eye_offset = (35.0 * s) as i32;

        // Left eye - huge
        draw_filled_ellipse_mut(img, (cx - eye_offset, eye_y), (30.0 * s) as i32, (38.0 * s) as i32, self.colors.eye_white);
        // Spiral effect - concentric circles
        draw_filled_circle_mut(img, (cx - eye_offset, eye_y), (15.0 * s) as i32, self.colors.eye_pupil);
        draw_filled_circle_mut(img, (cx - eye_offset, eye_y), (10.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx - eye_offset, eye_y), (5.0 * s) as i32, self.colors.eye_pupil);

        // Right eye - huge
        draw_filled_ellipse_mut(img, (cx + eye_offset, eye_y), (30.0 * s) as i32, (38.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx + eye_offset, eye_y), (15.0 * s) as i32, self.colors.eye_pupil);
        draw_filled_circle_mut(img, (cx + eye_offset, eye_y), (10.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx + eye_offset, eye_y), (5.0 * s) as i32, self.colors.eye_pupil);

        // Open mouth - huge O
        let mouth_y = cy + (50.0 * s) as i32;
        draw_filled_ellipse_mut(img, (cx, mouth_y), (35.0 * s) as i32, (40.0 * s) as i32, self.colors.outline);
        draw_filled_ellipse_mut(img, (cx, mouth_y), (30.0 * s) as i32, (35.0 * s) as i32, Rgba([80, 30, 30, 255])); // Dark mouth interior

        // Very raised eyebrows
        self.draw_eyebrow(img, cx - eye_offset, eye_y - (55.0 * s) as i32, s, -0.3);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - (55.0 * s) as i32, s, 0.3);
    }

    fn draw_whispering_face(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32) {
        // Looking to the side eyes
        let eye_y = cy - (10.0 * s) as i32;
        let eye_offset = (35.0 * s) as i32;

        // Left eye - looking right
        draw_filled_ellipse_mut(img, (cx - eye_offset, eye_y), (18.0 * s) as i32, (22.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx - eye_offset + (6.0 * s) as i32, eye_y), (10.0 * s) as i32, self.colors.eye_pupil);

        // Right eye - looking right
        draw_filled_ellipse_mut(img, (cx + eye_offset, eye_y), (18.0 * s) as i32, (22.0 * s) as i32, self.colors.eye_white);
        draw_filled_circle_mut(img, (cx + eye_offset + (6.0 * s) as i32, eye_y), (10.0 * s) as i32, self.colors.eye_pupil);

        // Small pursed mouth
        let mouth_y = cy + (40.0 * s) as i32;
        draw_filled_ellipse_mut(img, (cx, mouth_y), (10.0 * s) as i32, (8.0 * s) as i32, self.colors.mouth);

        // Slightly raised eyebrows
        self.draw_eyebrow(img, cx - eye_offset, eye_y - (32.0 * s) as i32, s, -0.1);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - (32.0 * s) as i32, s, 0.1);
    }

    fn draw_eyebrow(&self, img: &mut RgbaImage, cx: i32, cy: i32, s: f32, angle: f32) {
        let width = (30.0 * s) as i32;
        let height_offset = (angle * 15.0 * s) as i32;

        draw_line_segment_mut(
            img,
            ((cx - width / 2) as f32, (cy + height_offset) as f32),
            ((cx + width / 2) as f32, (cy - height_offset) as f32),
            self.colors.outline,
        );
        // Make it thicker
        draw_line_segment_mut(
            img,
            ((cx - width / 2) as f32, (cy + height_offset + 1) as f32),
            ((cx + width / 2) as f32, (cy - height_offset + 1) as f32),
            self.colors.outline,
        );
        draw_line_segment_mut(
            img,
            ((cx - width / 2) as f32, (cy + height_offset + 2) as f32),
            ((cx + width / 2) as f32, (cy - height_offset + 2) as f32),
            self.colors.outline,
        );
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new()
    }
}
