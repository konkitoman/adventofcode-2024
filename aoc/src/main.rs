use aoc::Solution;

struct Day1;
impl Solution for Day1 {}
struct Day2;
impl Solution for Day2 {}
struct Day3;
impl Solution for Day3 {}
struct Day4;
impl Solution for Day4 {}

mod day5;
use day5::Day5;

mod day6;
use day6::Day6;

mod day7;
use day7::Day7;

fn main() {
    let mut args = std::env::args();
    _ = args.next().unwrap(); // Path
    let day = args.next().expect("No day specified!");
    let part = args.next().expect("No part specified!");

    let day = day.parse::<usize>().expect("Day needs to be a number!");
    let part = part.parse::<usize>().expect("Part needs to be a number!");

    let solutions: &[&dyn Solution] = &[&Day1, &Day2, &Day3, &Day4, &Day5, &Day6, &Day7];

    let input = std::io::read_to_string(std::io::stdin()).expect("Cannot read input");

    let solution = solutions[day - 1];
    match part {
        1 => println!("{}", solution.part1(&input)),
        2 => println!("{}", solution.part2(&input)),

        _ => {
            eprintln!("Invalid part!");
            std::process::exit(1)
        }
    }
}
