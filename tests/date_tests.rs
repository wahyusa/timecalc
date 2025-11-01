use timecalc::*;
use chrono::NaiveDate;

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

#[test]
fn test_get_last_day_of_month_december() {
    // Test the "month == 12" branch
    let last_day = get_last_day_of_month(2025, 12);
    assert_eq!(last_day.day(), 31);
    assert_eq!(last_day.month(), 12);
    assert_eq!(last_day.year(), 2025);
}

#[test]
fn test_get_last_day_various_months() {
    // Test months with 30 days
    assert_eq!(get_last_day_of_month(2025, 4).day(), 30); // April
    assert_eq!(get_last_day_of_month(2025, 6).day(), 30); // June
    assert_eq!(get_last_day_of_month(2025, 9).day(), 30); // September
    assert_eq!(get_last_day_of_month(2025, 11).day(), 30); // November

    // Test months with 31 days
    assert_eq!(get_last_day_of_month(2025, 1).day(), 31); // January
    assert_eq!(get_last_day_of_month(2025, 3).day(), 31); // March
    assert_eq!(get_last_day_of_month(2025, 5).day(), 31); // May
    assert_eq!(get_last_day_of_month(2025, 7).day(), 31); // July
    assert_eq!(get_last_day_of_month(2025, 8).day(), 31); // August
    assert_eq!(get_last_day_of_month(2025, 10).day(), 31); // October
}

#[test]
fn test_get_last_day_february_non_leap() {
    // Specifically test February in non-leap year
    let last_day = get_last_day_of_month(2023, 2);
    assert_eq!(last_day.day(), 28);
    assert_eq!(last_day.month(), 2);
    assert_eq!(last_day.year(), 2023);
}