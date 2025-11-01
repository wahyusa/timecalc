use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime, TimeZone};
use chrono_tz::{Asia::Jakarta, America::Los_Angeles, Tz};

pub fn handle_future_date(args: &[String]) {
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

pub fn handle_past_date(args: &[String]) {
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

pub fn handle_timezone_convert(args: &[String]) {
    if args.len() < 4 {
        println!("ERROR: Invalid format");
        println!("Example: timecalc convert 4:00 UTC+7 to WIB");
        println!("         timecalc tz October 9, 2025 at 04:00AM UTC+8 to WIB");
        return;
    }

    // Find "to" keyword position
    let to_pos = args.iter().position(|s| s.to_lowercase() == "to");
    if to_pos.is_none() {
        println!("ERROR: Missing 'to' keyword");
        return;
    }
    let to_pos = to_pos.unwrap();

    // Extract parts: everything before "to" is source, after is destination
    let from_parts = &args[..to_pos];
    let to_tz_str = &args[to_pos + 1];

    // Parse the from_parts to extract date, time, and timezone
    let (naive_datetime, from_tz_str) = parse_datetime_and_tz(from_parts);
    if naive_datetime.is_none() {
        println!("ERROR: Could not parse date/time");
        println!("Examples:");
        println!("  timecalc tz 4:00 UTC+7 to WIB");
        println!("  timecalc tz 04:00AM UTC+8 to WIB");
        println!("  timecalc tz October 9, 2025 at 04:00AM UTC+8 to WIB");
        println!("  timecalc tz 2025-10-09 04:00 UTC+8 to WIB");
        return;
    }

    let naive_datetime = naive_datetime.unwrap();

    // Parse timezones
    let from_tz = parse_timezone(&from_tz_str);
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
    println!("FROM: {} {} {}",
             from_dt.format("%A, %B %d, %Y"),
             from_dt.format("%H:%M"),
             from_tz_str.to_uppercase());
    println!("TO:   {} {} {}",
             to_dt.format("%A, %B %d, %Y"),
             to_dt.format("%H:%M"),
             to_tz_str.to_uppercase());
    println!("=====================================\n");
}

pub fn handle_remaining(args: &[String]) {
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

pub fn handle_day_of_week(args: &[String]) {
    if args.is_empty() {
        println!("ERROR: Please provide a date");
        println!("Example: timecalc day 2025-12-25");
        return;
    }

    let date_str = &args[0];
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d");

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

pub fn parse_days(args: &[String]) -> Option<i64> {
    if args.is_empty() {
        return None;
    }

    let num_str = args[0].trim_end_matches("days")
        .trim_end_matches("day")
        .trim_end_matches('d')
        .trim();

    num_str.parse::<i64>().ok()
}

pub fn parse_timezone(tz_str: &str) -> Option<Tz> {
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

pub fn parse_datetime_and_tz(parts: &[String]) -> (Option<chrono::NaiveDateTime>, String) {
    // Try to find timezone at the end (last token before "to")
    if parts.is_empty() {
        return (None, String::new());
    }

    let tz_str = parts[parts.len() - 1].clone();
    let datetime_str = parts[..parts.len() - 1].join(" ");

    // Try various date/time formats
    let naive_dt = parse_flexible_datetime(&datetime_str);
    
    (naive_dt, tz_str)
}

pub fn parse_flexible_datetime(input: &str) -> Option<chrono::NaiveDateTime> {
    let input_lower = input.to_lowercase();

    // Remove "at" keyword if present
    let cleaned = input_lower.replace(" at ", " ");

    // Extract time (with optional AM/PM)
    let time = extract_time(&cleaned)?;

    // Extract date
    let date = extract_date(&cleaned);

    Some(date.and_time(time))
}

pub fn extract_time(input: &str) -> Option<NaiveTime> {
    use regex::Regex;

    // Look for time patterns: HH:MM, HH:MMAM, HH:MMPM, H:MM, etc.
    let pattern = r"(\d{1,2}):(\d{2})\s*(am|pm)?";

    if let Ok(re) = Regex::new(pattern) {
        if let Some(caps) = re.captures(input) {
            let mut hour: u32 = caps.get(1)?.as_str().parse().ok()?;
            let minute: u32 = caps.get(2)?.as_str().parse().ok()?;

            // Handle AM/PM
            if let Some(ampm) = caps.get(3) {
                match ampm.as_str() {
                    "pm" if hour != 12 => hour += 12,
                    "am" if hour == 12 => hour = 0, // 12 AM is 00:00
                    _ => {}
                }
            }

            return NaiveTime::from_hms_opt(hour, minute, 0);
        }
    }

    None
}

pub fn extract_date(input: &str) -> NaiveDate {
    // Try to parse dates like "October 9, 2025" or "2025-10-09"

    // Month names mapping
    let months = [
        ("january", 1), ("february", 2), ("march", 3), ("april", 4),
        ("may", 5), ("june", 6), ("july", 7), ("august", 8),
        ("september", 9), ("october", 10), ("november", 11), ("december", 12),
        ("jan", 1), ("feb", 2), ("mar", 3), ("apr", 4),
        ("jun", 6), ("jul", 7), ("aug", 8), ("sep", 9),
        ("sept", 9), ("oct", 10), ("nov", 11), ("dec", 12),
    ];

    // Look for "Month Day, Year" pattern
    for (month_name, month_num) in &months {
        if input.contains(month_name) {
            // Extract numbers after the month name
            let parts: Vec<&str> = input.split_whitespace().collect();
            let mut day = None;
            let mut year = None;

            for (i, part) in parts.iter().enumerate() {
                if part.contains(month_name) {
                    // Next parts should be day and year
                    if i + 1 < parts.len() {
                        let day_str = parts[i + 1].trim_end_matches(',');
                        day = day_str.parse::<u32>().ok();
                    }
                    if i + 2 < parts.len() {
                        year = parts[i + 2].parse::<i32>().ok();
                    }
                }
            }

            if let (Some(d), Some(y)) = (day, year) {
                if let Some(date) = NaiveDate::from_ymd_opt(y, *month_num, d) {
                    return date;
                }
            }
        }
    }

    // Try YYYY-MM-DD format
    if let Ok(date) = NaiveDate::parse_from_str(input.split_whitespace().next().unwrap_or(""), "%Y-%m-%d") {
        return date;
    }

    // Default to today if no date found
    Local::now().date_naive()
}

pub fn get_last_day_of_month(year: i32, month: u32) -> NaiveDate {
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };

    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
}

pub fn print_help() {
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