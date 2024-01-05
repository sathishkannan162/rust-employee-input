use core::panic;
use std::{collections::HashMap, io};

fn main() {
    let departments = vec!["Engineering", "Sales", "Information technology"];

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
    let department_number: usize = department_number_input
        .trim()
        .parse()
        .expect("Enter a valid number");

    // if department_number > convert_usize_i8(departments.len()) {
    //     panic!("Enter a valid department number");
    // }
    let department = departments
        .get(department_number - 1)
        .expect("Enter a valid department_number");

    let mut department_employee_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let current_vector = department_employee_map.get(department);
    println!("{:?}", current_vector);
    let value = department_employee_map.insert(&department, current_vector]);
    println!("{:?}", department_employee_map);
}

fn convert_usize_i8(u: usize) -> i8 {
    if u <= i8::MAX as usize {
        return u as i8;
    } else {
        return 0;
    }
}
