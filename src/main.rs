use crossterm::cursor::{position, MoveTo, RestorePosition, SavePosition};
use crossterm::event::{poll, read, Event, KeyCode, KeyCode::Char};
use crossterm::execute;
use crossterm::style::{Colorize, StyledContent, Styler};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::cmp::Ordering;
use std::fmt;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

struct Game {
    vec_board: Vec<Board>,
    vec_board_iter: Vec<u16>,
    board_iter: usize,
    players: [Player; 2],
    player_iter: usize,
    winner: WonPlayer,
    input_timeout_duration: u64,
    finished: bool,
    origin_coordinates: (u16, u16),
}
struct Board {
    name: u16,
    positions: [u8; 9],
    display_positions: Vec<StyledContent<String>>,
    completed: bool,
    completion_reason: String,
}

struct Player {
    name: String,
    points: u16,
}

enum WonPlayer {
    Circle,
    Crosses,
    None,
}

impl Game {
    fn new() -> Self {
        Self {
            vec_board: Vec::new(),
            vec_board_iter: Vec::new(),
            board_iter: 0,
            players: [Player::new("Player 1"), Player::new("Player 2")],
            player_iter: 0,
            winner: WonPlayer::None,
            input_timeout_duration: 999999,
            finished: false,
            origin_coordinates: (0, 0),
        }
    }
    fn board_gen(&mut self, num_boards: u16) {
        for num in 0..num_boards {
            self.vec_board.push(Board::new(num + 1));
            self.vec_board_iter.push(num);
        }
        println!("Number of boards set to: {}", num_boards);
    }
    fn set_tdl(&mut self, len: u64) {
        self.input_timeout_duration = len;
        println!("Duration for each turn set to: {}s", len);
    }
    fn grab_user_input(&self) -> crossterm::Result<KeyCode> {
        enable_raw_mode().expect("Could not enable terminal raw mode");
        let key_event = loop {
            if poll(Duration::from_secs(self.input_timeout_duration))? {
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
    fn game_loop(&mut self) {
        while !self.finished {
            for i in 0..self.vec_board_iter.len() {
                self.update_bi(i as usize);
                if !self.vec_board[self.board_iter].completed {
                    self.player_turns();
                    self.check_game_finished();
                } else {
                    break;
                }
            }
        }
        return;
    }
    fn execute_player_turn(&mut self) {
        if self.vec_board[self.board_iter].check_first_move() {
            self.vec_board[self.board_iter].update_pos(0, self.player_iter as u8);
        } else {
            print!(
                "{} Please make your move (1-9): ",
                self.players[self.player_iter].name
            );
            stdout().flush().unwrap();
            let keycode_input = loop {
                match self.grab_user_input() {
                    Ok(_input) => break _input,
                    Err(_err) => continue,
                }
            };
            match keycode_input {
                Char('0') => self.vec_board[self.board_iter].update_pos(0, self.player_iter as u8),
                Char('1') if { self.vec_board[self.board_iter].positions[0] == 0 } => {
                    self.vec_board[self.board_iter].update_pos(1, self.player_iter as u8)
                }
                Char('2') if { self.vec_board[self.board_iter].positions[1] == 0 } => {
                    self.vec_board[self.board_iter].update_pos(2, self.player_iter as u8)
                }
                Char('3') if { self.vec_board[self.board_iter].positions[2] == 0 } => {
                    self.vec_board[self.board_iter].update_pos(3, self.player_iter as u8)
                }
                Char('4') if { self.vec_board[self.board_iter].positions[3] == 0 } => {
                    self.vec_board[self.board_iter].update_pos(4, self.player_iter as u8)
                }
                Char('5') if { self.vec_board[self.board_iter].positions[4] == 0 } => {
                    self.vec_board[self.board_iter].update_pos(5, self.player_iter as u8)
                }
                Char('6') if { self.vec_board[self.board_iter].positions[5] == 0 } => {
                    self.vec_board[self.board_iter].update_pos(6, self.player_iter as u8)
                }
                Char('7') if { self.vec_board[self.board_iter].positions[6] == 0 } => {
                    self.vec_board[self.board_iter].update_pos(7, self.player_iter as u8)
                }
                Char('8') if { self.vec_board[self.board_iter].positions[7] == 0 } => {
                    self.vec_board[self.board_iter].update_pos(8, self.player_iter as u8)
                }
                Char('9') if { self.vec_board[self.board_iter].positions[8] == 0 } => {
                    self.vec_board[self.board_iter].update_pos(9, self.player_iter as u8)
                }
                Char('f') => self.forfeit(),
                Char('q') => self.quit(),
                _ => self.vec_board[self.board_iter].update_pos(0, self.player_iter as u8),
            }
        }
    }
    fn input_board_num(&self) -> u16 {
        let mut my_string = String::new();
        print!("Please enter the starting number of boards to be played (Greater than 0): ");
        stdout().flush().unwrap();

        let mut board_num = loop {
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
        if board_num == 0 {
            board_num = 1;
        }
        board_num
    }
    fn input_timeout_duration_len(&self) -> u64 {
        let mut my_string = String::new();
        print!(
            "Please enter the duration, in seconds, for each player's turn input (0 = infinite): "
        );
        stdout().flush().unwrap();

        let mut timeout_duration_len = loop {
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
        if timeout_duration_len == 0 {
            timeout_duration_len = 999999;
        }
        timeout_duration_len
    }
    fn update_bvi(&mut self, i: usize) {
        if self.vec_board_iter.len() > 1 {
            self.vec_board_iter.remove(i);
        } else {
            self.vec_board_iter.pop();
        }
    }
    fn update_bi(&mut self, i: usize) {
        self.board_iter = self.vec_board_iter[i] as usize;
    }
    fn update_pi(&mut self, i: usize) {
        self.player_iter = i;
    }
    fn display_board(&self) {
        reset_terminal();
        println!("{}", self.vec_board[self.board_iter]);
        cursor_move_to(self.origin_coordinates.0, self.origin_coordinates.1 + 7);
        self.score_board();
        cursor_move_to(self.origin_coordinates.0, self.origin_coordinates.1 + 4);
    }
    fn player_turns(&mut self) {
        for player_iter in 0..2 {
            self.update_pi(player_iter);
            self.display_board();
            self.execute_player_turn();
            if self.vec_board[self.board_iter].check_win(self.player_iter as u8) {
                self.vec_board[self.board_iter].update_cmpd(1, &self.players[player_iter].name);
                self.update_bvi(self.board_iter);
                self.players[player_iter].add_point();
                self.display_board();
                sleep(Duration::from_secs(3));
                return;
            } else if self.vec_board[self.board_iter].check_tie() {
                self.vec_board[self.board_iter].update_cmpd(0, &self.players[player_iter].name);
                self.update_bvi(self.board_iter);
                self.display_board();
                sleep(Duration::from_secs(3));
                return;
            }
        }
        if self.vec_board_iter.len() > 1 {
            self.display_board();
            sleep(Duration::from_secs(3));
        }
        return;
    }
    fn check_game_finished(&mut self) {
        if self.vec_board_iter.len() == 0 {
            self.finished = true;
        } else {
            self.finished = false;
        }
    }
    fn check_game_winner(&mut self) {
        match self.players[0].points.cmp(&self.players[1].points) {
            Ordering::Greater => self.winner = WonPlayer::Circle,
            Ordering::Less => self.winner = WonPlayer::Crosses,
            Ordering::Equal => self.winner = WonPlayer::None,
        };
    }
    fn game_continue(&mut self) {
        if self.prompt_continue() {
            self.vec_board
                .push(Board::new(self.vec_board.len() as u16 + 1));
            self.vec_board_iter.push(self.vec_board.len() as u16 - 1);
            self.check_game_finished();
        } else {
            println!("Game Terminated");
        }
    }
    fn prompt_continue(&mut self) -> bool {
        let mut my_string = String::new();
        println!("Since the game ended in a tie, would you like to continue with one more board?");
        print!("Respond with any form of yes or no: ");
        stdout().flush().unwrap();

        my_string.clear();

        stdin()
            .read_line(&mut my_string)
            .expect("Did not enter a correct string");
        my_string = my_string.trim().to_lowercase();
        if my_string == "y".to_string() || my_string == "yes".to_string() {
            return true;
        }
        false
    }
    fn end_msg_gen(&mut self) -> String {
        match self.winner {
            WonPlayer::Circle => format!(
                "{} has won with {} points, compared to {}'s {} points",
                self.players[0].name,
                self.players[0].points,
                self.players[1].name,
                self.players[1].points,
            ),
            WonPlayer::Crosses => format!(
                "{} has won with {} points, compared to {}'s {} points",
                self.players[1].name,
                self.players[1].points,
                self.players[0].name,
                self.players[0].points,
            ),
            WonPlayer::None => format!(
                "The game ended in a tie of {} points",
                self.players[0].points,
            ),
        }
    }
    fn score_board(&self) {
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
    fn forfeit(&mut self) {
        self.vec_board[self.board_iter].update_cmpd(2, &self.players[self.player_iter].name);
    }
    fn quit(&mut self) {
        self.finished = true;
        reset_terminal();
        println!("Game Terminated");
    }
}

impl Board {
    fn new(input_name: u16) -> Self {
        Self {
            name: input_name,
            positions: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            display_positions: vec![
                "-".to_string().bold(),
                "-".to_string().bold(),
                "-".to_string().bold(),
                "-".to_string().bold(),
                "-".to_string().bold(),
                "-".to_string().bold(),
                "-".to_string().bold(),
                "-".to_string().bold(),
                "-".to_string().bold(),
            ],
            completed: false,
            completion_reason: "Not Completed".to_string(),
        }
    }
    fn update_pos(&mut self, mut position_num: u8, turn: u8) {
        while position_num == 0 {
            position_num = self.gen_rand_pos() + 1;
        }
        self.positions[position_num as usize - 1] = turn + 1;
        for num in 0..9 {
            if !self.check_equal(num, 0) {
                if self.check_equal(num, 1) {
                    self.display_positions[num as usize] = "O".to_string().bold();
                } else {
                    self.display_positions[num as usize] = "X".to_string().bold();
                }
            }
        }
    }
    fn update_cmpd(&mut self, type_cmpd: u8, winner_name: &String) {
        self.completed = true;
        if type_cmpd == 0 {
            self.completion_reason = "Tied".to_string();
        } else if type_cmpd == 1 {
            self.completion_reason = format!("Won by {}", winner_name);
        } else if type_cmpd == 2 {
            self.completion_reason = format!("Forfeited by {}", winner_name);
        }
    }
    fn check_equal(&self, i: u8, value: u8) -> bool {
        let mut bool_var = false;
        if self.positions[i as usize] == value {
            bool_var = true
        }

        bool_var
    }
    fn check_win(&mut self, mut player_iter: u8) -> bool {
        player_iter = player_iter + 1;
        let mut num = 0;
        for n in 0..3 {
            let pos_iter = num;
            if self.check_equal(pos_iter, player_iter)
                && self.check_equal(pos_iter + 1, player_iter)
                && self.check_equal(pos_iter + 2, player_iter)
            {
                self.highlight_yellow(
                    pos_iter as usize,
                    pos_iter as usize + 1,
                    pos_iter as usize + 2,
                    player_iter,
                );
                return true;
            } else if self.check_equal(n, player_iter)
                && self.check_equal(n + 3, player_iter)
                && self.check_equal(n + 6, player_iter)
            {
                self.highlight_yellow(n as usize, n as usize + 3, n as usize + 6, player_iter);
                return true;
            }
            num = pos_iter + 3;
        }
        if self.check_equal(0, player_iter)
            && self.check_equal(4, player_iter)
            && self.check_equal(8, player_iter)
        {
            self.highlight_yellow(0 as usize, 4 as usize, 8 as usize, player_iter);
            return true;
        } else if self.check_equal(2, player_iter)
            && self.check_equal(4, player_iter)
            && self.check_equal(6, player_iter)
        {
            self.highlight_yellow(2 as usize, 4 as usize, 6 as usize, player_iter);
            return true;
        }
        return false;
    }
    fn check_tie(&self) -> bool {
        let mut tie = true;
        for num in 0..9 {
            if self.check_equal(num, 0) {
                tie = false
            }
        }
        tie
    }
    fn check_first_move(&self) -> bool {
        let mut first_move = true;
        for key in 0..9 {
            if !self.check_equal(key, 0) {
                first_move = false;
            }
        }

        first_move
    }
    fn gen_rand_pos(&self) -> u8 {
        let mut rand_pos_vec = Vec::new();
        for i in 0..9 {
            if self.positions[i] == 0 {
                rand_pos_vec.push(i);
            }
        }
        fastrand::shuffle(&mut rand_pos_vec);
        rand_pos_vec[0] as u8
    }
    fn highlight_yellow(&mut self, pos1: usize, pos2: usize, pos3: usize, player_iter: u8) {
        if player_iter == 0 {
            self.display_positions[pos1] = "0".to_string().bold().yellow();
            self.display_positions[pos2] = "0".to_string().bold().yellow();
            self.display_positions[pos3] = "0".to_string().bold().yellow();
        } else {
            self.display_positions[pos1] = "X".to_string().bold().yellow();
            self.display_positions[pos2] = "X".to_string().bold().yellow();
            self.display_positions[pos3] = "X".to_string().bold().yellow();
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = vec![
            format!("-=- Displaying Board {} -=-", self.name),
            format!(
                "           {}|{}|{}",
                self.display_positions[0], self.display_positions[1], self.display_positions[2],
            ),
            format!(
                "           {}|{}|{}",
                self.display_positions[3], self.display_positions[4], self.display_positions[5],
            ),
            format!(
                "           {}|{}|{}",
                self.display_positions[6], self.display_positions[7], self.display_positions[8],
            ),
        ];

        let full_text = text.join("\n");

        write!(f, "{}", full_text)
    }
}
impl Player {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: 0,
        }
    }
    fn add_point(&mut self) {
        self.points += 1;
    }
}
fn reset_terminal() {
    execute!(stdout(), RestorePosition, Clear(ClearType::FromCursorDown))
        .expect("Could not restore cursor position or clear terminal");
}
fn cursor_move_to(x: u16, y: u16) {
    execute!(stdout(), MoveTo(x, y)).expect("Could not move down two spaces");
}
fn set_starting_pos() -> (u16, u16) {
    execute!(stdout(), SavePosition).expect("Could not save cursor position");
    match position() {
        Ok(_pos_coordinates) => _pos_coordinates,
        Err(_err) => (0, 0),
    }
}
fn execute(mut game: Game) {
    let terminal_size = match size() {
        Ok(_size) => _size,
        Err(_size) => (0, 0),
    };
    if terminal_size > (80, 29) {
        game.origin_coordinates = set_starting_pos();
        game.board_gen(game.input_board_num());
        game.set_tdl(game.input_timeout_duration_len());
        while !game.finished {
            game.game_loop();
            reset_terminal();
            game.check_game_winner();
            println!(
                "-=-=-=-=- All Boards Have Been Completed! -=-=-=-=-\n{}",
                game.end_msg_gen(),
            );
            cursor_move_to(game.origin_coordinates.0, game.origin_coordinates.1 + 6);
            game.score_board();
            cursor_move_to(game.origin_coordinates.0, game.origin_coordinates.1 + 3);
            match game.winner {
                WonPlayer::None => game.game_continue(),
                WonPlayer::Circle => (),
                WonPlayer::Crosses => (),
            }
            cursor_move_to(
                game.origin_coordinates.0,
                game.origin_coordinates.1 + game.vec_board.len() as u16 + 8,
            );
        }
    } else {
        println!("Terminal size is too small, resize it and start again");
    }
}
fn main() {
    let game = Game::new();
    execute(game);
}
