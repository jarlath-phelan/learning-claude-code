//! Character sprite generation module
//!
//! Creates a polished 2D cartoon character with anti-aliased rendering
//! and smooth gradients for professional animated quality.

use image::{Rgba, RgbaImage};

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

/// Colors for the character with gradients
pub struct CharacterColors {
    pub skin: Rgba<u8>,
    pub skin_shadow: Rgba<u8>,
    pub skin_highlight: Rgba<u8>,
    pub outline: Rgba<u8>,
    pub hair: Rgba<u8>,
    pub hair_highlight: Rgba<u8>,
    pub hair_shadow: Rgba<u8>,
    pub eye_white: Rgba<u8>,
    pub eye_pupil: Rgba<u8>,
    pub eye_iris: Rgba<u8>,
    pub eye_shine: Rgba<u8>,
    pub mouth: Rgba<u8>,
    pub mouth_dark: Rgba<u8>,
    pub teeth: Rgba<u8>,
    pub tongue: Rgba<u8>,
    pub blush: Rgba<u8>,
    pub shirt: Rgba<u8>,
    pub shirt_shadow: Rgba<u8>,
    pub shirt_highlight: Rgba<u8>,
}

impl Default for CharacterColors {
    fn default() -> Self {
        Self {
            skin: Rgba([255, 218, 185, 255]),
            skin_shadow: Rgba([235, 180, 145, 255]),
            skin_highlight: Rgba([255, 238, 220, 255]),
            outline: Rgba([55, 45, 45, 255]),
            hair: Rgba([45, 32, 22, 255]),
            hair_highlight: Rgba([85, 60, 40, 255]),
            hair_shadow: Rgba([30, 22, 15, 255]),
            eye_white: Rgba([252, 252, 255, 255]),
            eye_pupil: Rgba([15, 15, 20, 255]),
            eye_iris: Rgba([75, 55, 35, 255]),
            eye_shine: Rgba([255, 255, 255, 255]),
            mouth: Rgba([210, 105, 105, 255]),
            mouth_dark: Rgba([60, 25, 25, 255]),
            teeth: Rgba([255, 255, 252, 255]),
            tongue: Rgba([220, 130, 130, 255]),
            blush: Rgba([255, 180, 180, 80]),
            shirt: Rgba([220, 55, 55, 255]),
            shirt_shadow: Rgba([170, 35, 35, 255]),
            shirt_highlight: Rgba([250, 95, 95, 255]),
        }
    }
}

/// Character renderer with anti-aliased quality
pub struct Character {
    colors: CharacterColors,
}

impl Character {
    pub fn new() -> Self {
        Self {
            colors: CharacterColors::default(),
        }
    }

    /// Render the character with the specified expression
    pub fn render(&self, expression: Expression, scale: f32) -> RgbaImage {
        let base_width = 500;
        let base_height = 600;
        let width = (base_width as f32 * scale) as u32;
        let height = (base_height as f32 * scale) as u32;

        let mut img = RgbaImage::new(width, height);
        let s = scale;
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;

        // Draw in order: body, neck, head, hair, face
        self.draw_body(&mut img, cx, cy + 180.0 * s, s);
        self.draw_neck(&mut img, cx, cy + 90.0 * s, s);
        self.draw_head(&mut img, cx, cy, s);
        self.draw_ears(&mut img, cx, cy, s);
        self.draw_hair(&mut img, cx, cy - 60.0 * s, s);
        self.draw_face(&mut img, cx, cy, s, expression);

        img
    }

    // Anti-aliased smooth ellipse with gradient
    fn draw_smooth_ellipse(&self, img: &mut RgbaImage, cx: f32, cy: f32, rx: f32, ry: f32, color: Rgba<u8>) {
        let x_start = (cx - rx - 2.0).max(0.0) as u32;
        let x_end = ((cx + rx + 2.0) as u32).min(img.width());
        let y_start = (cy - ry - 2.0).max(0.0) as u32;
        let y_end = ((cy + ry + 2.0) as u32).min(img.height());

        for y in y_start..y_end {
            for x in x_start..x_end {
                let dx = (x as f32 - cx) / rx;
                let dy = (y as f32 - cy) / ry;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < 1.0 {
                    // Fully inside
                    self.blend_pixel(img, x, y, color);
                } else if dist < 1.05 {
                    // Anti-aliased edge
                    let alpha = ((1.05 - dist) / 0.05 * color[3] as f32) as u8;
                    let aa_color = Rgba([color[0], color[1], color[2], alpha]);
                    self.blend_pixel(img, x, y, aa_color);
                }
            }
        }
    }

