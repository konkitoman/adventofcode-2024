use aoc::Solution;

pub struct Day14;

#[derive(Debug, Clone)]
pub struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    pub fn new(text: &str) -> Self {
        let mut pos = None;
        let mut vel = None;
        let (v1, v2) = text.split_once(' ').unwrap();
        for v in [v1, v2] {
            let (key, value) = v.split_once('=').unwrap();
            let (x, y) = value.split_once(',').unwrap();
            let (x, y) = (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap());
            match key {
                "p" => {
                    pos = Some((x, y));
                }
                "v" => {
                    vel = Some((x, y));
                }
                _ => unimplemented!(),
            }
        }

        Self {
            pos: pos.unwrap(),
            vel: vel.unwrap(),
        }
    }

    pub fn simulate(&mut self, width: isize, height: isize) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        if self.pos.0 < 0 {
            self.pos.0 += width;
        }
        if self.pos.1 < 0 {
            self.pos.1 += height;
        }

        if self.pos.0 >= width {
            self.pos.0 -= width;
        }
        if self.pos.1 >= height {
            self.pos.1 -= height;
        }
    }
}

fn print_world(width: isize, height: isize, robots: &[Robot]) {
    for y in 0..height {
        for x in 0..width {
            let count = robots.iter().filter(|robot| robot.pos == (x, y)).count();
            if count == 0 {
                eprint!(".");
            } else {
                eprint!("{count}");
            }
        }
        eprintln!()
    }
}

impl Solution for Day14 {
    fn part1(&self, input: &str) -> isize {
        let mut robots = input
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(Robot::new)
            .collect::<Vec<_>>();

        let width = 101;
        let height = 103;

        #[cfg(feature = "vizualize")]
        print_world(width, height, &robots);

        for i in 0..100 {
            for robot in robots.iter_mut() {
                robot.simulate(width, height);
            }
            #[cfg(feature = "vizualize")]
            {
                eprintln!("{i}:");
                print_world(width, height, &robots);
            }
        }

        let top_left = robots
            .iter()
            .filter(|robot| {
                for y in 0..height / 2 {
                    for x in 0..width / 2 {
                        if robot.pos == (x, y) {
                            return true;
                        }
                    }
                }
                false
            })
            .collect::<Vec<_>>();
        let top_right = robots
            .iter()
            .filter(|robot| {
                for y in 0..height / 2 {
                    for x in 1 + width / 2..width {
                        if robot.pos == (x, y) {
                            return true;
                        }
                    }
                }
                false
            })
            .collect::<Vec<_>>();
        let bottom_left = robots
            .iter()
            .filter(|robot| {
                for y in 1 + height / 2..height {
                    for x in 0..width / 2 {
                        if robot.pos == (x, y) {
                            return true;
                        }
                    }
                }
                false
            })
            .collect::<Vec<_>>();
        let bottom_right = robots
            .iter()
            .filter(|robot| {
                for y in 1 + height / 2..height {
                    for x in 1 + width / 2..width {
                        if robot.pos == (x, y) {
                            return true;
                        }
                    }
                }
                false
            })
            .collect::<Vec<_>>();

        (top_left.len() * top_right.len() * bottom_left.len() * bottom_right.len()) as isize
    }

    fn part2(&self, input: &str) -> isize {
        let mut robots = input
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(Robot::new)
            .collect::<Vec<_>>();

        let width = 101;
        let height = 103;

        let original = robots.clone();

        #[cfg(feature = "vizualize")]
        print_world(width, height, &robots);

        let mut min = (0, isize::MAX);

        for i in 0..10000 {
            for robot in robots.iter_mut() {
                robot.simulate(width, height);
            }

            let mut tl = 0;
            let mut tr = 0;
            let mut bl = 0;
            let mut br = 0;

            for robot in robots.iter() {
                if (0..height / 2).contains(&robot.pos.1) {
                    if (0..width / 2).contains(&robot.pos.0) {
                        tl += 1;
                        continue;
                    }
                    if (1 + width / 2..width).contains(&robot.pos.0) {
                        tr += 1;
                        continue;
                    }
                }

                if (1 + height / 2..height).contains(&robot.pos.1) {
                    if (0..width / 2).contains(&robot.pos.0) {
                        bl += 1;
                        continue;
                    }
                    if (1 + width / 2..width).contains(&robot.pos.0) {
                        br += 1;
                        continue;
                    }
                }
            }

            let safety_factor = (tl * tr * bl * br) as isize;

            #[cfg(feature = "vizualize")]
            eprintln!("I: {i}, Safety factor: {safety_factor}, Min: {min:?}\x1B[1A");
            if min.1 > safety_factor {
                min.0 = i + 1;
                min.1 = safety_factor;
            }
        }

        #[cfg(feature = "vizualize")]
        {
            eprintln!("Min: {min:?}");

            let mut robots = original;
            for robot in robots.iter_mut() {
                for _ in 0..min.0 {
                    robot.simulate(width, height);
                }
            }

            print_world(width, height, &robots);
        }

        min.0
    }
}
