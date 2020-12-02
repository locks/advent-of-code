use std::str::FromStr;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use itertools::Itertools;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| i32::from_str(&l).expect("Could not convert to number"))
        .collect()
}

fn day1part1() {
    let twenty_twenty = 2020;
    let haystack = lines_from_file("src/01.txt");

    for straw in &haystack {
        let needle = twenty_twenty - straw;
        if haystack.contains(&needle) {
            println!("day 01 part 1: {}", needle * straw);
            return;
        }
    }
}

fn day1part2() {
    let twenty_twenty = 2020;
    let haystack = lines_from_file("src/01.txt");

    let result = haystack.into_iter().combinations(3).find(|comb| twenty_twenty == comb.get(0).unwrap() + comb.get(1).unwrap() + comb.get(2).unwrap() ).map(|comb| comb.get(0).unwrap() * comb.get(1).unwrap() * comb.get(2).unwrap()).unwrap();

    println!("day 01 part 1: {:#?}", result);
}

fn main() {
    day1part2();
}