    // Anti-aliased ellipse with gradient shading
    fn draw_shaded_ellipse(&self, img: &mut RgbaImage, cx: f32, cy: f32, rx: f32, ry: f32,
                           base: Rgba<u8>, shadow: Rgba<u8>, highlight: Rgba<u8>) {
        let x_start = (cx - rx - 2.0).max(0.0) as u32;
        let x_end = ((cx + rx + 2.0) as u32).min(img.width());
        let y_start = (cy - ry - 2.0).max(0.0) as u32;
        let y_end = ((cy + ry + 2.0) as u32).min(img.height());

        for y in y_start..y_end {
            for x in x_start..x_end {
                let dx = (x as f32 - cx) / rx;
                let dy = (y as f32 - cy) / ry;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < 1.0 {
                    // Gradient from top-left (highlight) to bottom-right (shadow)
                    let gradient = ((dx + dy) / 2.0 * 0.5 + 0.5).clamp(0.0, 1.0);

                    // Top area gets highlight
                    let vert_gradient = ((dy + 1.0) / 2.0).clamp(0.0, 1.0);

                    let r = Self::lerp3(highlight[0], base[0], shadow[0], gradient, vert_gradient);
                    let g = Self::lerp3(highlight[1], base[1], shadow[1], gradient, vert_gradient);
                    let b = Self::lerp3(highlight[2], base[2], shadow[2], gradient, vert_gradient);

                    let color = Rgba([r as u8, g as u8, b as u8, 255]);
                    self.blend_pixel(img, x, y, color);
                } else if dist < 1.05 {
                    // Anti-aliased edge
                    let alpha = ((1.05 - dist) / 0.05 * 255.0) as u8;
                    let color = Rgba([base[0], base[1], base[2], alpha]);
                    self.blend_pixel(img, x, y, color);
                }
            }
        }
    }

