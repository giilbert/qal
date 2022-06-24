use crate::commands::expression::expression;
use crate::commands::history::history;

pub struct RunnerState {
    pub history: Vec<String>,
    pub command_number: i32,
}

pub fn run_command(command: &mut String, state: &mut RunnerState) {
    let tokens = command.split(" ").collect::<Vec<_>>();

    state.history.push(command.clone());

    match tokens[0] {
        "exit" => std::process::exit(0),
        "history" => history(&tokens, state),
        _ => expression(&tokens, state),
    }
}
