use std::io::BufRead;

struct Operation {
    left: Box<dyn Fn(i32) -> i32>,
    right: Box<dyn Fn(i32) -> i32>,
    op: Box<dyn Fn(i32, i32) -> i32>,
}

impl Operation {
    fn apply(&self, cur: i32) -> i32 {
        let left = (self.left)(cur);
        let right = (self.right)(cur);
        (self.op)(left, right)
    }
}

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    picker: Box<dyn Fn(i32) -> usize>,
    inspected: i32,
}

fn parse_picker(test_line: &str, true_line: &str, false_line: &str) -> Box<dyn Fn(i32) -> usize> {
    fn pick_last_number(s: &str) -> i32 {
        let mut parts = s.split_whitespace();
        parts.next_back().unwrap().parse().unwrap()
    }
    let modulo = pick_last_number(test_line);
    let if_true = pick_last_number(true_line) as usize;
    let if_false = pick_last_number(false_line) as usize;

    Box::new(move |x| {
        if x % modulo == 0 {
            if_true as usize
        } else {
            if_false as usize
        }
    })
}

fn parse_items(s: &str) -> Vec<i32> {
    s.split(",").map(|x| x.trim().parse().unwrap()).collect()
}

impl std::str::FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        parts.next(); // skip new
        parts.next(); // skip =

        let left = parts.next().ok_or("missing left")?;
        let op = parts.next().ok_or("missing op")?;
        let right = parts.next().ok_or("missing right")?;

        fn make_picker_from_operand(operand: &str) -> Box<dyn Fn(i32) -> i32> {
            match operand {
                "old" => Box::new(|x| x),
                _ => {
                    let val = operand.parse().unwrap();
                    Box::new(move |_| val)
                }
            }
        }

        let left = make_picker_from_operand(left);
        let right = make_picker_from_operand(right);

        let op = match op {
            "+" => |x, y| x + y,
            "-" => |x, y| x - y,
            "*" => |x, y| x * y,
            "/" => |x, y| x / y,
            _ => return Err(format!("unknown op: {}", op)),
        };

        Ok(Operation {
            left,
            right,
            op: Box::new(op),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let op = Operation {
            left: Box::new(|x| x + 1),
            right: Box::new(|x| x * 2),
            op: Box::new(|x, y| x - y),
        };
        assert_eq!(op.apply(1), 0);

        let op = Operation {
            left: Box::new(|x| x),
            right: Box::new(|_| 5),
            op: Box::new(|x, y| x + y),
        };
        assert_eq!(op.apply(1), 6);
    }

    #[test]
    fn test_parse_operation() {
        let operation = "new = old + 1".parse::<Operation>().unwrap();
        assert_eq!(operation.apply(1), 2);

        let operation = "new = old * 5".parse::<Operation>().unwrap();
        assert_eq!(operation.apply(2), 10);

        let operation = "new = old * old".parse::<Operation>().unwrap();
        assert_eq!(operation.apply(3), 9);
    }

    #[test]
    fn test_parse_picker() {
        let picker = parse_picker(
            "Test: divisible by 13",
            "If true: throw to monkey 5",
            "If false: throw to monkey 7",
        );
        assert_eq!(picker(3), 7);
        assert_eq!(picker(8), 7);
        assert_eq!(picker(26), 5);
    }

    #[test]
    fn test_parse_items() {
        let items = parse_items("1, 3, 4, 5");
        assert_eq!(items, vec![1, 3, 4, 5]);
    }
}

fn parse_monkey(monkey_input: &[String]) -> Monkey {
    let items = parse_items(&monkey_input[1][18..]);
    let operation: Operation = monkey_input[2][13..].parse().unwrap();
    let picker = parse_picker(&monkey_input[3], &monkey_input[4], &monkey_input[5]);
    Monkey {
        items,
        operation,
        picker,
        inspected: 0
    }
}

fn dump(monkeys: &[Monkey]) {
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("{}: {:?}", i, monkey.items);
    }
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut monkeys = lines
        .split(|x| x.is_empty())
        .map(parse_monkey)
        .collect::<Vec<_>>();


    let rounds = 20;
    for _ in 0..rounds {
        for cur_monkey_index in 0..monkeys.len() {
            let cur_monkey = &mut monkeys[cur_monkey_index];

            let new_positions = cur_monkey.items
                .iter()
                .map(|x| cur_monkey.operation.apply(*x))
                .map(|x| x / 3)
                .map(|x| ((cur_monkey.picker)(x), x))
                .collect::<Vec<_>>();

            cur_monkey.inspected += cur_monkey.items.len() as i32;
            cur_monkey.items.clear();

            for (monkey_index, item) in new_positions {
                monkeys[monkey_index].items.push(item);
            }
        }
    }

    let mut inspected = monkeys.iter()
        .map(|x| x.inspected)
        .collect::<Vec<_>>();

    inspected.sort();
    inspected.reverse();

    let answer = inspected[0] * inspected[1];

    println!("{}", answer);
}
