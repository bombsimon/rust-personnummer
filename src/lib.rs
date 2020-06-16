#[macro_use]
extern crate lazy_static;

use chrono::{Datelike, NaiveDate, Utc};
use regex::{Match, Regex};

/// Reduce from the ASCII value for each character to get the proper integer value.
const ASCII_REDUCE: i32 = 48;

/// The extra value added to coordination numbers.
const COORDINATION_NUMBER: u32 = 60;

#[allow(dead_code)]
/// Personnummer holds relevant data to check for valid personal identity numbers.
pub struct Personnummer {
    date: chrono::NaiveDate,
    serial: u32,
    control: u8,
    divider: char,
    coordination: bool,
}

/// FormattedPersonnummer holds two formats of a normalized personal identity number, one long and
/// one short format. The long format displays the full century while the short format only
/// displays the year.
pub struct FormattedPersonnummer {
    long: String,
    short: String,
}

impl FormattedPersonnummer {
    /// Returns the long format of a formatted personal identity number as a String.
    pub fn long(&self) -> String {
        self.long.clone()
    }

    /// Returns the short format of a formatted personal identity number as a String.
    pub fn short(&self) -> String {
        self.short.clone()
    }
}

impl Personnummer {
    /// Returns a new instance of a Personnummer. Panics for invalid dates but not for invalid
    /// personal identity numbers. Use valid() to check validity.
    pub fn new(pnr: &str) -> Personnummer {
        Personnummer::parse(pnr).expect("invalid personal identity number")
    }

    /// Same as new() but returns an Option instead of panicing on invalid dates.
    pub fn parse(pnr: &str) -> Option<Personnummer> {
        lazy_static! {
            static ref PNR_REGEX: Regex = Regex::new(
                r"^(?P<century>\d{2})?(?P<year>\d{2})(?P<month>\d{2})(?P<day>\d{2})(?P<divider>[-|+]?)?(?P<number>\d{3})(?P<control>\d?)$"
            ).unwrap();
        }

        let caps = PNR_REGEX.captures(pnr)?;

        let century = match caps.name("century") {
            Some(m) => m.as_str().parse::<u32>().unwrap_or(19) * 100,
            None => 1900,
        };

        #[inline]
        fn match_to_u32(m: Option<Match<'_>>) -> u32 {
            m.unwrap().as_str().parse::<u32>().unwrap_or(0)
        }

        let year = match_to_u32(caps.name("year"));
        let month = match_to_u32(caps.name("month"));
        let day = match_to_u32(caps.name("day"));
        let serial = match_to_u32(caps.name("number"));

        let control = caps
            .name("control")
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap_or(0);

        let divider = caps
            .name("divider")
            .unwrap()
            .as_str()
            .parse::<char>()
            .unwrap_or('\0');

        let date = match NaiveDate::from_ymd_opt(
            (century + year) as i32,
            month,
            day % COORDINATION_NUMBER,
        ) {
            Some(date) => date,
            None => return None,
        };

        Some(Personnummer {
            date,
            serial,
            control,
            divider,
            coordination: (day > 31),
        })
    }

    /// Returns a FormattedPersonnummer from a Personnummer which can be used to display a
    /// normalized version of the Personnummer.
    pub fn format(&self) -> FormattedPersonnummer {
        let day = self.date.day();
        let day_or_coordination = if self.coordination {
            day + COORDINATION_NUMBER
        } else {
            day
        };

        let long = format!(
            "{}{:02}{:02}-{:03}{}",
            self.date.year(),
            self.date.month(),
            day_or_coordination,
            self.serial,
            self.control
        );

        let short = String::from(&long.clone()[2..]);

        FormattedPersonnummer { long, short }
    }

    /// Validate a Personnummer. The validation requires a valid date and that the Luhn checksum
    /// matches the control digit.
    pub fn valid(&self) -> bool {
        let ymd = format!(
            "{:02}{:02}{:02}",
            self.date.year() % 100,
            self.date.month(),
            self.date.day()
        );

        let to_control = format!("{:06}{:03}", ymd, self.serial);

        self.serial > 0 && luhn(to_control) == self.control
    }

    /// Return the age of the person holding the personal identity number. The dates used for the
    /// person and the current date are naive dates.
    pub fn get_age(&self) -> i32 {
        let born = self.date.and_hms(0, 0, 0);
        let now = Utc::now();

        (now.naive_utc().signed_duration_since(born).num_days() as f64 / 365.25) as i32
    }

