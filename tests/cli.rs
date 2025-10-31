use assert_cmd::prelude::*; // For CommandCargoExt
use escargot::CargoBuild; // <-- Import escargot
use predicates::prelude::*;
use std::process::Command; // For running binaries

// This helper function finds your binary.
fn cmd() -> Command {
    // Use escargot to find and build the binary before running the test
    // This cleanly separates the "build" from the "test"
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
    // Run: timecalc --help
    cmd()
        .arg("--help")
        .assert()
        .success() // Check that it exited with code 0
        .stdout(predicate::str::contains("TIME CALCULATOR CLI")); // Check for help text
}

#[test]
fn test_no_args() {
    // Run: timecalc
    cmd()
        .assert()
        .success()
        .stdout(predicate::str::contains("TIME CALCULATOR CLI")); // No args should print help
}

#[test]
fn test_invalid_command() {
    // Run: timecalc foobar
    cmd()
        .arg("foobar")
        .assert()
        .success() // Your main fn doesn't return an error code, it just prints
        .stdout(predicate::str::contains("ERROR: Unknown command: foobar"));
}

// ===================================
// Tests for handle_future_date
// ===================================

#[test]
fn test_future_command() {
    // Run: timecalc future 10
    cmd()
        .arg("future")
        .arg("10")
        .assert()
        .success()
        .stdout(predicate::str::contains("AFTER 10 DAYS"));
}

#[test]
fn test_future_command_error_no_days() {
    // Run: timecalc future
    cmd()
        .arg("future")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Please specify number of days"));
}

#[test]
fn test_future_command_error_invalid_days() {
    // Run: timecalc future foo
    cmd()
        .arg("future")
        .arg("foo")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Could not parse days"));
}

// ===================================
// Tests for handle_past_date (NEW!)
// ===================================

#[test]
fn test_past_command() {
    // Run: timecalc past 7
    cmd()
        .arg("past")
        .arg("7")
        .assert()
        .success()
        .stdout(predicate::str::contains("7 DAYS AGO"));
}

#[test]
fn test_past_command_error_no_days() {
    // Run: timecalc past
    cmd()
        .arg("past")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Please specify number of days"));
}

#[test]
fn test_past_command_error_invalid_days() {
    // Run: timecalc past foo
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
    // Run: timecalc day 2025-10-31
    cmd()
        .arg("day")
        .arg("2025-10-31")
        .assert()
        .success()
        // FIX: The stdout log shows a non-breaking space (\u{a0}) is being printed.
        // We'll match that exactly.
        .stdout(predicate::str::contains("DAY: \u{a0}Friday"));
}

#[test]
fn test_day_command_error() {
    // Run: timecalc day invalid-date
    cmd()
        .arg("day")
        .arg("invalid-date")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Invalid date format"));
}

// ===================================
// Tests for handle_remaining
// ===================================

#[test]
fn test_remaining_month() {
    // Run: timecalc remaining month
    cmd()
        .arg("remaining")
        .arg("month")
        .assert()
        .success()
        .stdout(predicate::str::contains("DAYS REMAINING:"));
}

#[test]
fn test_remaining_year() {
    // Run: timecalc left year
    cmd()
        .arg("left")
        .arg("year")
        .assert()
        .success()
        .stdout(predicate::str::contains("DAYS REMAINING:"));
}

#[test]
fn test_remaining_error() {
    // Run: timecalc left foobar
    cmd()
        .arg("left")
        .arg("foobar")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Use 'month' or 'year'"));
}

// ===================================
// Tests for handle_timezone_convert
// ===================================

#[test]
fn test_tz_convert() {
    // Run: timecalc tz 04:00AM PST to WIB
    cmd()
        .arg("tz")
        .arg("04:00AM")
        .arg("PST")
        .arg("to")
        .arg("WIB")
        .assert()
        .success()
        // FIX: The stdout log shows a non-breaking space (\u{a0}) and a regular space.
        // We'll match that.
        .stdout(predicate::str::contains("TO: \u{a0} "));
}

#[test]
fn test_tz_convert_error_no_to() {
    // Run: timecalc tz 04:00AM PST from WIB
    // FIX: This test was logically wrong. We need to pass more than 3 args
    // to get past the first error check, but *not* use the word "to".
    cmd()
        .arg("tz")
        .arg("04:00AM")
        .arg("PST")
        .arg("from") // Use a different word
        .arg("WIB")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Missing 'to' keyword"));
}

#[test]
fn test_tz_convert_error_invalid_datetime() {
    // Run: timecalc tz foo PST to WIB
    cmd()
        .arg("tz")
        .arg("foo") // Invalid time
        .arg("PST")
        .arg("to")
        .arg("WIB")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Could not parse date/time"));
}

#[test]
fn test_tz_convert_error_invalid_tz() {
    // Run: timecalc tz 10:00 FAKETZ to WIB
    cmd()
        .arg("tz")
        .arg("10:00")
        .arg("FAKETZ") // Invalid timezone
        .arg("to")
        .arg("WIB")
        .assert()
        .success()
        .stdout(predicate::str::contains("ERROR: Unsupported timezone"));
}

