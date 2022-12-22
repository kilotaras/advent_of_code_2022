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
    ore: i16,
    ore_robots: i8,
    clay: i16,
    clay_robots: i8,
    obsidian: i16,
    obsidian_robots: i8,
}

fn div_up(a: i16, b: i16) -> i16 {
    a / b + if a % b == 0 { 0 } else { 1 }
}

fn time_to_ore_amount(state: &State, amount: i32) -> i16 {
    let amount = amount as i16;
    if state.ore >= amount {
        return 0;
    }

    let needed = amount - state.ore;
    div_up(needed, state.ore_robots as i16)
}

fn get_time_to_ore(state: &State, blueprint: &BluePrint) -> i16 {
    time_to_ore_amount(state, blueprint.ore_for_ore)
}

fn get_time_to_clay(state: &State, blueprint: &BluePrint) -> i16 {
    time_to_ore_amount(state, blueprint.ore_for_clay)
}

fn get_time_to_obsidian(state: &State, blueprint: &BluePrint) -> i16 {
    if state.clay_robots == 0 {
        return 100;
    }

    let time_for_ore = time_to_ore_amount(state, blueprint.ore_for_obsidian);

    if state.clay >= (blueprint.clay_for_obsidian as i16){
        return time_for_ore;
    }

    let needed = (blueprint.clay_for_obsidian as i16) - state.clay;
    let time_for_clay = div_up(needed, state.clay_robots as i16);

    std::cmp::max(time_for_ore, time_for_clay)
}

fn get_time_to_geode(state: &State, blueprint: &BluePrint) -> i16 {
    if state.obsidian_robots == 0 {
        return 100;
    }

    let time_for_ore = time_to_ore_amount(state, blueprint.ore_for_geode);

    if state.obsidian >= (blueprint.obsidian_for_geode as i16) {
        return time_for_ore;
    }

    let needed = (blueprint.obsidian_for_geode as i16) - state.obsidian;
    let time_for_obsidian = div_up(needed, state.obsidian_robots.into());

    std::cmp::max(time_for_ore, time_for_obsidian)
}

fn advance_state_in_time(state: &State, time: i16) -> State {
    State {
        ore: state.ore + time * (state.ore_robots as i16),
        clay: state.clay + time * (state.clay_robots as i16),
        obsidian: state.obsidian + time * (state.obsidian_robots as i16),
        ..*state
    }
}

const initial_state: State = State {
    ore: 0,
    ore_robots: 1,
    clay: 0,
    clay_robots: 0,
    obsidian: 0,
    obsidian_robots: 0,
};

fn get_answer(blueprint: &BluePrint, time: usize) -> i16 {
    let time = time;
    let mut states: Vec<HashMap<State, i16>> = vec![HashMap::new(); time + 1];
    let mut prev_pos = vec![vec![]; time + 1];
    states[0].insert(initial_state, 0);
    prev_pos[0].push((0, 0));
    let mut best = 0;

    let max_ore_amount = *([blueprint.ore_for_ore, blueprint.ore_for_clay, blueprint.ore_for_obsidian, blueprint.ore_for_geode].iter().max().unwrap()) as i16;

    for curent_t in 0..time {
        if curent_t > 20 {
            println!("{}: {}", curent_t, states[curent_t].len());
        }
        let (left, next_states) = states.split_at_mut(curent_t + 1);
        let current_states = left.last().unwrap();

        let time_left = time - curent_t;
        let max_ore_amount = max_ore_amount * (time_left as i16);
        let max_clay_amount = blueprint.clay_for_obsidian as i16 * (time_left as i16);
        let max_obsidian_amount = blueprint.obsidian_for_geode as i16 * (time_left as i16);

        let max_geodes_possible = {
            let itime = time_left as i16;
            itime*(itime-1)/2
        };

        let min_geodes = best - max_geodes_possible;

        for (id, (state, geodes)) in current_states.iter().enumerate() {
            if *geodes < min_geodes {
                continue;
            }
            if state.ore <= max_ore_amount {
                let time_to_ore = get_time_to_ore(state, blueprint) + 1;
                let nt = curent_t + (time_to_ore as usize);

                if nt <= time {
                    let mut new_state = advance_state_in_time(state, time_to_ore);
                    new_state.ore -= blueprint.ore_for_ore as i16;
                    new_state.ore_robots += 1;
                    let current = next_states[time_to_ore as usize - 1].entry(new_state).or_insert(*geodes);
                    *current = std::cmp::max(*current, *geodes);
                }
            }

            if state.clay <= max_clay_amount {
                let time_to_clay = get_time_to_clay(state, blueprint) + 1;
                let nt = curent_t + (time_to_clay as usize);

                if nt <= time {
                    let mut new_state = advance_state_in_time(state, time_to_clay);
                    new_state.ore -= blueprint.ore_for_clay as i16;
                    new_state.clay_robots += 1;
                    let current = next_states[time_to_clay as usize - 1].entry(new_state).or_insert(*geodes);
                    *current = std::cmp::max(*current, *geodes);
                    // prev_pos[nt].push((curent_t, id));
                }
            }

            if state.obsidian <= max_obsidian_amount {
                let time_to_obsidian = get_time_to_obsidian(state, blueprint) + 1;
                let nt = curent_t + (time_to_obsidian as usize);

                if nt <= time {
                    let mut new_state = advance_state_in_time(state, time_to_obsidian);
                    new_state.ore -= blueprint.ore_for_obsidian as i16;
                    new_state.clay -= blueprint.clay_for_obsidian as i16;
                    new_state.obsidian_robots += 1;

                    let current = next_states[time_to_obsidian as usize - 1].entry(new_state).or_insert(*geodes);
                    *current = std::cmp::max(*current, *geodes);
                    // prev_pos[nt].push((curent_t, id));
                }
            }

            let time_to_geode = get_time_to_geode(state, blueprint) + 1;
            let nt = curent_t + (time_to_geode as usize);

            if nt <= time {
                let mut new_state = advance_state_in_time(state, time_to_geode);
                new_state.ore -= blueprint.ore_for_geode as i16;
                new_state.obsidian -= blueprint.obsidian_for_geode as i16;

                let time_left = time - nt;

                // println!("{}->{}: +{}\n{:?}", curent_t, nt, time_left, state);

                let geodes = geodes + time_left as i16;

                best = std::cmp::max(best, geodes);
                let current = next_states[time_to_geode as usize - 1].entry(new_state).or_insert(geodes);
                *current = std::cmp::max(*current, geodes);
                // prev_pos[nt].push((curent_t, id));
            }
        }

        left.last_mut().unwrap().clear();
    }

    best
}

fn main() {
    let blueprints: Vec<BluePrint> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| parse_blueprint(&line.unwrap()))
        .collect();

    let p1: i32 = blueprints.iter()
        .enumerate()
        .map(|(id, blueprint)| {
            let answer = get_answer(blueprint, 24) as i32;
            println!("{}: {}", id, answer);
            (id + 1) as i32 * answer
        })
        .sum();

    let p2: i32 = blueprints.iter()
        .take(3)
        .enumerate()
        .map(|(id, blueprint)| {
            let answer = get_answer(blueprint, 32) as i32;
            println!("{}: {}", id, answer);
            answer
        })
        .product();

    println!("P1: {}", p1);
    println!("P2: {}", p2);

    // for (id, blueprint) in blueprints.iter().take(3).enumerate() {
    //    let best = get_answer(blueprint, time);

    //     println!("{}: {}", id, best);
    //     answer += (id + 1) as i32 * (best as i32);
    // }

    // println!("{}", answer);
}
