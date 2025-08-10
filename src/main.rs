use core::fmt;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
#[non_exhaustive]
enum ColourError {
    WrongString,
    ParseError(ParseIntError),
}

impl fmt::Display for ColourError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongString => write!(f, "This is not a valid colour format"),
            Self::ParseError(err) => write!(f, "Error parsing this Hex value: {}", err),
        }
    }
}

impl From<ParseIntError> for ColourError {
    fn from(value: ParseIntError) -> Self {
        ColourError::ParseError(value)
    }
}

impl std::error::Error for ColourError {}

#[derive(Debug, PartialEq, Default)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.red
    }
    fn g(&self) -> u8 {
        self.green
    }
    fn b(&self) -> u8 {
        self.blue
    }
}

impl FromStr for Rgb {
    type Err = ColourError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rgb = Rgb {
            red: 0,
            green: 0,
            blue: 0,
        };
        
        let trimmed_s = match s.starts_with("#") {
            true => s.trim_matches('#'),
            false => return  Err(ColourError::WrongString),
        };

        if trimmed_s.len() != 6 {
            return Err(ColourError::WrongString);
        }

        if !trimmed_s.chars().all(|x| x.is_ascii_hexdigit()) {
            return Err(ColourError::WrongString);
        }

        rgb.red = u8::from_str_radix(&trimmed_s[0..2], 16).unwrap();
        rgb.green = u8::from_str_radix(&trimmed_s[2..4], 16).unwrap();
        rgb.blue = u8::from_str_radix(&trimmed_s[4..=5], 16).unwrap();

        Ok(rgb)
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    //
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        println!("{}", hex);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
