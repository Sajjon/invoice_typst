use crate::prelude::*;

pub fn prepare_invoice_input_data(
    data_from_disk: DataFromDisk,
    target_month: YearAndMonth,
    invoiced_items: InvoicedItems,
) -> Result<DataTypstCompat> {
    // Prepare the data for the Typst source.
    // This is a placeholder function, you can add your own logic here.
    info!("Preparing invoice input data for PDF generation...");
    let partial = data_from_disk.to_partial(&target_month, &invoiced_items)?;
    let exchange_rates = get_exchange_rates_if_needed(partial.line_items())?;
    let input_data = partial.to_typst(exchange_rates)?;
    Ok(input_data)
}
