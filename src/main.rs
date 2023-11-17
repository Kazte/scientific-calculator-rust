use regex::Regex;
use std::io::stdin;

fn main() {
    // Regex

    // Add Regex -> (\d+)\s?\+\s?(\d+)
    // (\d+) -> One or more digits
    // \s? -> Space (optional)
    // \+ -> add

    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap(); // Unwrap -> Error handling.
    let re_sub = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();

    // Get user input
    println!("Enter expression: ");

    let mut expression = String::new();
    stdin().read_line(&mut expression).unwrap();

    // Multiplication
    loop {
        // Apply Operations
        let caps = re_mult.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        // Override variable
        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();

        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let multiplication = left_value * right_value;

        expression = expression.replace(cap_expression, &multiplication.to_string());
    }

    // Divition
    loop {
        // Apply Operations
        let caps = re_div.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        // Override variable
        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();

        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let divition = left_value / right_value;

        expression = expression.replace(cap_expression, &divition.to_string());
    }

    // Addition
    loop {
        // Apply Operations
        let caps = re_add.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        // Override variable
        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();

        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition = left_value + right_value;

        expression = expression.replace(cap_expression, &addition.to_string());
    }

    // Subtraction
    loop {
        // Apply Operations
        let caps = re_sub.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        // Override variable
        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();

        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let subtraction = left_value - right_value;

        expression = expression.replace(cap_expression, &subtraction.to_string());
    }

    // Show Result
    println!("Result: {}", expression);
}
