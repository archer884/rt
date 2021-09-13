use clap::Clap;

#[derive(Clap, Clone, Debug)]
struct Opts {
    message: String,

    #[clap(short, long, group = "offsetgrp")]
    offset: Option<u32>,

    /// an inverted offset
    ///
    /// Primarily useful for removing nonstandard offsets.
    #[clap(short, long, group = "offsetgrp")]
    invert: Option<u32>,
}

impl Opts {
    fn normal_offset(&self) -> i32 {
        if let Some(invert) = self.invert {
            let offset = invert as i32 % 26;
            26 - offset
        } else {
            let offset = self.offset.unwrap_or(13) as i32;
            offset % 26
        }
    }
}

fn main() {
    let opts = Opts::parse();
    let offset = opts.normal_offset();
    let message: String = opts.message.chars().map(|u| rotate(u, offset)).collect();

    println!("{}", message);
}

fn rotate(u: char, offset: i32) -> char {
    if !u.is_ascii_alphabetic() {
        return u;
    }

    let upper_case = u.is_uppercase();
    let u = (u.to_ascii_uppercase() as u8 - b'A') as i32;
    let normal = normalize(u + offset);

    if upper_case {
        normal as char
    } else {
        normal.to_ascii_lowercase() as char
    }
}

fn normalize(u: i32) -> u8 {
    (u % 26) as u8 + b'A'
}

#[cfg(test)]
mod tests {
    #[test]
    fn offset() {
        let opts = super::Opts { message: String::from("Hello, world!"), offset: None, invert: Some(23) };
        assert_eq!(3, opts.normal_offset());

        let opts = super::Opts { message: String::from("Hello, world!"), offset: None, invert: None };
        assert_eq!(13, opts.normal_offset());
    }

    #[test]
    fn rotate() {
        assert_eq!('I', super::rotate('V', 13));
        assert_eq!('V', super::rotate('I', 13));
        assert_eq!(',', super::rotate(',', 13));
    }
}
