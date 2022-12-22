use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

type ValueType = i64;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum MonkeyRequest {
    Value(ValueType),
    Operation(Operation, String, String)
}

fn apply(request: &MonkeyRequest, values: HashMap<String, ValueType>) -> Option<ValueType> {
    match request {
        MonkeyRequest::Value(value) => Some(*value),
        MonkeyRequest::Operation(op, left, right) => {
            let left = values.get(left)?;
            let right = values.get(right)?;
            let result = match op {
                Operation::Add => left + right,
                Operation::Subtract => left - right,
                Operation::Multiply => left * right,
                Operation::Divide => left / right,
            };
            Some(result)
        }
    }
}

fn parse_monkey_line(s: &str) -> (String, MonkeyRequest) {
    let parts: Vec<&str> = s.split_whitespace().collect();
    let name = parts[0][0..4].to_string();

    if parts.len() == 2 {
        let value = parts[1].parse().unwrap();
        (name, MonkeyRequest::Value(value))
    } else {
        let op = match parts[2] {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => panic!("Unknown operation"),
        };
        let left = parts[1].to_string();
        let right = parts[3].to_string();
        (name, MonkeyRequest::Operation(op, left, right))
    }
}

fn get_sample() -> Vec<(String, MonkeyRequest)> {
    include_str!("../sample.txt")
        .lines()
        .map(parse_monkey_line)
        .collect()
}

fn get_task() -> Vec<(String, MonkeyRequest)> {
    include_str!("../input")
        .lines()
        .map(parse_monkey_line)
        .collect()
}

fn main() {
    let input = get_task();
    let mut values = HashMap::new();

    while !values.contains_key("root".into()) {
        for (name, request) in &input {
            if values.contains_key(name) {
                continue;
            }
            if let Some(value) = apply(request, values.clone()) {
                values.insert(name.clone(), value);
            }
        }
    }

    println!("Result: {:?}", values.get("root").unwrap());
}
