# Rust Department Management System

This project is a simple command-line application written in Rust. It allows you to manage a department system where you can add and remove people from departments and list all people in a department or in all departments.

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
