use aoc::Solution;

pub struct Day8;

#[derive(Debug)]
struct World {
    antenas: Vec<(isize, isize, char)>,
    grid: Vec<Vec<bool>>,
}

impl World {
    pub fn new(text: &str) -> World {
        let mut antenas = Vec::default();
        let grid = text
            .split('\n')
            .filter(|x| !x.is_empty())
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, ch)| match ch {
                        '.' => false,
                        _ => {
                            antenas.push((x as isize, y as isize, ch));
                            false
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        World { antenas, grid }
    }

    pub fn format(&self, types: &[char]) {
        for (y, row) in self.grid.iter().enumerate() {
            'f: for (x, cell) in row.iter().enumerate() {
                for (ax, ay, antena) in self.antenas.iter() {
                    if types.contains(antena) && x as isize == *ax && y as isize == *ay {
                        eprint!("{}", *antena);
                        continue 'f;
                    }
                }
                if *cell {
                    eprint!("#");
                } else {
                    eprint!(".");
                }
            }
            eprintln!()
        }
    }

    pub fn simplate(&mut self, antena: char) {
        for (y, row) in self.grid.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let y = y as isize;
                let x = x as isize;
                for (ax, ay, t_antena) in self.antenas.iter() {
                    if *t_antena != antena {
                        continue;
                    }
                    for (sax, say, st_antena) in self.antenas.iter() {
                        if *st_antena != antena || (sax == ax && say == ay) {
                            continue;
                        }
                        let _ax = ax - x;
                        let _ay = ay - y;
                        let _sax = sax - x;
                        let _say = say - y;
                        let l = ((_ax * _ax) as f64 + (_ay * _ay) as f64).sqrt();
                        let nax = _ax as f64 / l;
                        let nay = _ay as f64 / l;
                        let sl = ((_sax * _sax) as f64 + (_say * _say) as f64).sqrt();
                        let nsax = _sax as f64 / sl;
                        let nsay = _say as f64 / sl;
                        let dot = ((nax * nsax) + (nay * nsay)).sqrt();

                        if !*cell {
                            *cell = (dot >= 0.99999) && (sl / 2. == l || l / 2. == sl);
                        }
                    }
                }
            }
        }
    }
}

impl Solution for Day8 {
    fn part1(&self, input: &str) -> isize {
        let mut world = World::new(input);
        let mut antena_types = world
            .antenas
            .iter()
            .map(|(_, _, ch)| *ch)
            .collect::<Vec<_>>();
        antena_types.dedup();

        for ty in antena_types {
            world.simplate(ty);
        }
        world
            .grid
            .iter()
            .map(|x| x.iter().map(|c| if *c { 1 } else { 0 }).sum::<isize>())
            .sum::<isize>()
    }
}