    /// Check if the person holding the personal identity number is a female.
    pub fn is_female(&self) -> bool {
        (self.serial % 10) % 2 == 0
    }

    /// Check if the person holding the personal identity number is a male.
    pub fn is_male(&self) -> bool {
        !self.is_female()
    }

    /// Check if the personal identity number is a coordination number.
    pub fn is_coordination_number(&self) -> bool {
        self.coordination
    }
}

/// Calculate the checksum based on luhn algorithm. See more information here:
/// https://en.wikipedia.org/wiki/Luhn_algorithm.
fn luhn(value: String) -> u8 {
    let mut sum = 0;

    for i in 0..value.len() {
        let mut digit = value.chars().nth(i).unwrap() as i32 - ASCII_REDUCE;

        if i % 2 == 0 {
            digit *= 2;

            if digit > 9 {
                digit -= 9
            }
        }

        sum += digit
    }

    let checksum = 10 - (sum % 10);
    if checksum == 10 {
        0
    } else {
        checksum as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use std::collections::HashMap;

    #[test]
    fn test_invalid_date() {
        let cases = vec!["19901301-1111", "2017-02-29", "", "not-a-date"];

        for tc in cases {
            assert!(Personnummer::parse(tc).is_none());
        }
    }

    #[test]
    fn test_valid_date_invalid_digits() {
        let cases = vec![
            "19900101-1111",
            "20160229-1111",
            "6403273814",
            "20150916-0006",
        ];

        for tc in cases {
            assert!(!Personnummer::new(tc).valid());
        }
    }

    #[test]
    fn test_valid_personal_identity_number() {
        let cases = vec![
            "19900101-0017",
            "196408233234",
            "000101-0107",
            "510818-9167",
            "19130401+2931",
        ];

        for tc in cases {
            assert!(Personnummer::new(tc).valid());
        }
    }

    #[test]
    fn test_age() {
        let now = Utc::now();

        let days_in_a_year = 365;
        let leap_years_in_20_years = 20 / 4;
        let twenty_years_ago = (days_in_a_year * 20) + leap_years_in_20_years;

        let leap_years_in_100_years = 100 / 4;
        let hundred_years_ago = (days_in_a_year * 100) + leap_years_in_100_years;

        let twenty_tomorrow_date = (now - Duration::days(twenty_years_ago - 1)).date();
        let twenty_tomorrow = format!(
            "{}{:02}{:02}-1111",
            twenty_tomorrow_date.year(),
            twenty_tomorrow_date.month(),
            twenty_tomorrow_date.day()
        );

        let twenty_yesterday_date = (now - Duration::days(twenty_years_ago + 1)).date();
        let twenty_yesterday = format!(
            "{}{:02}{:02}-1111",
            twenty_yesterday_date.year(),
            twenty_yesterday_date.month(),
            twenty_yesterday_date.day()
        );

        let hundred_years_ago_date = (now - Duration::days(hundred_years_ago)).date();
        let hundred_years_age = format!(
            "{}{:02}{:02}-1111",
            hundred_years_ago_date.year(),
            hundred_years_ago_date.month(),
            hundred_years_ago_date.day()
        );

        let mut cases: HashMap<&str, i32> = HashMap::new();

        cases.insert(twenty_tomorrow.as_str(), 19);
        cases.insert(twenty_yesterday.as_str(), 20);
        cases.insert(hundred_years_age.as_str(), 100);

        for (pnr, age) in cases {
            assert_eq!(Personnummer::new(pnr).get_age(), age);
        }
    }

    #[test]
    fn test_gender() {
        let mut cases: HashMap<&str, bool> = HashMap::new();

        cases.insert("19090903-6600", true);
        cases.insert("19900101-0017", false);
        cases.insert("800101-3294", false);
        cases.insert("000903-6609", true);
        cases.insert("800101+3294", false);

        for (pnr, is_female) in cases {
            let p = Personnummer::new(pnr);

            assert!(p.valid());
            assert_eq!(p.is_female(), is_female);
            assert_eq!(p.is_male(), !is_female);
        }
    }

    #[test]
    fn test_coordination() {
        let mut cases: HashMap<&str, bool> = HashMap::new();

        cases.insert("800161-3294", true);
        cases.insert("800101-3294", false);
        cases.insert("640327-3813", false);

        for (pnr, is_coordination) in cases {
            let p = Personnummer::new(pnr);

            assert!(p.valid());
            assert_eq!(p.is_coordination_number(), is_coordination);
        }
    }
}
