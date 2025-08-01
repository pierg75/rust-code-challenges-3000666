#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

/// Represents a single character
type Letter = Vec<Pulse>;

/// Represents a string of characters
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

fn main() {
    let greeting = "Hello, world".to_string().to_morse_code();

    print_morse_code(&greeting);
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        let mut ret: Message = Vec::with_capacity(self.capacity());
        for c in self.chars() {
            let pulse = match c.to_ascii_lowercase() {
                'a' => vec![Pulse::Short, Pulse::Long],
                'b' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
                'c' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short],
                'd' => vec![Pulse::Long, Pulse::Short, Pulse::Short],
                'e' => vec![Pulse::Short],
                'f' => vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short],
                'g' => vec![Pulse::Long, Pulse::Long, Pulse::Short],
                'h' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
                'i' => vec![Pulse::Short, Pulse::Short],
                'j' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long],
                'k' => vec![Pulse::Long, Pulse::Short, Pulse::Long],
                'l' => vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
                'm' => vec![Pulse::Long, Pulse::Long],
                'n' => vec![Pulse::Long, Pulse::Short],
                'o' => vec![Pulse::Long, Pulse::Long, Pulse::Long],
                'p' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short],
                'q' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long],
                'r' => vec![Pulse::Short, Pulse::Long, Pulse::Short],
                's' => vec![Pulse::Short, Pulse::Short, Pulse::Short],
                't' => vec![Pulse::Long],
                'u' => vec![Pulse::Short, Pulse::Short, Pulse::Long],
                'v' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long],
                'w' => vec![Pulse::Short, Pulse::Long, Pulse::Long],
                'x' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long],
                'y' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long],
                'z' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short],
                '0' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '1' => vec![
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '2' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '3' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '4' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                ],
                '5' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '6' => vec![
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '7' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '8' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '9' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                ],
                _ => continue,
            };
            ret.push(pulse);
        }
        ret
    }
}

//impl std::fmt::Display for Pulse {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        match self {
//            Self::Short => write!(f, "."),
//            Self::Long => write!(f, "_"),
//        }
//    }
//}

#[test]
fn hello_world() {
    use Pulse::*;

    let expected = vec![
        vec![Short, Short, Short, Short],
        vec![Short],
        vec![Short, Long, Short, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Long, Long],
        vec![Short, Long, Long],
        vec![Long, Long, Long],
        vec![Short, Long, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Short, Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();
}
