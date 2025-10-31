# TimeCalc

A fast, offline CLI tool for time calculations written in Rust.
Shit maybe I need to use other name...

BTW all code in here is AI Generated, but I using this tools daily BTW (BTW again)

Also first time using Rust BTW.

## Features

- **Date Calculations**: Calculate future/past dates
- **Timezone Conversion**: Convert between WIB, UTC, PST, EST, JST
- **Remaining Days**: Check days left in month/year
- **Day of Week**: Find what day any date falls on

## Installation

### From Source
```bash
git clone https://github.com/YOUR_USERNAME/timecalc.git
cd timecalc
cargo build --release
sudo cp target/release/timecalc /usr/local/bin/
```

### From GitHub Releases

Download the binary for your platform from [Releases](https://github.com/YOUR_USERNAME/timecalc/releases).

## Usage
```bash
# Calculate future dates
timecalc future 69 days

# Convert timezones
timecalc convert 4:00 UTC+7 to WIB

# Days remaining in month
timecalc remaining month

# Find day of week
timecalc day 2025-12-25
```

For full help:
```bash
timecalc help
```

## Testing
```bash
cargo test
```

## Building
```bash
cargo build --release
```

## License

MIT