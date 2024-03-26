use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

enum Command {
    Add { person: String, department: String },
    Remove { person: String, department: String },
    List { department: Option<String> },
    Unknown,
}

struct Database {
    data: HashMap<String, Vec<String>>,
}

impl Database {
    fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    fn add(&mut self, person: &str, department: &str) {
        let department_people = self.data.entry(department.to_string()).or_insert(Vec::new());

        match department_people.iter().find(|&x| x == person) {
            None => {
                department_people.push(person.to_string());
                department_people.sort_unstable();
            }
            Some(_) => println!("Person already in this department"),
        }
    }

    fn remove(&mut self, person: &str, department: &str) {
        let department_people = self.data.get_mut(&department.to_string());

        match department_people {
            None => {
                println!("Department not found.");
            }
            Some(people) => {
                people.retain(|x| x != person);
                people.sort_unstable();
            }
        }
    }

    fn list(&self, department: Option<String>) {
        match department {
            None => {
                for (key, value) in &self.data {
                    println!("{key} department:");

                    for person in value {
                        println!("\t {person}")
                    }
                }
            }
            Some(department) => {
                let department_people = &self.data.get(&department);

                match department_people {
                    None => println!("Department {department} not found"),
                    Some(people) => {
                        println!("{department} department:");
                        for person in *people {
                            println!("{}", person);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut database = Database::new();

    loop {
        println!("Hello, please type the command...");

        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to parse command");

        println!("============================================================");

        match parse_command(&text) {
            Command::Add { person, department } => {
                database.add(&person, &department);
                println!("{:?}", database.data);
            }
            Command::Remove { person, department } => {
                database.remove(&person, &department);
                println!("{:?}", database.data);
            }
            Command::List { department } => database.list(department),
            Command::Unknown => println!("Unknown command.")
        }

        println!("============================================================");
    }
}

fn parse_command(input: &str) -> Command {
    let words: Vec<&str> = input.split_whitespace().collect();

    match words.first() {
        Some(&"add") => {
            let person = words.get(1).unwrap_or(&"").to_string();
            let department = words.get(3).unwrap_or(&"").to_string();

            Command::Add { person, department }
        }
        Some(&"remove") => {
            let person = words.get(1).unwrap_or(&"").to_string();
            let department = words.get(3).unwrap_or(&"").to_string();

            Command::Remove { person, department }
        }
        Some(&"list") => {
            let department = words.get(2).map(|s| s.to_string());

            Command::List { department }
        }
        _ => Command::Unknown
    }
}