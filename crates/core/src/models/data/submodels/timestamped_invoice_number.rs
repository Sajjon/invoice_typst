use crate::prelude::*;

/// An invoice number timestamp with year and month, e.g. `(237, 2025-05)`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TypedBuilder, Getters)]
pub struct TimestampedInvoiceNumber {
    /// A base offset for the invoice number, e.g. `237`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    offset: InvoiceNumber,

    /// The month and year for when the `offset` was used, e.g. `2025-05`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    month: YearAndMonth,
}

impl HasSample for TimestampedInvoiceNumber {
    fn sample() -> Self {
        Self::builder()
            .offset(InvoiceNumber::from(17u16))
            .month(
                YearAndMonth::builder()
                    .year(1905)
                    .month(Month::October)
                    .build(),
            )
            .build()
    }
}
