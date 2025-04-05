pub mod consts;

use consts::*;

pub struct ByteSizeFormatter {
    divisor: u64,
    unit: String,
}

impl ByteSizeFormatter {
    /// Creates a byte size formatter for a specific unit.
    ///
    /// # Arguments
    ///
    /// * `system` - The numeral system (Binary or Decimal)
    /// * `magnitude` - The magnitude (Kilo, Mega, Giga, etc.)
    ///
    /// # Example
    /// ```
    /// use bittenhumans::ByteSizeFormatter;
    /// use bittenhumans::consts::{Magnitude, System};
    ///
    /// // Create a formatter for kilobytes (1000 bytes)
    /// let kb_formatter = ByteSizeFormatter::new(System::Decimal, Magnitude::Kilo);
    /// assert_eq!("1.00 KB", kb_formatter.format_value(1000));
    ///
    /// // Create a formatter for mebibytes (1024² bytes)
    /// let mib_formatter = ByteSizeFormatter::new(System::Binary, Magnitude::Mega);
    /// assert_eq!("1.00 MiB", mib_formatter.format_value(1024 * 1024));
    /// ```
    ///
    /// # Returns
    ///
    /// A ByteSizeFormatter configured for the specified system and magnitude
    pub fn new(system: System, magnitude: Magnitude) -> Self {
        let infix = match system {
            System::Binary => "i",
            System::Decimal => "",
        };
        let magnitude = magnitude as usize;
        Self {
            divisor: (system as u64).pow(magnitude as u32),
            unit: format!("{}{infix}B", MAGNITUDE_PREFIXES[magnitude - 1]),
        }
    }

    fn compute_divisor(system: System, magnitude: Magnitude) -> u64 {
        (system as u64).pow(magnitude as u32)
    }

    /// Creates a formatter for the largest magnitude that fits the given value under the specified numeral system.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte size value to fit
    /// * `system` - The numeral system to use (Binary or Decimal)
    ///
    /// # Example
    /// ```
    /// use bittenhumans::ByteSizeFormatter;
    /// use bittenhumans::consts::System;
    ///
    /// let disk_total = 1000000000;
    /// let disk_used = disk_total / 1000;
    ///
    /// let formatter = ByteSizeFormatter::fit(disk_total, System::Binary);
    /// assert_eq!("953.67 MiB", formatter.format_value(disk_total));
    /// assert_eq!("0.95 MiB", formatter.format_value(disk_used));
    /// ```
    ///
    /// # Returns
    ///
    /// A formatter configured with the appropriate magnitude for the value
    pub fn fit(value: u64, system: System) -> Self {
        let mut last = Magnitude::Kilo;
        for magnitude in enum_iterator::all::<Magnitude>() {
            if (value as f64 / Self::compute_divisor(system, magnitude) as f64) < 1.0 {
                break;
            }
            last = magnitude;
        }

        Self::new(system, last)
    }

    /// Formats a byte size value using the appropriate magnitude unit.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte size value to format
    /// * `system` - The numeral system to use (Binary or Decimal)
    ///// # Examples
    /// ```
    /// use bittenhumans::ByteSizeFormatter;
    /// use bittenhumans::consts::System;
    ///
    /// // Format a value using the decimal system (powers of 1000)
    /// let formatted = ByteSizeFormatter::format_auto(1500000, System::Decimal);
    /// assert_eq!("1.50 MB", formatted);
    ///
    /// // Format a value using the binary system (powers of 1024)
    /// let formatted = ByteSizeFormatter::format_auto(1500000, System::Binary);
    /// assert_eq!("1.43 MiB", formatted);
    /// ```
    ///
    /// # Returns
    ///
    /// A formatted string with the value and appropriate unit
    pub fn format_auto(value: u64, system: System) -> String {
        Self::fit(value, system).format_value(value)
    }

    pub fn get_unit(&self) -> &str {
        &self.unit
    }

    pub fn get_divisor(&self) -> &u64 {
        &self.divisor
    }

    pub fn format_value(&self, value: u64) -> String {
        format!("{:.2} {}", value as f64 / self.divisor as f64, self.unit)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new() {
        let kibibyte = ByteSizeFormatter::new(System::Binary, Magnitude::Kilo);
        assert_eq!("KiB", kibibyte.get_unit());
        assert_eq!(1024_u64, *kibibyte.get_divisor());

        let exabyte = ByteSizeFormatter::new(System::Decimal, Magnitude::Exa);
        assert_eq!("EB", exabyte.get_unit());
        assert_eq!(1_000_000_000_000_000_000_u64, *exabyte.get_divisor());
    }

    #[test]
    fn fit() {
        let kibibyte = ByteSizeFormatter::fit(1, System::Binary);
        assert_eq!("KiB", kibibyte.get_unit());
        assert_eq!(1024_u64, *kibibyte.get_divisor());

        let exabyte = ByteSizeFormatter::fit(1_000_000_000_000_000_001_u64, System::Decimal);
        assert_eq!("EB", exabyte.get_unit());
        assert_eq!(1_000_000_000_000_000_000_u64, *exabyte.get_divisor());
    }

    #[test]
    fn format() {
        let kib = ByteSizeFormatter::new(System::Binary, Magnitude::Kilo);
        assert_eq!("0.50 KiB".to_string(), kib.format_value(512));
        let gb = ByteSizeFormatter::new(System::Decimal, Magnitude::Giga);
        assert_eq!("1.00 GB".to_string(), gb.format_value(1_000_000_000));
    }
}
