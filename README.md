# bittenhumans

[![Crates.io](https://img.shields.io/crates/v/bittenhumans)](https://crates.io/crates/bittenhumans)
![Crates.io](https://img.shields.io/crates/l/bittenhumans)
[![Docs.rs](https://docs.rs/bittenhumans/badge.svg)](https://docs.rs/bittenhumans)

A lightweight, simple byte size humanization library for Rust.

## Features

- Basic humanization, supporting both **decimal** (KB, MB, GB) and **binary** (KiB, MiB, GiB) numeral systems
- Automatically fit values and reuse formatters

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bittenhumans = "1.0.0"
```

## Usage

### Basic Usage

```rust
use bittenhumans::ByteSizeFormatter;
use bittenhumans::consts::System;

fn main() {
    // Format with automatic unit selection
    let formatted = ByteSizeFormatter::format_auto(1_500_000, System::Decimal);
    assert_eq!(formatted, "1.50 MB");

    // Format using binary system (powers of 1024)
    let formatted = ByteSizeFormatter::format_auto(1_500_000, System::Binary);
    assert_eq!(formatted, "1.43 MiB");
}
```

### Creating Reusable Formatters

```rust
use bittenhumans::ByteSizeFormatter;
use bittenhumans::consts::{Magnitude, System};

fn main() {
    // Create a formatter for gigabytes
    let gb_formatter = ByteSizeFormatter::new(System::Decimal, Magnitude::Giga);

    // Use it multiple times with different values
    let total_space = gb_formatter.format_value(1_000_000_000_000); // "1000.00 GB"
    let free_space = gb_formatter.format_value(250_000_000_000); // "250.00 GB"
}
```

### Creating Size-Appropriate Formatters

```rust
use bittenhumans::ByteSizeFormatter;
use bittenhumans::consts::System;

fn main() {
    // Create a formatter that fits the largest value you'll format
    let disk_size = 2_000_000_000_000; // 2 TB
    let formatter = ByteSizeFormatter::fit(disk_size, System::Binary);

    // All values will use the same unit for consistent display
    println!("Disk size: {}", formatter.format_value(disk_size));     // "1.82 TiB"
    println!("Used space: {}", formatter.format_value(disk_size/4));  // "0.45 TiB"
}
```

## Documentation

For more detailed information, check the [API documentation](https://docs.rs/bittenhumans).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
