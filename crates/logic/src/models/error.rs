use crate::prelude::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error type for the logic crate, encapsulating various errors that can occur
/// during PDF generation and manipulation.
#[derive(Clone, Debug, ThisError)]
pub enum Error {
    /// The offset month must not be in the record of months off.
    #[error("Offset month must not be in the record of months off: {offset_month}")]
    OffsetMonthMustNotBeInRecordOfMonthsOff { offset_month: YearAndMonth },

    /// The manually specified output path does not exist.
    #[error("Specified output path does not exist: {path}")]
    SpecifiedOutputPathDoesNotExist { path: String },

    /// Failed to create the output directory for the PDF file.
    #[error("Failed to create output directory: {underlying}")]
    FailedToCreateOutputDirectory { underlying: String },

    /// Target month must have expenses, but it does not.
    #[error(
        "Target month {target_month} must have expenses, but it does not. Fill 
    in the `input/data/expenses.json` file with expenses for this month."
    )]
    TargetMonthMustHaveExpenses { target_month: YearAndMonth },

    /// Failed to load file
    #[error("Failed to load file: {path}")]
    FileNotFound { path: String },

    /// Failed to deserialize a type
    #[error("Failed to deserialize {type_name}, because: {error}")]
    Deserialize { type_name: String, error: String },

    /// Invalid day of the month, e.g. when the day is not between 1 and 31.
    #[error("Invalid day: {day}, reason: {reason}")]
    InvalidDay { day: i32, reason: String },

    /// Invalid month, e.g. when the month is not between 1 and 12.
    #[error("Invalid month: {month}, reason: {reason}")]
    InvalidMonth { month: i32, reason: String },

    /// Failed to parse expense item from a string, e.g. when the format is incorrect.
    #[error("Failed to parse expense item from string: {invalid_string}, reason: {reason}")]
    InvalidExpenseItem {
        invalid_string: String,
        reason: String,
    },

    /// The target month is in the record of months off, but it must not be.
    #[error("Target month {target_month} is in the record of months off, but it must not be.")]
    TargetMonthMustNotBeInRecordOfMonthsOff { target_month: YearAndMonth },

    /// Failed to parse PaymentTerms NetDays from a string, e.g. when the format is incorrect.
    #[error("Failed to PaymentTerms NetDays from string: {invalid_string}")]
    FailedToParsePaymentTermsNetDays { invalid_string: String },

    /// Failed to find the localization file for a specific language.
    #[error("Failed to find the localization file for language: {language}")]
    L18nNotFound {
        /// The language that was not found, e.g. "EN" for English.
        language: Language,
    },

    /// Failed to parse a string into a Hexcolor
    #[error("Invalid hex color format: {invalid_string}")]
    InvalidHexColor { invalid_string: String },

    /// Failed to parse a date, e.g. when the format is incorrect or the date is invalid.
    #[error("Failed to parse date, because: {underlying}")]
    FailedToParseDate { underlying: String },

    /// Error converting between currencies, e.g. when the exchange rate is not found.
    #[error("Found no exchange rate for {target} based on {base}")]
    FoundNoExchangeRate {
        /// The target currency for the exchange rate, e.g. "EUR".
        target: Currency,
        /// The base currency for the exchange rate, e.g. "USD".
        base: Currency,
    },

    /// Error when loading a resource for typst.
    #[error("Failed to load Typst source, because: {underlying}")]
    LoadSource { underlying: String },

    /// Error when compiling Typst source to a PagedDocument.
    #[error("Failed to compile Typst source, because: {underlying}")]
    BuildPdf { underlying: String },

    /// Error when exporting a PagedDocument to PDF.
    #[error("Failed to export PagedDocument to PDF, because: {underlying}")]
    ExportDocumentToPdf { underlying: String },

    /// Error when saving the PDF to a file.
    #[error("Failed to save PDF, because: {underlying}")]
    SavePdf { underlying: String },

    /// Error when fetching exchange rates from an API.
    #[error("Failed fetch exchange rate from API, because: {underlying}")]
    NetworkError { underlying: String },

    /// Error when parsing the response from the exchange rate API.
    #[error("Failed to parse exchange rate response, because: {underlying}")]
    ParseError { underlying: String },
}
