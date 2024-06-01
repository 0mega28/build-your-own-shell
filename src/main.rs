use std::collections::HashMap;
use std::io::{self, Write};
use std::{env, process};
use std::path::{Path, PathBuf};
use std::process::Command;
use colored::Colorize;

fn exit(_arg0: &str, args: &[&str]) -> u8 {
    let exit_code = args.get(0).unwrap_or(&"0");
    let exit_code = exit_code.parse::<i32>().unwrap_or(0);
    process::exit(exit_code);
}

fn echo(_arg0: &str, args: &[&str]) -> u8 {
    let output = args.join(" ");
    println!("{}", output);
    return 0;
}

fn type_builtin(arg0: &str, args: &[&str]) -> u8 {
    let builtins = vec!["exit", "echo", "type", "pwd", "cd"];

    return if let Some(command) = args.get(0) {
        if builtins.contains(command) {
            println!("{} is a shell {}", command.red(), "builtin".red());
        } else if let Some(path) = get_command_path(command) {
            println!("{} is {}", command.red(), path.into_os_string().into_string().unwrap());
        } else {
            println!("{} not found", command);
        }

        0
    } else {
        eprintln!("Usage: {} <command>", arg0);
        1
    };
}

fn pwd(_arg0: &str, _args: &[&str]) -> u8 {
    println!("{}", env::current_dir().unwrap().into_os_string().into_string().unwrap());
    return 0;
}

fn cd(arg0: &str, args: &[&str]) -> u8 {
    if let Some(path) = args.get(0) {
        let path = Path::new(path);
        if let Err(_err) = env::set_current_dir(&path) {
            eprintln!("{}: No such {} or directory", path.to_str().unwrap(), "file".red());
            return 1;
        }
        0
    } else {
        eprintln!("Usage: {} <directory>", arg0);
        1
    }
}

fn get_command_path(command: &&str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&command);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
    })
}

fn execute_command(arg0: &str, args: &Vec<&str>) {
    Command::new(arg0)
        .args(args)
        .spawn()
        .expect("failed to execute command")
        .wait()
        .expect("Failed to wait");
}

fn main() {
    let mut commands: HashMap<String, fn(&str, &[&str]) -> u8> = HashMap::new();

    commands.insert(String::from("hello"), |_arg0, _args| {
        println!("Hello, World!");
        return 0;
    });

    commands.insert(String::from("exit"), exit);
    commands.insert(String::from("echo"), echo);
    commands.insert(String::from("type"), type_builtin);
    commands.insert(String::from("pwd"), pwd);
    commands.insert(String::from("cd"), cd);

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
        } else if let Some(_path) = get_command_path(&arg0) {
            execute_command(arg0, &rest);
        } else {
            println!("{}: {} not found", arg0, "command".red());
        }
    }
}
