use clap::{arg, Command, Subcommand};
use std::fs::File;
use std::io::prelude::*;

fn cli() -> Command {
    Command::new("tdman")
        .about("To do manager - manage your tasks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("view").about("outputs current tasks").arg(arg!(-t --task-list <PATH> "Optional arguement to specify location of task list. Default is task.txt")))
        .subcommand(Command::new("add").about("add a new task"))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("view", _sub_matches)) => {
            read();
        }
        Some(("add", _sub_matches)) => {
            add();
        }

        _ => unreachable!(),
    }
}

fn add() {
    todo!("Implement add task command");
}

fn read() {
    let file_read = read_file();
    let content = file_read.unwrap();
    display_task(&content);
}

fn read_file() -> std::io::Result<String> {
    let mut file = File::open("task.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn display_task(task: &String) {
    println!("{}", task);
}
