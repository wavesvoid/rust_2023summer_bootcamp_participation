use chrono::{ NaiveDate, DateTime, FixedOffset };



const NOW: &str = "2019-06-26";
const DT_FORMAT: &str = "%Y-%m-%d";



fn main() {
    println!("Implement me!");
}


struct User(NaiveDate);

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        Self(NaiveDate::from_ymd_opt(year, month, day).unwrap())
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u32 {
        let date = NaiveDate::parse_from_str(NOW, DT_FORMAT).unwrap();
        date.years_since(self.0).unwrap_or(0u32)
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in vec![
            ((2032, 6, 25), 0),
            ((2019, 6, 27), 0),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn is_18yo() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), true),
            ((1990, 7, 4), true),
            ((0, 1, 1), true),
            ((2018, 1, 1), false),
            ((2001, 6, 27), false),
            ((2001, 6, 26), true),
            ((1970, 1, 1), true),
            ((2019, 6, 25), false),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.is_adult(), expected);
        }
    }
}
