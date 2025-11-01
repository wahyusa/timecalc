use timecalc::*;

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
fn test_parse_timezone_all_supported() {
    // WIB variants
    assert!(parse_timezone("WIB").is_some());
    assert!(parse_timezone("wib").is_some());
    assert!(parse_timezone("UTC+7").is_some());

    // UTC variants
    assert!(parse_timezone("UTC").is_some());
    assert!(parse_timezone("utc").is_some());
    assert!(parse_timezone("UTC+0").is_some());

    // PST
    assert!(parse_timezone("PST").is_some());
    assert!(parse_timezone("pst").is_some());
    assert!(parse_timezone("UTC-8").is_some());

    // EST
    assert!(parse_timezone("EST").is_some());
    assert!(parse_timezone("est").is_some());
    assert!(parse_timezone("UTC-5").is_some());

    // JST
    assert!(parse_timezone("JST").is_some());
    assert!(parse_timezone("jst").is_some());
    assert!(parse_timezone("UTC+9").is_some());

    // UTC+8
    assert!(parse_timezone("UTC+8").is_some());

    // UTC-7
    assert!(parse_timezone("UTC-7").is_some());
}