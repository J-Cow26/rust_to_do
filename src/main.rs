use clap::{arg, Command};
use colored::*;
use dirs::home_dir;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::io::{Error, ErrorKind};

fn cli() -> Command {
    Command::new("tdman")
        .about("To do manager - manage your tasks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(Command::new("open").about("opens the task list to view and edit").arg(arg!(-t --"task-list" <PATH> "Specify location of task list. Default is tasks.txt")))
        .subcommand(Command::new("view").about("outputs current tasks").arg(arg!(-t --"task-list" <PATH> "Specify location of task list. Default is tasks.txt")))
        .subcommand(Command::new("add").about("add a new task"))
        .subcommand(Command::new("clear").about("clears all tasks from task list"))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("open", _sub_matches)) => {
            open();
        }
        Some(("view", _sub_matches)) => {
            read();
        }
        Some(("add", _sub_matches)) => {
            add();
        }
        Some(("clear", _sub_matches)) => {
            let _ = clear();
        }
        _ => unreachable!(),
    }
}

/// Controller for open command
fn open() {
    read();
    let pre_addition_text = "\n\nAdd more tasks:\n";
    println!("{}", pre_addition_text.cyan().italic());
    add();
    let final_view_text = "\n\nFinal task list:\n";
    println!("{}", final_view_text.cyan().italic());
    read();
}

/// Controller for add command
fn add() {
    if !check_task_list_exists() {
        match File::create(home_dir().unwrap().join(".doit").join("tasks.txt")) {
            Ok(_file) => setup_entry(),
            Err(error) => println!("error: {error}"),
        };
    } else {
        setup_entry();
    }
}

fn setup_entry() {
    let mut adding_tasks: bool = true;

    while adding_tasks {
        // Take task input from user
        let user_instructions = "Enter task... (enter !done to exit)";
        println!("{}", user_instructions.purple());
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                if input.contains("!done") {
                    adding_tasks = false;
                } else {
                    let mut entry: String = input;
                    entry = format!("\n- {}", entry);
                    let _failing_function = append_to_file(&entry); // Find a better way of handling the error case
                }
            }
            Err(error) => println!("error: {error}"),
        }
    }
}

/// Append task to the end of the file
fn append_to_file(entry: &String) -> std::io::Result<()> {
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(home_dir().unwrap().join(".doit").join("tasks.txt"))?;

    // Write to a file
    data_file.write_all(entry.as_bytes())?;
    Ok(())
}

/// Controller for the read command
fn read() {
    let file_read = read_file();
    let content = file_read.unwrap();
    let current_tasks_intro = "Your tasks: \n";
    println!("{}", current_tasks_intro.cyan().italic());
    display_task(&content);
}

/// Reads contents from file
fn read_file() -> std::io::Result<String> {
    if check_task_list_exists() {
        let mut file = File::open(home_dir().unwrap().join(".doit").join("tasks.txt"))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            "No task list found. Make sure you have added at least one entry. Check --help",
        ))
    }
}

/// Formats and displays tasks
fn display_task(task: &String) {
    let task_formatted = format!(
        "_________________________\n\n{}\n_________________________",
        task
    );
    println!("{}", task_formatted);
}

/// Overwrites task list by creating a empty file on its path
fn clear() -> std::io::Result<()> {
    let task_delete_warning =
        "This action will delete all tasks in your task list. Would you like to continue [y/n]";
    println!("{}", task_delete_warning.red());
    let mut response = String::new();
    match io::stdin().read_line(&mut response) {
        Ok(_n) => {
            if response.to_lowercase().contains('y') {
                File::create(home_dir().unwrap().join(".doit").join("tasks.txt"))?;
            } else {
                std::process::exit(0);
            }
        }
        Err(error) => println!("error: {error}"),
    }
    Ok(())
}

fn check_task_list_exists() -> bool {
    let file_path = home_dir().unwrap().join(".doit").join("tasks.txt");
    fs::metadata(file_path).is_ok()
}
