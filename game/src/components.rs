// use std::fmt;

//pub mod components {

    pub const BOARD_SIZE: usize = 3;

    #[derive(Copy, Clone)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum CellState {
        Empty,
        X,
        O
    }

    impl std::fmt::Display for CellState {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
                CellState::Empty => write!(f, " "),
                CellState::X => write!(f, "X"),
                CellState::O => write!(f, "O"),
            }
        }
    }

    pub struct Player {
        pub symbol: CellState,
    }

    pub struct Game {
        pub board: [[Tile; BOARD_SIZE]; BOARD_SIZE],
        pub active_player: usize,
        pub players: [Player; 2],
    }

    #[derive(Copy, Clone)]
    pub struct Tile {
        pub state: CellState,
    }

    impl Game {
        pub fn new() -> Game  {
            let game = Game{
                board: [[Tile{
                    state: CellState::Empty,
                }; BOARD_SIZE]; BOARD_SIZE],
                players: [Player {
                    symbol: CellState::X,
                }, Player {
                    symbol: CellState::O,
                }],

                //active_player: players[0],
                active_player: 0,
            };

            game
        }        

        pub fn place_symbol_if_target_cell_available(&mut self, x: usize, y: usize) -> bool {
            let mut symbol_was_placed = false;

            if self.board[y][x].state == CellState::Empty {
                self.board[y][x].state = self.players[self.active_player].symbol;
                symbol_was_placed = true;
            }

            symbol_was_placed
        }

        pub fn next_player(&mut self) {
            self.active_player = if self.active_player == 0 { 1 } else { 0 };
        }

        pub fn is_over(&self) -> bool {
            if self.horizontal_win() || self.vertical_win() || self.diagonal_win() {
                return true;
            }
            
            false
        }

        pub fn horizontal_win(&self) -> bool {
            let mut to_match: CellState = CellState::Empty;
            for y in 0..BOARD_SIZE {
                for x in 0..BOARD_SIZE {
                    if x == 0 {
                        // set the symbol we need to match
                        to_match = self.board[y][x].state;
                        //println!("Horizontal cells need to match \'{}\'", to_match);
                        // ..however, if it's empty, just ignore the rest of the row
                        if to_match == CellState::Empty {
                            //println!("The first horizontal cell is empty.");
                            break;
                        }
                    } else if to_match != self.board[y][x].state {
                        // for all the row after the first cell, 
                        // they must match or ignore the rest of the row
                        //println!("{} at {},{} does not match {}", self.board[y][x].state, y,x, to_match);
                        break;
                    } else if x==BOARD_SIZE-1 {
                        println!("There is a horizontal win! Row {}", y+1);
                        return true;
                    }
                }
            }

            false
        }

        pub fn vertical_win(&self) -> bool {
            let mut to_match: CellState = CellState::Empty;
            for x in 0..BOARD_SIZE {
                for y in 0..BOARD_SIZE {
                    if y == 0 {
                        // set the symbol we need to match
                        to_match = self.board[y][x].state;
                        //println!("Vertical cells need to match \'{}\'", to_match);
                        // ..however, if it's empty, just ignore the rest of the col
                        if to_match == CellState::Empty {
                            //println!("The first cell is empty.");
                            break;
                        }
                    } else if to_match != self.board[y][x].state {
                            // for all the row after the first cell, 
                            // they must match or ignore the rest of the col
                            //println!("{} at {},{} does not match {}", self.board[y][x].state, y,x, to_match);
                            break;
                    } else if y==BOARD_SIZE-1 {
                        println!("There is a vertical win! Col {}", x+1);
                        return true;
                    }
                }
            }

            false
        }

        pub fn diagonal_win(&self) -> bool {
            return self.diagonal_win_top_left_to_bottom_right() || self.diagonal_win_top_right_to_bottom_left();
        }

        pub fn diagonal_win_top_left_to_bottom_right(&self) -> bool {
            let to_match: CellState = self.board[0][0].state;
            if to_match == CellState::Empty {
                return false;
            }

            for pos in 0..BOARD_SIZE {
                if self.board[pos][pos].state != to_match {
                    return false;
                }
            }

            println!("There was a diagonal win! (top left to bottom right)");
            true
        }

        // formula: top right (3,1)  x-1,y+1 (2,2)  x-1,y+1 (1,3)
        pub fn diagonal_win_top_right_to_bottom_left(&self) -> bool {
            let to_match: CellState = self.board[0][BOARD_SIZE-1].state;
            if to_match == CellState::Empty {
                return false;
            }
            
            let mut y:usize = 0;
            for x in (0..BOARD_SIZE).rev() {
                if self.board[y][x].state != to_match {
                    return false;
                }
                y += 1;
            }

            println!("There was a diagonal win! (top right to bottom left)");
            true
        }

    }
//}