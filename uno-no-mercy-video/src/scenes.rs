//! Scene management and composition
//!
//! Orchestrates the different scenes of the video.

use anyhow::Result;
use image::{Rgba, RgbaImage};

use crate::character::{Character, Expression};
use crate::cards::{Card, CardColor, CardFactory, CardRenderer};
use crate::effects::{Easing, Fade, Flash, Glow, Particles, PopIn, ScreenShake, Slide};
use crate::text::{AnimatedText, LowerThird, TextRenderer, TextStyle, TitleCard};
use crate::video::{Backgrounds, FrameComposer};
use crate::{VIDEO_HEIGHT, VIDEO_WIDTH};

/// Scene timing configuration
struct SceneTiming {
    start: f32,
    end: f32,
}

impl SceneTiming {
    fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }

    fn contains(&self, time: f32) -> bool {
        time >= self.start && time < self.end
    }

    fn progress(&self, time: f32) -> f32 {
        if time < self.start {
            0.0
        } else if time >= self.end {
            1.0
        } else {
            (time - self.start) / (self.end - self.start)
        }
    }
}

/// Manages all scenes in the video
pub struct SceneManager {
    composer: FrameComposer,
    character: Character,
    text_renderer: TextRenderer,
    scenes: Vec<SceneTiming>,
}

impl SceneManager {
    pub fn new() -> Self {
        // Define scene timings (in seconds)
        let scenes = vec![
            SceneTiming::new(0.0, 3.0),    // Scene 1: Hook
            SceneTiming::new(3.0, 15.0),   // Scene 2: The Basics
            SceneTiming::new(15.0, 30.0),  // Scene 3: Draw Cards
            SceneTiming::new(30.0, 45.0),  // Scene 4: Plot Twist
            SceneTiming::new(45.0, 60.0),  // Scene 5: Chaos Cards
            SceneTiming::new(60.0, 70.0),  // Scene 6: Golden Rule
            SceneTiming::new(70.0, 75.0),  // Scene 7: Outro
        ];

        Self {
            composer: FrameComposer::new(VIDEO_WIDTH, VIDEO_HEIGHT),
            character: Character::new(),
            text_renderer: TextRenderer::new(),
            scenes,
        }
    }

    /// Render a frame at the given time
    pub fn render_frame(&self, time: f32, frame_num: u32) -> Result<RgbaImage> {
        // Determine which scene we're in
        if self.scenes[0].contains(time) {
            self.render_scene_1_hook(time, frame_num)
        } else if self.scenes[1].contains(time) {
            self.render_scene_2_basics(time, frame_num)
        } else if self.scenes[2].contains(time) {
            self.render_scene_3_draw_cards(time, frame_num)
        } else if self.scenes[3].contains(time) {
            self.render_scene_4_plot_twist(time, frame_num)
        } else if self.scenes[4].contains(time) {
            self.render_scene_5_chaos(time, frame_num)
        } else if self.scenes[5].contains(time) {
            self.render_scene_6_golden_rule(time, frame_num)
        } else {
            self.render_scene_7_outro(time, frame_num)
        }
    }

