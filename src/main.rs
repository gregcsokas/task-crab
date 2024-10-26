use clap::{arg, Command};

#[derive(Debug, PartialEq)]
enum TaskPriority {
    High,
    Medium,
    Low
}

impl TaskPriority {

    fn to_string(&self) -> String {
        match self {
            TaskPriority::High => '🔴'.to_string(),
            TaskPriority::Medium => '🟡'.to_string(),
            TaskPriority::Low => '🟢'.to_string()
        }
    }

    fn from_str(priority: &str) -> Option<TaskPriority> {
        match priority {
            "high" => Some(TaskPriority::High),
            "medium" => Some(TaskPriority::Medium),
            "low" => Some(TaskPriority::Low),
            _ => None
        }
    }
}

fn main() {
    let _matches = Command::new("🦀 The Task-Crab 🦀")
        .version("1.0")
        .about("🦀 The Task-Crab 🦀")
        .arg(arg!(-d --debug ... "Turn debbugging information on"))
        .subcommand(
            Command::new("create")
            .about("Create a new task")
                .arg(arg!([name] "Name of the task to create"))
                .arg(arg!([description] "Description of the task to create"))
                .arg(
                    arg!([priority] "Task priority number")
                    .value_parser(["high", "medium", "low"])
                )
        )
        .subcommand(
            Command::new("list")
            .about("List tasks")
        )
        .subcommand(
            Command::new("delete")
            .about("Delete a task")
        )
        .subcommand(
            Command::new("update")
            .about("Update task")
        )
        .get_matches();
}
