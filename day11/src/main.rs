/**
Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!

By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).

The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:

If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
Otherwise, the seat's state does not change.
Floor (.) never changes; seats don't move, and nobody sits on the floor.

After one round of these rules, every seat in the example layout becomes occupied:

#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
After a second round, the seats with four or more occupied adjacent seats become empty again:

#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
This process continues for three more rounds:

#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.

Simulate your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?

--- Part Two ---
As soon as people start to arrive, you realize your mistake. People don't just care about adjacent seats - they care about the first seat they can see in each of those eight directions!

Now, instead of considering just the eight immediately adjacent seats, consider the first seat in each of those eight directions. For example, the empty seat below would see eight occupied seats:

.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....
The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:

.............
.L.L.#.#.#.#.
.............
The empty seat below would see no occupied seats:

.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.
Also, people seem to be more tolerant than you expected: it now takes five or more visible occupied seats for an occupied seat to become empty (rather than four or more from the previous rules). The other rules still apply: empty seats that see no occupied seats become occupied, seats matching no rule don't change, and floor never changes.

Given the same starting layout as above, these new rules cause the seating area to shift around as follows:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#
#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once this occurs, you count 26 occupied seats.

Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied?



*/
use common::Result;
use std::io::{self, Read};

#[derive(Copy, Clone, PartialEq)]
enum Seats {
    FLOOR,
    FREE,
    OCCUPIED,
}

type SeatsMap = Vec<Vec<Seats>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut mappa = parse_map(input);
    //print_map(&mappa);
    let mut prev = count_all_occupied(&mappa);
    mappa = evolve(&mappa);
    //print_map(&mappa);
    let mut current = count_all_occupied(&mappa);
    while current != prev {
        prev = current;
        mappa = evolve(&mappa);
        //print_map(&mappa);
        current = count_all_occupied(&mappa);
    }
    println!("Part 1: {}", current);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut mappa = parse_map(input);
    //print_map(&mappa);
    let mut prev = count_all_occupied(&mappa);
    mappa = evolve2(&mappa);
    //print_map(&mappa);
    let mut current = count_all_occupied(&mappa);
    while current != prev {
        prev = current;
        mappa = evolve2(&mappa);
        //print_map(&mappa);
        current = count_all_occupied(&mappa);
    }
    println!("Part 2: {}", current);
    Ok(())
}

fn count_all_occupied(input: &SeatsMap) -> usize {
    return input
        .iter()
        .map(|row| row.iter().filter(|&&val| val == Seats::OCCUPIED).count())
        .sum();
}

fn parse_map(input: &str) -> SeatsMap {
    let rows = input.lines().count();
    let column = input.lines().next().unwrap().chars().count();
    let mut result: SeatsMap = vec![vec![Seats::FLOOR; column]; rows];
    for (r, row) in input.lines().enumerate() {
        for (c, value) in row.chars().enumerate() {
            match value {
                '.' => (),
                'L' => result[r][c] = Seats::FREE,
                _ => panic!("Wrong Format"),
            }
        }
    }
    result
}

fn evolve(input: &SeatsMap) -> SeatsMap {
    let rows = input.iter().count();
    let columns = input.iter().next().unwrap().iter().count();
    let mut result: SeatsMap = vec![vec![Seats::FLOOR; columns]; rows];
    for (r, row) in input.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            if input[r][c] == Seats::FLOOR {
                result[r][c] = Seats::FLOOR;
                continue;
            }
            let occupied = count_around(&input, r, c);
            if input[r][c] == Seats::FREE && occupied == 0 {
                result[r][c] = Seats::OCCUPIED;
            } else if input[r][c] == Seats::OCCUPIED && occupied >= 4 {
                result[r][c] = Seats::FREE;
            } else {
                result[r][c] = input[r][c];
            }
        }
    }
    result
}

fn evolve2(input: &SeatsMap) -> SeatsMap {
    let rows = input.iter().count();
    let columns = input.iter().next().unwrap().iter().count();
    let mut result: SeatsMap = vec![vec![Seats::FLOOR; columns]; rows];
    for (r, row) in input.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            if input[r][c] == Seats::FLOOR {
                result[r][c] = Seats::FLOOR;
                continue;
            }
            let occupied = count_in_sight(&input, r, c);
            if input[r][c] == Seats::FREE && occupied == 0 {
                result[r][c] = Seats::OCCUPIED;
            } else if input[r][c] == Seats::OCCUPIED && occupied >= 5 {
                result[r][c] = Seats::FREE;
            } else {
                result[r][c] = input[r][c];
            }
        }
    }
    result
}

fn count_around(input: &SeatsMap, r: usize, c: usize) -> usize {
    let start_row = if r == 0 { 0 } else { r - 1 };
    let start_col = if c == 0 { 0 } else { c - 1 };
    //println!("R C = {} {}", start_row, start_col);
    let mut result = 0;
    for i in start_row..=r + 1 {
        for j in start_col..=c + 1 {
            if i != r || j != c {
                //print!("++ {}/{}", i, j);
                if i < input.len() && j < input[i].len() {
                    //print!("-- {}/{}", i, j);
                    if input[i][j] == Seats::OCCUPIED {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn count_in_sight(input: &SeatsMap, r: usize, c: usize) -> usize {
    let directions: Vec<(i32, i32)> = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    let mut result = 0;

    for (dr, dc) in directions {
        let mut i = r as i32 + dr;
        let mut j = c as i32 + dc;
        while i >= 0 && j >= 0 && i < input.len() as i32 && j < input[i as usize].len() as i32 {
            if input[i as usize][j as usize] == Seats::OCCUPIED {
                result += 1;
                break;
            } else if input[i as usize][j as usize] == Seats::FREE {
                break;
            }
            i += dr;
            j += dc;
        }
    }
    result
}

// fn print_map(input: &SeatsMap) {
//     for row in input.iter() {
//         for value in row.iter() {
//             match value {
//                 Seats::FLOOR => print!("."),
//                 Seats::FREE => print!("L"),
//                 Seats::OCCUPIED => print!("#"),
//             }
//         }
//         println!("");
//     }
// }
