use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime, TimeZone};
use chrono_tz::{Asia::Jakarta, America::Los_Angeles, Tz};
use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }

    let command = args[1].to_lowercase();
    
    match command.as_str() {
        "future" | "date" => handle_future_date(&args[2..]),
        "past" => handle_past_date(&args[2..]),
        "convert" | "tz" => handle_timezone_convert(&args[2..]),
        "remaining" | "left" => handle_remaining(&args[2..]),
        "day" => handle_day_of_week(&args[2..]),
        "help" | "--help" | "-h" => print_help(),
        _ => {
            println!("ERROR: Unknown command: {}", command);
            print_help();
        }
    }
}

fn handle_future_date(args: &[String]) {
    if args.is_empty() {
        println!("ERROR: Please specify number of days");
        println!("Example: timecalc future 69 days");
        return;
    }

    let days = parse_days(args);
    if days.is_none() {
        println!("ERROR: Could not parse days. Use format like: 69 days, 69d, or just 69");
        return;
    }

    let days = days.unwrap();
    let today = Local::now();
    let future_date = today + Duration::days(days);
    
    println!("\nDATE CALCULATION");
    println!("=====================================");
    println!("TODAY:        {} ({})", 
             today.format("%A, %B %d, %Y"), 
             today.format("%Y-%m-%d"));
    println!("AFTER {} DAYS: {} ({})", 
             days,
             future_date.format("%A, %B %d, %Y"),
             future_date.format("%Y-%m-%d"));
    println!("=====================================\n");
}

fn handle_past_date(args: &[String]) {
    if args.is_empty() {
        println!("ERROR: Please specify number of days");
        return;
    }

    let days = parse_days(args);
    if days.is_none() {
        println!("ERROR: Could not parse days");
        return;
    }

    let days = days.unwrap();
    let today = Local::now();
    let past_date = today - Duration::days(days);
    
    println!("\nDATE CALCULATION");
    println!("=====================================");
    println!("TODAY:        {} ({})", 
             today.format("%A, %B %d, %Y"), 
             today.format("%Y-%m-%d"));
    println!("{} DAYS AGO:  {} ({})", 
             days,
             past_date.format("%A, %B %d, %Y"),
             past_date.format("%Y-%m-%d"));
    println!("=====================================\n");
}

fn handle_timezone_convert(args: &[String]) {
    if args.len() < 4 {
        println!("ERROR: Invalid format");
        println!("Example: timecalc convert 4:00 UTC+7 to WIB");
        println!("         timecalc convert 10:00 PST to WIB");
        return;
    }

    let time_str = &args[0];
    let from_tz_str = &args[1];
    let to_tz_str = &args[3];

    // Parse time
    let time_parts: Vec<&str> = time_str.split(':').collect();
    if time_parts.len() < 2 {
        println!("ERROR: Invalid time format. Use HH:MM");
        return;
    }

    let hour: u32 = time_parts[0].parse().unwrap_or(0);
    let minute: u32 = time_parts[1].parse().unwrap_or(0);

    if hour > 23 || minute > 59 {
        println!("ERROR: Invalid time values");
        return;
    }

    let naive_time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
    let today = Local::now().date_naive();
    let naive_datetime = today.and_time(naive_time);

    // Parse timezones
    let from_tz = parse_timezone(from_tz_str);
    let to_tz = parse_timezone(to_tz_str);

    if from_tz.is_none() || to_tz.is_none() {
        println!("ERROR: Unsupported timezone");
        println!("Supported: WIB, UTC, UTC+7, UTC-7, PST, EST, JST");
        return;
    }

    let from_tz = from_tz.unwrap();
    let to_tz = to_tz.unwrap();

    let from_dt = from_tz.from_local_datetime(&naive_datetime).unwrap();
    let to_dt = from_dt.with_timezone(&to_tz);

    println!("\nTIMEZONE CONVERSION");
    println!("=====================================");
    println!("FROM: {} {}", from_dt.format("%H:%M"), from_tz_str.to_uppercase());
    println!("TO:   {} {}", to_dt.format("%H:%M"), to_tz_str.to_uppercase());
    println!("DATE: {}", to_dt.format("%A, %B %d, %Y"));
    println!("=====================================\n");
}

fn handle_remaining(args: &[String]) {
    if args.is_empty() {
        println!("ERROR: Specify 'month' or 'year'");
        return;
    }

    let period = args[0].to_lowercase();
    let today = Local::now();

    match period.as_str() {
        "month" => {
            let last_day = get_last_day_of_month(today.year(), today.month());
            let remaining = last_day.signed_duration_since(today.date_naive()).num_days();
            
            println!("\nDAYS REMAINING");
            println!("=====================================");
            println!("TODAY:           {}", today.format("%B %d, %Y"));
            println!("END OF MONTH:    {}", last_day.format("%B %d, %Y"));
            println!("DAYS REMAINING:  {} days", remaining);
            println!("DAYS PASSED:     {} days", today.day());
            println!("=====================================\n");
        }
        "year" => {
            let last_day_of_year = NaiveDate::from_ymd_opt(today.year(), 12, 31).unwrap();
            let remaining = last_day_of_year.signed_duration_since(today.date_naive()).num_days();
            let day_of_year = today.ordinal();
            
            println!("\nDAYS REMAINING");
            println!("=====================================");
            println!("TODAY:           {}", today.format("%B %d, %Y"));
            println!("END OF YEAR:     December 31, {}", today.year());
            println!("DAYS REMAINING:  {} days", remaining);
            println!("DAYS PASSED:     {} days", day_of_year);
            println!("=====================================\n");
        }
        _ => println!("ERROR: Use 'month' or 'year'"),
    }
}

