const BOARD_COL: usize = 3;
const BOARD_ROW: usize = 3;
const BOARD_SIZE: usize = BOARD_COL * BOARD_ROW;

pub struct Game {
    pub(crate) state: GameState,
    board: [CellState; BOARD_SIZE],
    num_of_empty_cells: usize,
    is_player_one: bool,
    cur_pos: Position,
    pub(crate) menu_option: MenuOption,
}

#[derive(Clone, Copy)]
pub enum GameState {
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

pub enum Cmd {
    Select,
    Move(Direction),
}

pub(crate) enum MenuOption {
    SinglePlayer,
    TwoPlayer,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::Menu,
            board: [CellState::Empty; BOARD_SIZE],
            num_of_empty_cells: BOARD_SIZE,
            is_player_one: true,
            cur_pos: Position { x: 0, y: 0 },
            menu_option: MenuOption::SinglePlayer,
        }
    }

    fn move_cursor(&mut self, dir: Direction) {
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

    fn menu_handler(&mut self, cmd: Cmd) {
        use Cmd::*;
        use Direction::*;
        use GameState::*;
        use MenuOption::*;

        match cmd {
            Select => {
                self.state = GamePlay;
            }
            Move(Down) => match self.menu_option {
                SinglePlayer => self.menu_option = TwoPlayer,
                TwoPlayer => self.menu_option = SinglePlayer,
            },
            Move(Up) => match self.menu_option {
                SinglePlayer => self.menu_option = TwoPlayer,
                TwoPlayer => self.menu_option = SinglePlayer,
            },
            Move(_) => {}
        }
    }

    fn game_play_handler(&mut self, cmd: Cmd) {
        use Cmd::*;

        match cmd {
            Select => {
                self.select_cell();
            }
            Move(dir) => {
                self.move_cursor(dir);
            }
        }
    }

    fn result_handler(&mut self, cmd: Cmd) {
        use Cmd::*;
        use GameState::*;
        use MenuOption::*;

        match cmd {
            Select => {
                self.state = Menu;
                self.menu_option = SinglePlayer;
            }
            _ => {}
        }
    }

    pub fn handle_input(&mut self, cmd: Cmd) {
        use GameState::*;

        match self.state {
            Menu => self.menu_handler(cmd),
            GamePlay => self.game_play_handler(cmd),
            Result => self.result_handler(cmd),
        }
    }

    pub fn get_state(&mut self) -> GameState {
        self.state
    }
}

#[cfg(test)]
mod test {
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

    #[test]
    fn draw() {
        use CellState::*;
        let mut game = Game::new();
        game.board = [
            Empty, Empty, Empty, //
            Empty, Empty, Empty, //
            Empty, Empty, Empty, //
        ];
        game.num_of_empty_cells = 0;
        assert_eq!(game.check_player_one_result(), PlayerResult::Drew);
    }

    #[test]
    fn on_going() {
        use CellState::*;
        let mut game = Game::new();
        game.board = [
            Empty, Empty, Empty, //
            Empty, Empty, Empty, //
            Empty, Empty, Empty, //
        ];
        assert_eq!(game.check_player_one_result(), PlayerResult::OnGoing);
    }
}
