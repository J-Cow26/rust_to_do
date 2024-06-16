use clap::{arg, Command};
use colored::*;
use dirs::home_dir;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Write};

fn cli() -> Command {
    Command::new("tdman")
        .about("To do manager - manage your tasks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(Command::new("open").about("opens the task list to view and edit").arg(arg!(-t --"task-list" <PATH> "Specify location of task list. Default is tasks.txt")))
        .subcommand(Command::new("view").about("outputs current tasks").arg(arg!(-t --"task-list" <PATH> "Specify location of task list. Default is tasks.txt")))
        .subcommand(Command::new("add").about("add a new task"))
        .subcommand(Command::new("remove").about("removes a specified task from task list").arg(arg!(-s --select <NUMBER> "Specify the task number to be removed.").value_parser(clap::value_parser!(usize))))
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
        Some(("remove", sub_matches)) => {
            let _ = delete_task(sub_matches.get_one::<usize>("select").unwrap().to_owned());
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
        match fs::create_dir_all(home_dir().unwrap().join(".doit")){
            Ok(()) => println!("Directory created"),
            Err(error) => eprintln!("{}", error),
        };
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
                    entry = format!("- {}", entry);
                    let _failing_function = append_to_file(entry); // Find a better way of handling the error case
                }
            }
            Err(error) => println!("error: {error}"),
        }
    }
}

/// Append task to the end of the file
fn append_to_file(entry: String) -> std::io::Result<()> {
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(home_dir().unwrap().join(".doit").join("tasks.txt"))?;

    // Count the number of lines in task list
    let number_of_lines =
        match fs::read_to_string(home_dir().unwrap().join(".doit").join("tasks.txt")) {
            Ok(content) => content,
            Err(e) => format!("{e}"),
        }
        .lines()
        .count() + 1;

    let final_entry = format!("<{}> {}", number_of_lines, entry,);
    // Write to a file
    data_file.write_all(final_entry.as_bytes())?;
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

fn delete_task(task_number: usize) -> io::Result<()> {
    let input_file = home_dir().unwrap().join(".doit").join("tasks.txt");
    let line_number_to_skip: usize = task_number;

    // Open the input file
    let input = File::open(&input_file)?;
    let reader = BufReader::new(input);

    // Create the output file
    let mut output_file = input_file.clone();
    output_file.set_extension("tmp");
    let mut output = File::create(&output_file)?;

    // Read the input file line by line and write to output file
    for (index, line) in reader.lines().enumerate() {
        let mut line = line?;
        if index + 1 != line_number_to_skip {
            if index + 1 > line_number_to_skip {
                // Need to decrease line number shown before task
                let line_number = index.to_string();
                line = edit_section(&line, &line_number);
            }
            writeln!(output, "{}", line)?;
        }
    }

    // Replace task-list with new task list without specified line
    fs::rename(output_file, input_file)?;
    Ok(())
}

fn edit_section(original: &str, replacement: &str) -> String {
    if let (Some(start), Some(end)) = (original.find('<'), original.find('>')) {
        let mut result = String::new();

        // Add the part before the start position
        result.push_str(&original[..start + 1]);

        // Add the replacement string
        result.push_str(replacement);

        // Add the part after the end position
        result.push_str(&original[end..]);

        result
    } else {
        // If no <> are found, return the original string
        original.to_string()
    }
}
