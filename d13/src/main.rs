use std::io::BufRead;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Value {
    Int(i32),
    List(Vec<Value>),
}

// Parse an S-expression into a Value
fn parse(input: &str) -> Value {
    let mut chars = input.chars();
    let mut stack: Vec<Vec<Value>> = vec![Vec::new(); 1];
    let mut current = String::new();


    while let Some(c) = chars.next() {
        match c {
            '[' => {
                stack.push(Vec::new());
            }
            ']' => {
                if !current.is_empty() {
                    let v = Value::Int(current.parse().unwrap());
                    current.clear();
                    stack.last_mut().unwrap().push(v);
                }
                let closed = stack.pop().unwrap();
                let value = Value::List(closed);
                stack.last_mut().unwrap().push(value);
            }
            ',' => {
                if !current.is_empty() {
                    let v = Value::Int(current.parse().unwrap());
                    current.clear();
                    stack.last_mut().unwrap().push(v);
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    assert!(stack.len() == 1);
    let top = stack.pop().unwrap();
    assert!(top.len() == 1);
    top[0].clone()
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[test]
fn test_parse() {
    let input = include_str!("../input").lines();
    for line in input {
        if line.is_empty() {
            continue;
        }
        let parsed = parse(line);
        let unparsed = format!("{}", parsed);
        assert_eq!(line, unparsed);
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn wrap_in_list(v: &Value) -> Value {
            Value::List(vec![v.clone()])
        }

        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    let cmp = a.cmp(b);
                    if cmp != std::cmp::Ordering::Equal {
                        return cmp;
                    }
                }
                a.len().cmp(&b.len())
            }
            (Value::Int(_), Value::List(_)) => {
                wrap_in_list(self).cmp(other)
            }
            (Value::List(_), Value::Int(_)) => {
                self.cmp(&wrap_in_list(other))
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Value { }


fn main() {
    let mut values: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.is_empty())
        .map(|x| parse(&x))
        .collect();

    let key1: Value = parse("[[2]]");
    let key2: Value = parse("[[6]]");

    values.push(key1.clone());
    values.push(key2.clone());

    values.sort();

    let key1_index = values.iter().position(|x| x == &key1).unwrap() as i32;
    let key2_index = values.iter().position(|x| x == &key2).unwrap() as i32;

    let answer = (key1_index + 1) * (key2_index + 1);
    println!("{}", answer);
}
