mod commands;
mod utils;

use crate::commands::runner::*;
use crate::utils::io::*;
use colored::Colorize;

fn main() {
    let mut state = RunnerState {
        history: vec![],
        command_number: 0,
    };

    loop {
        print!("[{}] ", state.command_number.to_string().blue());
        let mut i = input();
        state.command_number += 1;

        run_command(&mut i, &mut state);
    }
}