    fn lerp3(highlight: u8, base: u8, shadow: u8, horiz: f32, vert: f32) -> f32 {
        let mid = Self::lerp(base as f32, shadow as f32, horiz);
        Self::lerp(highlight as f32, mid, vert)
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    // Anti-aliased circle
    fn draw_smooth_circle(&self, img: &mut RgbaImage, cx: f32, cy: f32, r: f32, color: Rgba<u8>) {
        self.draw_smooth_ellipse(img, cx, cy, r, r, color);
    }

    // Draw outline around ellipse
    fn draw_ellipse_outline(&self, img: &mut RgbaImage, cx: f32, cy: f32, rx: f32, ry: f32,
                            thickness: f32, color: Rgba<u8>) {
        let outer_rx = rx + thickness;
        let outer_ry = ry + thickness;

        let x_start = (cx - outer_rx - 2.0).max(0.0) as u32;
        let x_end = ((cx + outer_rx + 2.0) as u32).min(img.width());
        let y_start = (cy - outer_ry - 2.0).max(0.0) as u32;
        let y_end = ((cy + outer_ry + 2.0) as u32).min(img.height());

        for y in y_start..y_end {
            for x in x_start..x_end {
                let dx = (x as f32 - cx) / rx;
                let dy = (y as f32 - cy) / ry;
                let inner_dist = (dx * dx + dy * dy).sqrt();

                let odx = (x as f32 - cx) / outer_rx;
                let ody = (y as f32 - cy) / outer_ry;
                let outer_dist = (odx * odx + ody * ody).sqrt();

                // In the ring between inner and outer
                if inner_dist >= 1.0 && outer_dist <= 1.0 {
                    // Anti-alias inner edge
                    let inner_aa = if inner_dist < 1.05 {
                        (inner_dist - 1.0) / 0.05
                    } else {
                        1.0
                    };
                    // Anti-alias outer edge
                    let outer_aa = if outer_dist > 0.95 {
                        (1.0 - outer_dist) / 0.05
                    } else {
                        1.0
                    };

                    let alpha = (inner_aa.min(outer_aa) * color[3] as f32) as u8;
                    let aa_color = Rgba([color[0], color[1], color[2], alpha]);
                    self.blend_pixel(img, x, y, aa_color);
                }
            }
        }
    }

    fn blend_pixel(&self, img: &mut RgbaImage, x: u32, y: u32, src: Rgba<u8>) {
        if x >= img.width() || y >= img.height() {
            return;
        }

        let src_a = src[3] as f32 / 255.0;
        if src_a < 0.001 {
            return;
        }

        let dest = img.get_pixel(x, y);
        let dest_a = dest[3] as f32 / 255.0;
        let out_a = src_a + dest_a * (1.0 - src_a);

        if out_a < 0.001 {
            return;
        }

        let r = (src[0] as f32 * src_a + dest[0] as f32 * dest_a * (1.0 - src_a)) / out_a;
        let g = (src[1] as f32 * src_a + dest[1] as f32 * dest_a * (1.0 - src_a)) / out_a;
        let b = (src[2] as f32 * src_a + dest[2] as f32 * dest_a * (1.0 - src_a)) / out_a;

        img.put_pixel(x, y, Rgba([r as u8, g as u8, b as u8, (out_a * 255.0) as u8]));
    }

    fn draw_body(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        // Shirt with gradient shading
        self.draw_shaded_ellipse(img, cx, cy, 140.0 * s, 120.0 * s,
            self.colors.shirt, self.colors.shirt_shadow, self.colors.shirt_highlight);

        // Shirt outline
        self.draw_ellipse_outline(img, cx, cy, 140.0 * s, 120.0 * s, 3.0 * s, self.colors.outline);

        // Collar V-shape detail
        self.draw_smooth_ellipse(img, cx, cy - 75.0 * s, 35.0 * s, 20.0 * s, self.colors.shirt_shadow);
    }

    fn draw_neck(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        // Neck with shading
        self.draw_shaded_ellipse(img, cx, cy, 35.0 * s, 55.0 * s,
            self.colors.skin, self.colors.skin_shadow, self.colors.skin);
    }

    fn draw_head(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        // Head outline first (slightly larger)
        self.draw_smooth_ellipse(img, cx, cy, 118.0 * s, 138.0 * s, self.colors.outline);

        // Main head with gradient
        self.draw_shaded_ellipse(img, cx, cy, 115.0 * s, 135.0 * s,
            self.colors.skin, self.colors.skin_shadow, self.colors.skin_highlight);
    }

    fn draw_ears(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        // Left ear
        self.draw_shaded_ellipse(img, cx - 108.0 * s, cy - 10.0 * s, 22.0 * s, 32.0 * s,
            self.colors.skin, self.colors.skin_shadow, self.colors.skin);
        self.draw_smooth_ellipse(img, cx - 108.0 * s, cy - 10.0 * s, 12.0 * s, 18.0 * s,
            self.colors.skin_shadow);

        // Right ear
        self.draw_shaded_ellipse(img, cx + 108.0 * s, cy - 10.0 * s, 22.0 * s, 32.0 * s,
            self.colors.skin, self.colors.skin_shadow, self.colors.skin);
        self.draw_smooth_ellipse(img, cx + 108.0 * s, cy - 10.0 * s, 12.0 * s, 18.0 * s,
            self.colors.skin_shadow);
    }

    fn draw_hair(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        // Main hair mass with gradient
        self.draw_shaded_ellipse(img, cx, cy - 25.0 * s, 105.0 * s, 75.0 * s,
            self.colors.hair, self.colors.hair_shadow, self.colors.hair_highlight);

        // Stylized hair spikes with shading
        let spikes = [
            (-65.0, -55.0, 25.0),
            (-38.0, -72.0, 28.0),
            (-8.0, -82.0, 32.0),
            (25.0, -75.0, 30.0),
            (52.0, -60.0, 26.0),
            (72.0, -42.0, 22.0),
            (-80.0, -32.0, 20.0),
            (82.0, -28.0, 18.0),
        ];

        for (ox, oy, size) in spikes.iter() {
            self.draw_shaded_ellipse(img, cx + ox * s, cy + oy * s,
                size * s, size * 1.3 * s,
                self.colors.hair, self.colors.hair_shadow, self.colors.hair_highlight);
        }

        // Hair highlights (shiny spots)
        self.draw_smooth_circle(img, cx - 32.0 * s, cy - 62.0 * s, 14.0 * s, self.colors.hair_highlight);
        self.draw_smooth_circle(img, cx + 12.0 * s, cy - 68.0 * s, 12.0 * s, self.colors.hair_highlight);
        self.draw_smooth_circle(img, cx + 42.0 * s, cy - 52.0 * s, 10.0 * s, self.colors.hair_highlight);

        // Bangs over forehead
        self.draw_shaded_ellipse(img, cx - 32.0 * s, cy + 28.0 * s, 38.0 * s, 22.0 * s,
            self.colors.hair, self.colors.hair_shadow, self.colors.hair_highlight);
        self.draw_shaded_ellipse(img, cx + 22.0 * s, cy + 24.0 * s, 32.0 * s, 20.0 * s,
            self.colors.hair, self.colors.hair_shadow, self.colors.hair_highlight);
    }

    fn draw_face(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32, expression: Expression) {
        // Subtle cheek blush
        self.draw_smooth_ellipse(img, cx - 58.0 * s, cy + 32.0 * s, 28.0 * s, 16.0 * s, self.colors.blush);
        self.draw_smooth_ellipse(img, cx + 58.0 * s, cy + 32.0 * s, 28.0 * s, 16.0 * s, self.colors.blush);

        match expression {
            Expression::Neutral => self.draw_neutral_face(img, cx, cy, s),
            Expression::Shocked => self.draw_shocked_face(img, cx, cy, s),
            Expression::Serious => self.draw_serious_face(img, cx, cy, s),
            Expression::Mischievous => self.draw_mischievous_face(img, cx, cy, s),
            Expression::MindBlown => self.draw_mind_blown_face(img, cx, cy, s),
            Expression::Whispering => self.draw_whispering_face(img, cx, cy, s),
        }
    }

    fn draw_eye(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32,
                width: f32, height: f32, pupil_ox: f32, pupil_oy: f32) {
        // Eye white
        self.draw_smooth_ellipse(img, cx, cy, width * s, height * s, self.colors.eye_white);

        // Eye outline
        self.draw_ellipse_outline(img, cx, cy, width * s, height * s, 2.5 * s, self.colors.outline);

        // Iris
        let iris_r = height * 0.55;
        self.draw_smooth_circle(img, cx + pupil_ox * s, cy + pupil_oy * s, iris_r * s, self.colors.eye_iris);

        // Pupil
        let pupil_r = iris_r * 0.6;
        self.draw_smooth_circle(img, cx + pupil_ox * s, cy + pupil_oy * s, pupil_r * s, self.colors.eye_pupil);

        // Eye shine (large and small)
        self.draw_smooth_circle(img, cx + (pupil_ox - 6.0) * s, cy + (pupil_oy - 6.0) * s,
            6.0 * s, self.colors.eye_shine);
        self.draw_smooth_circle(img, cx + (pupil_ox + 4.0) * s, cy + (pupil_oy + 4.0) * s,
            3.0 * s, self.colors.eye_shine);
    }

    fn draw_eyebrow(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32, angle: f32, thickness: f32) {
        // Draw eyebrow as a smooth curved shape
        let width = 38.0 * s;
        let height = (thickness * 0.6) * s;

        // Angled by adjusting y based on x
        let x_start = (cx - width).max(0.0) as u32;
        let x_end = ((cx + width) as u32).min(img.width());
        let y_start = (cy - height * 3.0).max(0.0) as u32;
        let y_end = ((cy + height * 3.0) as u32).min(img.height());

        for y in y_start..y_end {
            for x in x_start..x_end {
                let dx = (x as f32 - cx) / width;
                let angle_offset = dx * angle * 20.0 * s;
                let dy = (y as f32 - cy - angle_offset) / height;

                let dist = (dx * dx + dy * dy).sqrt();

                if dist < 1.0 {
                    self.blend_pixel(img, x, y, self.colors.outline);
                } else if dist < 1.1 {
                    let alpha = ((1.1 - dist) / 0.1 * 255.0) as u8;
                    let aa = Rgba([self.colors.outline[0], self.colors.outline[1],
                                   self.colors.outline[2], alpha]);
                    self.blend_pixel(img, x, y, aa);
                }
            }
        }
    }

    fn draw_nose(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        self.draw_smooth_ellipse(img, cx, cy, 9.0 * s, 6.0 * s, self.colors.skin_shadow);
    }

    fn draw_smile(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32,
                  width: f32, height: f32, show_teeth: bool) {
        // Mouth outline
        self.draw_ellipse_outline(img, cx, cy, width * s, height * s, 2.0 * s, self.colors.outline);

        // Mouth interior
        self.draw_smooth_ellipse(img, cx, cy, width * s, height * s, self.colors.mouth);

        if show_teeth {
            self.draw_smooth_ellipse(img, cx, cy - height * 0.25 * s,
                width * 0.75 * s, height * 0.4 * s, self.colors.teeth);
        }
    }

    fn draw_neutral_face(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        let eye_y = cy - 18.0 * s;
        let eye_offset = 48.0 * s;

        // Eyes
        self.draw_eye(img, cx - eye_offset, eye_y, s, 26.0, 32.0, 0.0, 0.0);
        self.draw_eye(img, cx + eye_offset, eye_y, s, 26.0, 32.0, 0.0, 0.0);

        // Eyebrows
        self.draw_eyebrow(img, cx - eye_offset, eye_y - 42.0 * s, s, 0.0, 5.5);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - 42.0 * s, s, 0.0, 5.5);

        // Nose
        self.draw_nose(img, cx, cy + 18.0 * s, s);

        // Smile
        self.draw_smile(img, cx, cy + 58.0 * s, s, 32.0, 16.0, true);
    }

