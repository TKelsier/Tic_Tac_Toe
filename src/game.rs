use crossterm::cursor::{ position, MoveTo, RestorePosition, SavePosition}; 
use crossterm::event::{ poll, read, Event, KeyCode, KeyCode::Char};
use crossterm::execute;
use crossterm::terminal::{ disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::cmp::Ordering;
use std::io::{ stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

mod board;
mod player;

pub struct Game {
    pub vec_board: Vec<board::Board>,
    vec_board_iter: Vec<u16>,
    players: [player::Player; 2],
    pub winner: player::WonPlayer,
    timeout_duration_len: u64,
    pub finished: bool,
    pub origin_coordinates: (u16, u16),
    pub end_msg: String,
    board_num: u16,
}

impl Game {
    pub fn new() -> Self {
        Self {
            vec_board: Vec::new(),
            vec_board_iter: Vec::new(),
            players: [player::Player::new("Player 1"), player::Player::new("Player 2")],
            winner: player::WonPlayer::None,
            timeout_duration_len: 999999,
            finished: false,
            origin_coordinates: (0, 0),
            end_msg: "".to_string(),
            board_num: 1,
        }
    }
    pub fn board_gen(&mut self) {
        for num in 0..self.board_num {
            self.vec_board.push(board::Board::new(num + 1));
            self.vec_board_iter.push(num);
        }
        println!("Number of boards set to: {}", self.board_num);
    }
    fn grab_user_input(&mut self) -> crossterm::Result<KeyCode> {
        enable_raw_mode().expect("Could not enable terminal raw mode");
        let key_event = loop {
            if poll(Duration::from_secs(self.timeout_duration_len))? {
                match read()? {
                    Event::Key(event) => break event.code,
                    _ => continue,
                }
            } else {
                break Char('0');
            }
        };
        disable_raw_mode().expect("Could not disable terminal raw mode");
        Ok(key_event)
    }
    pub fn game_loop(&mut self) {
        while !self.finished {
            for board_iter in 0..self.vec_board_iter.len() {
                if !self.vec_board[board_iter].completed {
                    self.player_turns(board_iter);
                    self.check_game_finished();
                } else {
                    break;
                }
            }
        }
    }
    fn execute_player_turn(&mut self, board_iter: usize, player_iter: usize) {
        if self.vec_board[board_iter].first_move {
            self.vec_board[board_iter].update_pos(0, player_iter as u8);
        } else {
            print!(
                "{} Please make your move (1-9): ",
                self.players[player_iter].name
            );
            stdout().flush().unwrap();
            let keycode_input = loop {
                match self.grab_user_input() {
                    Ok(_input) => break _input,
                    Err(_err) => continue,
                }
            };
            match keycode_input {
                Char('0') => self.vec_board[board_iter].update_pos(0, player_iter as u8),
                Char('1') if { self.vec_board[board_iter].positions[0] == 0 } => {
                    self.vec_board[board_iter].update_pos(1, player_iter as u8)
                }
                Char('2') if { self.vec_board[board_iter].positions[1] == 0 } => {
                    self.vec_board[board_iter].update_pos(2, player_iter as u8)
                }
                Char('3') if { self.vec_board[board_iter].positions[2] == 0 } => {
                    self.vec_board[board_iter].update_pos(3, player_iter as u8)
                }
                Char('4') if { self.vec_board[board_iter].positions[3] == 0 } => {
                    self.vec_board[board_iter].update_pos(4, player_iter as u8)
                }
                Char('5') if { self.vec_board[board_iter].positions[4] == 0 } => {
                    self.vec_board[board_iter].update_pos(5, player_iter as u8)
                }
                Char('6') if { self.vec_board[board_iter].positions[5] == 0 } => {
                    self.vec_board[board_iter].update_pos(6, player_iter as u8)
                }
                Char('7') if { self.vec_board[board_iter].positions[6] == 0 } => {
                    self.vec_board[board_iter].update_pos(7, player_iter as u8)
                }
                Char('8') if { self.vec_board[board_iter].positions[7] == 0 } => {
                    self.vec_board[board_iter].update_pos(8, player_iter as u8)
                }
                Char('9') if { self.vec_board[board_iter].positions[8] == 0 } => {
                    self.vec_board[board_iter].update_pos(9, player_iter as u8)
                }
                Char('f') => self.forfeit(board_iter, player_iter),
                Char('q') => self.quit(),
                _ => self.vec_board[board_iter].update_pos(0, player_iter as u8),
            }
        }
    }
    pub fn input_board_num(&mut self) {
        let mut my_string = String::new();
        print!("Please enter the starting number of boards to be played (Greater than 0): ");
        stdout().flush().unwrap();

        self.board_num = loop {
            my_string.clear();

            stdin()
                .read_line(&mut my_string)
                .expect("Did not enter a correct string");
            match my_string.trim().parse::<u16>() {
                Ok(_num) => break _num,
                Err(_err) => print!("That number was either less than 0 or too big: "),
            }
            stdout().flush().unwrap();
        };
        if self.board_num == 0 {
            self.board_num = 1;
        }
    }
    pub fn set_timeout_duration_len(&mut self) {
        let mut my_string = String::new();
        print!(
            "Please enter the duration, in seconds, for each player's turn input (0 = infinite): "
        );
        stdout().flush().unwrap();

        self.timeout_duration_len = loop {
            my_string.clear();

            stdin()
                .read_line(&mut my_string)
                .expect("Did not enter a correct string");

            match my_string.trim().parse::<u64>() {
                Ok(_duration_len) => break _duration_len,
                Err(_err) => print!("Try again! That number was less than 0: "),
            }
            stdout().flush().unwrap();
        };
        if self.timeout_duration_len == 0 {
            self.timeout_duration_len = 999999;
        }
    }
    fn update_bvi(&mut self, i: usize) {
        if self.vec_board_iter.len() > 1 {
            self.vec_board_iter.remove(i);
        } else {
            self.vec_board_iter.pop();
        }
    }
    fn display_board(&self, board_iter: usize) {
        reset_terminal();
        println!("{}", self.vec_board[board_iter]);
        cursor_move_to(self.origin_coordinates.0, self.origin_coordinates.1 + 7);
        self.score_board();
        cursor_move_to(self.origin_coordinates.0, self.origin_coordinates.1 + 4);
    }
    fn player_turns(&mut self, board_iter: usize) {
        for player_iter in 0..2 {
            self.display_board(board_iter);
            self.vec_board[board_iter].check_first_move();
            self.execute_player_turn(board_iter, player_iter);
            self.vec_board[board_iter]
                .check_completed(player_iter as u8 + 1, &self.players[player_iter].name);
            if self.vec_board[board_iter].winner != 0 {
                self.players[player_iter].add_point();
                self.update_bvi(board_iter);
                self.display_board(board_iter);
                sleep(Duration::from_secs(3));
                return;
            }
        }
        if self.vec_board_iter.len() > 1 {
            self.display_board(board_iter);
            sleep(Duration::from_secs(3));
        }
    }
    fn check_game_finished(&mut self) {
        if self.vec_board_iter.is_empty() {
            self.finished = true;
        } else {
            self.finished = false;
        }
    }
    pub fn check_game_winner(&mut self) {
        match self.players[0].points.cmp(&self.players[1].points) {
            Ordering::Greater => self.winner = player::WonPlayer::Circle,
            Ordering::Less => self.winner = player::WonPlayer::Crosses,
            Ordering::Equal => self.winner = player::WonPlayer::None,
        };
    }
    pub fn game_continue(&mut self) {
        self.prompt_continue();
        if !self.finished {
            self.vec_board
                .push(board::Board::new(self.vec_board.len() as u16 + 1));
            self.vec_board_iter.push(self.vec_board.len() as u16 - 1);
            self.finished = true;
        } else {
            reset_terminal();
            println!("Game Terminated");
        }
    }
    fn prompt_continue(&mut self) {
        let mut my_string = String::new();
        println!("Since the game ended in a tie, would you like to continue with one more board?");
        print!("Respond with any form of yes or no: ");
        stdout().flush().unwrap();

        my_string.clear();

        stdin()
            .read_line(&mut my_string)
            .expect("Did not enter a correct string");
        my_string = my_string.trim().to_lowercase();
        if my_string == *"y".to_string() || my_string == *"yes".to_string() {
            self.finished = false;
        }
    }
    pub fn end_msg_gen(&mut self) {
        match self.winner {
            player::WonPlayer::Circle => {
                self.end_msg = format!(
                    "{} has won with {} points, compared to {}'s {} points",
                    self.players[0].name,
                    self.players[0].points,
                    self.players[1].name,
                    self.players[1].points,
                )
            }
            player::WonPlayer::Crosses => {
                self.end_msg = format!(
                    "{} has won with {} points, compared to {}'s {} points",
                    self.players[1].name,
                    self.players[1].points,
                    self.players[0].name,
                    self.players[0].points,
                )
            }
            player::WonPlayer::None => {
                self.end_msg = format!(
                    "The game ended in a tie of {} points",
                    self.players[0].points,
                )
            }
        }
    }
    pub fn score_board(&self) {
        let mut text = Vec::new();
        for i in 0..self.vec_board.len() {
            text.push(format!(
                "Board {}: {}",
                self.vec_board[i].name, self.vec_board[i].completion_reason
            ));
        }

        let full_text = text.join("\n");

        println!("-=-=- Leaderboard -=-=-\n{}", full_text);
    }
    fn forfeit(&mut self, board_iter: usize, player_iter: usize) {
        self.vec_board[board_iter].update_cmpd(2, &self.players[player_iter].name);
    }
    fn quit(&mut self) {
        self.finished = true;
        reset_terminal();
        println!("Game Terminated");
    }
}

pub fn reset_terminal() {
    execute!(stdout(), RestorePosition, Clear(ClearType::FromCursorDown))
        .expect("Could not restore cursor position or clear terminal");
}
pub fn cursor_move_to(x: u16, y: u16) {
    execute!(stdout(), MoveTo(x, y)).expect("Could not move down two spaces");
}
pub fn set_starting_pos() -> (u16, u16) {
    execute!(stdout(), SavePosition).expect("Could not save cursor position");
    match position() {
        Ok(_pos_coordinates) => _pos_coordinates,
        Err(_err) => (0, 0),
    }
}
