use embedded_graphics::{
    Drawable,
    geometry::Point,
    mono_font::{
        MonoTextStyle,
        ascii::{FONT_5X8, FONT_9X15, FONT_10X20},
    },
    pixelcolor::{Rgb565, RgbColor, WebColors},
    primitives::{Line, Primitive, PrimitiveStyle, Rectangle},
    text::{Text, renderer::CharacterStyle},
};

use crate::logic::{Game, GameState, MenuOption};

impl Drawable for Game {
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        use GameState::*;
        match self.state {
            Menu => {
                Text::new(
                    "Tic Tac Toe",
                    Point { x: 10, y: 20 },
                    MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN),
                )
                .draw(target)?;

                let mut text_single_player_style = MonoTextStyle::new(&FONT_9X15, Rgb565::BLUE);
                let mut text_two_player_style = MonoTextStyle::new(&FONT_9X15, Rgb565::BLUE);
                match self.menu_option {
                    MenuOption::SinglePlayer => {
                        text_single_player_style.set_background_color(Some(Rgb565::WHITE));
                        text_two_player_style.set_background_color(Some(Rgb565::BLACK));
                    }
                    MenuOption::TwoPlayer => {
                        text_single_player_style.set_background_color(Some(Rgb565::BLACK));
                        text_two_player_style.set_background_color(Some(Rgb565::WHITE));
                    }
                }

                Text::new(
                    "Single Player",
                    Point { x: 10, y: 40 },
                    text_single_player_style,
                )
                .draw(target)?;

                Text::new("Two Player", Point { x: 10, y: 60 }, text_two_player_style)
                    .draw(target)?;
            }
            GamePlay => {
                const X_OFFSET: i32 = (160 - 120) / 2;
                const Y_OFFSET: i32 = (128 - 120) / 2;
                Line::new(
                    Point {
                        x: 40 + X_OFFSET,
                        y: 0 + Y_OFFSET,
                    },
                    Point {
                        x: 40 + X_OFFSET,
                        y: 120 + Y_OFFSET,
                    },
                )
                .into_styled(PrimitiveStyle::with_stroke(Self::Color::WHITE, 2))
                .draw(target)?;

                Line::new(
                    Point {
                        x: 80 + X_OFFSET,
                        y: 0 + Y_OFFSET,
                    },
                    Point {
                        x: 80 + X_OFFSET,
                        y: 120 + Y_OFFSET,
                    },
                )
                .into_styled(PrimitiveStyle::with_stroke(Self::Color::WHITE, 2))
                .draw(target)?;

                Line::new(
                    Point {
                        x: 0 + X_OFFSET,
                        y: 40 + Y_OFFSET,
                    },
                    Point {
                        x: 120 + X_OFFSET,
                        y: 40 + Y_OFFSET,
                    },
                )
                .into_styled(PrimitiveStyle::with_stroke(Self::Color::WHITE, 2))
                .draw(target)?;

                Line::new(
                    Point {
                        x: 0 + X_OFFSET,
                        y: 80 + Y_OFFSET,
                    },
                    Point {
                        x: 120 + X_OFFSET,
                        y: 80 + Y_OFFSET,
                    },
                )
                .into_styled(PrimitiveStyle::with_stroke(Self::Color::WHITE, 2))
                .draw(target)?;
            }
            Result => {
                Text::new(
                    "Game Over",
                    Point { x: 40, y: 50 },
                    MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN),
                )
                .draw(target)?;
            }
        }
        Ok(())
    }
}
