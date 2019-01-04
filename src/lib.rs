#[macro_use]

extern crate lazy_static;

use chrono::NaiveDate;
use regex::Regex;

/// Reduce from the ASCII value for each character to get the proper integer value.
const ASCII_REDUCE: i32 = 48;

/// Will check if a given string is a valid Swedish social security number.
///
/// The validation will be a combination of regular expression, luhn algorithm and date validation.
///
/// ```
/// let result = personnummer::valid("6403273813");
/// assert!(result);
/// ```
pub fn valid(pnr: &str) -> bool {
    lazy_static! {
        static ref PNR_REGEX: Regex = Regex::new(
            r"^(\d{2})?(?P<year>\d{2})(?P<month>\d{2})(?P<day>\d{2})([-|+]?)?(?P<number>\d{3})(?P<control>\d?)$"
        ).unwrap();
    }

    let cap = PNR_REGEX.captures(pnr);
    if cap.is_none() {
        return false;
    }

    println!("{}", pnr);

    let matches = cap.unwrap();
    let year = &matches["year"].parse::<i32>().unwrap();
    let month = &matches["month"].parse::<i32>().unwrap();
    let day = &matches["day"].parse::<i32>().unwrap();
    let number = &matches["number"].parse::<i32>().unwrap();
    let control = &matches["control"].parse::<i32>().unwrap_or(0);

    let ymd = format!("{:02}{:02}{:02}", year, month, day);
    let luhn_value = format!("{:06}{:03}", ymd, number);

    let checksum = luhn(&luhn_value);

    return (&checksum == control) && valid_date(*year, *month, *day);
}

/// Calculate the checksum based on luhn algorithm. See more information here:
/// https://en.wikipedia.org/wiki/Luhn_algorithm.
fn luhn(value: &str) -> i32 {
    let mut sum = 0;
    let mut temp: i32;

    for i in 0..value.len() {
        temp = value.chars().nth(i).unwrap() as i32 - ASCII_REDUCE;

        if i % 2 == 0 {
            temp *= 2;

            if temp > 9 {
                temp -= 9
            }
        }

        sum += temp
    }

    sum = 10 - (sum % 10);

    return sum;
}

/// Validate if a date is valid by passing year, month and day. Year must be passed as two digits
/// only.
fn valid_date(y: i32, m: i32, d: i32) -> bool {
    let final_day = d % 60;
    let ymd = format!("{}-{:02}-{:02}", y, m, final_day);
    return match NaiveDate::parse_from_str(&ymd, "%y-%m-%d") {
        Ok(_) => true,
        Err(_) => false,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_with_control_digit() {
        assert!(valid("6403273813"));
        assert!(valid("510818-9167"));
        assert!(valid("19900101-0017"));
        assert!(valid("19130401+2931"));
        assert!(valid("196408233234"));
        assert!(valid("0001010107"));
        assert!(valid("000101-0107"));
    }

    #[test]
    fn valid_without_control_digit() {
        assert!(!valid("640327-381"));
        assert!(!valid("510818-916"));
        assert!(!valid("19900101-001"));
        assert!(!valid("100101+001"));
    }

    #[test]
    fn valid_invalid_values() {
        assert!(!valid("A string"));
        assert!(!valid("Two"));
        assert!(!valid("222"));
    }

    #[test]
    fn valid_coordination_numbers() {
        assert!(valid("701063-2391"));
        assert!(valid("640883-3231"));
    }

    #[test]
    fn valid_bad_coordination_numbers() {
        assert!(!valid("900161-0017"));
        assert!(!valid("640893-3231"));
    }
}
