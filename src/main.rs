extern crate either;
extern crate structopt;

use either::Either;
use std::fs::File;
use std::io::{self, Read};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Config {
    #[structopt(short = "p", long = "path")]
    path: Option<String>,
    #[structopt(short = "o", long = "offset")]
    offset: Option<u8>,
    #[structopt(short = "r", long = "reverse")]
    reverse: bool,
}

impl Config {
    fn build(&self) -> io::Result<Application> {
        use either::Either::*;

        let mapping = match self.offset {
            None => Left(Rot13),
            Some(n) => Right(RotBy::new(n, self.reverse)),
        };

        let source = match self.path {
            None => None,
            Some(ref path) => Some(File::open(path)?),
        };
        
        Ok(Application { mapping, source })
    }
}

struct MappingTransform<M, R> {
    mapping: M,
    stream: R,
}

impl<M, R> MappingTransform<M, R> {
    fn new(mapping: M, stream: R) -> Self {
        MappingTransform { mapping, stream }
    }
}

impl<M, R> Read for MappingTransform<M, R>
where
    M: Mapping,
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let result = self.stream.read(buf)?;
        for place in buf.iter_mut() {
            match self.mapping.get(*place) {
                None => (),
                Some(replace_with) => *place = replace_with,
            }
        }
        Ok(result)
    }
}

struct Application {
    mapping: Either<Rot13, RotBy>,
    source: Option<File>,
}

impl Application {
    fn run(self) -> io::Result<()> {
        let Self { mapping, source } = self;

        match source {
            None => map_input(mapping, io::stdin())?,
            Some(file) => map_input(mapping, file)?,
        };

        Ok(())
    }
}

fn map_input(mapping: impl Mapping, input: impl Read) -> io::Result<u64> {
    io::copy(&mut MappingTransform::new(mapping, input), &mut io::stdout())
}

trait Mapping {
    fn get(&self, u: u8) -> Option<u8>;

    fn map_str(&self, s: &str) -> String {
        s.bytes()
            .map(|u| self.get(u).unwrap_or(u) as char)
            .collect()
    }
}

struct Rot13;

impl Mapping for Rot13 {
    fn get(&self, u: u8) -> Option<u8> {
        fn try_map(u: u8) -> Option<u8> {
            match u {
                b'a' => Some(b'n'),
                b'b' => Some(b'o'),
                b'c' => Some(b'p'),
                b'd' => Some(b'q'),
                b'e' => Some(b'r'),
                b'f' => Some(b's'),
                b'g' => Some(b't'),
                b'h' => Some(b'u'),
                b'i' => Some(b'v'),
                b'j' => Some(b'w'),
                b'k' => Some(b'x'),
                b'l' => Some(b'y'),
                b'm' => Some(b'z'),
                b'n' => Some(b'a'),
                b'o' => Some(b'b'),
                b'p' => Some(b'c'),
                b'q' => Some(b'd'),
                b'r' => Some(b'e'),
                b's' => Some(b'f'),
                b't' => Some(b'g'),
                b'u' => Some(b'h'),
                b'v' => Some(b'i'),
                b'w' => Some(b'j'),
                b'x' => Some(b'k'),
                b'y' => Some(b'l'),
                b'z' => Some(b'm'),

                _ => None,
            }
        }

        try_map(to_lowerish(u)).map(|new| set_case(new, u))
    }
}

struct RotBy(u8);

impl RotBy {
    fn new(n: u8, reverse: bool) -> Self {
        if reverse {
            RotBy(26 - n)
        } else {
            RotBy(n)
        }
    }

    // FIXME: I worry how this will handle negative offsets.
    fn shift(&self, u: u8) -> u8 {
        (u + self.0) % 26
    }
}

impl Mapping for RotBy {
    fn get(&self, u: u8) -> Option<u8> {
        const OFFSET: u8 = b'a';

        let result = match to_lowerish(u) {
            letter @ b'a'...b'z' => Some(self.shift(letter - OFFSET) + OFFSET),
            _ => None,
        };

        result.map(|new| set_case(new, u))
    }
}

impl Mapping for Either<Rot13, RotBy> {
    fn get(&self, u: u8) -> Option<u8> {
        use either::Either::*;
        
        match *self {
            Left(ref mapping) => mapping.get(u),
            Right(ref mapping) => mapping.get(u),
        }
    }
}

fn main() -> io::Result<()> {
    let config = Config::from_args();
    let application = config.build()?;

    application.run()

    // for message in grabinput::from_stdin() {
    //     print!("{}", Rot13.map_str(&message));
    // }

    // io::stdout().flush()
}

/// Converts a byte to a lowercase-ish byte by setting the 32-bit if it is not set.
///
/// In theory, this works because the difference between upper- and lower-case ASCII characters
/// is the state of the 32-bit.
fn to_lowerish(u: u8) -> u8 {
    u | 32
}

/// Sets the case of a byte based on the case of the original byte.
///
/// In theory, this one works by magic. Gfy.
fn set_case(u_new: u8, u_original: u8) -> u8 {
    if u_original | 32 == u_original {
        u_new
    } else {
        u_new & 0b11011111
    }
}

#[cfg(test)]
mod tests {
    use super::{Mapping, RotBy};

    #[test]
    fn rot_by_1() {
        let mapping = RotBy::new(1, false);
        assert_eq!("Ifmmp, xpsme!", &*mapping.map_str("Hello, world!"));
    }

    #[test]
    fn unrot_by_1() {
        let mapping = RotBy::new(1, true);
        assert_eq!("Hello, world!", &*mapping.map_str("Ifmmp, xpsme!"));
    }
}
