use colored::Colorize;

pub fn input() -> String {
    use std::io::{stdin, stdout, Write};

    print!("{}", "? ".blue());
    let _ = stdout().flush();

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    return s;
}

pub fn output(text: String) {
    println!("{} {}", ">".green(), text)
}
