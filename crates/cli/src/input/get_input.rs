use crate::prelude::*;

/// The CLI arguments for generating an invoice PDF.
#[derive(Debug, Clone, TypedBuilder, Getters, Parser)]
#[command(name = "invoice")]
#[command(about = "Generate an invoice PDF", long_about = None)]
pub struct Input {
    /// The month for which the invoice is generated.
    #[arg(long, short = 'm', default_value_t = TargetMonth::Last)]
    #[builder(setter(into), default = TargetMonth::Last)]
    #[getset(get = "pub")]
    month: TargetMonth,

    /// The language for which the invoice is generated.
    #[arg(long, short = 'l', default_value_t = Language::EN)]
    #[builder(setter(into), default = Language::EN)]
    #[getset(get = "pub")]
    language: Language,

    /// The items to be invoiced, either expenses our consulting services
    /// with an optional number of days off.
    #[command(subcommand)]
    #[builder(setter(into, strip_option), default = None)]
    #[getset(get = "pub")]
    items: Option<TargetItems>,

    /// An optional override of where to save the output PDF file.
    #[arg(long, short = 'o')]
    #[builder(setter(into, strip_option), default = None)]
    out: Option<PathBuf>,
}

impl Input {
    fn _invoiced_items(&self) -> Result<InvoicedItems> {
        match self.items.clone().unwrap_or_default() {
            TargetItems::Ooo { days } => Ok(InvoicedItems::Service {
                days_off: if days == 0 {
                    None
                } else {
                    Some(Day::try_from(days)?)
                },
            }),
            TargetItems::Expenses => Ok(InvoicedItems::Expenses),
        }
    }

    pub fn parsed(self) -> Result<ValidInput> {
        if let Some(path) = &self.out {
            let parent = path
                .parent()
                .expect("Invalid path specified, no parent found, don't specify an empty path, a root or a prefix.");
            if !parent.exists() {
                Err(Error::SpecifiedOutputPathDoesNotExist {
                    path: path.display().to_string(),
                })?;
            }
        }
        let items = self._invoiced_items()?;
        let valid = ValidInput::builder()
            .month(self.month.year_and_month())
            .items(items)
            .language(*self.language())
            .maybe_output_path(self.out)
            .build();
        Ok(valid)
    }
}

pub fn get_input() -> Result<ValidInput> {
    let input = Input::parse();
    input.parsed()
}

#[cfg(test)]
mod tests_input {
    use super::*;
    use test_log::test;

    #[test]
    fn test_input_parsing_month() {
        let input = Input::parse_from(["invoice", "--month", "last"]);
        assert_eq!(input.month, TargetMonth::Last);
    }

    #[test]
    fn test_input_parsing_language_specified() {
        let input = Input::parse_from(["invoice", "--language", "swedish"]);
        assert_eq!(input.language, Language::SV);
    }

    #[test]
    fn test_input_parsing_language_default() {
        let input = Input::parse_from(["invoice"]);
        assert_eq!(input.language, Language::EN);
    }

    #[test]
    fn test_input_parsing_items_specified_ooo() {
        let input = Input::parse_from(["invoice", "ooo", "3"]);
        assert_eq!(input.items, Some(TargetItems::Ooo { days: 3 }));
    }

    #[test]
    fn test_input_parsing_items_specified_expenses() {
        let input = Input::parse_from(["invoice", "expenses"]);
        assert_eq!(input.items, Some(TargetItems::Expenses));
    }

    #[test]
    fn test_input_parsing_items_default() {
        let input = Input::parse_from(["invoice"]);
        assert_eq!(input.items, None);
    }

    #[test]
    fn test_input_parsing_out_specified() {
        let input = Input::parse_from(["invoice", "--out", "/tmp/invoice.pdf"]);
        assert_eq!(input.out, Some(PathBuf::from("/tmp/invoice.pdf")));
    }

    #[test]
    fn test_input_parsing_out_default() {
        let input = Input::parse_from(["invoice"]);
        assert_eq!(input.out, None);
    }
}

#[cfg(test)]
mod tests_parsed_input {
    use super::*;
    use test_log::test;

    #[test]
    fn test_input_parsing_items_services() {
        let input = Input::builder()
            .items(TargetItems::Ooo { days: 25 })
            .build();
        let input = input.parsed().unwrap();
        assert_eq!(
            *input.items(),
            InvoicedItems::Service {
                days_off: Some(Day::try_from(25).unwrap())
            }
        );
    }

    #[test]
    fn test_input_parsing_items_expenses() {
        let input = Input::builder().items(TargetItems::Expenses).build();
        let input = input.parsed().unwrap();
        assert_eq!(*input.items(), InvoicedItems::Expenses);
    }

    #[test]
    fn test_input_parsing_out() {
        let input = Input::builder().out("/tmp/invoice.pdf").build();
        let input = input.parsed().unwrap();
        assert_eq!(
            *input.maybe_output_path(),
            Some(PathBuf::from("/tmp/invoice.pdf"))
        );
    }

    #[test]
    #[should_panic]
    fn test_input_parsing_out_at_root_crashes() {
        let input = Input::builder().out("/").build();
        let _ = input.parsed();
    }
}
