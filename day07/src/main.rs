/**
--- Day 7: Handy Haversacks ---
You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!

For example, consider the following rules:

light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

A bright white bag, which can hold your shiny gold bag directly.
A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)

--- Part Two ---
It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!

Consider again your shiny gold bag and the rules from the above example:

faded blue bags contain 0 other bags.
dotted black bags contain 0 other bags.
vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!

Here's another example:

shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
In this example, a single shiny gold bag must contain 126 other bags.

How many individual bags are required inside your single shiny gold bag?

*/
use common::Result;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let ruleset = parse_rules(input);
    let result: usize = ruleset
        .keys()
        .map(|x| if contains_gold(x, &ruleset) { 1 } else { 0 })
        .sum();
    println!("Part 1: {}", result);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let ruleset = parse_rules(input);
    println!("Part 2: {}", count_bags("shiny gold", &ruleset));
    Ok(())
}

fn contains_gold(bag: &str, ruleset: &HashMap<String, Vec<(usize, String)>>) -> bool {
    if ruleset[bag].len() == 0 {
        false
    } else if ruleset[bag]
        .iter()
        .any(|(_, y)| y == &"shiny gold".to_owned())
    {
        true
    } else {
        ruleset[bag].iter().any(|x| contains_gold(&x.1, ruleset))
    }
}

fn count_bags(bag: &str, ruleset: &HashMap<String, Vec<(usize, String)>>) -> usize {
    if ruleset[bag].len() == 0 {
        0
    } else {
        ruleset[bag]
            .iter()
            .map(|x| x.0 * (1 + count_bags(&x.1, ruleset)))
            .sum()
    }
}

fn parse_rules(input: &str) -> HashMap<String, Vec<(usize, String)>> {
    let mut result = HashMap::new();
    let re = Regex::new(
        r"^([a-z]*\s[a-z]*)\sbags\scontain\s((?:no\sother\sbags\.)|(?:(?:\d)\s(?:[a-z]*\s[a-z]*)\sbag[s]*.\s*)+)",
    ).unwrap();
    let re2 = Regex::new(r"(\d)\s([a-z]*\s[a-z]*)\sbag[s]*.\s*").unwrap();
    for rule in input.lines() {
        let captures = re.captures(rule).unwrap();
        let key = captures[1].to_owned();
        if &captures[2] == "no other bags." {
            result.insert(key, vec![]);
        } else {
            let values = re2
                .captures_iter(&captures[2])
                .map(|x| (x[1].parse().unwrap(), x[2].to_owned()))
                .collect();
            result.insert(key, values);
        }
    }
    return result;
}
