/**
--- Day 2: Password Philosophy ---
Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

```
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
```

Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?

--- Part Two ---
While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

1-3 a: abcde is valid: position 1 contains a and position 3 does not.
1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

How many passwords are valid according to the new interpr

 */
use common::Result;
use std::io::{self, Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut result = 0;
    for password_line in input.lines() {
        let elements = password_line.split(" ").collect::<Vec<&str>>();
        let rule = elements[0].split("-").collect::<Vec<&str>>();
        let min = rule[0].parse::<usize>().unwrap();
        let max = rule[1].parse::<usize>().unwrap();
        let character = elements[1].chars().next().unwrap();
        let password = elements[2];
        let count_char = password.chars().filter(|x| x == &character).count();
        result += if count_char >= min && count_char <= max {
            1
        } else {
            0
        }
    }
    println!("Part 1: {}", result);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut result = 0;
    for password_line in input.lines() {
        let elements = password_line.split(" ").collect::<Vec<&str>>();
        let rule = elements[0].split("-").collect::<Vec<&str>>();
        let idx1 = rule[0].parse::<usize>().unwrap();
        let idx2 = rule[1].parse::<usize>().unwrap();
        let character = elements[1].chars().next().unwrap();
        let password_chars = elements[2].chars().collect::<Vec<char>>();
        if (password_chars[idx1 - 1] != character && password_chars[idx2 - 1] == character)
            || (password_chars[idx1 - 1] == character && password_chars[idx2 - 1] != character)
        {
            result += 1;
        }
    }
    println!("Part 2: {}", result);
    Ok(())
}
