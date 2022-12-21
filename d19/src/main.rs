use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
struct BluePrint {
    ore_for_ore: i32,
    ore_for_clay: i32,
    ore_for_obsidian: i32,
    clay_for_obsidian: i32,
    ore_for_geode: i32,
    obsidian_for_geode: i32,
}

fn parse_blueprint(s: &str) -> BluePrint {
    let numbers: Vec<i32> = s
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .filter(Result::is_ok)
        .map(|r| r.unwrap())
        .collect();

    BluePrint {
        ore_for_ore: numbers[0],
        ore_for_clay: numbers[1],
        ore_for_obsidian: numbers[2],
        clay_for_obsidian: numbers[3],
        ore_for_geode: numbers[4],
        obsidian_for_geode: numbers[5],
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    ore: i32,
    ore_robots: i32,
    clay: i32,
    clay_robots: i32,
    obsidian: i32,
    obsidian_robots: i32,
}

fn brute_force(blueprint: &BluePrint, mem: &mut HashMap<(i32, State), i32>,time_left: i32, state: State) -> i32 {
    if time_left == 0 {
        return 0;
    }

    let time_left = time_left - 1;

    if mem.contains_key(&(time_left, state)) {
        return mem[&(time_left, state)];
    }

    if time_left == 18 {
        println!("!")
    }

    let new_state = State {
        ore: state.ore + state.ore_robots,
        clay: state.clay + state.clay_robots,
        obsidian: state.obsidian + state.obsidian_robots,
        ..state
    };

    let mut best = 0;

    let mut can = 0;

    if state.ore >= blueprint.ore_for_ore {
        can += 1;
        let new_state = State {
            ore: new_state.ore - blueprint.ore_for_ore,
            ore_robots: new_state.ore_robots + 1,
            ..new_state
        };

        let attempt = brute_force(blueprint, mem, time_left, new_state);
        if attempt > best {
            best = attempt;
        }
    }

    if state.ore >= blueprint.ore_for_clay {
        can += 1;
        let new_state = State {
            ore: new_state.ore - blueprint.ore_for_clay,
            clay_robots: new_state.clay_robots + 1,
            ..new_state
        };

        let attempt = brute_force(blueprint, mem, time_left, new_state);
        if attempt > best {
            best = attempt;
        }
    }

    if state.ore >= blueprint.ore_for_obsidian && state.clay >= blueprint.clay_for_obsidian {
        can += 1;
        let new_state = State {
            ore: new_state.ore - blueprint.ore_for_obsidian,
            clay: new_state.clay - blueprint.clay_for_obsidian,
            obsidian_robots: new_state.obsidian_robots + 1,
            ..new_state
        };

        let attempt = brute_force(blueprint, mem, time_left, new_state);
        if attempt > best {
            best = attempt;
        }
    }

    if state.ore >= blueprint.ore_for_geode && state.obsidian >= blueprint.obsidian_for_geode {
        can += 1;
        let new_state = State {
            ore: new_state.ore - blueprint.ore_for_geode,
            obsidian: new_state.obsidian - blueprint.obsidian_for_geode,
            ..new_state
        };

        let attempt = brute_force(blueprint, mem, time_left, new_state) + time_left;
        if attempt > best {
            best = attempt;
        }
    }

    if can != 4 {
        // do nothing
        let attempt = brute_force(blueprint, mem, time_left, new_state);
        if attempt > best {
            best = attempt;
        }
    }

    mem.insert((time_left, state), best);
    best
}

fn main() {
    let blueprints: Vec<BluePrint> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| parse_blueprint(&line.unwrap()))
        .collect();

    let initital_state = State {
        ore: 0,
        ore_robots: 1,
        clay: 0,
        clay_robots: 0,
        obsidian: 0,
        obsidian_robots: 0,
    };

    let time = 24;
    let mut answer = 0;
    for (id, blueprint) in blueprints.iter().enumerate() {
        let mut mem = HashMap::new();
        let attempt = brute_force(blueprint, &mut mem, time, initital_state);
        println!("{}", &attempt);
        answer += attempt*((id+1) as i32);
    }
    println!("{}", answer);
}