    /// Scene 1: Hook (0-3 sec)
    /// "So you think you know UNO? Nah. Let me tell you about NO MERCY."
    fn render_scene_1_hook(&self, time: f32, _frame_num: u32) -> Result<RgbaImage> {
        let progress = self.scenes[0].progress(time);

        // Background - dramatic dark with moving lights
        let mut frame = Backgrounds::dramatic_dark(VIDEO_WIDTH, VIDEO_HEIGHT, time);

        // Add floating sparkles
        let sparkles = Particles::sparkles(VIDEO_WIDTH, VIDEO_HEIGHT, 15, time, 42);
        self.composer.composite(&mut frame, &sparkles, 0, 0);

        // Determine expression based on timing
        let expression = if progress < 0.3 {
            Expression::Neutral
        } else if progress < 0.6 {
            Expression::Serious
        } else {
            Expression::Shocked
        };

        // Character - zoom in effect with subtle bounce
        let base_scale = 1.5;
        let zoom_scale = if progress > 0.5 {
            let zoom_progress = (progress - 0.5) * 2.0;
            let bounce = (zoom_progress * 10.0).sin() * 0.02 * (1.0 - zoom_progress);
            base_scale + Easing::ease_out(zoom_progress) * 0.5 + bounce
        } else {
            base_scale
        };

        let char_img = self.character.render(expression, zoom_scale);
        let char_x = (VIDEO_WIDTH as i32 - char_img.width() as i32) / 2;
        let char_y = VIDEO_HEIGHT as i32 - char_img.height() as i32 + 100;
        self.composer.composite(&mut frame, &char_img, char_x, char_y);

        // Title text - "UNO NO MERCY" with glow and shake effect
        if progress > 0.4 {
            let text_progress = (progress - 0.4) / 0.6;
            let scale = PopIn::get_scale(text_progress, 1.2);

            if scale > 0.1 {
                let style = TextStyle::red_bold();
                let title = self.text_renderer.render("UNO NO MERCY", 120.0 * scale, &style);

                // Add glow to title
                let glowing_title = Glow::apply(&title, Rgba([255, 100, 50, 180]), 15, 0.8);

                // Add shake
                let shake = if progress > 0.6 {
                    let shake_time = progress - 0.6;
                    let intensity = 10.0 * (1.0 - shake_time * 2.0).max(0.0);
                    AnimatedText::shake_offset(shake_time * 20.0, intensity)
                } else {
                    (0, 0)
                };

                let x = (VIDEO_WIDTH as i32 - glowing_title.width() as i32) / 2 + shake.0;
                let y = (VIDEO_HEIGHT as f32 * 0.22) as i32 + shake.1;
                self.composer.composite(&mut frame, &glowing_title, x, y);
            }
        }

        // Energy wave effect at reveal
        if progress > 0.4 && progress < 0.7 {
            let wave_time = (progress - 0.4) / 0.3;
            let wave = Particles::energy_wave(VIDEO_WIDTH, VIDEO_HEIGHT, wave_time * 2.0, Rgba([255, 200, 100, 80]));
            self.composer.composite(&mut frame, &wave, 0, 0);
        }

        // Flash effect at the reveal
        if progress > 0.38 && progress < 0.45 {
            let flash_progress = (progress - 0.38) / 0.07;
            let flash_intensity = if flash_progress < 0.5 {
                flash_progress * 2.0
            } else {
                (1.0 - flash_progress) * 2.0
            };
            let flash = Flash::white(VIDEO_WIDTH, VIDEO_HEIGHT, flash_intensity * 0.7);
            self.composer.composite(&mut frame, &flash, 0, 0);
        }

        // Fade from black at start
        if progress < 0.15 {
            let fade = Fade::from_black(VIDEO_WIDTH, VIDEO_HEIGHT, progress / 0.15);
            self.composer.composite(&mut frame, &fade, 0, 0);
        }

        Ok(frame)
    }

