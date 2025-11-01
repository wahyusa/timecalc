use timecalc::*;

// Handler function tests to increase coverage
#[test]
fn test_handle_future_date_valid() {
    let args = vec!["10".to_string()];
    handle_future_date(&args);
}

#[test]
fn test_handle_future_date_empty() {
    let args: Vec<String> = vec![];
    handle_future_date(&args);
}

#[test]
fn test_handle_future_date_invalid() {
    let args = vec!["invalid".to_string()];
    handle_future_date(&args);
}

#[test]
fn test_handle_past_date_valid() {
    let args = vec!["7".to_string()];
    handle_past_date(&args);
}

#[test]
fn test_handle_past_date_empty() {
    let args: Vec<String> = vec![];
    handle_past_date(&args);
}

#[test]
fn test_handle_past_date_invalid() {
    let args = vec!["invalid".to_string()];
    handle_past_date(&args);
}

#[test]
fn test_handle_day_of_week_valid() {
    let args = vec!["2025-12-25".to_string()];
    handle_day_of_week(&args);
}

#[test]
fn test_handle_day_of_week_empty() {
    let args: Vec<String> = vec![];
    handle_day_of_week(&args);
}

#[test]
fn test_handle_day_of_week_invalid() {
    let args = vec!["invalid-date".to_string()];
    handle_day_of_week(&args);
}

#[test]
fn test_handle_remaining_month() {
    let args = vec!["month".to_string()];
    handle_remaining(&args);
}

#[test]
fn test_handle_remaining_year() {
    let args = vec!["year".to_string()];
    handle_remaining(&args);
}

#[test]
fn test_handle_remaining_empty() {
    let args: Vec<String> = vec![];
    handle_remaining(&args);
}

#[test]
fn test_handle_remaining_invalid() {
    let args = vec!["invalid".to_string()];
    handle_remaining(&args);
}

#[test]
fn test_handle_timezone_convert_valid() {
    let args = vec![
        "04:00AM".to_string(),
        "UTC".to_string(),
        "to".to_string(),
        "PST".to_string(),
    ];
    handle_timezone_convert(&args);
}

#[test]
fn test_handle_timezone_convert_insufficient_args() {
    let args = vec!["04:00AM".to_string()];
    handle_timezone_convert(&args);
}

#[test]
fn test_handle_timezone_convert_no_to_keyword() {
    let args = vec![
        "04:00AM".to_string(),
        "UTC".to_string(),
        "from".to_string(),
        "PST".to_string(),
    ];
    handle_timezone_convert(&args);
}

#[test]
fn test_handle_timezone_convert_invalid_datetime() {
    let args = vec![
        "invalid".to_string(),
        "UTC".to_string(),
        "to".to_string(),
        "PST".to_string(),
    ];
    handle_timezone_convert(&args);
}

#[test]
fn test_handle_timezone_convert_invalid_timezone() {
    let args = vec![
        "04:00AM".to_string(),
        "INVALID".to_string(),
        "to".to_string(),
        "PST".to_string(),
    ];
    handle_timezone_convert(&args);
}

#[test]
fn test_handle_timezone_convert_with_date() {
    let args = vec![
        "October".to_string(),
        "9,".to_string(),
        "2025".to_string(),
        "04:00AM".to_string(),
        "UTC+8".to_string(),
        "to".to_string(),
        "WIB".to_string(),
    ];
    handle_timezone_convert(&args);
}

#[test]
fn test_print_help_executes() {
    // Just call print_help to increase coverage
    // This test ensures the function executes without panicking
    print_help();
}