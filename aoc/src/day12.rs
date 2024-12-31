use std::collections::{HashMap, HashSet};

use aoc::{BColor256, BColor8, FColor256, FColor8, Solution};

pub struct Day12;

pub struct Region {
    poses: HashSet<(usize, usize)>,
}

impl Region {
    pub fn contains(&self, pos: (usize, usize)) -> bool {
        self.poses.contains(&pos)
    }
}

struct Plot {
    regions: Vec<Region>,
}

impl Plot {
    pub fn contains(&self, pos: (usize, usize)) -> bool {
        for region in self.regions.iter() {
            if region.contains(pos) {
                return true;
            }
        }
        false
    }
}

struct World {
    data: Vec<Vec<char>>,
}

impl World {
    pub fn new(text: &str) -> Self {
        let world = text
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|c| c.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { data: world }
    }

    pub fn get_plots(&self) -> HashMap<char, Plot> {
        let mut plots = HashMap::<char, Plot>::default();

        plots.insert(
            self.data[0][0],
            Plot {
                regions: vec![Region {
                    poses: HashSet::from_iter([(0, 0)]),
                }],
            },
        );

        let mut queue = Vec::<(usize, usize)>::default();

        let mut o_current = Some((self.data[0][0], 0));

        while let Some(current) = o_current.take() {
            let plot = plots.get_mut(&current.0).unwrap();
            let region = &mut plot.regions[current.1];
            let mut grow_directions = Vec::default();

            for (x, y) in region.poses.iter().copied() {
                if x > 0 {
                    grow_directions.push((x - 1, y));
                }
                if y > 0 {
                    grow_directions.push((x, y - 1));
                }
                if x < self.data[0].len() {
                    grow_directions.push((x + 1, y));
                }
                if y < self.data.len() {
                    grow_directions.push((x, y + 1));
                }
            }

            let mut finished = false;
            for grow_direction in grow_directions {
                if let Some(ch) = self
                    .data
                    .get(grow_direction.1)
                    .and_then(|row| row.get(grow_direction.0))
                    .copied()
                {
                    if ch == current.0 {
                        if region.poses.insert(grow_direction) {
                            finished = true;
                        }
                    } else {
                        queue.push((grow_direction.0, grow_direction.1));
                    }
                }
            }

            if !finished {
                #[cfg(feature = "vizualize")]
                display_region(current.0, region, &self.data);
                queue.sort();
                queue.dedup();
                queue.retain(|(x, y)| {
                    let x = *x;
                    let y = *y;
                    if let Some(plot) = plots.get(&self.data[y][x]) {
                        if plot.contains((x, y)) {
                            return false;
                        }
                    }
                    true
                });

                if let Some((x, y)) = queue.pop() {
                    let entry = plots.entry(self.data[y][x]);
                    let plot = entry.or_insert(Plot {
                        regions: Vec::default(),
                    });
                    let i = plot.regions.len();
                    plot.regions.push(Region {
                        poses: HashSet::from_iter([(x, y)]),
                    });
                    o_current = Some((self.data[y][x], i));

                    #[cfg(feature = "vizualize")]
                    for _ in 0..self.data.len() + 2 {
                        eprintln!()
                    }
                }
                continue;
            }

            #[cfg(feature = "vizualize")]
            display_region(current.0, region, &self.data);

            o_current = Some(current);
        }
        #[cfg(feature = "vizualize")]
        {
            eprint!("\x1B[0J");
            for (y, row) in self.data.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    for (i, plot) in plots.values().enumerate() {
                        if plot.contains((x, y)) {
                            eprint!("{}", FColor256(i as u8 + 2));
                        }
                    }
                    eprint!("{cell}");
                    eprint!("{}", FColor8::Default);
                }
                eprintln!();
            }

            eprint!("{}", FColor8::Default);
            eprint!("{}", BColor8::Default);
        }

        plots
    }
}

