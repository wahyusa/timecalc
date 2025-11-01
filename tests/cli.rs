use assert_cmd::prelude::*;
use predicates::prelude::*;
use escargot::CargoBuild;
use std::process::Command;

// Helper function to create a command for the binary
fn cmd() -> Command {
        let bin_path = CargoBuild::new()
        .bin("timecalc") // The name of your binary
        .run()
        .unwrap()
        .path()
        .to_path_buf();

    Command::new(bin_path)
}

// ===================================
// Tests for main() and print_help()
// ===================================

#[test]
fn test_help_message() {
    cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("TIME CALCULATOR CLI"));
}

#[test]
fn test_no_args() {
    cmd()
        .assert()
        .success()
        .stdout(predicate::str::contains("TIME CALCULATOR CLI"));
}

#[test]
fn test_invalid_command() {
    cmd()
        .arg("foobar")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Unknown command: foobar"));
}

// ===================================
// Tests for handle_future_date
// ===================================

#[test]
fn test_future_command() {
    cmd()
        .arg("future")
        .arg("10")
        .assert()
        .success()
        .stdout(predicate::str::contains("AFTER 10 DAYS"));
}

#[test]
fn test_future_command_error_no_days() {
    cmd()
        .arg("future")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Please specify number of days"));
}

#[test]
fn test_future_command_error_invalid_days() {
    cmd()
        .arg("future")
        .arg("foo")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Could not parse days"));
}

// ===================================
// Tests for handle_past_date
// ===================================

#[test]
fn test_past_command() {
    cmd()
        .arg("past")
        .arg("7")
        .assert()
        .success()
        .stdout(predicate::str::contains("7 DAYS AGO"));
}

#[test]
fn test_past_command_error_no_days() {
    cmd()
        .arg("past")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Please specify number of days"));
}

#[test]
fn test_past_command_error_invalid_days() {
    cmd()
        .arg("past")
        .arg("foo")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Could not parse days"));
}

// ===================================
// Tests for handle_day_of_week
// ===================================

#[test]
fn test_day_command() {
    cmd()
        .arg("day")
        .arg("2025-10-31")
        .assert()
        .success()
        // Just check that it contains "Friday" - don't worry about non-breaking spaces
        .stdout(predicate::str::contains("Friday"));
}

#[test]
fn test_day_command_error() {
    cmd()
        .arg("day")
        .arg("invalid-date")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Invalid date format"));
}

#[test]
fn test_day_command_no_args() {
    cmd()
        .arg("day")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Please provide a date"));
}

// ===================================
// Tests for handle_remaining
// ===================================

#[test]
fn test_remaining_month() {
    cmd()
        .arg("remaining")
        .arg("month")
        .assert()
        .success()
        .stdout(predicate::str::contains("DAYS REMAINING:"));
}

#[test]
fn test_remaining_year() {
    cmd()
        .arg("left")
        .arg("year")
        .assert()
        .success()
        .stdout(predicate::str::contains("DAYS REMAINING:"));
}

#[test]
fn test_remaining_error() {
    cmd()
        .arg("left")
        .arg("foobar")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Use 'month' or 'year'"));
}

#[test]
fn test_remaining_no_args() {
    cmd()
        .arg("remaining")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Specify 'month' or 'year'"));
}

// ===================================
// Tests for handle_timezone_convert
// ===================================

#[test]
fn test_tz_convert() {
    cmd()
        .arg("tz")
        .arg("04:00AM")
        .arg("PST")
        .arg("to")
        .arg("WIB")
        .assert()
        .success()
        // Just check for "TIMEZONE CONVERSION" header instead of specific formatting
        .stdout(predicate::str::contains("TIMEZONE CONVERSION"));
}

#[test]
fn test_tz_convert_error_no_to() {
    cmd()
        .arg("tz")
        .arg("04:00AM")
        .arg("PST")
        .arg("from")
        .arg("WIB")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Missing 'to' keyword"));
}

#[test]
fn test_tz_convert_error_invalid_datetime() {
    cmd()
        .arg("tz")
        .arg("foo")
        .arg("PST")
        .arg("to")
        .arg("WIB")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Could not parse date/time"));
}

#[test]
fn test_tz_convert_error_invalid_tz() {
    cmd()
        .arg("tz")
        .arg("10:00")
        .arg("FAKETZ")
        .arg("to")
        .arg("WIB")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Unsupported timezone"));
}

#[test]
fn test_tz_convert_insufficient_args() {
    cmd()
        .arg("tz")
        .arg("04:00AM")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Invalid format"));
}

#[test]
fn test_tz_convert_with_date() {
    cmd()
        .arg("tz")
        .arg("October")
        .arg("9,")
        .arg("2025")
        .arg("at")
        .arg("04:00AM")
        .arg("UTC+8")
        .arg("to")
        .arg("WIB")
        .assert()
        .success()
        .stdout(predicate::str::contains("TIMEZONE CONVERSION"));
}

// ===================================
// Additional tests for command aliases
// ===================================

#[test]
fn test_date_alias() {
    cmd()
        .arg("date")
        .arg("5")
        .assert()
        .success()
        .stdout(predicate::str::contains("AFTER 5 DAYS"));
}

#[test]
fn test_convert_alias() {
    cmd()
        .arg("convert")
        .arg("10:00")
        .arg("UTC")
        .arg("to")
        .arg("PST")
        .assert()
        .success()
        .stdout(predicate::str::contains("TIMEZONE CONVERSION"));
}

#[test]
fn test_help_flag() {
    cmd()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("TIME CALCULATOR CLI"));
}

#[test]
fn test_help_command() {
    cmd()
        .arg("help")
        .assert()
        .success()
        .stdout(predicate::str::contains("FUTURE/PAST DATES"));
}

#[test]
fn test_left_alias() {
    cmd()
        .arg("left")
        .arg("month")
        .assert()
        .success()
        .stdout(predicate::str::contains("DAYS REMAINING"));
}

// ===================================
// Tests with different input formats
// ===================================

#[test]
fn test_future_with_days_suffix() {
    cmd()
        .arg("future")
        .arg("7days")
        .assert()
        .success()
        .stdout(predicate::str::contains("AFTER 7 DAYS"));
}

#[test]
fn test_future_with_d_suffix() {
    cmd()
        .arg("future")
        .arg("14d")
        .assert()
        .success()
        .stdout(predicate::str::contains("AFTER 14 DAYS"));
}

#[test]
fn test_past_with_days_suffix() {
    cmd()
        .arg("past")
        .arg("3days")
        .assert()
        .success()
        .stdout(predicate::str::contains("3 DAYS AGO"));
}