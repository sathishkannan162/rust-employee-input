use core::panic;
use std::{
    collections::HashMap,
    io::{self, stdin},
};

enum YesNo {
    Yes,
    No,
}

fn main() {
    let departments = vec!["Engineering", "Sales", "Information technology"];
    let mut department_employee_map: HashMap<&str, Vec<String>> = HashMap::new();
    println!("Enter the data of the Employees");
    loop {
        let mut name_input = String::new();
        println!("Enter Employee Name: ");
        io::stdin()
            .read_line(&mut name_input)
            .expect("failed to read line");
        let name = name_input.trim();
        println!("Which departments do you want to assign to");
        for (index, department) in departments.iter().enumerate() {
            println!("{}: {department}", index + 1);
        }
        let mut department_number_input = String::new();
        io::stdin()
            .read_line(&mut department_number_input)
            .expect("failed to read line");
        println!("department number input: {department_number_input}");
        let department_number: usize = department_number_input
            .trim()
            .parse()
            .expect("Enter a valid number");

        if department_number > departments.len() {
            panic!("Enter a valid department number");
        }
        let department = departments
            .get(department_number - 1)
            .expect("Enter a valid department_number");

        let temp_vector: Vec<String> = Vec::new();
        let mut current_vector = department_employee_map
            .get(department)
            .unwrap_or(&temp_vector)
            .clone();
        current_vector.push(name.to_string());
        department_employee_map.insert(&department, current_vector);
        println!("department values {:?}", department_employee_map);
        println!("Do you want to continue? (y or n):");
        let mut choice_input = String::new();
        io::stdin()
            .read_line(&mut choice_input)
            .expect("failed to read line.");
        let choice = match choice_input.trim().to_lowercase().as_str() {
            "y" => YesNo::Yes,
            "n" => YesNo::No,
            _ => {
                println!("Invalid choice, defaulting to No.");
                YesNo::No
            }
        };
        if let YesNo::No = choice {
            break;
        }
        // below match statement is also works.
        // match choice {
        //     YesNo::Yes => (),
        //     YesNo::No => break,
        // }
    }
    loop {
        println!("Do you want to retrieve the data? (y or n)");
        let mut choice_input = String::new();
        io::stdin()
            .read_line(&mut choice_input)
            .expect("failed to read line.");
        let choice = match choice_input.trim().to_lowercase().as_str() {
            "y" => YesNo::Yes,
            "n" => YesNo::No,
            _ => {
                println!("Invalid choice, defaulting to No.");
                YesNo::No
            }
        };
        if let YesNo::Yes = choice {
            println!("Options:");
            println!("1. Get all employees \n2. Get emplyee from a specific department.");

            let mut choice_input = String::new();
            io::stdin()
                .read_line(&mut choice_input)
                .expect("failed to read line.");
            let choice = match choice_input.trim().to_lowercase().as_str() {
                "1" => 1,
                "2" => 2,
                _ => {
                    println!("Invalid option");
                    break;
                }
            };
            if choice == 1 {
                department_employee_map.iter().for_each(|(_key, value)| {
                    value.iter().enumerate().for_each(|(index, employee)| {
                        println!("{index}. {employee}");
                    });
                });
            }
            if choice == 2 {
                println!("Choose your department");
                for (index, department) in departments.iter().enumerate() {
                    println!("{}: {department}", index + 1);
                }
                let mut department_number_input = String::new();
                io::stdin()
                    .read_line(&mut department_number_input)
                    .expect("failed to read line");
                println!("department number input: {department_number_input}");
                let department_number: usize = department_number_input
                    .trim()
                    .parse()
                    .expect("Enter a valid number");

                if department_number > departments.len() {
                    panic!("Enter a valid department number");
                }
                let department = departments
                    .get(department_number - 1)
                    .expect("Enter a valid department_number");
                let department_employees = department_employee_map.get(department);

                match department_employees {
                    Some(value) => {
                        for (index, employee) in value.iter().enumerate() {
                            println!("{index}. {employee}");
                        }
                    }
                    None => {
                        println!("There are no employees in this department.")
                    }
                }
            }
        }
        if let YesNo::No = choice {
            break;
        }
    }
}
