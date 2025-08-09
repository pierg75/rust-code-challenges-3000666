use core::fmt;
use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum DigitError {
    TooLong,
    TooShort,
    FailedCHecksum,
    InvalidDigit,
}

impl fmt::Display for DigitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DigitError::TooLong => write!(f, "Digit too long!"),
            DigitError::TooShort => write!(f, "Digit too short!"),
            DigitError::FailedCHecksum => write!(f, "Checksum not valid!"),
            DigitError::InvalidDigit => write!(f, "Invalid digit!"),
        }
    }
}

impl FromStr for Isbn {
    type Err = DigitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut isbn = Isbn {
            raw: s.to_string(),
            digits: Vec::new(),
        };
        for c in s.chars() {
            if c.is_ascii_digit() {
                let Ok(ctoi) = c.to_string().parse::<u8>() else {
                    return Err(DigitError::InvalidDigit);
                };
                isbn.digits.push(ctoi);
            }
        }
        if isbn.digits.len() > 13 {
            return Err(DigitError::TooLong);
        }
        if isbn.digits.len() < 13 {
            return Err(DigitError::TooShort);
        }
        Ok(isbn)
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    // Multiply the right digits by 3 and sum them all.
    // The result will be modulo 10.
    let m_numbers: Vec<u8> = digits
        .iter()
        .enumerate()
        .map(|(idx, digit)| if idx % 2 == 0 { *digit } else { *digit * 3 })
        .collect();
    let last_digit = 10 - (m_numbers.into_iter().sum::<u8>() % 10);
    // The last digit has to be a single digit, with zero replacing ten.
    if last_digit == 10 {
        0
    } else {
        last_digit
    }
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        println!("case: {:?} check {:?}", case, check);
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
