use clap::{arg, Command};
use std::fs::File;
use std::io::prelude::*;

fn cli() -> Command {
    Command::new("tdman")
        .about("To do manager - manage your tasks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("view").about("outputs current tasks").arg(arg!(-t --"task-list" <PATH> "Specify location of task list. Default is tasks.txt")))
        .subcommand(Command::new("add").about("add a new task"))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("view", sub_matches)) => {
            let default = "tasks.txt";
            read(sub_matches.get_one::<String>("task-list").unwrap_or(&default.to_string()));
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

fn read(task_list: &String) {
    let file_read = read_file(task_list);
    let content = file_read.unwrap();
    display_task(&content);
}

fn read_file(file: &String) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn display_task(task: &String) {
    println!("{}", task);
}
