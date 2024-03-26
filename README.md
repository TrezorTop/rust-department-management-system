# Rust Department Management System

This is a command-line application written in Rust that allows you to manage a department system. You can add or remove people from departments and list all people in a department or in all departments. The project was created as part of a Rust learning journey, serving as a practical way to understand and apply new concepts such as Rust's ownership, borrowing, and lifetimes, as well as its powerful enum and match features.

## Features

- Add a person to a department
- Remove a person from a department
- List all people in a department
- List all people in all departments

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust: You can download Rust from the official website [here](https://www.rust-lang.org/tools/install).

### Installing

1. Clone the repository
```bash
git clone https://github.com/TrezorTop/rust-department-management-system.git
```
2. Navigate to the cloned repository
```bash
cd rust-department-management-system
```
3. Build the project
```bash
cargo build
```
4. Run the project
```bash
cargo run
```

## Usage

After running the project, you will be prompted to enter a command. The available commands are:

- `add [person] to [department]`: This command adds a person to a department.
- `remove [person] from [department]`: This command removes a person from a department.
- `list [department]`: This command lists all people in a department. If no department is provided, it lists all people in all departments.
