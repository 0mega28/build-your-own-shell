use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};
use colored::Colorize;

fn main() {
    let mut commands: HashMap<String, fn(&str, &[&str]) -> u8> = HashMap::new();

    commands.insert(String::from("hello"), |_arg0, _args| {
        println!("Hello, World!");
        return 0;
    });

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut split = input.split_whitespace();

        let Some(arg0) = split.next() else {
            continue;
        };

        let rest: Vec<_> = split.collect();

        if let Some(cmd) = commands.get(arg0) {
            cmd(arg0, &rest);
        } else {
            println!("{}: {} not found", arg0, "command".red());
        }
    }
}
