use crossterm::style::{ Colorize, StyledContent, Styler};
use std::fmt;

pub struct Board {
    pub name: u16,
    pub positions: [u8; 9],
    display_positions: Vec<StyledContent<String>>,
    pub completed: bool,
    pub winner: u8,
    pub completion_reason: String,
    pub first_move: bool,
}

impl Board {
    pub fn new(input_name: u16) -> Self {
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
            winner: 0,
            completion_reason: "Not Completed".to_string(),
            first_move: true,
        }
    }
    pub fn update_pos(&mut self, mut position_num: u8, turn: u8) {
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
    pub fn update_cmpd(&mut self, type_cmpd: u8, winner_name: &str) {
        self.completed = true;
        if type_cmpd == 0 {
            self.completion_reason = "Tied".to_string();
        } else if type_cmpd == 1 {
            self.completion_reason = format!("Won by {}", winner_name);
        } else if type_cmpd == 2 {
            self.completion_reason = format!("Forfeited by {}", winner_name);
        }
    }
    pub fn check_equal(&self, i: u8, value: u8) -> bool {
        let mut bool_var = false;
        if self.positions[i as usize] == value {
            bool_var = true
        }

        bool_var
    }
    pub fn check_completed(&mut self, player_iter: u8, player_name: &str) {
        let win_conditions: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for condition in &win_conditions {
            if self.positions[condition[0]] == player_iter
                && self.positions[condition[1]] == player_iter
                && self.positions[condition[2]] == player_iter
            {
                self.update_cmpd(1, player_name);
                self.highlight_yellow(condition, player_iter);
                self.winner = player_iter;
                break;
            }
        }
        if !self.completed {
            let mut x: u8 = 0;
            for num in 0..9 {
                if self.check_equal(num, 0) {
                    x += 1;
                } else {
                    break;
                }
            }
            if x == 9 {
                self.update_cmpd(0, player_name);
            }
        }
    }
    pub fn check_first_move(&mut self) {
        for pos in 0..9 {
            if !self.check_equal(pos, 0) {
                self.first_move = false;
            }
        }
    }
    pub fn gen_rand_pos(&self) -> u8 {
        let mut rand_pos_vec = Vec::new();
        for i in 0..9 {
            if self.positions[i] == 0 {
                rand_pos_vec.push(i);
            }
        }
        fastrand::shuffle(&mut rand_pos_vec);
        rand_pos_vec[0] as u8
    }
    fn highlight_yellow(&mut self, condition: &[usize; 3], player_iter: u8) {
        if player_iter == 0 {
            for pos in condition {
                self.display_positions[*pos] = "0".to_string().bold().yellow();
            }
        } else {
            for pos in condition {
                self.display_positions[*pos] = "X".to_string().bold().yellow();
            }
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