    /// Scene 2: The Basics (3-15 sec)
    /// "168 cards. SIX players max. And if you get 25 cards..."
    fn render_scene_2_basics(&self, time: f32, _frame_num: u32) -> Result<RgbaImage> {
        let progress = self.scenes[1].progress(time);
        let local_time = time - self.scenes[1].start;

        // Background
        let mut frame = Backgrounds::solid_with_vignette(
            VIDEO_WIDTH, VIDEO_HEIGHT,
            Rgba([25, 20, 35, 255]),
            0.5
        );

        // Character talking
        let expression = if local_time < 4.0 {
            Expression::Neutral
        } else if local_time < 8.0 {
            Expression::Serious
        } else {
            Expression::Mischievous
        };

        let char_img = self.character.render(expression, 1.2);
        let char_x = (VIDEO_WIDTH as i32 - char_img.width() as i32) / 2;
        let char_y = VIDEO_HEIGHT as i32 - char_img.height() as i32 + 50;
        self.composer.composite(&mut frame, &char_img, char_x, char_y);

        // Flying cards effect
        if progress > 0.1 {
            let flying_cards = CardRenderer::render_flying_cards(100, 150, 15, (time * 2.0) as u64);
            let scaled = FrameComposer::scale_image(&flying_cards, 1.5);
            let card_y = Slide::from_top(
                -(scaled.height() as i32),
                (VIDEO_HEIGHT as f32 * 0.15) as i32,
                ((progress - 0.1) * 2.0).min(1.0),
                Easing::ease_out
            );
            self.composer.composite_with_alpha(&mut frame, &scaled, 50, card_y, 0.7);
        }

        // Text overlays appearing in sequence
        let texts = [
            (0.1, "168 CARDS"),
            (0.35, "6 PLAYERS MAX"),
            (0.6, "25 CARDS = ELIMINATED"),
        ];

        for (threshold, text) in texts.iter() {
            if progress > *threshold {
                let text_progress = ((progress - threshold) / 0.15).min(1.0);
                let scale = PopIn::get_scale(text_progress, 0.8);

                if scale > 0.1 {
                    let style = TextStyle::yellow_impact();
                    let text_img = self.text_renderer.render(text, 80.0 * scale, &style);

                    let y_offset = match *threshold {
                        t if t < 0.2 => 0.15,
                        t if t < 0.5 => 0.25,
                        _ => 0.35,
                    };

                    let x = (VIDEO_WIDTH as i32 - text_img.width() as i32) / 2;
                    let y = (VIDEO_HEIGHT as f32 * y_offset) as i32;
                    self.composer.composite(&mut frame, &text_img, x, y);
                }
            }
        }

        // Skull emoji effect for elimination
        if progress > 0.75 {
            let skull_progress = (progress - 0.75) / 0.25;
            let scale = PopIn::get_scale(skull_progress, 1.5);
            if scale > 0.1 {
                // Simple skull representation with text
                let style = TextStyle {
                    color: Rgba([255, 255, 255, 255]),
                    outline_color: Some(Rgba([0, 0, 0, 255])),
                    outline_width: 4,
                    shadow: true,
                    shadow_offset: (3, 3),
                    shadow_color: Rgba([0, 0, 0, 200]),
                };
                let skull = self.text_renderer.render("X_X", 100.0 * scale, &style);
                let x = (VIDEO_WIDTH as i32 - skull.width() as i32) / 2;
                let y = (VIDEO_HEIGHT as f32 * 0.45) as i32;
                self.composer.composite(&mut frame, &skull, x, y);
            }
        }

        Ok(frame)
    }

