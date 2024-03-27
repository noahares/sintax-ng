use anyhow::{bail, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use std::{io::Write, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Path to the database fasta file
    #[arg(short, long)]
    pub database_path: PathBuf,
    /// Path to the query file
    #[arg(short = 'i', long)]
    pub query_file: PathBuf,
    /// Number of rounds per query
    #[arg(short, long, default_value_t = 100)]
    pub num_iterations: usize,
    /// Number of 8-mers
    #[arg(short = 'k', long, default_value_t = 32)]
    pub num_k_mers: usize,
    /// 8-mer hit-threshold
    #[arg(short = 'f', long, default_value_t = 1.0 / 3.0)]
    pub min_hit_fraction: f64,
    /// Confidence threshold
    #[arg(short = 'c', long, default_value_t = 0.8)]
    pub min_confidence: f64,
    /// Number of output species per query
    #[arg(short = 'm', long, default_value_t = 5)]
    pub max_target_seqs: usize,
    /// The MSE of none-zero values in the hit buffer for early stopping
    /// (This should be around 10e-5 to 10e-7 depending on the required accuracy)
    #[arg(short = 'e', long, verbatim_doc_comment)]
    pub early_stop_mse: Option<f64>,
    /// Fraction of iterations to run before checking the MSE
    /// Has no effect if --early-stop-mse is not provided
    #[arg(short = 'p', long, default_value_t = 0.1, verbatim_doc_comment)]
    pub min_iterations: f64,
    /// Number of threads
    #[arg(short, long, default_value_t = 0)]
    pub threads: usize,
    /// Seed
    #[arg(short, long, default_value_t = 42)]
    pub seed: u64,
    /// Output path
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    /// confidence output path
    #[arg(short = 'u', long)]
    pub confidence_output: Option<PathBuf>,
    /// Force override of existing output files
    #[arg(long)]
    pub redo: bool,
    #[command(flatten)]
    pub verbosity: Verbosity,
}

impl Args {
    pub fn get_output(&self) -> Result<Box<dyn Write>> {
        let path = self
            .output
            .clone()
            .unwrap_or(self.query_file.with_extension("sintax.out"));
        if path.is_file() && !self.redo {
            bail!("Output file {} already exists! Please specify another file with -o <PATH> or run with --redo to force overriding existing files!", path.display());
        }
        Ok(std::fs::File::create(path).map(|f| Box::new(f) as Box<dyn Write>)?)
    }
    pub fn get_confidence_output(&self) -> Result<Box<dyn Write>> {
        let path = self.confidence_output.clone().unwrap_or(
            self.query_file
                .with_extension(format!("sintax.conf{}.out", self.min_confidence)),
        );
        if path.is_file() && !self.redo {
            bail!("Output file {} already exists! Please specify another file with -u <PATH> or run with --redo to force overriding existing files!", path.display());
        }
        Ok(std::fs::File::create(path).map(|f| Box::new(f) as Box<dyn Write>)?)
    }
}
