use crate::output;
use crate::RunnerState;

pub fn history(tokens: &Vec<&str>, state: &mut RunnerState) {
    let mut acc = "".to_string();
    let last_index = state.history.len() - 1;

    println!("{}", last_index);

    for (i, command) in state.history.iter().enumerate() {
        println!("{}", i);
        if i == last_index {
            acc += format!("{}", command).as_str();
        } else {
            acc += format!("{}, ", command).as_str();
        }
    }
    output(&acc);
}
