use chrono::Datelike;

fn main () {
    let age = calculate_age(1997, 4, 1).unwrap();
    println!("{}", age);
}

fn calculate_age(year: i32, month: u8, day: u8) -> Result<i32, String>{
    let month_ext = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    if !(1..=12).contains(&month) {
        return Err("Month cannot be greater than 12".to_string());
    }

    match month {
        2 => {
            if year % 4 == 0 && day > 29 {
                return Err(format!("Day cannot be greater than 29 on {}", month_ext[month as usize - 1]));
            } else if day > 28 && year % 4 != 0{
                return Err(format!("Day cannot be greater than 28 on {}", month_ext[month as usize - 1]));
            }
        },
        4 | 6 | 9 | 11 => {
            if day > 30 {
                return Err(format!("Day cannot be greater than 30 on {}", month_ext[month as usize - 1]));
            }
        },
        _ => {
            if day > 31 {
                return Err(format!("Day cannot be greater than 31 on {}", month_ext[month as usize - 1]));
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
