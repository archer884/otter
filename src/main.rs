extern crate grabinput;

use std::io::{self, Write};

trait Mapping {
    fn get(&self, u: u8) -> Option<u8>;

    fn map_str(&self, s: &str) -> String {
        s.bytes().map(|u| self.get(u).unwrap_or(u) as char).collect()
    }
}

struct Rot13;

impl Mapping for Rot13 {
    fn get(&self, u: u8) -> Option<u8> {
        match to_lowerish(u) {
            b'a' => Some(set_case(b'n', u)),
            b'b' => Some(set_case(b'o', u)),
            b'c' => Some(set_case(b'p', u)),
            b'd' => Some(set_case(b'q', u)),
            b'e' => Some(set_case(b'r', u)),
            b'f' => Some(set_case(b's', u)),
            b'g' => Some(set_case(b't', u)),
            b'h' => Some(set_case(b'u', u)),
            b'i' => Some(set_case(b'v', u)),
            b'j' => Some(set_case(b'w', u)),
            b'k' => Some(set_case(b'x', u)),
            b'l' => Some(set_case(b'y', u)),
            b'm' => Some(set_case(b'z', u)),
            b'n' => Some(set_case(b'a', u)),
            b'o' => Some(set_case(b'b', u)),
            b'p' => Some(set_case(b'c', u)),
            b'q' => Some(set_case(b'd', u)),
            b'r' => Some(set_case(b'e', u)),
            b's' => Some(set_case(b'f', u)),
            b't' => Some(set_case(b'g', u)),
            b'u' => Some(set_case(b'h', u)),
            b'v' => Some(set_case(b'i', u)),
            b'w' => Some(set_case(b'j', u)),
            b'x' => Some(set_case(b'k', u)),
            b'y' => Some(set_case(b'l', u)),
            b'z' => Some(set_case(b'm', u)),

            _ => None,
        }
    }
}

fn main() -> io::Result<()> {
    for message in grabinput::from_stdin() {
        print!("{}", Rot13.map_str(&message));
    }

    io::stdout().flush()
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