fn display_inside(map: &[Vec<char>], func: impl Fn((usize, usize)) -> (Option<u8>, Option<u8>)) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let res = func((x, y));
            if let Some(f) = res.0 {
                eprint!("{}", FColor256(f));
            }
            if let Some(b) = res.1 {
                eprint!("{}", BColor256(b));
            }
            eprint!("{cell}");
            eprint!("{}{}", FColor8::Default, BColor8::Default);
        }
        eprintln!();
    }
}

fn display_region(ch: char, region: &Region, map: &[Vec<char>]) {
    eprintln!("{}:", ch);
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if region.contains((x, y)) {
                eprint!("{}", FColor256(1));
            }
            eprint!("{cell}");
            eprint!("{}", FColor8::Default);
        }
        eprintln!();
    }
    eprint!("\x1B[{}A", map.len() + 1);

    std::thread::sleep(std::time::Duration::from_secs_f64(0.1));
}

impl Solution for Day12 {
    fn part1(&self, input: &str) -> isize {
        let world = World::new(input);
        let plots = world.get_plots();

        let mut sum = 0;

        for plot in plots {
            #[cfg(feature = "vizualize")]
            eprintln!("{}:", plot.0);
            for region in plot.1.regions {
                let mut gplots = region.poses.iter().copied().collect::<Vec<_>>();
                gplots.iter_mut().for_each(|(x, y)| {
                    *x += 1;
                    *y += 1;
                });

                let mut fences = Vec::<(usize, usize)>::default();
                for (x, y) in gplots.iter().copied() {
                    assert_ne!(x, 0);
                    assert_ne!(y, 0);
                    for pos in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                        if !gplots.contains(&pos) {
                            fences.push(pos);
                        }
                    }
                }

                sum += gplots.len() * fences.len();
                #[cfg(feature = "vizualize")]
                {
                    eprintln!("Plots: {}, Fences: {}", gplots.len(), fences.len());
                    let mut max = (0usize, 0usize);
                    let mut min = (usize::MAX, usize::MAX);
                    for (x, y) in fences.iter().copied() {
                        max.0 = max.0.max(x);
                        max.1 = max.1.max(y);
                        min.0 = min.0.min(x);
                        min.1 = min.1.min(y);
                    }

                    for y in min.1..=max.1 {
                        for x in min.0..=max.0 {
                            if gplots.contains(&(x, y)) {
                                eprint!("{}{}{}", FColor8::Green, plot.0, FColor8::Default);
                            } else if fences.contains(&(x, y)) {
                                eprint!("{}+{}", FColor8::Red, FColor8::Default);
                            } else {
                                eprint!(".");
                            }
                        }
                        eprintln!()
                    }
                }
            }
            #[cfg(feature = "vizualize")]
            eprintln!()
        }

