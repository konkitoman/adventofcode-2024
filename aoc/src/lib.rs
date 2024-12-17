pub trait Solution {
    fn part1(&self, input: &str) -> isize {
        _ = input;
        unimplemented!()
    }

    fn part2(&self, input: &str) -> isize {
        _ = input;
        unimplemented!()
    }
}

pub struct FColor256(pub u8);

impl std::fmt::Display for FColor256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\x1B[38;5;{}m", self.0))
    }
}

pub struct BColor256(pub u8);

impl std::fmt::Display for BColor256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\x1B[48;5;{}m", self.0))
    }
}

pub enum FColor8 {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    Reset,
}

impl std::fmt::Display for FColor8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\x1B[{}m",
            match self {
                FColor8::Black => "30",
                FColor8::Red => "31",
                FColor8::Green => "32",
                FColor8::Yellow => "33",
                FColor8::Blue => "34",
                FColor8::Magenta => "35",
                FColor8::Cyan => "36",
                FColor8::White => "37",
                FColor8::Default => "39",
                FColor8::Reset => "0",
            }
        ))
    }
}

pub enum BColor8 {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    Reset,
}

impl std::fmt::Display for BColor8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\x1B[{}m",
            match self {
                BColor8::Black => "40",
                BColor8::Red => "41",
                BColor8::Green => "42",
                BColor8::Yellow => "43",
                BColor8::Blue => "44",
                BColor8::Magenta => "45",
                BColor8::Cyan => "46",
                BColor8::White => "47",
                BColor8::Default => "49",
                BColor8::Reset => "0",
            }
        ))
    }
}