    /// Scene 3: Draw Cards (15-30 sec)
    /// "+2? That's cute. +4? Getting warmer. +10!"
    fn render_scene_3_draw_cards(&self, time: f32, _frame_num: u32) -> Result<RgbaImage> {
        let progress = self.scenes[2].progress(time);
        let local_time = time - self.scenes[2].start;

        // Background
        let mut frame = Backgrounds::uno_theme(VIDEO_WIDTH, VIDEO_HEIGHT, time);

        // Character reacting
        let expression = if local_time < 3.0 {
            Expression::Neutral
        } else if local_time < 6.0 {
            Expression::Serious
        } else if local_time < 10.0 {
            Expression::Shocked
        } else {
            Expression::Mischievous
        };

        let char_img = self.character.render(expression, 1.0);
        let char_x = VIDEO_WIDTH as i32 - char_img.width() as i32 - 50;
        let char_y = VIDEO_HEIGHT as i32 - char_img.height() as i32;
        self.composer.composite(&mut frame, &char_img, char_x, char_y);

        // Show cards in sequence with growing size
        let cards_data = [
            (0.0, 0.2, CardFactory::plus_two(CardColor::Red), 0.8, "+2"),
            (0.2, 0.4, CardFactory::plus_four(CardColor::Blue), 1.0, "+4"),
            (0.4, 0.7, CardFactory::plus_ten(), 1.4, "+10"),
        ];

        for (start, end, card, scale, label) in cards_data.iter() {
            if progress >= *start && progress < *end {
                let card_progress = (progress - start) / (end - start);

                // Card flies in from left
                let card_img = card.render(150, 220);
                let scaled = FrameComposer::scale_image(&card_img, *scale);

                let x = Slide::from_left(
                    -(scaled.width() as i32),
                    (VIDEO_WIDTH as f32 * 0.25) as i32,
                    card_progress,
                    Easing::ease_out
                );
                let y = (VIDEO_HEIGHT as f32 * 0.35) as i32;
                self.composer.composite(&mut frame, &scaled, x, y);

                // Label
                let style = TextStyle::yellow_impact();
                let label_img = self.text_renderer.render(label, 100.0 * scale, &style);
                let label_x = (VIDEO_WIDTH as i32 - label_img.width() as i32) / 2;
                let label_y = (VIDEO_HEIGHT as f32 * 0.15) as i32;
                self.composer.composite(&mut frame, &label_img, label_x, label_y);
            }
        }

        // Stacking demonstration
        if progress >= 0.7 {
            let stack_progress = (progress - 0.7) / 0.3;

            // Show stacking equation
            let style = TextStyle::white_with_black_outline();

            if stack_progress < 0.5 {
                let text = "4 + 6 = 10";
                let text_img = self.text_renderer.render(text, 90.0, &style);
                let x = (VIDEO_WIDTH as i32 - text_img.width() as i32) / 2;
                self.composer.composite(&mut frame, &text_img, x, (VIDEO_HEIGHT as f32 * 0.2) as i32);
            } else {
                // Show pile dumping
                let pile_text = "THEY DRAW EVERYTHING!";
                let pile_img = self.text_renderer.render(pile_text, 70.0, &TextStyle::red_bold());
                let shake = AnimatedText::shake_offset(stack_progress * 10.0, 5.0);
                let x = (VIDEO_WIDTH as i32 - pile_img.width() as i32) / 2 + shake.0;
                let y = (VIDEO_HEIGHT as f32 * 0.2) as i32 + shake.1;
                self.composer.composite(&mut frame, &pile_img, x, y);
            }

            // Cards stack
            let cards = vec![
                CardFactory::plus_four(CardColor::Red),
                CardFactory::plus_six(),
            ];
            let stack = CardRenderer::render_stack(&cards, 100, 150, 20);
            let stack_x = (VIDEO_WIDTH as i32 - stack.width() as i32) / 2;
            let stack_y = (VIDEO_HEIGHT as f32 * 0.4) as i32;
            self.composer.composite(&mut frame, &stack, stack_x, stack_y);
        }

        Ok(frame)
    }

