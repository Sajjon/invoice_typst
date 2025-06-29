use crate::prelude::*;
use klirr_render::prelude::render;

fn init_data_directory(provide_data: impl FnOnce() -> Result<Data>) -> Result<()> {
    init_data_directory_at(data_dir(), provide_data)
}

fn validate_data_directory() -> Result<()> {
    validate_data_directory_with_base_path(data_dir()).map(|_| ())
}

fn record_expenses(month: &YearAndMonth, expenses: &[Item]) -> Result<()> {
    record_expenses_with_base_path(month, expenses, data_dir())
}

fn record_month_off(month: &YearAndMonth) -> Result<()> {
    record_month_off_with_base_path(month, data_dir())
}

pub fn run_data_command(command: &DataAdminInputCommands) -> Result<()> {
    match command {
        DataAdminInputCommands::Init => init_data_directory(ask_for_data),
        DataAdminInputCommands::Validate => validate_data_directory(),
        DataAdminInputCommands::MonthOff(month_off_input) => {
            record_month_off(month_off_input.month())
        }
        DataAdminInputCommands::Expenses(expenses_input) => {
            record_expenses(expenses_input.month(), expenses_input.expenses())
        }
    }
}

pub fn render_sample() -> Result<PathBuf> {
    let path = dirs_next::home_dir()
        .expect("Expected to be able to find HOME dir")
        .join("klirr_sample.pdf");
    create_pdf_with_data(
        Data::sample(),
        ValidInput::builder()
            .maybe_output_path(path)
            .month(YearAndMonth::last())
            .build(),
        render,
    )
}

fn run_invoice_command_with_base_path(
    input: InvoiceInput,
    data_path: impl AsRef<Path>,
) -> Result<PathBuf> {
    let input = input.parsed()?;
    info!("🔮 Starting PDF creation, input: {}...", input);
    let pdf_location = create_pdf_with_data_base_path(data_path, input, render)?;
    save_pdf_location_to_tmp_file(pdf_location.clone())?;
    Ok(pdf_location)
}

pub fn run_invoice_command(input: InvoiceInput) -> Result<PathBuf> {
    run_invoice_command_with_base_path(input, data_dir())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::InvoiceInput;
    use test_log::test;

    #[test]
    fn test_run_invoice_command() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let tempfile = tempdir.path().join("out.pdf");
        save_data_with_base_path(Data::sample(), tempdir.path()).unwrap();
        let input = InvoiceInput::parse_from([
            "invoice",
            "--out",
            &format!("{}", tempfile.as_path().display()),
        ]);
        let result = run_invoice_command_with_base_path(input, tempdir.path());
        assert!(result.is_ok(), "Expected run to succeed, got: {:?}", result);
    }
}
