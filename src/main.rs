use clap::{Parser, Arg};
use lopdf::{Document};
use walkdir::WalkDir;
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "PDF Merger", about = "A tool to merge all PDFs in a given directory.")]
struct Cli {
    /// The folder to search for PDF files. Uses the current folder if not specified.
    #[clap(long, parse(from_os_str), default_value = ".")]
    folder: PathBuf,

    /// The output file to save the merged PDF. Defaults to "merged_output.pdf" in the current directory.
    #[clap(long, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut merged_document = Document::with_version("1.5");

    for entry in WalkDir::new(&args.folder) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "pdf") {
            println!("Merging: {:?}", path.display());
            let mut doc = Document::load(path)?;
            merged_document.append_document(&mut doc);
        }
    }

    // Determine output file path
    let output_path = args.output.unwrap_or_else(|| PathBuf::from("merged_output.pdf"));
    let mut output_file = File::create(&output_path)?;

    // Save the merged PDF
    merged_document.save_to(&mut output_file)?;

    println!("PDFs merged into {:?}", output_path.display());
    
    Ok(())
}

