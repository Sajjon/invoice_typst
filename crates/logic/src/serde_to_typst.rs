use crate::prelude::*;
use indoc::indoc;

/// Converts any `Serialize`able Rust struct into Typst syntax.
pub fn to_typst_let<T: Serialize>(input: &T) -> String {
    let value = serde_json::to_value(input).expect("Serialization failed");
    format!(
        indoc! {r#"
        #let provide() = {{
          {}
        }}
        "#},
        to_typst_value(&value, 0)
    )
}

/// Recursively converts a serde_json::Value into pretty-printed Typst syntax.
fn to_typst_value(value: &Value, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    let next_indent = indent + 1;
    let next_indent_str = "  ".repeat(next_indent);

    match value {
        Value::Object(map) => {
            // Flatten single-entry enum-like objects (e.g. { "Net": 30 }) to (net: 30)
            if map.len() == 1 {
                if let Some((variant, inner)) = map.iter().next() {
                    if inner.is_number() || inner.is_string() || inner.is_object() {
                        return format!(
                            "(\n{}{}: {},\n{})",
                            next_indent_str,
                            variant.to_lowercase(),
                            to_typst_value(inner, next_indent),
                            indent_str
                        );
                    }
                }
            }

            let fields = map
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}{}: {}",
                        next_indent_str,
                        k,
                        to_typst_value(v, next_indent)
                    )
                })
                .collect::<Vec<_>>()
                .join(",\n");

            format!("(\n{},\n{})", fields, indent_str)
        }

        Value::Array(arr) => {
            let items = arr
                .iter()
                .map(|v| format!("{}{}", next_indent_str, to_typst_value(v, next_indent)))
                .collect::<Vec<_>>()
                .join(",\n");

            format!("(\n{},\n{})", items, indent_str)
        }

        Value::String(s) => format!("\"{}\"", s),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Null => "none".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_data_to_typst {
        ($sample:expr, $input:expr, $expected:expr) => {{
            let input = $sample
                .to_partial($input)
                .unwrap()
                .to_typst(ExchangeRatesMap::from_iter([
                    (Currency::GBP, UnitPrice::from(1.174)),
                    (Currency::SEK, UnitPrice::from(11.05)),
                ]))
                .unwrap();
            let typst = to_typst_let(&input);
            pretty_assertions::assert_eq!(typst, $expected);
        }};
    }

    macro_rules! test_l18n_to_typst {
        ($input:expr, $expected:expr) => {{
            let typst = to_typst_let($input.content());
            pretty_assertions::assert_eq!(typst, $expected);
        }};
    }

    #[test]
    fn sample_expenses_to_typst() {
        test_data_to_typst!(
            DataFromDisk::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Expenses)
                .month(YearAndMonth::sample())
                .language(Language::EN)
                .build(),
            include_str!("./fixtures/expected_input_expenses.typ")
        );
    }

    #[test]
    fn sample_services_to_typst() {
        test_data_to_typst!(
            DataFromDisk::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Service { days_off: None })
                .month(YearAndMonth::sample())
                .language(Language::EN)
                .build(),
            include_str!("./fixtures/expected_input_services.typ")
        );
    }

    #[test]
    fn l18n_english_to_typst_macro() {
        test_l18n_to_typst!(
            &L18n::new(Language::EN).unwrap(),
            include_str!("./fixtures/expected_l18n_english.typ")
        );
    }
}
