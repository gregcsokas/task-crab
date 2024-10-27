use clap::{arg, Command};

enum Status {
    Todo,
    InProgress,
    Done
}

#[derive(Debug, PartialEq)]
enum Priority {
    High,
    Medium,
    Low
}

impl Priority {

    fn to_string(&self) -> String {
        match self {
            Priority::High => '🔴'.to_string(),
            Priority::Medium => '🟡'.to_string(),
            Priority::Low => '🟢'.to_string()
        }
    }

    fn from_str(priority: &str) -> Option<Priority> {
        match priority {
            "high" => Some(Priority::High),
            "medium" => Some(Priority::Medium),
            "low" => Some(Priority::Low),
            _ => None
        }
    }
}

struct Task {
    title: String,
    status: Status,
    priority: Priority,

}

fn main() {
    let _matches = Command::new("task-crab")
        .version("1.0")
        .about("🦀 The Task-Crab 🦀")
        .arg(arg!(-d --debug ... "Turn debugging information on"))
        .subcommand(
            Command::new("create")
            .about("Create a new task")
                .arg(arg!([title] "Title of the task to create"))
                .arg(arg!([description] "Description of the task to create"))
                .arg(
                    arg!([priority] "Task priority number")
                    .value_parser(["high", "medium", "low"])
                )
        )
        .subcommand(
            Command::new("list")
            .about("List tasks")
                .arg(arg!(-p --priority "List by priority"))
                .arg(arg!(-s --status "List by status"))
                .arg(arg!(-t --title "List by title"))
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
