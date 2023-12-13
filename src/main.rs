extern crate chrono;

use chrono::{Datelike, NaiveDate};

fn main() {
    // Define the target date (September 1, 1993)
    let target_date = NaiveDate::from_ymd(1993, 9, 1);

    // Get the current date
    let current_date = chrono::Local::today().naive_utc();

    // Calculate the difference in days
    let days_passed = (current_date - target_date).num_days();

    println!("September {}, 1993", days_passed + 1);
}
