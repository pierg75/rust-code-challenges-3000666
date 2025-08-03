use chrono::{Datelike, NaiveDate};

fn weeks_between(a: &str, b: &str) -> i32 {
    let parsed_datea = NaiveDate::parse_from_str(a, "%Y-%m-%d").unwrap();
    let parsed_dateb = NaiveDate::parse_from_str(b, "%Y-%m-%d").unwrap();

    let weeka = parsed_datea.iso_week().week() as i32;
    let weekb = parsed_dateb.iso_week().week() as i32;
    weekb - weeka
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}
