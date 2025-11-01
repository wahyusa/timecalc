use timecalc::*;
use chrono::{Local, NaiveDate, NaiveTime};

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
fn test_parse_days_invalid() {
    let args = vec!["foo".to_string()];
    assert_eq!(parse_days(&args), None);
    
    let args = vec!["".to_string()];
    assert_eq!(parse_days(&args), None);
    
    let args = vec![];
    assert_eq!(parse_days(&args), None);
}

#[test]
fn test_parse_days_edge_cases() {
    // Test "day" (singular) suffix
    let args = vec!["1day".to_string()];
    assert_eq!(parse_days(&args), Some(1));

    // Test with extra whitespace
    let args = vec!["  10  ".to_string()];
    assert_eq!(parse_days(&args), Some(10));

    // Test large numbers
    let args = vec!["365".to_string()];
    assert_eq!(parse_days(&args), Some(365));

    let args = vec!["1000days".to_string()];
    assert_eq!(parse_days(&args), Some(1000));
}

#[test]
fn test_extract_time_simple() {
    assert_eq!(extract_time("4:00"), NaiveTime::from_hms_opt(4, 0, 0));
    assert_eq!(extract_time("14:30"), NaiveTime::from_hms_opt(14, 30, 0));
    assert_eq!(extract_time("08:15"), NaiveTime::from_hms_opt(8, 15, 0));
}

#[test]
fn test_extract_time_am_pm() {
    // 12:00 AM is 00:00
    assert_eq!(extract_time("12:00am"), NaiveTime::from_hms_opt(0, 0, 0));
    assert_eq!(extract_time("04:00am"), NaiveTime::from_hms_opt(4, 0, 0));
    assert_eq!(extract_time("12:00pm"), NaiveTime::from_hms_opt(12, 0, 0));
    assert_eq!(extract_time("04:00pm"), NaiveTime::from_hms_opt(16, 0, 0));
    assert_eq!(extract_time("1:00 pm"), NaiveTime::from_hms_opt(13, 0, 0));
}

#[test]
fn test_extract_time_invalid() {
    assert_eq!(extract_time("invalid"), None);
    assert_eq!(extract_time("25:00"), None);
    assert_eq!(extract_time("4:80"), None);
}

#[test]
fn test_extract_time_edge_cases() {
    // Single digit hours
    assert_eq!(extract_time("1:00"), NaiveTime::from_hms_opt(1, 0, 0));
    assert_eq!(extract_time("9:59"), NaiveTime::from_hms_opt(9, 59, 0));

    // 11 AM (not 12)
    assert_eq!(extract_time("11:00am"), NaiveTime::from_hms_opt(11, 0, 0));

    // 11 PM (should become 23:00)
    assert_eq!(extract_time("11:00pm"), NaiveTime::from_hms_opt(23, 0, 0));

    // Edge of valid times
    assert_eq!(extract_time("00:00"), NaiveTime::from_hms_opt(0, 0, 0));
    assert_eq!(extract_time("23:59"), NaiveTime::from_hms_opt(23, 59, 0));
}

#[test]
fn test_extract_date_formats() {
    let today = Local::now().date_naive();

    // YYYY-MM-DD
    assert_eq!(
        extract_date("2025-10-31 04:00pm"),
        NaiveDate::from_ymd_opt(2025, 10, 31).unwrap()
    );

    // Month Day, Year
    assert_eq!(
        extract_date("october 9, 2025 04:00am"),
        NaiveDate::from_ymd_opt(2025, 10, 9).unwrap()
    );

    // Short month
    assert_eq!(
        extract_date("jan 1, 2024"),
        NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
    );

    // Default to today
    assert_eq!(extract_date("4:00pm"), today);
}