    fn draw_shocked_face(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        let eye_y = cy - 18.0 * s;
        let eye_offset = 48.0 * s;

        // Wide eyes
        self.draw_eye(img, cx - eye_offset, eye_y, s, 34.0, 42.0, 0.0, -4.0);
        self.draw_eye(img, cx + eye_offset, eye_y, s, 34.0, 42.0, 0.0, -4.0);

        // Raised eyebrows
        self.draw_eyebrow(img, cx - eye_offset, eye_y - 58.0 * s, s, -0.35, 5.0);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - 58.0 * s, s, 0.35, 5.0);

        // Nose
        self.draw_nose(img, cx, cy + 18.0 * s, s);

        // Open mouth "O"
        self.draw_ellipse_outline(img, cx, cy + 62.0 * s, 32.0 * s, 38.0 * s, 3.0 * s, self.colors.outline);
        self.draw_smooth_ellipse(img, cx, cy + 62.0 * s, 32.0 * s, 38.0 * s, self.colors.mouth_dark);
        self.draw_smooth_ellipse(img, cx, cy + 50.0 * s, 24.0 * s, 10.0 * s, self.colors.teeth);
    }

    fn draw_serious_face(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        let eye_y = cy - 18.0 * s;
        let eye_offset = 48.0 * s;

        // Narrowed eyes
        self.draw_eye(img, cx - eye_offset, eye_y, s, 30.0, 20.0, 0.0, 0.0);
        self.draw_eye(img, cx + eye_offset, eye_y, s, 30.0, 20.0, 0.0, 0.0);

        // Furrowed eyebrows
        self.draw_eyebrow(img, cx - eye_offset, eye_y - 32.0 * s, s, 0.45, 6.5);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - 32.0 * s, s, -0.45, 6.5);

        // Nose
        self.draw_nose(img, cx, cy + 18.0 * s, s);

        // Flat mouth
        self.draw_smooth_ellipse(img, cx, cy + 58.0 * s, 28.0 * s, 5.0 * s, self.colors.outline);
    }

    fn draw_mischievous_face(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        let eye_y = cy - 18.0 * s;
        let eye_offset = 48.0 * s;

        // Sly eyes looking to side
        self.draw_eye(img, cx - eye_offset, eye_y, s, 26.0, 24.0, 5.0, 0.0);
        self.draw_eye(img, cx + eye_offset, eye_y, s, 26.0, 24.0, 5.0, 0.0);

        // Asymmetric eyebrows
        self.draw_eyebrow(img, cx - eye_offset, eye_y - 38.0 * s, s, 0.25, 5.0);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - 45.0 * s, s, -0.3, 5.0);

        // Nose
        self.draw_nose(img, cx, cy + 18.0 * s, s);

        // Smirk
        self.draw_smooth_ellipse(img, cx + 10.0 * s, cy + 58.0 * s, 34.0 * s, 14.0 * s, self.colors.mouth);
        self.draw_ellipse_outline(img, cx + 10.0 * s, cy + 58.0 * s, 34.0 * s, 14.0 * s, 2.0 * s, self.colors.outline);
        self.draw_smooth_ellipse(img, cx + 22.0 * s, cy + 54.0 * s, 14.0 * s, 7.0 * s, self.colors.teeth);
    }

    fn draw_mind_blown_face(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        let eye_y = cy - 18.0 * s;
        let eye_offset = 48.0 * s;

        // Huge spiral eyes
        for side in [-1.0, 1.0] {
            let ex = cx + side * eye_offset;
            self.draw_smooth_ellipse(img, ex, eye_y, 38.0 * s, 48.0 * s, self.colors.eye_white);
            self.draw_ellipse_outline(img, ex, eye_y, 38.0 * s, 48.0 * s, 2.5 * s, self.colors.outline);

            // Spiral pattern
            self.draw_smooth_circle(img, ex, eye_y, 22.0 * s, self.colors.eye_pupil);
            self.draw_smooth_circle(img, ex, eye_y, 16.0 * s, self.colors.eye_white);
            self.draw_smooth_circle(img, ex, eye_y, 10.0 * s, self.colors.eye_pupil);
            self.draw_smooth_circle(img, ex, eye_y, 5.0 * s, self.colors.eye_white);
        }

        // Very raised eyebrows
        self.draw_eyebrow(img, cx - eye_offset, eye_y - 65.0 * s, s, -0.4, 5.0);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - 65.0 * s, s, 0.4, 5.0);

        // Nose
        self.draw_nose(img, cx, cy + 18.0 * s, s);

        // Huge open mouth
        self.draw_ellipse_outline(img, cx, cy + 68.0 * s, 42.0 * s, 48.0 * s, 3.0 * s, self.colors.outline);
        self.draw_smooth_ellipse(img, cx, cy + 68.0 * s, 42.0 * s, 48.0 * s, self.colors.mouth_dark);
        self.draw_smooth_ellipse(img, cx, cy + 52.0 * s, 32.0 * s, 12.0 * s, self.colors.teeth);
        self.draw_smooth_ellipse(img, cx, cy + 82.0 * s, 24.0 * s, 18.0 * s, self.colors.tongue);
    }

    fn draw_whispering_face(&self, img: &mut RgbaImage, cx: f32, cy: f32, s: f32) {
        let eye_y = cy - 18.0 * s;
        let eye_offset = 48.0 * s;

        // Eyes looking to side
        self.draw_eye(img, cx - eye_offset, eye_y, s, 24.0, 28.0, 9.0, 0.0);
        self.draw_eye(img, cx + eye_offset, eye_y, s, 24.0, 28.0, 9.0, 0.0);

        // Slightly raised eyebrows
        self.draw_eyebrow(img, cx - eye_offset, eye_y - 40.0 * s, s, -0.12, 5.0);
        self.draw_eyebrow(img, cx + eye_offset, eye_y - 40.0 * s, s, 0.12, 5.0);

        // Nose
        self.draw_nose(img, cx, cy + 18.0 * s, s);

        // Small pursed mouth
        self.draw_smooth_ellipse(img, cx, cy + 58.0 * s, 14.0 * s, 12.0 * s, self.colors.mouth);
        self.draw_ellipse_outline(img, cx, cy + 58.0 * s, 14.0 * s, 12.0 * s, 2.0 * s, self.colors.outline);
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new()
    }
}
