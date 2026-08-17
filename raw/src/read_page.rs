use anyhow::{Context, Result, bail};
use std::process::Command;
use crate::warn;

pub fn compile_paper_sheet(image_path: &str) -> Result<String> {
    let img = image::open(image_path)
        .with_context(|| format!("Failed to open paper image at path: {}", image_path))?
        .grayscale();

    let processed_path = "processed_paper_temp.png";
    img.save(processed_path)
        .context("Failed to save temporary processed image")?;
    let tesseract_result = Command::new("tesseract")
        .arg(processed_path)
        .arg("stdout")
        .arg("eng")
        .arg("--psm")
        .arg("6")
        .output();
    let output = match tesseract_result {
        Ok(out) if out.status.success() => out,
        _ => {
            let fallback_path = r"C:\Program Files\Tesseract-OCR\tesseract.exe";
            let fallback_result = Command::new(fallback_path)
                .arg(processed_path)
                .arg("stdout")
                .arg("eng")
                .arg("--psm")
                .arg("6")
                .output();

            match fallback_result {
                Ok(out) => out,
                Err(_) => {
                    warn::warning(&format!("Tesseract execution failed completely."));
                    bail!("Could not execute tesseract via PATH or Program Files fallback.");
                }
            }
        }
    };
    let _ = std::fs::remove_file(processed_path);
    let raw_code = String::from_utf8(output.stdout)
        .context("Tesseract output contained invalid UTF-8")?;
    Ok(raw_code)
}