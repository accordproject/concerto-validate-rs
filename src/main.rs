use clap::{Parser, Subcommand};
use concerto_validator_rs::{validate_metamodel, ValidationError};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "concerto-validator")]
#[command(about = "A CLI tool for validating Concerto model JSON ASTs")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate Concerto model JSON files
    Validate {
        /// Input JSON files to validate (can be specified multiple times)
        #[arg(short, long, value_name = "FILE")]
        input: Vec<PathBuf>,

        /// Stop validation at the first error
        #[arg(long)]
        fail_early: bool,
    },
}

#[derive(Debug)]
struct ValidationReport {
    total_files: usize,
    successful: usize,
    failed: usize,
    errors: Vec<(PathBuf, ValidationError)>,
}

impl ValidationReport {
    fn new() -> Self {
        Self {
            total_files: 0,
            successful: 0,
            failed: 0,
            errors: Vec::new(),
        }
    }

    fn add_success(&mut self) {
        self.total_files += 1;
        self.successful += 1;
    }

    fn add_error(&mut self, file: PathBuf, error: ValidationError) {
        self.total_files += 1;
        self.failed += 1;
        self.errors.push((file, error));
    }

    fn print_summary(&self) {
        println!("\n=== Validation Report ===");
        println!("Total files processed: {}", self.total_files);
        println!("Successful validations: {}", self.successful);
        println!("Failed validations: {}", self.failed);

        if !self.errors.is_empty() {
            println!("\nErrors:");
            for (file, error) in &self.errors {
                println!("  {}: {}", file.display(), error);
            }
        }

        if self.failed == 0 {
            println!("\n✅ All validations passed!");
        } else {
            println!("\n❌ {} validation(s) failed", self.failed);
        }
    }

    fn has_errors(&self) -> bool {
        self.failed > 0
    }
}

fn main() {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        Commands::Validate { input, fail_early } => handle_validate_command(input, fail_early),
    };

    std::process::exit(exit_code);
}

fn handle_validate_command(input_files: Vec<PathBuf>, fail_early: bool) -> i32 {
    if input_files.is_empty() {
        eprintln!(
            "Error: No input files specified. Use --input to specify JSON files to validate."
        );
        return 1;
    }

    let mut report = ValidationReport::new();

    for file_path in input_files {
        match validate_file(&file_path) {
            Ok(()) => {
                println!("✅ {}: Valid", file_path.display());
                report.add_success();
            }
            Err(error) => {
                println!("❌ {}: {}", file_path.display(), error);
                report.add_error(file_path, error);

                if fail_early {
                    println!("\nStopping validation due to --fail-early flag.");
                    break;
                }
            }
        }
    }

    if !fail_early {
        report.print_summary();
    }

    if report.has_errors() {
        1
    } else {
        0
    }
}

fn validate_file(file_path: &PathBuf) -> Result<(), ValidationError> {
    // Read the file
    let content = fs::read_to_string(file_path).map_err(ValidationError::IoError)?;

    // Validate the content
    validate_metamodel(&content)
}