    /// Scene 4: Plot Twist (30-45 sec)
    /// "That +4? It's NOT a wild card anymore..."
    fn render_scene_4_plot_twist(&self, time: f32, _frame_num: u32) -> Result<RgbaImage> {
        let progress = self.scenes[3].progress(time);
        let local_time = time - self.scenes[3].start;

        // Dramatic spotlight background with movement
        let spotlight_x = 0.5 + (time * 0.4).sin() * 0.15;
        let spotlight_y = 0.4 + (time * 0.3).cos() * 0.05;
        let mut frame = Backgrounds::spotlight(VIDEO_WIDTH, VIDEO_HEIGHT, spotlight_x, spotlight_y, 0.9);

        // Add subtle sparkles
        let sparkles = Particles::sparkles(VIDEO_WIDTH, VIDEO_HEIGHT, 10, time, 123);
        self.composer.composite(&mut frame, &sparkles, 0, 0);

        // Character gets serious
        let expression = if local_time < 4.0 {
            Expression::Serious
        } else if local_time < 8.0 {
            Expression::Whispering
        } else {
            Expression::MindBlown
        };

        let char_img = self.character.render(expression, 1.3);
        let char_x = (VIDEO_WIDTH as i32 - char_img.width() as i32) / 2;
        let char_y = VIDEO_HEIGHT as i32 - char_img.height() as i32 + 80;
        self.composer.composite(&mut frame, &char_img, char_x, char_y);

        // "PLOT TWIST" text
        if progress < 0.3 {
            let text_progress = progress / 0.3;
            let scale = PopIn::get_scale(text_progress, 1.5);
            if scale > 0.1 {
                let style = TextStyle::red_bold();
                let text = self.text_renderer.render("PLOT TWIST", 100.0 * scale, &style);
                let shake = AnimatedText::shake_offset(time * 15.0, 4.0 * (1.0 - text_progress));
                let x = (VIDEO_WIDTH as i32 - text.width() as i32) / 2 + shake.0;
                let y = (VIDEO_HEIGHT as f32 * 0.15) as i32 + shake.1;
                self.composer.composite(&mut frame, &text, x, y);
            }
        }

        // Show +4 with color (not wild)
        if progress >= 0.25 && progress < 0.55 {
            let card_progress = (progress - 0.25) / 0.3;

            let card = CardFactory::plus_four(CardColor::Red);
            let card_img = card.render(180, 270);

            let x = (VIDEO_WIDTH as f32 * 0.2) as i32;
            let y = Slide::from_bottom(
                VIDEO_HEIGHT as i32,
                (VIDEO_HEIGHT as f32 * 0.35) as i32,
                card_progress.min(1.0),
                Easing::ease_out
            );
            self.composer.composite(&mut frame, &card_img, x, y);

            // "HAS A COLOR" text
            if card_progress > 0.5 {
                let style = TextStyle::yellow_impact();
                let text = self.text_renderer.render("HAS A COLOR!", 60.0, &style);
                let text_x = x + card_img.width() as i32 + 30;
                let text_y = y + 100;
                self.composer.composite(&mut frame, &text, text_x, text_y);
            }
        }

        // Show actual wild cards
        if progress >= 0.55 {
            let wild_progress = (progress - 0.55) / 0.45;

            let style = TextStyle::white_with_black_outline();
            let header = self.text_renderer.render("THE REAL WILDS:", 50.0, &style);
            let header_x = (VIDEO_WIDTH as i32 - header.width() as i32) / 2;
            self.composer.composite(&mut frame, &header, header_x, (VIDEO_HEIGHT as f32 * 0.12) as i32);

            // Wild cards display
            let wilds = [
                (CardFactory::plus_six(), "Draw 6"),
                (CardFactory::plus_ten(), "Draw 10"),
                (CardFactory::reverse_draw_four(), "Rev +4"),
                (CardFactory::color_roulette(), "Roulette"),
            ];

            for (i, (card, label)) in wilds.iter().enumerate() {
                let card_appear = (i as f32 * 0.15).min(wild_progress);
                if card_appear > 0.0 {
                    let scale = PopIn::get_scale(card_appear / 0.15, 0.5);
                    if scale > 0.1 {
                        let card_img = card.render(100, 150);
                        let scaled = FrameComposer::scale_image(&card_img, scale);

                        let spacing = VIDEO_WIDTH / 5;
                        let x = spacing as i32 * (i as i32 + 1) - scaled.width() as i32 / 2;
                        let y = (VIDEO_HEIGHT as f32 * 0.3) as i32;
                        self.composer.composite(&mut frame, &scaled, x, y);

                        // Label below card
                        let label_style = TextStyle::blue_clean();
                        let label_img = self.text_renderer.render(label, 28.0, &label_style);
                        let label_x = x + (scaled.width() as i32 - label_img.width() as i32) / 2;
                        let label_y = y + scaled.height() as i32 + 10;
                        self.composer.composite(&mut frame, &label_img, label_x, label_y);
                    }
                }
            }
        }

        Ok(frame)
    }

