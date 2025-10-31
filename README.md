# TimeCalc

A fast, offline CLI tool for time calculations written in Rust.
Shit maybe I need to use other name...

BTW all code in here is AI Generated, but I using this tools daily BTW (BTW again)

Also first time using Rust BTW.

Really blazingly fast 🤯

## Features

- **Date Calculations**: Calculate future/past dates
- **Timezone Conversion**: Convert between WIB, UTC, PST, EST, JST
- **Remaining Days**: Check days left in month/year
- **Day of Week**: Find what day any date falls on

## Installation

### From Source

```bash
git clone https://github.com/wahyusa/timecalc.git
cd timecalc
cargo build --release
sudo cp target/release/timecalc /usr/local/bin/
```

### From GitHub Releases

Download the binary for your platform from [Releases](https://github.com/wahyusa/timecalc/releases).

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

## Development

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run clippy for linting
cargo clippy -- -D warnings
```

### Building

```bash
cargo build --release
```

## Releasing

To create a new release:

```bash
# 1. Update version in Cargo.toml
# 2. Commit the changes
git add Cargo.toml
git commit -m "Bump version to v0.2.0"
git push

# 3. Create and push a tag
git tag v0.2.0
git push origin v0.2.0
```

GitHub Actions will automatically:
- Run tests
- Build binaries for Linux (amd64, arm64), macOS (amd64, arm64), and Windows
- Create a GitHub Release with all binaries attached

### Fix Wrong Tag

If you pushed a tag without 'v' prefix:

```bash
# Delete wrong tag
git tag -d 0.1.0
git push origin :refs/tags/0.1.0

# Create correct tag
git tag v0.1.0
git push origin v0.1.0
```

## License

MIT