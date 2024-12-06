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

    fn part2(&self, input: &str) -> isize {
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
            .map(|update| {
                let rules = ordering_rules
                    .iter()
                    .filter(|rules| rules.iter().all(|num| update.contains(num)))
                    .collect::<Vec<_>>();
                (update, rules)
            })
            .filter(|(update, rules)| {
                !rules.iter().all(|rule| {
                    let f = update.iter().position(|x| *x == rule[0]).unwrap();
                    let s = update.iter().position(|x| *x == rule[1]).unwrap();
                    f < s
                })
            })
            .map(|(update, rules)| {
                let mut update = update.clone();

                let mut was_invalid = true;

                while was_invalid {
                    was_invalid = false;
                    rules.iter().for_each(|rule| {
                        let f = update.iter().position(|x| *x == rule[0]).unwrap();
                        let s = update.iter().position(|x| *x == rule[1]).unwrap();

                        if f > s {
                            was_invalid = true;
                            let value = update[s];
                            update.remove(s);
                            update.insert(f, value);
                        }
                    });
                }

                update
            })
            .collect::<Vec<_>>();

        valids
            .iter()
            .map(|valid| valid[valid.len() / 2])
            .sum::<usize>() as isize
    }
}
