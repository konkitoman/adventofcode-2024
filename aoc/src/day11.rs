use std::collections::HashMap;

use aoc::Solution;

pub struct Day11;

trait TDigits {
    fn digits(&self) -> usize;
}

impl TDigits for usize {
    fn digits(&self) -> usize {
        let mut num = *self;
        let mut count = 0;

        while num != 0 {
            count += 1;
            num /= 10;
        }

        count
    }
}

fn simulate_plutonian_pebbles(nums: &mut Vec<usize>) {
    let mut new_nums = Vec::with_capacity(nums.len() * 2);
    for num in nums.drain(..) {
        if num == 0 {
            new_nums.push(1);
        } else if num.digits() % 2 == 0 {
            let left = num / (10usize.pow((num.digits() / 2) as u32));
            let right = num % (10usize.pow((num.digits() / 2) as u32));

            new_nums.push(left);
            new_nums.push(right);
        } else {
            new_nums.push(num * 2024);
        }
    }
    *nums = new_nums;
}

fn simulate_plutonian_pebbles_len(
    cache: &mut HashMap<(usize, usize), usize>,
    num: usize,
    steps: usize,
) -> usize {
    if let Some(res) = cache.get(&(num, steps)) {
        return *res;
    }

    if steps == 0 {
        return 1;
    }

    let res = if num == 0 {
        simulate_plutonian_pebbles_len(cache, 1, steps - 1)
    } else if num.digits() % 2 == 0 {
        let left = num / (10usize.pow((num.digits() / 2) as u32));
        let right = num % (10usize.pow((num.digits() / 2) as u32));
        simulate_plutonian_pebbles_len(cache, left, steps - 1)
            + simulate_plutonian_pebbles_len(cache, right, steps - 1)
    } else {
        simulate_plutonian_pebbles_len(cache, num * 2024, steps - 1)
    };

    cache.insert((num, steps), res);
    res
}

impl Solution for Day11 {
    fn part1(&self, input: &str) -> isize {
        let mut numbers = input
            .split(' ')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        #[cfg(feature = "vizualize")]
        eprintln!("{numbers:?}");
        for simulation in 0..25 {
            simulate_plutonian_pebbles(&mut numbers);
            #[cfg(feature = "vizualize")]
            eprintln!("{simulation}: {numbers:?}");
        }

        numbers.len() as isize
    }

    fn part2(&self, input: &str) -> isize {
        let numbers = input
            .split(' ')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut cache = HashMap::default();
        numbers
            .iter()
            .map(|num| simulate_plutonian_pebbles_len(&mut cache, *num, 75))
            .sum::<usize>() as isize
    }
}
