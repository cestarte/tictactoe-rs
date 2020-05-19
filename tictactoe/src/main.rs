extern crate game;
use game::components::Game;
use game::components::BOARD_SIZE;
mod input;
mod input_test;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn };

fn main() {
    let _logger_init_result = setup_logger().expect("[ERROR] Failed to initialize logging.");
    info!("App starting up!");
    ctrlc::set_handler(move || {
        debug!("Received Ctrl+C. Shutting down..");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
    debug!("App init complete.");

    let mut game = Game::new();

    let mut done = false;
    while !done {
        take_turn(&mut game);

        if game.is_over() {
            done = true;
            print_board(&game);
            info!("Game over!");
        } else {
            trace!("Player {}'s turn is over. Next player!", game.active_player+1);
            game.next_player();
        }
    }
}

fn print_board(game: &Game) {
    for y in 0..BOARD_SIZE {
        if y==0 {
            print!("   ");
            for i in 0..BOARD_SIZE {
                print!("{}  ", i+1);
            }
            print!("\n");
        }

        for x in 0..BOARD_SIZE {
            if x == 0 {
                print!("{}  ", y+1);
            }
            print!("{}  ", game.board[y][x].state);
        }
        print!("\n");
    }
}

fn take_turn(game: &mut Game) {
    println!();
    println!();
    print_board(&game);
    println!();
    input::get_input(game);
}


fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::max())
        //.chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}