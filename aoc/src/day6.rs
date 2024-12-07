use std::fmt::Write as _;

use aoc::Solution;

pub struct Day6;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }
}

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Block,
    Obstacle,
    Walked { vertical: bool, horizontal: bool },
}

impl Cell {
    pub fn walked_horizontal(&self) -> bool {
        if let Cell::Walked { horizontal, .. } = self {
            *horizontal
        } else {
            false
        }
    }
    pub fn walked_vertical(&self) -> bool {
        if let Cell::Walked { vertical, .. } = self {
            *vertical
        } else {
            false
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Empty => '.',
            Cell::Block => '#',
            Cell::Obstacle => 'O',
            Cell::Walked {
                vertical,
                horizontal,
            } => match (*vertical, *horizontal) {
                (true, false) => '|',
                (false, true) => '-',
                (true, true) => '+',
                (false, false) => unreachable!(),
            },
        })
    }
}

#[derive(Clone)]
struct World {
    grid: Vec<Vec<Cell>>,
    guards: Vec<(usize, usize, Direction)>,
}

impl World {
    pub fn new(text: &str) -> World {
        let mut guards = Vec::default();
        let grid = text
            .split('\n')
            .filter(|x| !x.is_empty())
            .enumerate()
            .map(|(y, row)| {
                let mut r = Vec::default();
                r.extend(row.chars().enumerate().map(|(x, ch)| match ch {
                    '.' => Cell::Empty,
                    '#' => Cell::Block,
                    '^' => {
                        guards.push((x, y, Direction::Up));
                        Cell::Empty
                    }
                    'v' => {
                        guards.push((x, y, Direction::Down));
                        Cell::Empty
                    }
                    '<' => {
                        guards.push((x, y, Direction::Left));
                        Cell::Empty
                    }
                    '>' => {
                        guards.push((x, y, Direction::Right));
                        Cell::Empty
                    }
                    '|' => Cell::Walked {
                        vertical: true,
                        horizontal: false,
                    },
                    '-' => Cell::Walked {
                        vertical: false,
                        horizontal: true,
                    },
                    '+' => Cell::Walked {
                        vertical: true,
                        horizontal: true,
                    },
                    _ => unimplemented!("Invalid character, {ch}"),
                }));
                r
            })
            .collect::<Vec<_>>();

        World { grid, guards }
    }

    pub fn step(&mut self) {
        self.guards.retain_mut(|(x, y, direction)| {
            let (next_x, next_y) = match direction {
                Direction::Up => (*x, y.wrapping_sub(1)),
                Direction::Down => (*x, *y + 1),
                Direction::Left => (x.wrapping_sub(1), *y),
                Direction::Right => (*x + 1, *y),
            };

            let last_cell = self.grid[*y][*x];
            self.grid[*y][*x] = match direction {
                Direction::Up | Direction::Down => Cell::Walked {
                    vertical: true,
                    horizontal: last_cell.walked_horizontal(),
                },
                Direction::Left | Direction::Right => Cell::Walked {
                    vertical: last_cell.walked_vertical(),
                    horizontal: true,
                },
            };

            let mut rotate = false;
            if let Some(row) = self.grid.get(next_y) {
                if let Some(cell) = row.get(next_x) {
                    match cell {
                        Cell::Empty | Cell::Walked { .. } => {
                            *x = next_x;
                            *y = next_y;
                        }
                        Cell::Block | Cell::Obstacle => {
                            rotate = true;
                        }
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }

            if rotate {
                direction.rotate_right();
            }
            true
        });
    }

    pub fn guards(&self) -> &[(usize, usize, Direction)] {
        &self.guards
    }

    pub fn was_in_loop(&mut self) -> bool {
        let mut i = 0;
        while !self.guards.is_empty() {
            self.step();

            i += 1;
            if self.grid[0].len() * self.grid.len() < i {
                return true;
            }
        }

        false
    }

    pub fn walked(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().flat_map(move |(x, cell)| {
                    if let Cell::Walked { .. } = cell {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>()
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                'print: {
                    for (guard_x, guard_y, direction) in self.guards.iter() {
                        if x == *guard_x && y == *guard_y {
                            f.write_char(match direction {
                                Direction::Up => '^',
                                Direction::Down => 'v',
                                Direction::Left => '<',
                                Direction::Right => '>',
                            })?;
                            break 'print;
                        }
                    }
                    std::fmt::Display::fmt(cell, f)?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Solution for Day6 {
    fn part1(&self, input: &str) -> isize {
        let mut world = World::new(input);
        let mut i = 0;
        while !world.guards().is_empty() {
            world.step();
            i += 1;
            #[cfg(feature = "vizualize")]
            if i % 40 == 0 {
                if i != 0 {
                    let x = world.grid.len();
                    let y = world.grid[0].len() + 1;
                    eprint!("\x1b[{x}D\x1b[{y}A\0");
                }
                eprintln!("{world}");
            }
        }
        world.walked().len() as isize
    }

    fn part2(&self, input: &str) -> isize {
        let world = World::new(input);
        let mut i = 0;
        let guard = world.guards[0];

        let mut w = world.clone();
        while !w.guards().is_empty() {
            w.step();
        }

        for (x, y) in w.walked() {
            let mut world = world.clone();
            world.guards.clear();
            world.guards.push(guard);
            let Some(row) = world.grid.get_mut(y) else {
                continue;
            };
            let Some(cell) = row.get_mut(x) else {
                continue;
            };
            *cell = Cell::Obstacle;

            if world.was_in_loop() {
                #[cfg(feature = "vizualize")]
                {
                    eprintln!("World {i}:\n{world}\n");
                }
                i += 1;
            }
        }
        i
    }
}
