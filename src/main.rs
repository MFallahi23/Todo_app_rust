//--------------------------------------------//
//--Todo App using Rust-----------------------//
//--------------------------------------------//

mod db;
use db::Database;
use console::Style;
use console::Term;
use dialoguer::{ theme::ColorfulTheme, Select };
use rusqlite::Result;

// Task struct: name and status

struct Task {
    name: String,
    status: bool,
}

impl Task {
    // Method for creating new tasks
    fn new(name: String) -> Self {
        Task { name, status: false }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initializing a new instance of the database
    let database = Database::new()?;
    let term = Term::stdout();
    let cyan = Style::new().cyan();

    // Main loop for interacting with todo app
    loop {
        term.clear_last_lines(100)?;
        // Displaying a welcome message
        welcome_message(&term, &cyan)?;

        // Displaying menu options (created with dialoguer crate)
        let selection = Select::with_theme(&ColorfulTheme::default())
            .item("1. Add a task")
            .item("2. View tasks")
            .item("3. Remove tasks")
            .item("4. Mark as complete")
            .item("5. Exit")
            .default(0)
            .interact_on_opt(&term)?;

        // Matching user's selection with correponding actions
        match selection {
            Some(0) => add_tasks(&term, &database)?,
            Some(1) => display_tasks(&term, &database)?,
            Some(2) => remove_tasks(&term, &database)?,
            Some(3) => mark_complete(&term, &database)?,
            Some(4) => {
                term.clear_last_lines(100)?;
                break;
            }
            _ => term.write_line("Invalid choice!")?,
        }
    }
    Ok(())
}

//--------Function for adding tasks--------//
fn add_tasks(term: &Term, database: &Database) -> Result<(), std::io::Error> {
    term.write_line("Enter task name:")?;

    // Reading the task from the user
    let task_name = term.read_line()?.trim().to_string();
    if task_name.is_empty() {
        return Err(
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Task should have a name")
        );
    }

    // Creating a new instance of Task
    let task = Task::new(task_name);

    // Adding the task to the database
    match database.add_task(&task) {
        Ok(_) => (),
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    }
    Ok(())
}

//--------Function for displaying tasks--------//
fn display_tasks(term: &Term, database: &Database) -> Result<(), std::io::Error> {
    loop {
        term.clear_last_lines(100)?;
        term.write_line("-----------")?;
        term.write_line("Todo tasks:")?;
        term.write_line("-----------")?;

        // Retrieving tasks from the database
        let task_collection = match database.get_tasks() {
            Ok(coll) => coll,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        };
        if task_collection.len() == 0 {
            println!("No tasks!");
        }

        // Displaying tasks to the user
        for task in task_collection {
            if task.status {
                let strikethrough_text: String = task.name
                    .chars()
                    .map(|c| format!("{}{}", c, '\u{0336}'))
                    .collect();
                term.write_line(&format!("{}", strikethrough_text))?;
            } else {
                term.write_line(&format!("{}", task.name))?;
            }
        }

        // Prompting the user to continue
        let selection = match
            Select::with_theme(&ColorfulTheme::default())
                .item("Continue")
                .default(0)
                .interact_on_opt(&term)
        {
            Ok(selection) => selection,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        };
        match selection {
            Some(0) => {
                break;
            }
            _ => {
                break;
            }
        }
    }
    Ok(())
}

//--------Function for removing tasks--------//
fn remove_tasks(term: &Term, database: &Database) -> Result<(), std::io::Error> {
    loop {
        term.clear_last_lines(100)?;

        // Retrieving tasks from the database
        let task_items = match database.get_tasks() {
            Ok(coll) => coll,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        };

        // Displaying tasks and getting user's selection
        display_tasks_and_select(term, &task_items, "remove", |task_name| {
            match database.remove_task(task_name) {
                Ok(_) => Ok(()),
                Err(e) => { Err(std::io::Error::new(std::io::ErrorKind::Other, e)) }
            }
        })?;
        break;
    }
    Ok(())
}

//--------Function for marking as complete--------//
fn mark_complete(term: &Term, database: &Database) -> Result<(), std::io::Error> {
    loop {
        term.clear_last_lines(100)?;

        // Retrieving tasks from database
        let task_items = match database.get_tasks() {
            Ok(coll) => coll,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        };
        // Displaying tasks and getting user's selection
        display_tasks_and_select(term, &task_items, "mark as complete", |task_name| {
            match database.mark_complete(task_name) {
                Ok(_) => Ok(()),
                Err(e) => { Err(std::io::Error::new(std::io::ErrorKind::Other, e)) }
            }
        })?;
        break;
    }
    Ok(())
}

// Function displaying the Welcome message
fn welcome_message(term: &Term, cyan: &Style) -> Result<(), std::io::Error> {
    term.clear_last_lines(100)?;
    // Displaying a welcome message
    term.write_line("------------------------")?;
    term.write_line(&format!("Welcome to the {}!", cyan.apply_to("Todo App")))?;
    term.write_line("------------------------")?;
    term.write_line("What would you like to do?")?;
    term.write_line("")?;

    Ok(())
}

// Function to display tasks and get user selection

fn display_tasks_and_select<'a>(
    term: &Term,
    task_items: &'a [Task],
    _action_message: &str,
    action_fn: impl Fn(&str) -> Result<(), std::io::Error>
) -> Result<(), std::io::Error> {
    term.clear_last_lines(100)?;

    let mut displayed_collections = Vec::new();
    for task in task_items {
        if task.status {
            let strikethrough_text: String = task.name
                .chars()
                .map(|c| format!("{}{}", c, '\u{0336}'))
                .collect();
            displayed_collections.push(strikethrough_text);
        } else {
            displayed_collections.push(task.name.clone());
        }
    }

    // Displaying tasks to the user and prompt for selection
    let selection = match
        Select::with_theme(&ColorfulTheme::default())
            .item("Continue")
            .items(&displayed_collections)
            .default(0)
            .interact_on_opt(&term)
    {
        Ok(selection) => selection,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    // Processing user's selection
    match selection {
        Some(0) => Ok(()),
        Some(i) if i >= 1 && i <= task_items.len() => {
            let task_name = &task_items[i - 1].name;
            action_fn(task_name)?;
            Ok(())
        }
        _ => {
            println!("Index out of range.");
            Ok(())
        }
    }
}
