use embedded_graphics::{
    Drawable,
    geometry::{Point, Size},
    mono_font::{
        MonoTextStyle,
        ascii::{FONT_9X15, FONT_10X20},
    },
    pixelcolor::{Rgb565, RgbColor},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
    text::{Text, renderer::CharacterStyle},
};

use crate::logic::{BOARD_COL, Game, GameState, MenuOption};

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
                const X_OFFSET: usize = 30;
                const Y_OFFSET: usize = 10;
                const SQUARE_SIZE: usize = 35;

                for cell in self.board.iter().enumerate() {
                    let mut cell_style = PrimitiveStyle::with_stroke(Rgb565::WHITE, 1);

                    if cell.0 == (self.cur_pos.x * BOARD_COL + self.cur_pos.y) {
                        cell_style.fill_color = Some(Rgb565::GREEN);
                    } else {
                        cell_style.fill_color = Some(Rgb565::BLACK);
                    }
                    Rectangle::new(
                        Point {
                            x: (X_OFFSET + SQUARE_SIZE * (cell.0 / BOARD_COL)) as i32,
                            y: (Y_OFFSET + SQUARE_SIZE * (cell.0 % BOARD_COL)) as i32,
                        },
                        Size {
                            width: SQUARE_SIZE as u32,
                            height: SQUARE_SIZE as u32,
                        },
                    )
                    .into_styled(cell_style)
                    .draw(target)?;
                }
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
