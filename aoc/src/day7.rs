use std::fmt::Write as _;

use aoc::Solution;

pub struct Day7;

#[derive(Debug)]
pub struct Test {
    value: isize,
    values: Vec<isize>,
}

pub enum Operator {
    Add = 0,
    Mul = 1,
}

impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Operator::Add => '+',
            Operator::Mul => '*',
        })
    }
}

impl TryFrom<usize> for Operator {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Add,
            1 => Self::Mul,
            _ => return Err(()),
        })
    }
}

impl Operator {
    pub fn get_all_operators_for(len: usize) -> Vec<Vec<Operator>> {
        let mut operators = Vec::default();
        for ii in 0..2usize.pow(len as u32) {
            let mut x = ii;
            let mut ops = Vec::with_capacity(len);
            for _ in 0..len {
                ops.push(Operator::try_from(x % 2).unwrap());
                x /= 2;
            }
            operators.push(ops);
        }
        operators
    }

    pub fn compute(&self, left: isize, right: isize) -> isize {
        match self {
            Operator::Add => left + right,
            Operator::Mul => left * right,
        }
    }
}

impl Test {
    pub fn run(&self, operators: &[Operator]) -> bool {
        assert_eq!(self.values.len() - 1, operators.len());

        let mut ops = operators.iter();
        let mut last = None;
        for value in self.values.iter() {
            last = Some(if let Some(last) = last {
                ops.next().unwrap().compute(last, *value)
            } else {
                *value
            });
        }

        last.unwrap() == self.value
    }
}

impl Solution for Day7 {
    fn part1(&self, input: &str) -> isize {
        let tests = input
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|line| {
                let mut iter = line.split(':');
                let value = iter.next().unwrap().parse::<isize>().unwrap();
                let values = iter
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(|x| x.trim())
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>();
                Test { value, values }
            })
            .collect::<Vec<_>>();

        let mut sum = 0;

        'testing: for test in tests {
            let operators = Operator::get_all_operators_for(test.values.len() - 1);
            for operators in operators {
                if test.run(&operators) {
                    sum += test.value;
                    continue 'testing;
                }
            }
        }

        sum
    }
}
