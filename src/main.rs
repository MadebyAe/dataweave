use itertools::Itertools;
use serde_json;
use serde_yaml;
use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File};
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "Test Case Matrix Generator")]
#[command(version = "1.0")]
#[command(about = "Generate a CSV test case matrix from JSON or YAML file", long_about = None)]
struct Args {
    /// Input file path (JSON or YAML format)
    #[arg(short = 'f', long)]
    file: String,

    /// Output CSV file path (optional, defaults to 'output.csv')
    #[arg(short = 'o', long, default_value = "output.csv")]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Auto-detect the file format based on the extension
    let extension = detect_extension(&args.file)?;

    // Load parameters from file
    let parameters = load_parameters_from_file(&args.file, &extension)?;

    // Extract parameter names and values
    let parameter_names: Vec<String> = parameters.keys().cloned().collect();
    let parameter_values: Vec<Vec<String>> = parameters.values().cloned().collect();

    // Generate all possible permutations
    let all_permutations: Vec<Vec<String>> = parameter_values.into_iter().multi_cartesian_product().collect();

    // Write CSV header
    // Create and write to the CSV file
    let mut wtr = csv::Writer::from_writer(File::create(&args.output)?);
    wtr.write_record(&parameter_names)?;

    // Write all test cases
    for row in &all_permutations {
        wtr.write_record(row)?;
    }

    let permutation_count = all_permutations.len();

    wtr.flush()?;
    println!("✅ Test case matrix saved as {} with {} permutations.", &args.output, permutation_count);

    Ok(())
}

// Function to detect the file format (JSON or YAML)
fn detect_extension(file_path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(file_path);
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => Ok("json".to_string()),
        Some("yaml") | Some("yml") => Ok("yaml".to_string()),
        _ => Err("❌ Unsupported file format. Use a .json or .yaml file.".into()),
    }
}

// Function to load parameters from a file
fn load_parameters_from_file(file_path: &str, extension: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    parse_parameters(&content, extension)
}

// Function to parse parameters from JSON or YAML
fn parse_parameters(input: &str, extension: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    if extension == "json" {
        serde_json::from_str::<HashMap<String, Vec<String>>>(input).map_err(|e| e.into())
    } else {
        serde_yaml::from_str::<HashMap<String, Vec<String>>>(input).map_err(|e| e.into())
    }
}
