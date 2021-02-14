use crossterm::terminal::size;
mod game;


fn execute(mut game: game::Game) {
    let terminal_size = match size() {
        Ok(_size) => _size,
        Err(_size) => (0, 0),
    };
    if terminal_size > (80, 29) {
        game.origin_coordinates = game::set_starting_pos();
        game.input_board_num();
        game.board_gen();
        game.set_timeout_duration_len();
        while !game.finished {
            game.game_loop();
            game::reset_terminal();
            game.check_game_winner();
            game.end_msg_gen();
            println!(
                "-=-=-=-=- All Boards Have Been Completed! -=-=-=-=-\n{}",
                game.end_msg,
            );
            game::cursor_move_to(game.origin_coordinates.0, game.origin_coordinates.1 + 6);
            game.score_board();
            game::cursor_move_to(game.origin_coordinates.0, game.origin_coordinates.1 + 3);
            match game.winner {
                game::player::WonPlayer::None => game.game_continue(),
                game::player::WonPlayer::Circle => (),
                game::player::WonPlayer::Crosses => (),
            }
            game::cursor_move_to(
                game.origin_coordinates.0,
                game.origin_coordinates.1 + game.vec_board.len() as u16 + 8,
            );
        }
    } else {
        println!("Terminal size is too small, resize it and start again");
    }
}

fn main() {
    let game = game::Game::new();
    execute(game);
}