    /// Scene 5: Chaos Cards (45-60 sec)
    /// "Play a 7, you SWAP... Play a 0, EVERYONE passes..."
    fn render_scene_5_chaos(&self, time: f32, _frame_num: u32) -> Result<RgbaImage> {
        let progress = self.scenes[4].progress(time);
        let local_time = time - self.scenes[4].start;

        // Chaotic background with enhanced effects
        let mut frame = Backgrounds::chaos(VIDEO_WIDTH, VIDEO_HEIGHT, time, 42);

        // Add sparkles for chaos energy
        let sparkles = Particles::sparkles(VIDEO_WIDTH, VIDEO_HEIGHT, 25 + (progress * 30.0) as usize, time, 99);
        self.composer.composite(&mut frame, &sparkles, 0, 0);

        // Energy waves during chaos
        if (local_time * 2.0) as i32 % 3 == 0 {
            let wave = Particles::energy_wave(VIDEO_WIDTH, VIDEO_HEIGHT, local_time, Rgba([255, 100, 100, 50]));
            self.composer.composite(&mut frame, &wave, 0, 0);
        }

        // Screen shake for chaos effect - intensifies
        let shake_intensity = 3.0 + progress * 8.0;
        let shake = ScreenShake::new(shake_intensity, 10.0);
        let (shake_x, shake_y) = shake.get_offset(time);

        // Character increasingly unhinged
        let expression = if local_time < 3.0 {
            Expression::Mischievous
        } else if local_time < 8.0 {
            Expression::Shocked
        } else {
            Expression::MindBlown
        };

        let char_img = self.character.render(expression, 0.9);
        let char_x = 50 + shake_x;
        let char_y = VIDEO_HEIGHT as i32 - char_img.height() as i32 + shake_y;
        self.composer.composite(&mut frame, &char_img, char_x, char_y);

        // Rapid-fire card rules
        let rules = [
            (0.0, 0.15, "7", "SWAP HANDS!", CardColor::Blue),
            (0.15, 0.30, "0", "PASS ALL HANDS!", CardColor::Green),
            (0.30, 0.50, "SKIP", "SKIP EVERYONE!", CardColor::Red),
            (0.50, 0.70, "ALL", "DISCARD ALL!", CardColor::Yellow),
            (0.70, 1.0, "?", "COLOR ROULETTE!", CardColor::Wild),
        ];

        for (start, end, card_text, description, color) in rules.iter() {
            if progress >= *start && progress < *end {
                let rule_progress = (progress - start) / (end - start);

                // Card on right side
                let card = Card::new(*color, crate::cards::CardType::Number(
                    if *card_text == "7" { 7 } else if *card_text == "0" { 0 } else { 0 }
                ));
                let card_img = if *card_text == "SKIP" {
                    CardFactory::skip_everyone().render(150, 220)
                } else if *card_text == "ALL" {
                    CardFactory::discard_all(*color).render(150, 220)
                } else if *card_text == "?" {
                    CardFactory::color_roulette().render(150, 220)
                } else {
                    card.render(150, 220)
                };

                let card_x = Slide::from_right(
                    VIDEO_WIDTH as i32,
                    150,
                    rule_progress.min(0.5) * 2.0,
                    Easing::ease_out
                );
                let card_y = (VIDEO_HEIGHT as f32 * 0.3) as i32 + shake_y;
                self.composer.composite(&mut frame, &card_img, card_x + shake_x, card_y);

                // Description text
                if rule_progress > 0.2 {
                    let text_progress = (rule_progress - 0.2) / 0.3;
                    let scale = PopIn::get_scale(text_progress.min(1.0), 1.0);
                    if scale > 0.1 {
                        let style = TextStyle::yellow_impact();
                        let text = self.text_renderer.render(description, 70.0 * scale, &style);
                        let x = (VIDEO_WIDTH as i32 - text.width() as i32) / 2 + shake_x;
                        let y = (VIDEO_HEIGHT as f32 * 0.15) as i32 + shake_y;
                        self.composer.composite(&mut frame, &text, x, y);
                    }
                }
            }
        }

        Ok(frame)
    }

