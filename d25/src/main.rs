#[derive(Debug, Clone, PartialEq, Eq)]
struct SNAFU {
    values: Vec<i32>,
}

impl From<i64> for SNAFU {
    fn from(input: i64) -> Self {
        fn step_once(val: i64) -> (i64, i32) {
            let (div, rem) = (val / 5, val % 5);
            if rem > 2 {
                (div + 1, (rem - 5) as i32)
            } else {
                (div, rem as i32)
            }
        }
        let mut values = Vec::new();

        let mut input = input;

        while input > 0 {
            let (new_input, last) = step_once(input);
            values.push(last);
            input = new_input;
        }

        SNAFU { values }
    }
}

impl From<i32> for SNAFU {
    fn from(input: i32) -> Self {
        SNAFU::from(input as i64)
    }
}

impl From<&SNAFU> for i64 {
    fn from(v: &SNAFU) -> Self {
        let mut result: i64 = 0;
        let mut multiplier: i64 = 1;
        for value in v.values.iter() {
            result += (*value as i64) * multiplier;
            multiplier *= 5;
        }
        result
    }
}

impl From<SNAFU> for i64 {
    fn from(v: SNAFU) -> Self {
        i64::from(&v)
    }
}

impl std::str::FromStr for SNAFU {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::new();
        for c in s.chars() {
            let value = match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => return Err(format!("Invalid character: {}", c)),
            };
            values.push(value);
        }

        values.reverse();

        Ok(SNAFU { values })
    }
}

// implements formatter for SNAFU
impl std::fmt::Display for SNAFU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for value in self.values.iter().rev() {
            let c: char = match value {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => unreachable!(),
            };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_transform_samples() -> Vec<(&'static str, &'static str)> {
        include_str!("../sample_transforms.txt")
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let snafu_str = parts.next().unwrap();
                let input_str = parts.next().unwrap();
                (snafu_str, input_str)
            })
            .collect()
    }

    #[test]
    fn test_parse_and_transform() {
        for (snafu_str, input_str) in get_transform_samples() {
            let snafu = snafu_str.parse::<SNAFU>().unwrap();
            let input_i = input_str.parse::<i32>().unwrap();

            let snafu_to_i = i64::from(snafu.clone()) as i32;
            assert_eq!(
                snafu_to_i, input_i,
                "SNAFU {} should be {}, got {}",
                snafu_str, input_i, snafu_to_i
            );
        }
    }

    #[test]
    fn test_parse_and_print() {
        for (snafu_str, _) in get_transform_samples() {
            let snafu = snafu_str.parse::<SNAFU>().unwrap();
            let snafu_str2 = snafu.to_string();
            assert_eq!(snafu_str, snafu_str2, "SNAFU {} fot parsed to {}", snafu_str, snafu_str2);
        }
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_roundtrip_from_i(input in 0i32..) {
            let snafu = SNAFU::from(input);
            let back_to_i = i64::from(snafu) as i32;
            assert_eq!(back_to_i, input);
        }
    }
}

fn get_sample() -> Vec<SNAFU> {
    include_str!("../sample.txt")
        .lines()
        .map(|line| line.parse::<SNAFU>().unwrap())
        .collect()
}

fn get_input() -> Vec<SNAFU> {
    include_str!("../input")
        .lines()
        .map(|line| line.parse::<SNAFU>().unwrap())
        .collect()
}


fn solve_part1(input: &[SNAFU]) -> i64 {
    let mut result = 0;
    for snafu in input {
        let i = i64::from(snafu);
        result += i;
    }
    result
}

#[test]
fn test_sample() {
    let input = get_sample();
    let result = solve_part1(&input);
    assert_eq!(result, 4890);
}


fn main() {
    let input = if std::env::args().count() > 1 {
        get_sample()
    } else {
        get_input()
    };

    for v in &input {
        println!("{}", v);
    }

    let result = solve_part1(&input);
    let snafu_result = SNAFU::from(result);
    println!("P1: {} => {}", result, snafu_result);
}
