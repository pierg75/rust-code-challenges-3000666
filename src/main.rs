use chrono::NaiveDate;

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`.
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let trimmed = text.trim();

    if !trimmed.chars().any(|x| x.is_numeric()) {
        return None;
    }

    let date: Vec<&str> = trimmed.split(['.', '-', '/']).collect();

    if date.len() != 3 {
        return None;
    }

    let first = date[0];
    let second = date[1];
    let _third = date[2];
    if first.chars().all(|x| x.is_alphabetic()) && first.len() == 3 {
        return NaiveDate::parse_from_str(date.concat().as_str(), "%b%d%Y").ok();
    }
    if first.chars().all(|x| x.is_numeric()) && first.len() == 4 {
        if !second.is_empty() && second.len() == 2 && second.chars().all(|x| x.is_numeric()) {
            return NaiveDate::parse_from_str(date.concat().as_str(), "%Y%m%d").ok();
        }
        return NaiveDate::parse_from_str(date.concat().as_str(), "%Y%b%d").ok();
    }
    if first.chars().all(|x| x.is_numeric()) && first.len() == 2 {
        if !second.is_empty() && second.len() == 3 && second.chars().all(|x| x.is_alphabetic()) {
                return NaiveDate::parse_from_str(date.concat().as_str(), "%d%b%Y").ok();
        }
        return NaiveDate::parse_from_str(date.concat().as_str(), "%d%m%Y").ok();
    }
    None
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        " Mar.05.2021 ",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }
}

#[test]
fn ymd_hyphen() {
    assert_eq!(
        flexible_date_parse("2010-12-11"),
        Some(NaiveDate::from_ymd(2010, 12, 11))
    )
}

#[test]
fn ymd_hyphen_with_spaces() {
    assert_eq!(
        flexible_date_parse("   2010-10-05 "),
        Some(NaiveDate::from_ymd(2010, 10, 05))
    )
}

#[test]
fn ymd_hyphen_wrong() {
    assert_eq!(flexible_date_parse("2010-12-11-02"), None)
}

#[test]
fn ymd_slash() {
    assert_eq!(
        flexible_date_parse("1999/Mar/02"),
        Some(NaiveDate::from_ymd(1999, 3, 2))
    )
}

#[test]
fn dmy_dot() {
    assert_eq!(
        flexible_date_parse("01.Mar.2021"),
        Some(NaiveDate::from_ymd(2021, 3, 1))
    )
}

#[test]
fn mdy_dot() {
    assert_eq!(
        flexible_date_parse("Apr.05.2021"),
        Some(NaiveDate::from_ymd(2021, 4, 5))
    )
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}
