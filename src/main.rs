use regex::Regex;
use std::io::stdin;

fn main() {
    // Regex
    let re_add = Regex::new(r"(-?\d+)\s?\+\s?(\d+)").unwrap(); // Unwrap -> Error handling.
    let re_sub = Regex::new(r"(-?\d+)\s?-\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(-?\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();

    // Get user input
    println!("Enter expression: ");

    let mut expression: String = String::new();
    stdin().read_line(&mut expression).unwrap();

    expression = make_operation(re_mult, expression, "*");
    expression = make_operation(re_div, expression, "/");
    expression = make_operation(re_add, expression, "+");
    expression = make_operation(re_sub, expression, "-");

    // Show Result
    println!("Result: {}", expression);
}

fn make_operation(reg: Regex, mut expression: String, operation: &str) -> String {
    loop {
        // Apply Operations
        let caps = reg.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        // Override variable
        let caps = caps.unwrap();

        println!("{:?}", caps);

        let cap_expression = caps.get(0).unwrap().as_str();

        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let result = match operation {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };

        expression = expression.replace(cap_expression, &result.to_string());
    }

    return expression;
}
