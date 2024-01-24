/*
- Using a hash map and vectors, create a text interface to allow a user to add employee names
to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
Then let the user retrieve a list of all people in a department or all people in the company
by department, sorted alphabetically.
 */

pub fn main() {
    use std::collections::HashMap;
    use std::io;

    let mut company = HashMap::new();
    let mut department = String::new();
    let mut employee = String::new();
    let mut input = String::new();
    let mut department_list = Vec::new();
    let mut employee_list = Vec::new();

    loop {
        println!("Enter a command:");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        let input: Vec<&str> = input.split_whitespace().collect();
        if input.len() == 4 && input[0] == "Add" && input[2] == "to" {
            department = input[3].to_string();
            employee = input[1].to_string();
            company
                .entry(department.clone())
                .or_insert(Vec::new())
                .push(employee.clone());
            department_list.push(department.clone());
            employee_list.push(employee.clone());
        } else if input.len() == 3 && input[0] == "List" && input[1] == "all" {
            if input[2] == "departments" {
                department_list.sort();
                department_list.dedup();
                println!("The departments are:");
                for i in &department_list {
                    println!("{}", i);
                }
            } else if input[2] == "employees" {
                employee_list.sort();
                employee_list.dedup();
                println!("The employees are:");
                for i in &employee_list {
                    println!("{}", i);
                }
            } else {
                println!("Invalid command");
            }
        } else if input.len() == 4 && input[0] == "List" && input[2] == "in" {
            if input[3] == "department" {
                department = input[3].to_string();
                if company.contains_key(&department) {
                    println!("The employees in {} are:", department);
                    for i in company.get(&department).unwrap() {
                        println!("{}", i);
                    }
                } else {
                    println!("{} is not a valid department", department);
                }
            } else {
                println!("Invalid command");
            }
        } else {
            println!("Invalid command");
        }
    }
}
