use aoc::Solution;

pub struct Day10;

#[derive(Debug)]
struct Topographic {
    world: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Topographic {
    pub fn new(text: &str) -> Self {
        let world: Vec<Vec<u8>> = text
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|row| row.chars().map(|ch| (ch as u8) - b'0').collect())
            .collect();
        let width = world[0].len();
        let height = world.len();

        for row in world.iter() {
            assert_eq!(row.len(), width)
        }

        Self {
            world,
            width,
            height,
        }
    }

    pub fn get_positions_with(&self, height: u8) -> Vec<(usize, usize)> {
        let mut positions = Vec::default();

        for (y, row) in self.world.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == height {
                    positions.push((x, y));
                }
            }
        }

        positions
    }

    pub fn get(&self, pos: (usize, usize)) -> Option<u8> {
        self.world
            .get(pos.1)
            .and_then(|row| row.get(pos.0).copied())
    }
}

struct Tailhead<'a> {
    map: &'a Topographic,
    start: (usize, usize),
    routes: Vec<Vec<(usize, usize)>>,
}

impl<'a> Tailhead<'a> {
    pub fn new(map: &'a Topographic, start: (usize, usize)) -> Option<Self> {
        let last = map.get(start).unwrap();
        assert_eq!(last, 0);

        let mut finished_routes = Vec::<Vec<(usize, usize)>>::default();
        let mut routes = vec![vec![start]];

        while !routes.is_empty() {
            for route in std::mem::take(&mut routes) {
                let pos = *route.last().unwrap();
                let last = map.get(pos).unwrap();
                if last == 9 {
                    finished_routes.push(route);
                    continue;
                }

                let mut new_poses = Vec::with_capacity(4);
                if pos.0 > 0 {
                    new_poses.push((pos.0 - 1, pos.1));
                }
                if pos.1 > 0 {
                    new_poses.push((pos.0, pos.1 - 1));
                }
                if pos.0 < map.width - 1 {
                    new_poses.push((pos.0 + 1, pos.1));
                }
                if pos.1 < map.height - 1 {
                    new_poses.push((pos.0, pos.1 + 1));
                }

                routes.extend(
                    new_poses
                        .into_iter()
                        .flat_map(|(new_x, new_y)| {
                            let cell = map
                                .get((new_x, new_y))
                                .expect("Something is wrong with the new possitions!");
                            // Needs to be uphill
                            if cell <= last || cell - last != 1 {
                                return None;
                            }

                            let mut route = route.clone();
                            route.push((new_x, new_y));

                            Some(route)
                        })
                        .collect::<Vec<_>>(),
                );
            }
        }

        if !finished_routes.is_empty() {
            Some(Self {
                map,
                start,
                routes: finished_routes,
            })
        } else {
            None
        }
    }
}

fn print_route(map: &Topographic, route: &[(usize, usize)]) {
    let mut traveled_posses = Vec::with_capacity(route.len());
    for pos in route {
        traveled_posses.push(*pos);

        if traveled_posses.len() != 1 {
            eprint!("\x1b[{}A", map.height);
        }

        for (y, row) in map.world.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let lit = traveled_posses.contains(&(x, y));
                if lit {
                    if pos == &(x, y) {
                        eprint!("\x1b[48;5;2m")
                    } else {
                        eprint!("\x1b[48;5;1m")
                    }
                }

                eprint!("{cell}");

                if lit {
                    eprint!("\x1b[49m")
                }
            }
            eprintln!();
        }

        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    }

    eprintln!();
}

impl Solution for Day10 {
    fn part1(&self, input: &str) -> isize {
        let map = Topographic::new(input);
        let mut tailheads = map
            .get_positions_with(0)
            .into_iter()
            .flat_map(|pos| Tailhead::new(&map, pos))
            .collect::<Vec<_>>();

        tailheads.iter_mut().for_each(|tailhead| {
            let mut end_poses = Vec::default();
            tailhead.routes.retain(|route| {
                let pos = route.last().unwrap();
                if end_poses.contains(pos) {
                    false
                } else {
                    end_poses.push(*pos);
                    true
                }
            });
        });

        #[cfg(feature = "vizualize")]
        for (i, tailhead) in tailheads.iter().enumerate() {
            for (ii, route) in tailhead.routes.iter().enumerate() {
                println!("{i} {ii}:");
                print_route(&map, route);
            }
        }

        tailheads
            .iter()
            .map(|tailhead| tailhead.routes.len())
            .sum::<usize>() as isize
    }
}
