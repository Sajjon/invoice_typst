use crate::prelude::*;

/// Tries to get the path to a temporary file where the path to the PDF will be saved.
/// This is determined by the environment variable `TMP_FILE_FOR_PATH_TO_PDF`.
///
/// Typically this is set by the makefile to a tmp file, used so that the
/// make file can open the PDF automatically after it has been created.
fn get_tmp_file_for_path_to_pdf() -> Option<PathBuf> {
    let path_to_tmp_file_where_we_write_dir_of_pdf =
        std::env::var("TMP_FILE_FOR_PATH_TO_PDF").ok()?;
    Some(PathBuf::from(path_to_tmp_file_where_we_write_dir_of_pdf))
}

/// Saves the path to the PDF file to a temporary file, if the environment variable
/// `TMP_FILE_FOR_PATH_TO_PDF` is set.
pub fn save_pdf_location_to_tmp_file(pdf_location: PathBuf) -> Result<()> {
    let Some(path_to_tmp_file_where_we_write_dir_of_pdf) = get_tmp_file_for_path_to_pdf() else {
        return Ok(());
    };

    trace!(
        "Saving path to PDF to temp file '{}'",
        path_to_tmp_file_where_we_write_dir_of_pdf.display()
    );
    if let Err(e) = std::fs::write(
        &path_to_tmp_file_where_we_write_dir_of_pdf,
        pdf_location.to_string_lossy().as_bytes(),
    ) {
        error!(
            "⚠️ Failed to write output path to {}: {} (scripts e.g. makefile will not be able to open the PDF automatically)",
            path_to_tmp_file_where_we_write_dir_of_pdf.display(),
            e
        );
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use test_log::test;

    #[test]
    fn test_save_pdf_location_to_tmp_file() {
        let tmp_file = NamedTempFile::new().unwrap();
        let tmp_file = tmp_file.path();
        unsafe {
            std::env::set_var("TMP_FILE_FOR_PATH_TO_PDF", tmp_file.display().to_string());
        }
        let path = get_tmp_file_for_path_to_pdf();
        assert_eq!(path.unwrap(), PathBuf::from(tmp_file));
    }
}
