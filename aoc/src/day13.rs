use aoc::Solution;

pub struct Day13;

#[derive(Debug)]
struct Segment {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

impl Segment {
    fn new(text: &str) -> Segment {
        let mut button_a = None;
        let mut button_b = None;
        let mut prize = None;

        fn read_button(value: &str, x: &mut Option<isize>, y: &mut Option<isize>) {
            for s in value.split(',') {
                let (key, value) = s.trim().split_once('+').unwrap();
                match key {
                    "X" => {
                        *x = Some(value.parse::<isize>().unwrap());
                    }
                    "Y" => {
                        *y = Some(value.parse::<isize>().unwrap());
                    }
                    _ => unimplemented!("Function Key: {key}"),
                }
            }
        }

        for line in text.split('\n').filter(|x| !x.is_empty()) {
            let mut line = line.splitn(2, ':');
            let key = line.next().unwrap();
            match key {
                "Button A" => {
                    let value = line.next().unwrap().trim();
                    let mut x = None;
                    let mut y = None;

                    read_button(value, &mut x, &mut y);

                    button_a = Some((x.unwrap(), y.unwrap()));
                }
                "Button B" => {
                    let value = line.next().unwrap().trim();
                    let mut x = None;
                    let mut y = None;

                    read_button(value, &mut x, &mut y);

                    button_b = Some((x.unwrap(), y.unwrap()));
                }
                "Prize" => {
                    let value = line.next().unwrap().trim();
                    let mut x = None;
                    let mut y = None;
                    for s in value.split(',') {
                        let (key, value) = s.trim().split_once('=').unwrap();
                        match key {
                            "X" => {
                                x = Some(value.parse::<isize>().unwrap());
                            }
                            "Y" => {
                                y = Some(value.parse::<isize>().unwrap());
                            }
                            _ => unimplemented!("Function Key: {key}"),
                        }
                    }
                    prize = Some((x.unwrap(), y.unwrap()));
                }
                _ => unimplemented!("Key: {key}"),
            }
        }

        Segment {
            button_a: button_a.unwrap(),
            button_b: button_b.unwrap(),
            prize: prize.unwrap(),
        }
    }
}

impl Solution for Day13 {
    fn part1(&self, input: &str) -> isize {
        let segments = input.split("\n\n").map(Segment::new).collect::<Vec<_>>();

        let mut sum = 0;

        for segment in segments {
            let px = segment.prize.0 as f32;
            let py = segment.prize.1 as f32;
            let bx = segment.button_b.0 as f32;
            let by = segment.button_b.1 as f32;
            let ax = segment.button_a.0 as f32;
            let ay = segment.button_a.1 as f32;

            let button_a_c = (px * by - py * bx) / (ax * by - ay * bx);
            let button_b_c = (px - ax * button_a_c) / bx;

            let good = button_b_c % 1. == 0. && button_a_c % 1. == 0.;
            eprintln!(
                "Good: {}, Count: {}, A: {button_a_c}, B: {button_b_c}",
                good,
                button_b_c + button_a_c
            );

            if good {
                sum += ((button_a_c * 3.) + (button_b_c)) as isize;
            }
        }

        sum
    }

    fn part2(&self, input: &str) -> isize {
        let segments = input.split("\n\n").map(Segment::new).collect::<Vec<_>>();

        let mut sum = 0;

        for segment in segments {
            let px = segment.prize.0 as f64 + 10000000000000.;
            let py = segment.prize.1 as f64 + 10000000000000.;
            let bx = segment.button_b.0 as f64;
            let by = segment.button_b.1 as f64;
            let ax = segment.button_a.0 as f64;
            let ay = segment.button_a.1 as f64;

            let button_a_c = (px * by - py * bx) / (ax * by - ay * bx);
            let button_b_c = (px - ax * button_a_c) / bx;

            let good = button_b_c % 1. == 0. && button_a_c % 1. == 0.;
            eprintln!(
                "Good: {}, Count: {}, A: {button_a_c}, B: {button_b_c}",
                good,
                button_b_c + button_a_c
            );

            if good {
                sum += ((button_a_c * 3.) + (button_b_c)) as isize;
            }
        }

        sum
    }
}