        sum as isize
    }

    fn part2(&self, input: &str) -> isize {
        let mut world = World::new(input);
        let plots = world.get_plots();

        {
            let line = (0..world.data[0].len()).map(|_| '.').collect::<Vec<_>>();
            world.data.push(line.clone());
            world.data.insert(0, line);

            for row in world.data.iter_mut() {
                row.insert(0, '.');
                row.push('.');
            }
        }

        let mut sum = 0;

        fn print(world: &World, region: &Region, p: (usize, usize)) {
            display_inside(&world.data, |pos| {
                let b = region.contains(pos).then_some(1);
                if pos == p {
                    (Some(2), b)
                } else {
                    (None, b)
                }
            });
        }

        for mut plot in plots {
            for region in plot.1.regions.iter_mut() {
                region.poses = region
                    .poses
                    .iter()
                    .map(|(x, y)| (x + 1, y + 1))
                    .collect::<HashSet<_>>();

                let mut edge = HashSet::<(usize, usize)>::default();

                for (x, y) in region.poses.iter().copied() {
                    for (ox, oy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let x = (x as isize + ox) as usize;
                        let y = (y as isize + oy) as usize;

                        if !region.poses.contains(&(x, y)) {
                            edge.insert((x, y));
                        }
                    }
                }

                #[derive(Clone, Copy, Debug, PartialEq)]
                enum Direction {
                    Top,
                    Right,
                    Down,
                    Left,
                }

                impl Direction {
                    fn rotate_right(&self) -> Self {
                        match self {
                            Direction::Top => Self::Right,
                            Direction::Right => Self::Down,
                            Direction::Down => Self::Left,
                            Direction::Left => Self::Top,
                        }
                    }

                    fn rotate_left(&self) -> Self {
                        match self {
                            Direction::Top => Self::Left,
                            Direction::Right => Self::Top,
                            Direction::Down => Self::Right,
                            Direction::Left => Self::Down,
                        }
                    }
                }

                impl std::ops::Add<(usize, usize)> for Direction {
                    type Output = Option<(usize, usize)>;

                    fn add(self, (x, y): (usize, usize)) -> Self::Output {
                        Some(match self {
                            Direction::Top => {
                                if y == 0 {
                                    return None;
                                } else {
                                    (x, y - 1)
                                }
                            }
                            Direction::Right => (x + 1, y),
                            Direction::Down => (x, y + 1),
                            Direction::Left => {
                                if x == 0 {
                                    return None;
                                } else {
                                    (x - 1, y)
                                }
                            }
                        })
                    }
                }

                let mut walked = Vec::default();
                let mut pos = edge.iter().max().copied().unwrap();
                let mut direction = Direction::Top;
                let mut i = 0;
                let mut edges = 0;
                loop {
                    if i > 4 {
                        break;
                    }
                    if let Some(next) = direction + pos {
                        if !region.contains(next)
                            && region.contains((direction.rotate_right() + pos).unwrap())
                        {
                            break;
                        }
                    };
                    direction = direction.rotate_right();
                    i += 1;
                }
                let mut o_current = Some((pos, direction));

                i = 0;
                while let Some(((x, y), direction)) = o_current.take() {
                    i += 1;

                    if walked.contains(&((x, y), direction)) {
                        let out = edge
                            .iter()
                            .find(|pos| !walked.iter().map(|(x, _)| x).any(|p| &p == pos));
                        if let Some(out) = out {
                            i = 0;
                            pos = *out;
                            let mut direction = Direction::Top;
                            loop {
                                if i > 4 {
                                    break;
                                }
                                if let Some(next) = direction + pos {
                                    if !region.contains(next)
                                        && region
                                            .contains((direction.rotate_right() + pos).unwrap())
                                    {
                                        break;
                                    }
                                };
                                direction = direction.rotate_right();
                                i += 1;
                            }
                            o_current = Some((pos, direction));
                            continue;
                        }
                        break;
                    }

                    walked.push(((x, y), direction));

                    let (nx, ny) = (direction + (x, y)).unwrap();
                    let (urx, ury) = (direction.rotate_right() + (nx, ny)).unwrap();
                    #[cfg(feature = "vizualize")]
                    {
                        if i != 1 {
                            eprintln!("\x1B[{}A", world.data.len() + 1)
                        }
                        display_inside(&world.data, |(_x, _y)| {
                            let b = region.contains((_x, _y)).then_some(1);
                            if _x == x && _y == y {
                                (Some(2), b)
                            } else if (_x, _y) == (nx, ny) {
                                (Some(3), b)
                            } else if (_x, _y) == (urx, ury) {
                                (Some(6), b)
                            } else {
                                (None, b)
                            }
                        });
                    }

                    if region.contains((nx, ny)) {
                        o_current = Some(((x, y), direction.rotate_left()));
                        edges += 1;
                    } else if !region.contains((urx, ury)) {
                        o_current = Some(((nx, ny), direction.rotate_right()));
                        edges += 1;
                    } else {
                        o_current = Some(((nx, ny), direction));
                    }

                    #[cfg(feature = "vizualize")]
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }

                eprintln!("Edges: {edges}\n");
                sum += edges * region.poses.len();

                #[cfg(feature = "vizualize")]
                eprintln!();
            }
        }

        sum as isize
    }
}
