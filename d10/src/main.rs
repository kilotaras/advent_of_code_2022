use std::io::BufRead;

#[cfg(test)]
use proptest_derive::Arbitrary;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(Arbitrary))]
enum Operation {
    NoOp,
    AddX(i32),
}

// implements parse for Operation
impl std::str::FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Operation::NoOp)
        } else if s.starts_with("addx") {
            let value = s[5..].parse().unwrap();
            Ok(Operation::AddX(value))
        } else {
            Err(())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_parse(operation: Operation) {
            let s = match operation {
                Operation::NoOp => "noop".to_string(),
                Operation::AddX(value) => format!("addx {}", value),
            };
            assert_eq!(s.parse::<Operation>().unwrap(), operation);
        }
    }
}

fn main() {
    let mut state: i32 = 1;
    let mut total = 0;

    let mut cycles_to_state = Vec::new();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let operation = line.parse::<Operation>().unwrap();
        cycles_to_state.push(state);
        if let Operation::AddX(value) = operation {
            cycles_to_state.push(state);
            state += value;
        }
    }

    for (i, state) in cycles_to_state.iter().enumerate().skip(19).step_by(40) {
        println!("{}: {}", i + 1, state);
        total += (i as i32 + 1) * state
    }

    let mut output = String::new();

    for (i, state) in cycles_to_state.iter().enumerate() {
        let row_position = (i % 40) as i32;

        if (row_position - state).abs() <= 1 {
            output.push('#');
        } else {
            output.push('.');
        }

        if row_position == 39 {
            output.push('\n')
        }
    }

    println!("{}", total);
    println!("{}", output);
}
