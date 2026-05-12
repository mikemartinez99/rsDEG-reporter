use clap::Parser;
use serde::Deserialize;
use std::error::Error;

// Ensure the `Parser` trait is implemented
#[derive(Parser)]
struct Cli {
    /// Path to the input CSV file
    #[arg(short = 'i', long, help = "Path to the input CSV file")]
    input_csv: std::path::PathBuf,

    /// Path to the output CSV file
    #[arg(short = 'o', long, help = "Path to the output CSV file")]
    output_csv: std::path::PathBuf,

    /// Adjusted p-value threshold
    #[arg(short = 'p', long, help = "Adjusted p-value threshold")]
    padj_thresh: f32,

    /// Log2 fold change threshold
    #[arg(short = 'l', long, help = "Log2 fold change threshold")]
    log2fc_thresh: f32,

    /// Name of the numerator condition
    #[arg(short = 'n', long, help = "Name of the numerator condition")]
    numerator: String,

    /// Name of the denominator condition
    #[arg(short = 'd', long, help = "Name of the denominator condition")]
    denominator: String,
}

// Create a struct for input data that matches the input file's columns
#[derive(Debug, Deserialize)]
struct InputDegInfo {
    #[serde(rename = "geneName")]
    gene_name: String,
    #[serde(rename = "baseMean")]
    base_mean: Option<f32>,
    #[serde(rename = "log2FoldChange")]
    log2_fold_change: Option<f32>,
    padj: Option<f32>,
}

// Create a struct for output data
#[derive(Debug, serde::Serialize)]
struct DegInfo {
    #[serde(rename = "geneName")]
    gene_name: String,
    #[serde(rename = "baseMean")]
    base_mean: f32,
    #[serde(rename = "log2FoldChange")]
    log2_fold_change: f32,
    padj: f32,
    regulation: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // I/O reader
    let mut unfiltered = csv::Reader::from_path(&args.input_csv)?;
    let mut filtered = csv::Writer::from_path(&args.output_csv)?;

    // Main loop
    for result in unfiltered.deserialize::<InputDegInfo>() {
        let record = result?;

        // Skip records with missing or invalid values
        if let (Some(base_mean), Some(log2_fold_change), Some(padj)) = (
            record.base_mean,
            record.log2_fold_change,
            record.padj,
        ) {
            // Main filtering logic
            if padj < args.padj_thresh && log2_fold_change.abs() > args.log2fc_thresh {
                let regulation = if log2_fold_change > 0.0 {
                    format!("Downregulated in {}", args.denominator)
                } else {
                    format!("Upregulated in {}", args.denominator)
                };

                // Add filtered information to out struct
                let out = DegInfo {
                    gene_name: record.gene_name,
                    base_mean,
                    log2_fold_change,
                    padj,
                    regulation,
                };

                filtered.serialize(out)?;
            }
        }
    }
    filtered.flush()?;

    Ok(())
}
