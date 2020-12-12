/**
--- Day 12: Rain Risk ---
Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!

Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.

The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:

Action N means to move north by the given value.
Action S means to move south by the given value.
Action E means to move east by the given value.
Action W means to move west by the given value.
Action L means to turn left the given number of degrees.
Action R means to turn right the given number of degrees.
Action F means to move forward by the given value in the direction the ship is currently facing.
The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)

For example:

F10
N3
F7
R90
F11
These instructions would be handled as follows:

F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
N3 would move the ship 3 units north to east 10, north 3.
F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
F11 would move the ship 11 units south to east 17, south 8.
At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.

Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?

--- Part Two ---
Before you can give the destination to the captain, you realize that the actual action meanings were printed on the back of the instructions the whole time.

Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:

Action N means to move the waypoint north by the given value.
Action S means to move the waypoint south by the given value.
Action E means to move the waypoint east by the given value.
Action W means to move the waypoint west by the given value.
Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
Action F means to move forward to the waypoint a number of times equal to the given value.
The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.

For example, using the same instructions as above:

F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.
After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.

Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?

*/
use common::Result;
use std::io::{self, Read};

// (x, y)
type State = (i32, i32);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut state = (0, 0);
    let mut direction = 1;
    // Direction = N,0 E,1, S,2 W,3
    for command in input.lines() {
        let result = step(state, direction, command);
        state = result.1;
        direction = result.0;
        //println!("{:?}", state);
    }
    println!("Part 1: {}", state.0.abs() + state.1.abs());
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut state = (0, 0);
    let mut waypoint = (10, 1);
    for command in input.lines() {
        let result = step2(state, waypoint, command);
        state = result.1;
        waypoint = result.0;
        //println!("{} -> {:?} - W: {:?}", command, state, waypoint);
    }
    println!("Part 1: {}", state.0.abs() + state.1.abs());
    Ok(())
}

fn step(state: State, direction: i32, command_str: &str) -> (i32, State) {
    let (x, y) = state;
    let d = direction;
    let mut forward = (0, 1);
    for _ in 0..d {
        forward = (forward.1, -forward.0);
    }
    let command = command_str.chars().next().unwrap();
    let value: i32 = command_str
        .chars()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    match command {
        'L' => ((d + (3 * (value / 90)) % 4), (x, y)),
        'R' => ((d + (1 * (value / 90)) % 4), (x, y)),
        'N' => (d, (x, y + value)),
        'S' => (d, (x, y - value)),
        'E' => (d, (x + value, y)),
        'W' => (d, (x - value, y)),
        'F' => (d, (x + value * forward.0, y + value * forward.1)),
        _ => panic!("Invalid Command"),
    }
}

fn step2(state: State, waypoint: State, command_str: &str) -> (State, State) {
    let (x, y) = state;
    let (wx, wy) = waypoint;

    let command = command_str.chars().next().unwrap();
    let value: i32 = command_str
        .chars()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    match command {
        'L' => {
            let mut new_waypoint = (wx, wy);
            for _ in 0..(value / 90) {
                new_waypoint = (-new_waypoint.1, new_waypoint.0)
            }
            (new_waypoint, (x, y))
        }
        'R' => {
            let mut new_waypoint = (wx, wy);
            for _ in 0..(value / 90) {
                new_waypoint = (new_waypoint.1, -new_waypoint.0)
            }
            (new_waypoint, (x, y))
        }
        'N' => ((wx, wy + value), state),
        'S' => ((wx, wy - value), state),
        'E' => ((wx + value, wy), state),
        'W' => ((wx - value, wy), state),
        'F' => (waypoint, (x + value * wx, y + value * wy)),
        _ => panic!("Invalid Command"),
    }
}