    /// Scene 6: Golden Rule (60-70 sec)
    /// "You draw until you CAN play. No stopping."
    fn render_scene_6_golden_rule(&self, time: f32, _frame_num: u32) -> Result<RgbaImage> {
        let progress = self.scenes[5].progress(time);

        // Dark dramatic background
        let mut frame = Backgrounds::spotlight(VIDEO_WIDTH, VIDEO_HEIGHT, 0.5, 0.3, 0.6);

        // Character dead serious
        let char_img = self.character.render(Expression::Serious, 1.4);
        let char_x = (VIDEO_WIDTH as i32 - char_img.width() as i32) / 2;
        let char_y = VIDEO_HEIGHT as i32 - char_img.height() as i32 + 100;
        self.composer.composite(&mut frame, &char_img, char_x, char_y);

        // "DRAW UNTIL YOU CAN PLAY" text
        let style = TextStyle::red_bold();
        let main_text = self.text_renderer.render("DRAW UNTIL", 80.0, &style);
        let main_x = (VIDEO_WIDTH as i32 - main_text.width() as i32) / 2;
        self.composer.composite(&mut frame, &main_text, main_x, (VIDEO_HEIGHT as f32 * 0.12) as i32);

        let sub_text = self.text_renderer.render("YOU CAN PLAY", 80.0, &style);
        let sub_x = (VIDEO_WIDTH as i32 - sub_text.width() as i32) / 2;
        self.composer.composite(&mut frame, &sub_text, sub_x, (VIDEO_HEIGHT as f32 * 0.20) as i32);

        // Animated card counter
        let count = (progress * 25.0) as u32;
        let count_style = TextStyle::yellow_impact();
        let count_text = self.text_renderer.render(&format!("{}", count.min(25)), 150.0, &count_style);
        let count_x = (VIDEO_WIDTH as i32 - count_text.width() as i32) / 2;
        let count_y = (VIDEO_HEIGHT as f32 * 0.35) as i32;
        self.composer.composite(&mut frame, &count_text, count_x, count_y);

        // Cards piling up
        if progress > 0.2 {
            let pile_progress = (progress - 0.2) / 0.8;
            let num_cards = (pile_progress * 20.0) as usize;

            for i in 0..num_cards.min(20) {
                let colors = [CardColor::Red, CardColor::Blue, CardColor::Green, CardColor::Yellow];
                let color = colors[i % 4];
                let card = CardFactory::number(color, (i % 10) as u8);
                let card_img = card.render(60, 90);

                let offset_x = ((i as f32 * 17.0).sin() * 100.0) as i32;
                let offset_y = i as i32 * 3;
                let x = (VIDEO_WIDTH as i32 / 2) - 30 + offset_x;
                let y = (VIDEO_HEIGHT as f32 * 0.55) as i32 + offset_y;
                self.composer.composite(&mut frame, &card_img, x, y);
            }
        }

        // "Just pain." text at end
        if progress > 0.8 {
            let pain_progress = (progress - 0.8) / 0.2;
            let style = TextStyle::white_with_black_outline();
            let pain_text = self.text_renderer.render("Just pain.", 60.0, &style);
            let alpha = pain_progress;
            let x = (VIDEO_WIDTH as i32 - pain_text.width() as i32) / 2;
            let y = (VIDEO_HEIGHT as f32 * 0.85) as i32;
            self.composer.composite_with_alpha(&mut frame, &pain_text, x, y, alpha);
        }

        Ok(frame)
    }

