mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut new_text = String::new();
        let mut counter = 1;

        for (idx, value) in text.chars().enumerate() {
            if let Some(next_char) = text.chars().nth(idx + 1) {
                if next_char.ne(&value) || counter >= 9 {
                    new_text.push_str(format!("{}{}", counter, value).as_str());
                    counter = 1;
                } else {
                    counter += 1;
                }
            } else {
                new_text.push_str(format!("{}{}", counter, value).as_str());
            }
        }
        new_text
    }

    pub fn decode(text: &str) -> String {
        let mut new_text = String::new();
        let mut count: char = ' ';
        for c in text.chars() {
            if c.is_numeric() {
                count = c;
            } else {
                let counti = match count.to_digit(10) {
                    Some(n) => n as usize,
                    None => continue,
                };
                new_text.push_str(String::from(c).repeat(counti).as_str());
            }
        }
        new_text

    }
}

fn main() {
    //
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn empty() {
    use run_length_encoding::*;

    assert_eq!(encode(""), "");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
