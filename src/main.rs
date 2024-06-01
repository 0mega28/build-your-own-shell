use std::collections::HashMap;
use std::io::{self, Write};
use std::process;
use colored::Colorize;

fn exit(_arg0: &str, args: &[&str]) -> u8 {
    let exit_code = args.get(0).unwrap_or(&"0");
    let exit_code = exit_code.parse::<i32>().unwrap_or(0);
    process::exit(exit_code);
}

fn main() {
    let mut commands: HashMap<String, fn(&str, &[&str]) -> u8> = HashMap::new();

    commands.insert(String::from("hello"), |_arg0, _args| {
        println!("Hello, World!");
        return 0;
    });

    commands.insert(String::from("exit"), exit);

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