    /// Scene 7: Outro (70-75 sec)
    /// "This game has ended friendships..."
    fn render_scene_7_outro(&self, time: f32, _frame_num: u32) -> Result<RgbaImage> {
        let progress = self.scenes[6].progress(time);

        // Dramatic dark background
        let mut frame = Backgrounds::dramatic_dark(VIDEO_WIDTH, VIDEO_HEIGHT, time);

        // Add menacing sparkles
        let sparkles = Particles::sparkles(VIDEO_WIDTH, VIDEO_HEIGHT, 12, time, 666);
        self.composer.composite(&mut frame, &sparkles, 0, 0);

        // Character expression changes
        let expression = if progress < 0.5 {
            Expression::Serious
        } else {
            Expression::Mischievous
        };

        // Character zooms out then evil smile
        let scale = if progress < 0.5 {
            1.5 - progress * 0.8
        } else {
            0.7 + (progress - 0.5) * 0.4
        };

        let char_img = self.character.render(expression, scale);

        let char_x = if progress < 0.5 {
            (VIDEO_WIDTH as i32 - char_img.width() as i32) / 2
        } else {
            // Move to corner
            let target_x = VIDEO_WIDTH as i32 - char_img.width() as i32 - 50;
            let center_x = (VIDEO_WIDTH as i32 - char_img.width() as i32) / 2;
            let t = (progress - 0.5) * 2.0;
            (center_x as f32 + (target_x - center_x) as f32 * Easing::ease_in_out(t)) as i32
        };

        let char_y = VIDEO_HEIGHT as i32 - char_img.height() as i32;
        self.composer.composite(&mut frame, &char_img, char_x, char_y);

        // Impactful closing statements
        if progress < 0.5 {
            let statements = [
                (0.0, "Ended friendships."),
                (0.15, "Ruined holidays."),
                (0.30, "Created villains."),
            ];

            for (threshold, text) in statements.iter() {
                if progress > *threshold && progress < threshold + 0.15 {
                    let text_progress = (progress - threshold) / 0.15;
                    let alpha = if text_progress < 0.5 {
                        text_progress * 2.0
                    } else {
                        (1.0 - text_progress) * 2.0
                    };

                    let style = TextStyle::red_bold();
                    let text_img = self.text_renderer.render(text, 70.0, &style);
                    let x = (VIDEO_WIDTH as i32 - text_img.width() as i32) / 2;
                    let y = (VIDEO_HEIGHT as f32 * 0.3) as i32;
                    self.composer.composite_with_alpha(&mut frame, &text_img, x, y, alpha);
                }
            }
        }

        // Final question with evil emoji and glow
        if progress > 0.6 {
            let final_progress = (progress - 0.6) / 0.4;
            let scale = PopIn::get_scale(final_progress, 0.8);

            if scale > 0.1 {
                let style = TextStyle::yellow_impact();
                let question = self.text_renderer.render("who wants to play?", 80.0 * scale, &style);

                // Add menacing glow
                let glowing_question = Glow::apply(&question, Rgba([255, 200, 50, 150]), 12, 0.7);

                let x = (VIDEO_WIDTH as i32 - glowing_question.width() as i32) / 2;
                let y = (VIDEO_HEIGHT as f32 * 0.23) as i32;
                self.composer.composite(&mut frame, &glowing_question, x, y);

                // Devil emoji representation with glow
                if final_progress > 0.5 {
                    let emoji_style = TextStyle {
                        color: Rgba([220, 50, 50, 255]),
                        outline_color: Some(Rgba([0, 0, 0, 255])),
                        outline_width: 4,
                        shadow: true,
                        shadow_offset: (5, 5),
                        shadow_color: Rgba([0, 0, 0, 220]),
                    };
                    let emoji = self.text_renderer.render(">:)", 140.0, &emoji_style);
                    let glowing_emoji = Glow::apply(&emoji, Rgba([255, 80, 50, 180]), 18, 1.0);

                    let emoji_x = (VIDEO_WIDTH as i32 - glowing_emoji.width() as i32) / 2;
                    let emoji_y = (VIDEO_HEIGHT as f32 * 0.38) as i32;
                    self.composer.composite(&mut frame, &glowing_emoji, emoji_x, emoji_y);
                }
            }
        }

        // Fade to black at end
        if progress > 0.9 {
            let fade_progress = (progress - 0.9) / 0.1;
            let fade = Fade::to_black(VIDEO_WIDTH, VIDEO_HEIGHT, fade_progress);
            self.composer.composite(&mut frame, &fade, 0, 0);
        }

        Ok(frame)
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}
