use aoc::Solution;

pub struct Day5;

impl Solution for Day5 {
    fn part1(&self, input: &str) -> isize {
        let mut i = input.split("\n\n");
        let ordering_rules = i
            .next()
            .unwrap()
            .split('\n')
            .map(|x| {
                x.split('|')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let updates = i
            .next()
            .unwrap()
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let valids = updates
            .iter()
            .filter(|update| {
                let rules = ordering_rules
                    .iter()
                    .filter(|rules| rules.iter().all(|num| update.contains(num)))
                    .collect::<Vec<_>>();
                rules.iter().all(|rule| {
                    let f = update.iter().position(|x| *x == rule[0]).unwrap();
                    let s = update.iter().position(|x| *x == rule[1]).unwrap();
                    f < s
                })
            })
            .collect::<Vec<_>>();

        valids
            .iter()
            .map(|valid| valid[valid.len() / 2])
            .sum::<usize>() as isize
    }
}
