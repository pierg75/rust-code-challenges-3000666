use core::fmt;
use std::{ffi::CString, fmt::{Debug, Display}};


//fn info<T: AsRef<str>>(a: T) {
//    println!("{}", a.as_ref());
//}

fn info<T: Debug>(a: T) {
    println!("{:?}", a)
}

// impl fmt::Display for Path {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", self.display())
//    }
// }

// impl fmt::Display for CString {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//       write!(f, "{}", self)
//   }
//}

fn main() {
    let a = "?";
    let b = "?".to_string();
    info(&a);
    info(&b);

    // Advanced 1
    use std::ffi::CString;
    
    let c = CString::new("?").unwrap();
    // info(c.into_string().unwrap());
    info(&c);

    // Advanced 2
    use std::path::Path;
    let d = Path::new("/tmp/linkedin-learning");
    info(d);
}


#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

#[test]
fn chars() {
    let input = 'r';
    info(&input);
}

#[test]
fn cstring() {
    use std::ffi::{CString};
    let input = CString::new("Rust").unwrap();
    info(&input);
}

#[test]
fn path() {
    use std::path::Path;
    let input = Path::new("/tmp/rust");
    info(input);
}