fn handle_day_of_week(args: &[String]) {
    if args.is_empty() {
        println!("ERROR: Please provide a date");
        println!("Example: timecalc day 2025-12-25");
        return;
    }

    let date_str = &args[0];
    let date = NaiveDate::from_str(date_str);

    if date.is_err() {
        println!("ERROR: Invalid date format. Use YYYY-MM-DD");
        return;
    }

    let date = date.unwrap();
    let weekday = date.format("%A");
    
    println!("\nDAY OF WEEK");
    println!("=====================================");
    println!("DATE: {}", date.format("%B %d, %Y"));
    println!("DAY:  {}", weekday);
    println!("=====================================\n");
}

fn parse_days(args: &[String]) -> Option<i64> {
    if args.is_empty() {
        return None;
    }

    let num_str = args[0].trim_end_matches("days")
                          .trim_end_matches("day")
                          .trim_end_matches('d')
                          .trim();
    
    num_str.parse::<i64>().ok()
}

fn parse_timezone(tz_str: &str) -> Option<Tz> {
    let tz_upper = tz_str.to_uppercase();
    
    match tz_upper.as_str() {
        "WIB" | "UTC+7" => Some(Jakarta),
        "UTC" | "UTC+0" => Some(Tz::UTC),
        "PST" | "UTC-8" => Some(Los_Angeles),
        "EST" | "UTC-5" => Some(Tz::EST5EDT),
        "JST" | "UTC+9" => Some(Tz::Japan),
        "UTC+8" => Some(Tz::Hongkong),
        "UTC-7" => Some(Tz::MST7MDT),
        _ => None,
    }
}

fn get_last_day_of_month(year: i32, month: u32) -> NaiveDate {
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };
    
    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
}

fn print_help() {
    println!("\nTIME CALCULATOR CLI");
    println!("=========================================================");
    println!("\nFUTURE/PAST DATES:");
    println!("  timecalc future 69 days    - Calculate date 69 days from now");
    println!("  timecalc future 69d        - Short form");
    println!("  timecalc past 30 days      - Calculate date 30 days ago");
    
    println!("\nTIMEZONE CONVERSION:");
    println!("  timecalc convert 4:00 UTC+7 to WIB");
    println!("  timecalc convert 10:00 PST to WIB");
    println!("  timecalc tz 14:30 WIB to UTC");
    
    println!("\nREMAINING DAYS:");
    println!("  timecalc remaining month   - Days left in current month");
    println!("  timecalc remaining year    - Days left in current year");
    println!("  timecalc left month        - Same as above");
    
    println!("\nDAY OF WEEK:");
    println!("  timecalc day 2025-12-25    - What day is this date?");
    
    println!("\nSUPPORTED TIMEZONES:");
    println!("  WIB (UTC+7), UTC, PST (UTC-8), EST (UTC-5), JST (UTC+9)");
    println!("  UTC+7, UTC+8, UTC-7, etc.");
    
    println!("\n=========================================================\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_days_numeric() {
        let args = vec!["69".to_string()];
        assert_eq!(parse_days(&args), Some(69));
    }

    #[test]
    fn test_parse_days_with_suffix() {
        let args = vec!["69days".to_string()];
        assert_eq!(parse_days(&args), Some(69));
        
        let args = vec!["30d".to_string()];
        assert_eq!(parse_days(&args), Some(30));
    }

    #[test]
    fn test_parse_timezone_wib() {
        assert!(parse_timezone("WIB").is_some());
        assert!(parse_timezone("wib").is_some());
        assert!(parse_timezone("UTC+7").is_some());
    }

    #[test]
    fn test_parse_timezone_utc() {
        assert!(parse_timezone("UTC").is_some());
        assert!(parse_timezone("utc").is_some());
    }

    #[test]
    fn test_parse_timezone_invalid() {
        assert!(parse_timezone("INVALID").is_none());
        assert!(parse_timezone("XYZ").is_none());
    }

    #[test]
    fn test_get_last_day_of_month() {
        // January has 31 days
        let last_day = get_last_day_of_month(2025, 1);
        assert_eq!(last_day.day(), 31);
        
        // February 2025 has 28 days (not leap year)
        let last_day = get_last_day_of_month(2025, 2);
        assert_eq!(last_day.day(), 28);
        
        // April has 30 days
        let last_day = get_last_day_of_month(2025, 4);
        assert_eq!(last_day.day(), 30);
    }

    #[test]
    fn test_get_last_day_of_month_leap_year() {
        // February 2024 has 29 days (leap year)
        let last_day = get_last_day_of_month(2024, 2);
        assert_eq!(last_day.day(), 29);
    }
}