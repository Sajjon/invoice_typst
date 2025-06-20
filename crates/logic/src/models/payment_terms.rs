use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::prelude::*;

/// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentTerms {
    /// Net payment due in a specific number of days, e.g. `Net(30)`
    Net(NetDays),
}

impl Default for PaymentTerms {
    fn default() -> Self {
        Self::net30()
    }
}

impl PaymentTerms {
    /// Creates a new `PaymentTerms` with net payment due in 30 days.
    pub fn net30() -> Self {
        PaymentTerms::Net(NetDays::net30())
    }
}

impl HasSample for PaymentTerms {
    fn sample() -> Self {
        Self::net30()
    }
}

#[derive(
    Clone, Copy, Debug, SerializeDisplay, DeserializeFromStr, TypedBuilder, Getters, Display,
)]
#[display("Net {}", due_in)]
pub struct NetDays {
    /// The number of days until payment is due
    #[builder(setter(into))]
    #[getset(get = "pub")]
    due_in: Day,
}
impl FromStr for NetDays {
    type Err = crate::prelude::Error;

    /// Tries to parse a string in the format "Net {days}", e.g. "Net 30".
    /// /// # Errors
    /// Returns an error if the string is not in the correct format or if
    /// the number of days is invalid.
    /// /// # Examples
    /// ```
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
    /// let net_days: NetDays = "Net 30".parse().unwrap();
    /// assert_eq!(net_days.due_in(), &Day::try_from(30).unwrap());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let days = s
            .split("Net ")
            .nth(1)
            .ok_or(Error::FailedToParsePaymentTermsNetDays {
                invalid_string: s.to_owned(),
            })?;
        let days = Day::from_str(days).map_err(|_| Error::FailedToParsePaymentTermsNetDays {
            invalid_string: s.to_owned(),
        })?;
        Ok(Self::builder().due_in(days).build())
    }
}
impl NetDays {
    pub fn net30() -> Self {
        Self::builder()
            .due_in(Day::try_from(30).expect("LEQ 31 days"))
            .build()
    }
}
