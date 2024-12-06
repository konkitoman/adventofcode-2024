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
    Guard { direction: Direction },
    Block,
    Walked,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Empty => '.',
            Cell::Guard { direction } => match direction {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            },
            Cell::Block => '#',
            Cell::Walked => 'X',
        })
    }
}

struct World {
    grid: Vec<Vec<Cell>>,
}

impl World {
    pub fn new(text: &str) -> World {
        let grid = text
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|row| {
                let mut r = Vec::default();
                r.extend(row.chars().map(|ch| match ch {
                    '.' => Cell::Empty,
                    '#' => Cell::Block,
                    '^' => Cell::Guard {
                        direction: Direction::Up,
                    },
                    'v' => Cell::Guard {
                        direction: Direction::Down,
                    },
                    '<' => Cell::Guard {
                        direction: Direction::Left,
                    },
                    '>' => Cell::Guard {
                        direction: Direction::Right,
                    },
                    _ => unimplemented!("Invalid character, {ch}"),
                }));
                r
            })
            .collect::<Vec<_>>();

        World { grid }
    }

    pub fn step(&mut self) {
        let to_move = self.guards();

        for ((x, y), direction) in to_move {
            let (next_x, next_y) = match direction {
                Direction::Up => (x, y.wrapping_sub(1)),
                Direction::Down => (x, y + 1),
                Direction::Left => (x.wrapping_sub(1), y),
                Direction::Right => (x + 1, y),
            };

            let last_cell = self.grid[y][x];
            let mut moved = false;
            let mut rotate = false;
            if let Some(row) = self.grid.get_mut(next_y) {
                if let Some(cell) = row.get_mut(next_x) {
                    match cell {
                        Cell::Empty | Cell::Walked => {
                            *cell = last_cell;
                            moved = true;
                        }
                        Cell::Guard { .. } => unimplemented!(),
                        Cell::Block => {
                            rotate = true;
                        }
                    }
                } else {
                    moved = true;
                }
            } else {
                moved = true;
            }

            if moved {
                self.grid[y][x] = Cell::Walked;
            }

            if rotate {
                if let Cell::Guard { direction } = &mut self.grid[y][x] {
                    direction.rotate_right();
                }
            }
        }
    }

    pub fn guards(&self) -> Vec<((usize, usize), Direction)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().flat_map(move |(x, cell)| {
                    if let Cell::Guard { direction } = cell {
                        Some(((x, y), *direction))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>()
    }

    pub fn walked(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().flat_map(move |(x, cell)| {
                    if let Cell::Walked = cell {
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
        for row in &self.grid {
            for cell in row {
                std::fmt::Display::fmt(cell, f)?;
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
}
