use chrono::{Datelike, Month};
use num_traits::FromPrimitive;

fn main() {
    let age = calculate_age(1997, 4, 1).unwrap();
    println!("{}", age);
}

fn calculate_age(year: i32, month: u8, day: u8) -> Result<i32, String> {
    if !(1..=12).contains(&month) {
        return Err("Month cannot be greater than 12".to_string());
    }
    
    match month {
        2 => {
            if year % 4 == 0 && day > 29 {
                return format_err(month, day);
            }
            if day > 28 && year % 4 != 0 {
                return format_err(month, day);
            }
        }
        4 | 6 | 9 | 11 => {
            if day > 30 {
                return format_err(month, day);
            }
        }
        _ => {
            if day > 31 {
                return format_err(month, day);
            }
        }
    }

    let current_date = chrono::Utc::now();

    let current_month = current_date.month();
    let current_day = current_date.day();
    let current_year = current_date.year();

    if month > current_month as u8 || month == current_month as u8 && day > current_day as u8 {
        Ok(current_year - year - 1)
    } else {
        Ok(current_year - year)
    }
}

fn format_err(month: u8, day: u8) -> Result<i32, String> {
    return Err(format!(
        "Day {} not exists on month {:?}",
        day,
        Month::from_u32(month as u32).unwrap()
    ));
}
