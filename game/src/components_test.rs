//use pub components;
//use components::Game;
//use components::CellState;
//pub mod components_test {

#[test]
pub fn horizontal_win_top() {
    let mut game = Game::new();
    game.board[0][0].state = CellState::O;
    game.board[0][1].state = CellState::O;
    game.board[0][2].state = CellState::O;

    assert_eq!(true, game.horizontal_win());
}

#[test]
pub fn horizontal_win_middle() {
    let mut game = Game::new();
    game.board[1][0].state = CellState::O;
    game.board[1][1].state = CellState::O;
    game.board[1][2].state = CellState::O;

    assert_eq!(true, game.horizontal_win());
}

#[test]
pub fn horizontal_win_bottom() {
    let mut game = Game::new();
    game.board[2][0].state = CellState::O;
    game.board[2][1].state = CellState::O;
    game.board[2][2].state = CellState::O;

    assert_eq!(true, game.horizontal_win());
}

#[test]
pub fn vertical_win_left() {
    let mut game = Game::new();
    game.board[0][0].state = CellState::O;
    game.board[1][0].state = CellState::O;
    game.board[2][0].state = CellState::O;

    assert_eq!(true, game.vertical_win());
}

#[test]
pub fn vertical_win_middle() {
    let mut game = Game::new();
    game.board[0][1].state = CellState::O;
    game.board[1][1].state = CellState::O;
    game.board[2][1].state = CellState::O;

    assert_eq!(true, game.vertical_win());
}

#[test]
pub fn vertical_win_right() {
    let mut game = Game::new();
    game.board[0][2].state = CellState::O;
    game.board[1][2].state = CellState::O;
    game.board[2][2].state = CellState::O;

    assert_eq!(true, game.vertical_win());
}

#[test]
pub fn diagonal_win_top_left_to_bottom_right() {
    let mut game = Game::new();
    game.board[0][0].state = CellState::O;
    game.board[1][1].state = CellState::O;
    game.board[2][2].state = CellState::O;

    assert_eq!(true, game.diagonal_win_top_left_to_bottom_right());
}

#[test]
pub fn diagonal_win_top_right_to_bottom_left() {
    let mut game = Game::new();
    game.board[0][2].state = CellState::O;
    game.board[1][1].state = CellState::O;
    game.board[2][0].state = CellState::O;

    assert_eq!(true, game.diagonal_win_top_right_to_bottom_left());
}

#[test]
pub fn active_player_defaults_to_0() {
    let game = Game::new();

    assert_eq!(0, game.active_player);
}

#[test]
pub fn next_player_switches_from_1_to_0() {
    let mut game = Game::new();
    game.active_player = 0;
    game.next_player();
    assert_eq!(1, game.active_player);
}

#[test]
pub fn next_player_switches_from_0_to_1() {
    let mut game = Game::new();
    game.active_player = 1;
    game.next_player();
    assert_eq!(0, game.active_player);
}

#[test]
pub fn place_symbol_if_target_cell_available_should_do_it_if_so() {
    let mut game = Game::new();
    game.active_player = 0;
    game.players[0].symbol = CellState::X;
    let was_available = game.place_symbol_if_target_cell_available(0, 0);
    assert_eq!(CellState::X, game.board[0][0].state);
    assert_eq!(true, was_available);
}

#[test]
pub fn place_symbol_if_target_cell_available_should_return_false_if_not_available() {
    let mut game = Game::new();
    game.active_player = 0;
    game.players[0].symbol = CellState::X;
    game.board[0][0].state = CellState::O;
    let was_available = game.place_symbol_if_target_cell_available(0, 0);
    assert_eq!(CellState::O, game.board[0][0].state);
    assert_eq!(false, was_available);
}

//}