#[test]
fn test_extract_date_various_months() {
    // Test all month names
    assert_eq!(
        extract_date("january 15, 2025"),
        NaiveDate::from_ymd_opt(2025, 1, 15).unwrap()
    );
    assert_eq!(
        extract_date("february 20, 2025"),
        NaiveDate::from_ymd_opt(2025, 2, 20).unwrap()
    );
    assert_eq!(
        extract_date("march 10, 2025"),
        NaiveDate::from_ymd_opt(2025, 3, 10).unwrap()
    );
    assert_eq!(
        extract_date("april 5, 2025"),
        NaiveDate::from_ymd_opt(2025, 4, 5).unwrap()
    );
    assert_eq!(
        extract_date("may 1, 2025"),
        NaiveDate::from_ymd_opt(2025, 5, 1).unwrap()
    );
    assert_eq!(
        extract_date("june 30, 2025"),
        NaiveDate::from_ymd_opt(2025, 6, 30).unwrap()
    );
    assert_eq!(
        extract_date("july 4, 2025"),
        NaiveDate::from_ymd_opt(2025, 7, 4).unwrap()
    );
    assert_eq!(
        extract_date("august 15, 2025"),
        NaiveDate::from_ymd_opt(2025, 8, 15).unwrap()
    );
    assert_eq!(
        extract_date("september 1, 2025"),
        NaiveDate::from_ymd_opt(2025, 9, 1).unwrap()
    );
    assert_eq!(
        extract_date("november 11, 2025"),
        NaiveDate::from_ymd_opt(2025, 11, 11).unwrap()
    );
    assert_eq!(
        extract_date("december 25, 2025"),
        NaiveDate::from_ymd_opt(2025, 12, 25).unwrap()
    );

    // Test short month names
    assert_eq!(
        extract_date("feb 14, 2025"),
        NaiveDate::from_ymd_opt(2025, 2, 14).unwrap()
    );
    assert_eq!(
        extract_date("mar 17, 2025"),
        NaiveDate::from_ymd_opt(2025, 3, 17).unwrap()
    );
    assert_eq!(
        extract_date("apr 1, 2025"),
        NaiveDate::from_ymd_opt(2025, 4, 1).unwrap()
    );
    assert_eq!(
        extract_date("jun 15, 2025"),
        NaiveDate::from_ymd_opt(2025, 6, 15).unwrap()
    );
    assert_eq!(
        extract_date("jul 20, 2025"),
        NaiveDate::from_ymd_opt(2025, 7, 20).unwrap()
    );
    assert_eq!(
        extract_date("aug 31, 2025"),
        NaiveDate::from_ymd_opt(2025, 8, 31).unwrap()
    );
    assert_eq!(
        extract_date("sep 5, 2025"),
        NaiveDate::from_ymd_opt(2025, 9, 5).unwrap()
    );
    assert_eq!(
        extract_date("sept 10, 2025"),
        NaiveDate::from_ymd_opt(2025, 9, 10).unwrap()
    );
    assert_eq!(
        extract_date("oct 31, 2025"),
        NaiveDate::from_ymd_opt(2025, 10, 31).unwrap()
    );
    assert_eq!(
        extract_date("nov 5, 2025"),
        NaiveDate::from_ymd_opt(2025, 11, 5).unwrap()
    );
    assert_eq!(
        extract_date("dec 31, 2025"),
        NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
    );
}

#[test]
fn test_extract_date_iso_format() {
    // Test YYYY-MM-DD format parsing
    assert_eq!(
        extract_date("2025-01-01"),
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
    );
    assert_eq!(
        extract_date("2025-12-31"),
        NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
    );
    assert_eq!(
        extract_date("2024-02-29"),
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
    );
}

#[test]
fn test_extract_date_invalid_dates() {
    // Invalid dates should fall back to today
    let today = Local::now().date_naive();

    // February 30th doesn't exist - should default to today
    assert_eq!(extract_date("february 30, 2025"), today);

    // Month 13 doesn't exist
    assert_eq!(extract_date("month13 15, 2025"), today);
}

#[test]
fn test_parse_flexible_datetime() {
    // Simple time today
    let result = parse_flexible_datetime("4:00");
    assert!(result.is_some());
    let dt = result.unwrap();
    assert_eq!(dt.time(), NaiveTime::from_hms_opt(4, 0, 0).unwrap());

    // With date
    let result = parse_flexible_datetime("october 9, 2025 04:00am");
    assert!(result.is_some());
    let dt = result.unwrap();
    assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2025, 10, 9).unwrap());
    assert_eq!(dt.time(), NaiveTime::from_hms_opt(4, 0, 0).unwrap());

    // With "at" keyword
    let result = parse_flexible_datetime("october 9, 2025 at 04:00am");
    assert!(result.is_some());
    let dt = result.unwrap();
    assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2025, 10, 9).unwrap());
    assert_eq!(dt.time(), NaiveTime::from_hms_opt(4, 0, 0).unwrap());

    // Invalid time returns None
    assert!(parse_flexible_datetime("invalid").is_none());
}

#[test]
fn test_parse_flexible_datetime_edge_cases() {
    // Test with PM time
    let result = parse_flexible_datetime("2:30pm");
    assert!(result.is_some());
    let dt = result.unwrap();
    assert_eq!(dt.time(), NaiveTime::from_hms_opt(14, 30, 0).unwrap());

    // Test with ISO date
    let result = parse_flexible_datetime("2025-12-25 10:00");
    assert!(result.is_some());
    let dt = result.unwrap();
    assert_eq!(dt.date(), NaiveDate::from_ymd_opt(2025, 12, 25).unwrap());
    assert_eq!(dt.time(), NaiveTime::from_hms_opt(10, 0, 0).unwrap());
}

#[test]
fn test_parse_datetime_and_tz() {
    // Valid input
    let parts = vec![
        "04:00AM".to_string(),
        "UTC+8".to_string(),
    ];
    let (dt, tz) = parse_datetime_and_tz(&parts);
    assert!(dt.is_some());
    assert_eq!(tz, "UTC+8");

    // With date
    let parts = vec![
        "october".to_string(),
        "9,".to_string(),
        "2025".to_string(),
        "at".to_string(),
        "04:00AM".to_string(),
        "UTC+8".to_string(),
    ];
    let (dt, tz) = parse_datetime_and_tz(&parts);
    assert!(dt.is_some());
    assert_eq!(tz, "UTC+8");

    // Empty input
    let parts: Vec<String> = vec![];
    let (dt, tz) = parse_datetime_and_tz(&parts);
    assert!(dt.is_none());
    assert_eq!(tz, "");

    // Single element
    let parts = vec!["UTC+8".to_string()];
    let (dt, tz) = parse_datetime_and_tz(&parts);
    assert!(dt.is_none());
    assert_eq!(tz, "UTC+8");
}