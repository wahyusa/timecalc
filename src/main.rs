use std::env;
use timecalc::*;

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