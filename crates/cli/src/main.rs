mod init_logging;

use invoice_typst_render::prelude::*;
use std::path::{Path, PathBuf};

/// Saves the PDF file to the specified path.
fn save_pdf(pdf: Pdf, pdf_name: impl AsRef<Path>) -> Result<PathBuf> {
    // now save the PDF to a file
    let output_path = PathBuf::from(pdf_name.as_ref());
    std::fs::write(&output_path, pdf.as_ref()).map_err(|e| {
        let msg = format!("Failed to write PDF to {}: {}", output_path.display(), e);
        error!("{}", msg);
        Error::SavePdf { underlying: msg }
    })?;
    Ok(output_path)
}

fn get_localization() -> Result<L18n> {
    // Read the input data from a file or other source.
    // This is a placeholder function, you can add your own logic here.
    info!("☑️ Reading localisation data...");
    warn!("Using sample data for demonstration purposes.");
    let l18n = L18n::english();
    Ok(l18n)
}

fn read_data_from_disk() -> Result<DataFromDisk> {
    // Read the input data from a file or other source.
    // This is a placeholder function, you can add your own logic here.
    info!("☑️ Reading invoice input data...");
    warn!("Using sample data for demonstration purposes.");
    let input_data = DataFromDisk::sample();
    Ok(input_data)
}

/// Compile the Typst source into a PDF and safe it at the specified path.
fn create_pdf(input: Input) -> Result<PathBuf> {
    let output_path = input.output_path();
    let data_from_disk = read_data_from_disk()?;
    let data = prepare_invoice_input_data(
        data_from_disk,
        input.target_month.year_and_month(),
        input.invoiced_items,
    )?;
    let l18n = get_localization()?;
    let pdf = render(Path::new("./crates/render/src/invoice.typ"), l18n, data)?;
    save_pdf(pdf, output_path)
}

#[derive(Debug, Clone, Copy, Display, Default)]
pub enum TargetMonth {
    Current,
    #[default]
    Last,
    Specific(YearAndMonth),
}
impl TargetMonth {
    pub fn year_and_month(&self) -> YearAndMonth {
        match self {
            TargetMonth::Current => YearAndMonth::current(),
            TargetMonth::Last => YearAndMonth::last(),
            TargetMonth::Specific(specific) => *specific,
        }
    }
}

#[derive(Debug, Clone, Display, TypedBuilder, Getters)]
#[display(
    "Month: {}, out: {:?}, items: {}",
    target_month,
    output_path,
    invoiced_items
)]
pub struct Input {
    #[getset(get = "pub")]
    target_month: TargetMonth,
    #[getset(get = "pub")]
    invoiced_items: InvoicedItems,
    output_path: Option<PathBuf>,
}
impl Input {
    pub fn output_path(&self) -> PathBuf {
        self.output_path
            .clone()
            .unwrap_or(PathBuf::from("output.pdf"))
    }
}

fn get_input() -> Result<Input> {
    // This function is a placeholder for getting user input.
    // You can implement your own logic to get the target month and invoiced items.
    info!("☑️ Getting user input...");
    warn!("Using hardcoded values for demonstration purposes.");
    let target_month = TargetMonth::Specific(YearAndMonth::builder().year(2025).month(5).build());
    let invoiced_items = InvoicedItems::Service { days_off: 3 };
    Ok(Input::builder()
        .invoiced_items(invoiced_items)
        .output_path(None)
        .target_month(target_month)
        .build())
}

fn run() -> Result<PathBuf> {
    let input = get_input()?;
    info!("🔮 Starting PDF creation, input: {}...", input);
    create_pdf(input)
}

fn main() {
    init_logging::init_logging();
    match run() {
        Ok(path) => info!("✅ PDF created successfully at: {}", path.display()),
        Err(e) => error!("Error creating PDF: {}", e),
    }
}
