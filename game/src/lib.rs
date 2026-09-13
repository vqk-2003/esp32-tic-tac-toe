#![no_std]

use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::Text,
};

const BOARD_COL: usize = 3;
const BOARD_ROW: usize = 3;
const BOARD_SIZE: usize = BOARD_COL * BOARD_ROW;

pub struct Game {
    state: GameState,
    board: [CellState; BOARD_SIZE],
    num_of_empty_cells: usize,
    is_player_one: bool,
    cur_pos: Position,
}

enum GameState {
    Menu,
    GamePlay,
    Result,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellState {
    X,
    O,
    Empty,
}

#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug)]
pub enum PlayerResult {
    Won,
    Lost,
    Drew,
    OnGoing,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::Menu,
            board: [CellState::Empty; BOARD_SIZE],
            num_of_empty_cells: BOARD_SIZE,
            is_player_one: true,
            cur_pos: Position { x: 0, y: 0 },
        }
    }

    pub fn move_cursor(&mut self, dir: Direction) {
        use Direction::*;
        match dir {
            Up => {
                if self.cur_pos.x == 0 {
                    self.cur_pos.x = BOARD_COL - 1;
                } else {
                    self.cur_pos.x -= 1;
                }
            }
            Down => {
                if self.cur_pos.x == (BOARD_COL - 1) {
                    self.cur_pos.x = 0;
                } else {
                    self.cur_pos.x += 1;
                }
            }
            Left => {
                if self.cur_pos.y == 0 {
                    self.cur_pos.y = BOARD_ROW - 1;
                } else {
                    self.cur_pos.y -= 1;
                }
            }
            Right => {
                if self.cur_pos.y == (BOARD_ROW - 1) {
                    self.cur_pos.y = 0;
                } else {
                    self.cur_pos.y += 1;
                }
            }
        }
    }

    fn get_cell_state(&mut self, row: usize, col: usize) -> CellState {
        self.board[row * BOARD_COL + col]
    }

    pub fn check_player_one_result(&mut self) -> PlayerResult {
        use CellState::*;
        use PlayerResult::*;
        // Check columns
        'col: for col in 0..BOARD_COL {
            let cell = self.get_cell_state(0, col);
            if cell == Empty {
                continue;
            }
            for row in 1..BOARD_ROW {
                if cell != self.get_cell_state(row, col) {
                    continue 'col;
                }
            }
            return if cell == X { Won } else { Lost };
        }

        // Check rows
        'row: for row in 0..BOARD_ROW {
            let cell = self.get_cell_state(row, 0);
            if cell == Empty {
                continue;
            }
            for col in 1..BOARD_COL {
                if cell != self.get_cell_state(row, col) {
                    continue 'row;
                }
            }
            return if cell == X { Won } else { Lost };
        }

        // Check diagonals
        let cell = self.get_cell_state(0, 0);
        if (cell != Empty)
            && (cell == self.get_cell_state(1, 1))
            && (cell == self.get_cell_state(2, 2))
        {
            return if cell == X { Won } else { Lost };
        }

        let cell = self.get_cell_state(2, 0);
        if (cell != Empty)
            && (cell == self.get_cell_state(1, 1))
            && (cell == self.get_cell_state(2, 0))
        {
            return if cell == X { Won } else { Lost };
        }

        if self.num_of_empty_cells > 0 {
            OnGoing
        } else {
            Drew
        }
    }

    fn modify_cell(&mut self, state: CellState, pos: Position) {
        self.board[pos.x * BOARD_COL + pos.y] = state;
    }

    pub fn select_cell(&mut self) {
        use CellState::*;

        if self.get_cell_state(self.cur_pos.x, self.cur_pos.y) != Empty {
            return;
        }

        if self.is_player_one {
            self.modify_cell(X, self.cur_pos);
        } else {
            self.modify_cell(O, self.cur_pos);
        }

        self.is_player_one = !self.is_player_one;
    }
}

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

#[cfg(test)]
mod test {
    use crate::Game;

    extern crate std;
    use super::*;

    #[test]
    fn col_all_x() {
        use CellState::*;
        let mut game = Game::new();
        game.board = [
            Empty, Empty, X, //
            Empty, Empty, X, //
            Empty, Empty, X, //
        ];
        assert_eq!(game.check_player_one_result(), PlayerResult::Won);
    }

    #[test]
    fn row_all_x() {
        use CellState::*;
        let mut game = Game::new();
        game.board = [
            Empty, Empty, Empty, //
            X, X, X, //
            Empty, Empty, Empty, //
        ];
        assert_eq!(game.check_player_one_result(), PlayerResult::Won);
    }

    #[test]
    fn diagonal_all_x() {
        use CellState::*;
        let mut game = Game::new();
        game.board = [
            X, Empty, Empty, //
            Empty, X, Empty, //
            Empty, Empty, X, //
        ];
        assert_eq!(game.check_player_one_result(), PlayerResult::Won);
    }

    #[test]
    fn col_all_o() {
        use CellState::*;
        let mut game = Game::new();
        game.board = [
            O, Empty, Empty, //
            O, Empty, Empty, //
            O, Empty, Empty, //
        ];
        assert_eq!(game.check_player_one_result(), PlayerResult::Lost);
    }

    #[test]
    fn row_all_o() {
        use CellState::*;
        let mut game = Game::new();
        game.board = [
            O, O, O, //
            Empty, Empty, Empty, //
            Empty, Empty, Empty, //
        ];
        assert_eq!(game.check_player_one_result(), PlayerResult::Lost);
    }

    #[test]
    fn diagonal_all_o() {
        use CellState::*;
        let mut game = Game::new();
        game.board = [
            Empty, Empty, O, //
            Empty, O, Empty, //
            O, Empty, Empty, //
        ];
        assert_eq!(game.check_player_one_result(), PlayerResult::Lost);
    }
}
