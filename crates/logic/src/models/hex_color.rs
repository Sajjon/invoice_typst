use serde_with::DeserializeFromStr;

use crate::prelude::*;

#[derive(Clone, Debug, SerializeDisplay, DeserializeFromStr, derive_more::Display, Getters)]
#[display("#{:02x}{:02x}{:02x}", red, green, blue)]
pub struct HexColor {
    #[getset(get = "pub")]
    red: u8,
    #[getset(get = "pub")]
    green: u8,
    #[getset(get = "pub")]
    blue: u8,
}

impl Default for HexColor {
    fn default() -> Self {
        Self::from_str("#3ed6c0").expect("Failed to create default HexColor")
    }
}

impl HasSample for HexColor {
    fn sample() -> Self {
        Self::from_str("#e6007a").expect("Failed to create sample HexColor")
    }
}

impl FromStr for HexColor {
    type Err = crate::prelude::Error;

    /// Parses a hex color string in the format "#RRGGBB" or "#RRGGBBAA".
    /// The string must start with a '#' and be followed by 6 hexadecimal
    /// digits.
    ///
    /// # Examples
    /// ```
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
    /// let color: HexColor = "#e6007a".parse().unwrap();
    /// assert_eq!(*color.red(), 230);
    /// assert_eq!(*color.green(), 0);
    /// assert_eq!(*color.blue(), 122);
    /// ```
    ///
    /// # Errors
    /// Returns an error if the string does not start with '#' or if it does not
    /// contain exactly 6 hexadecimal digits after the '#'.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') && s.len() == 7 {
            let s = &s[1..];
            let parse_u8 = |start: usize, end: usize| {
                u8::from_str_radix(&s[start..end], 16).map_err(|_| Error::InvalidHexColor {
                    invalid_string: s.to_string(),
                })
            };
            let red = parse_u8(0, 2)?;
            let green = parse_u8(2, 4)?;
            let blue = parse_u8(4, 6)?;
            Ok(Self { red, green, blue })
        } else {
            Err(Error::InvalidHexColor {
                invalid_string: s.to_string(),
            })
        }
    }
}
