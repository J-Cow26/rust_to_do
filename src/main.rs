use clap::{arg, Command};
use colored::*;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::Write;

fn cli() -> Command {
    Command::new("tdman")
        .about("To do manager - manage your tasks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("open").about("opens the task list to view and edit").arg(arg!(-t --"task-list" <PATH> "Specify location of task list. Default is tasks.txt")))
        .subcommand(Command::new("view").about("outputs current tasks").arg(arg!(-t --"task-list" <PATH> "Specify location of task list. Default is tasks.txt")))
        .subcommand(Command::new("add").about("add a new task"))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("open", _sub_matches)) => {
            let _default = "tasks.txt";
            open();
        }
        Some(("view", sub_matches)) => {
            let default = "tasks.txt";
            read(
                sub_matches
                    .get_one::<String>("task-list")
                    .unwrap_or(&default.to_string()),
            );
        }
        Some(("add", _sub_matches)) => {
            add();
        }

        _ => unreachable!(),
    }
}

fn open() {
    todo!("Implement open command")
}

fn add() {
    let mut adding_tasks: bool = true;
    while adding_tasks {
        // Take task input from user
        let user_instructions = "Enter task... (enter !done to exit)";
        println!("{}", user_instructions.cyan());
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                if input.contains("!done"){
                    adding_tasks = false;
                } else {
                    let mut entry: String = input;
                    entry = format!("\n{}", entry);
                    let _failing_function = append_to_file(&entry); // Find a better way of handling the error case
                }
            }
            Err(error) => println!("error: {error}"),
        }
    }
}

fn append_to_file(entry: &String) -> std::io::Result<()> {
    let mut data_file = OpenOptions::new()
        .append(true)
        .open("task.txt")?;

    // Write to a file
    data_file.write_all(entry.as_bytes())?;
    Ok(())
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
    let task_formatted = format!("******\n{}\n******", task);
    println!("{}", task_formatted.cyan());
}
