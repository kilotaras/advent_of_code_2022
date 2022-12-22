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
    Op(Operation, String, String)
}

impl MonkeyRequest {
    fn derive(&self, name: &String) -> Vec<(String, MonkeyRequest)> {
        use MonkeyRequest::*;
        use Operation::*;
        match self {
            Value(_) => vec![],
            Op(Add, left, right) => {
                vec![
                    (right.clone(), Op(Subtract, name.clone(), left.clone())),
                    (left.clone(), Op(Subtract, name.clone(), right.clone())),
                ]
            },
            Op(Subtract, left, right) => {
                vec![
                    (left.clone(), Op(Add, name.clone(), right.clone())),
                    (right.clone(), Op(Subtract, left.clone(), name.clone())),
                ]
            },
            Op(Multiply, left, right) => {
                vec![
                    (left.clone(), Op(Divide, name.clone(), right.clone())),
                    (right.clone(), Op(Divide, name.clone(), left.clone())),
                ]
            },
            Op(Divide, left, right) => {
                vec![
                    (left.clone(), Op(Multiply, name.clone(), right.clone())),
                    (right.clone(), Op(Divide, left.clone(), name.clone())),
                ]
            },
        }
    }
}

fn apply(request: &MonkeyRequest, values: &HashMap<String, ValueType>) -> Option<ValueType> {
    match request {
        MonkeyRequest::Value(value) => Some(*value),
        MonkeyRequest::Op(op, left, right) => {
            let left = values.get(left)?;
            let right = values.get(right)?;
            let result = match op {
                Operation::Add => left + right,
                Operation::Subtract => left - right,
                Operation::Multiply => left * right,
                Operation::Divide => {
                    if left%right == 0 {
                        left/right
                    } else {
                        return None;
                    }
                }
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
        (name, MonkeyRequest::Op(op, left, right))
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
            if let Some(value) = apply(request, &values) {
                values.insert(name.clone(), value);
            }
        }
    }

    let p1_answer = *values.get("root").unwrap();

    println!("Part 1: {}", p1_answer);

    values.clear();
    values.insert("root".into(), 0);

    let input = {
        let mut transformed: Vec<(String, MonkeyRequest)> =
            input.iter()
            .filter(|(name, _)| name != "humn")
            .map(|(a,b)| (a.clone(), b.clone()))
            .collect();

        let root_pos = transformed.iter().position(|(name, _)| name == "root").unwrap();
        let root = transformed.remove(root_pos);

        let new_request = match root.1 {
            MonkeyRequest::Value(_) => panic!("Root is a value"),
            MonkeyRequest::Op(op, left, right) => {
                MonkeyRequest::Op(Operation::Subtract, left, right)
            }
        };

        transformed.push(("root".into(), new_request));

        let mut derived = vec![];

        for (name, request) in &transformed {
            derived.extend(request.derive(name));
        }

        transformed.extend(derived);

        transformed
    };

    while !values.contains_key("humn".into()) {
        for (name, request) in &input {
            if values.contains_key(name) {
                continue;
            }
            if let Some(value) = apply(request, &values) {
                values.insert(name.clone(), value);
            }
        }
    }

    let p2_answer = *values.get("humn").unwrap();

    println!("Part 1: {}", p1_answer);
    println!("Part 2: {}", p2_answer);
}
