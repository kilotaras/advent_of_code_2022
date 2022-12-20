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
    let mut cycles: i32 = 1;
    let mut state: i32 = 1;

    let is_interesting = |cycles: i32| {
        (cycles - 20)%40 == 0
    };

    let mut total = 0;

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let operation = line.parse::<Operation>().unwrap();
        let prev_state = state;
        (cycles, state) = match operation {
            Operation::NoOp => (cycles + 1, state),
            Operation::AddX(value) => (cycles + 2, state + value),
        };

        if is_interesting(cycles) {
            total += cycles * state;
        }

        if let Operation::AddX(_) = operation {
            if is_interesting(cycles - 1) {
                total += (cycles - 1) * prev_state;
            }
        }
    }

    println!("{}", total);
}
