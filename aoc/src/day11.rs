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
    let mut i = 0;
    while i < nums.len() {
        let num = &mut nums[i];
        if *num == 0 {
            *num = 1
        } else if num.digits() % 2 == 0 {
            let left = *num / (10usize.pow((num.digits() / 2) as u32));
            let right = *num % (10usize.pow((num.digits() / 2) as u32));

            *num = left;
            nums.insert(i + 1, right);
            i += 1;
        } else {
            *num *= 2024;
        }
        i += 1;
    }
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
}
