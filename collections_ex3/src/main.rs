// Task: Using a hash map and vectors, create a text interface to allow a user
// to add employee names to a department in a company; for example, “Add Sally
// to Engineering” or “Add Amir to Sales.” Then, let the user retrieve a list
// of all people in a department or all people in the company by department,
// sorted alphabetically.

use std::{
    collections::HashMap,
    io::{self, Write},
};

fn retrieve_department(
    departments: &HashMap<&str, Vec<String>>,
    department: &str,
) -> Option<Vec<String>> {
    let employees = departments.get(department);

    match employees {
        None => None,
        Some(list) => {
            let mut cloned_list = list.clone();

            if list.is_sorted() {
                Some(cloned_list)
            } else {
                cloned_list.sort_unstable();
                Some(cloned_list)
            }
        }
    }
}

fn retrieve_all(departments: &HashMap<&str, Vec<String>>) -> Option<String> {
    if departments.is_empty() {
        return None;
    }

    let mut department_names: Vec<&str> = departments.keys().copied().collect();

    if !department_names.is_sorted() {
        department_names.sort_unstable();
    }

    let mut output = String::new();

    for department in department_names {
        let mut employees = departments[department].clone();

        if employees.is_empty() {
            continue;
        }

        if !employees.is_sorted() {
            employees.sort_unstable();
        }

        output += &*format!("{department}: {}.\n", employees.join(", "));
    }

    if output.is_empty() {
        None
    } else {
        Some(output)
    }
}

fn add_employee(
    departments: &mut HashMap<&str, Vec<String>>,
    department: &str,
    employee: String,
) -> Result<String, String> {
    if let Some(employee_list) = departments.get_mut(department) {
        // Necessary to use the name in the result after pushing the original
        // `String` to the `Vec`.
        let employee_clone = employee.clone();

        employee_list.push(employee);

        Ok(format!(
            "A new employee {employee_clone} added to the {department} department successfuly!"
        ))
    } else {
        Err(format!("There's no department called {department}!"))
    }
}

enum Command<'a> {
    /// `Add(department, name)`
    Add(&'a str, String),
    GetDepartmentEmployees(&'a str),
    GetEmployees,
}

/// Parses and checks the `string` and outputs a `Result` with either the
/// corresponding `Command` `enum` or with an `Err`, if it fails to parse the
/// `string` and produce a valid command.
///
/// Valid commands:
/// - `Add [name] to [department]`
/// - `Get [department] employees`
/// - `Get employees`
///
/// Note: `add`, `get` are case-insensitive. Everything else is case-sensitive.
///
/// Example command:
/// - `Add Mikhail to IT`
fn transform_to_valid_command(string: &str) -> Result<Command<'_>, String> {
    if string.is_empty() {
        return Err("Empty string does nothing".to_string());
    }

    if let Some(after_cmd_idx) = string.find(' ')
        && let Some(cmd) = string.get(..after_cmd_idx)
    {
        let lowercase_cmd = &*cmd.to_lowercase();

        match lowercase_cmd {
            "add" => {
                if let Some(after_name_idx) = string.find(" to ")
                    && let Some(name) = string.get(after_cmd_idx + 1..after_name_idx)
                {
                    if let Some(department) = string.get(after_name_idx + 4..) {
                        Ok(Command::Add(department.trim(), name.trim().to_string()))
                    } else {
                        Err("No name specified!".to_string())
                    }
                } else {
                    Err(
                        "Wrong command! To add [name] to a [department], type `Add [name] to [department]`.".to_string(),
                    )
                }
            }
            "get" => {
                if let Some(i) = string.find(" employees") {
                    if i != after_cmd_idx {
                        if let Some(department) = string.get(after_cmd_idx..i) {
                            Ok(Command::GetDepartmentEmployees(department.trim()))
                        } else {
                            Err(
                                "Wrong command! To get the list of employees of a particular department, type `Get [department] employees.".to_string(),
                            )
                        }
                    } else {
                        Ok(Command::GetEmployees)
                    }
                } else {
                    Err(
                        "Wrong command! To get the list of all employees, type `Get employees`. To get the list of employees of a particular department, type `Get [department] employees`".to_string(),
                    )
                }
            }
            _ => Err(format!("Unsupported command! ({string})")),
        }
    } else {
        Err(format!("Invalid command! ({string})"))
    }
}

fn main() {
    let mut departments: HashMap<&str, Vec<String>> = HashMap::from([
        ("engineering", Vec::new()),
        ("sales", Vec::new()),
        ("cleaning", Vec::new()),
        ("it", Vec::new()),
        ("marketing", Vec::new()),
    ]);

    let mut department_names: Vec<&str> = departments.keys().copied().collect();
    department_names.sort_unstable();

    println!(
        "To add an employee to a department, type `Add [name] to [department]`. E.g. `Add Mikhail to IT`.\nTo get the list of employees in a department, type `Get [department] employees`.\nTo get the list of all employees, type `Get employees`.\nImportant: Keywords `get` and `add` are case-insensitive; other words are case-sensitive.\n\nExisting departments: {}.\n",
        department_names.join(", ")
    );

    let mut input_buffer = String::new();

    loop {
        print!("Enter command: ");
        io::stdout().flush().expect("Failed to flush `stdout`");

        input_buffer.clear();

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Unexpected error while reading input.");

        let input_trimmed = input_buffer.trim();

        let command_result = match transform_to_valid_command(input_trimmed) {
            Err(err) => Err(err),
            Ok(res) => match res {
                Command::Add(department, name) => add_employee(&mut departments, department, name),
                Command::GetDepartmentEmployees(department) => {
                    match retrieve_department(&departments, department) {
                        None => Err(format!("There's no department {department}!")),
                        Some(list) => Ok(format!("Work in {department}: {}.", list.join(", "))),
                    }
                }
                Command::GetEmployees => match retrieve_all(&departments) {
                    None => Err("There are no departments...".to_string()),
                    Some(output) => Ok(output),
                },
            },
        };

        match command_result {
            Ok(output) => {
                println!("{output}\n");
            }
            Err(err) => {
                eprintln!("{err}\n");
            }
        }
    }
}
