use std::fmt;

const BOARD_WIDTH: usize = 3;
const BOARD_HEIGHT: usize = 3;

#[derive(Copy, Clone)]
enum CellState {
    Empty,
    X,
    O
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CellState::Empty => write!(f, " "),
            CellState::X => write!(f, "X"),
            CellState::O => write!(f, "O"),
        }
    }
}


struct Game {
    board: [[Tile; BOARD_WIDTH]; BOARD_HEIGHT]
}

#[derive(Copy, Clone)]
struct Tile {
    row: u8,
    col: u8,
    state: CellState,
}

impl Game {
    fn new() -> Game  {
        let mut game = Game{
            board: [[Tile{
                row: 0,
                col: 0,
                state: CellState::Empty
            }; BOARD_WIDTH]; BOARD_HEIGHT]
        };

        game
    }

    fn print(&self) {
        for y in 0..BOARD_HEIGHT {
            if y==0 {
                print!("   ");
                for i in 0..BOARD_WIDTH {
                    print!("{}  ", i+1);
                }
                print!("\n");
            }

            for x in 0..BOARD_WIDTH {
                if (x == 0) {
                    print!("{}  ", y+1);
                }
                print!("{}  ", self.board[y][x].state);
            }
            print!("\n");
        }
    }

    // fn isOver(&self) -> bool {
    //     // TODO check for winner
    //     let filtered = self.board.filter(|&filteredTiles| filteredTiles.CellState == CellState::O);
    //         false
        
    //     true
    // }
}


fn main() {
    let game = Game::new();
    game.print();

    // let mut done = false;
    // while !done {
    //     game.print();

    //     if (game.isOver())
    //         done = true;
    // }
    
}
