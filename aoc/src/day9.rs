use aoc::Solution;

pub struct Day9;

#[derive(Clone, Copy)]
enum Block {
    File { id: usize },
    Free,
}

struct DiskMap {
    data: Vec<(u8, Block)>,
}

struct Blocks {
    data: Vec<Block>,
}

impl DiskMap {
    pub fn new(text: &str) -> DiskMap {
        let mut data = Vec::default();
        let mut id = 0;
        let mut file = true;
        for ch in text.chars() {
            match ch {
                '0'..='9' => {
                    let size = (ch as u8) - b'0';
                    if file {
                        data.push((size, Block::File { id }));
                        id += 1;
                        file = false;
                    } else {
                        data.push((size, Block::Free));
                        file = true;
                    }
                }
                _ => {
                    unimplemented!()
                }
            }
        }
        DiskMap { data }
    }

    pub fn to_blocks(&self) -> Blocks {
        let mut data = Vec::default();

        for (size, block) in self.data.iter() {
            for _ in 0..*size {
                data.push(*block);
            }
        }

        Blocks { data }
    }
}

impl Blocks {
    pub fn step_rearrange(&mut self) {
        let mut last_block = self.data.len() - 1;
        while let Some(Block::Free) = self.data.get(last_block) {
            if last_block == 0 {
                return;
            }
            last_block -= 1;
        }

        let mut first_free_block = 0;
        while let Block::File { .. } = self.data[first_free_block] {
            first_free_block += 1;
        }

        if first_free_block < last_block {
            self.data.swap(last_block, first_free_block);
        }
    }

    pub fn is_arranged(&self) -> bool {
        let mut iter = self.data.iter();
        for block in iter.by_ref() {
            if let Block::Free = block {
                break;
            }
        }
        for block in iter {
            if let Block::File { .. } = block {
                return false;
            }
        }

        true
    }

    pub fn checksum(&self) -> usize {
        let mut sum = 0;
        let mut i = 0;
        for block in self.data.iter() {
            if let Block::File { id } = block {
                sum += i * id;
                i += 1;
            }
        }

        sum
    }
}

impl std::fmt::Display for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.data.iter() {
            match block {
                Block::File { id } => {
                    f.write_fmt(format_args!("{id}"))?;
                }
                Block::Free => {
                    f.write_str(".")?;
                }
            }
        }
        Ok(())
    }
}

impl Solution for Day9 {
    fn part1(&self, input: &str) -> isize {
        let disk_map = DiskMap::new(input.trim());
        let mut blocks = disk_map.to_blocks();

        while !blocks.is_arranged() {
            blocks.step_rearrange();
            #[cfg(feature = "vizualize")]
            eprintln!("{blocks}");
        }

        blocks.checksum() as isize
    }
}
