use clap::{Parser};
mod json;
use json::read_jsonl;

#[macro_use]
extern crate simple_log;
use simple_log::LogConfigBuilder;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Research info jasonl file
    #[arg(short, long, help="The file containing the cleaned research-outputs.")]
    research: Option<String>,
    
    /// Sets the level of logging;
    /// error (highest priority), warn, info, debug, or trace
    #[arg(short, long, default_value = "warn")]
    log_level: String,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let config = LogConfigBuilder::builder()
        .path("./lucris.log")
        .output_file()
        .level(cli.log_level)?
        .time_format("%Y-%m-%d %H:%M:%S")
        .output_console()
        .build();
    simple_log::new(config)?;
    debug!("Starting lucris-rs.");

    if let Some(research_filename) = cli.research {
        info!("Research file {:?}.", research_filename);
        match read_jsonl(&research_filename) {
            Ok(data) => println!("{:?}", data),
            Err(e) => eprintln!("Error reading JSON: {}", e),
        }
    }
    
    debug!("Ending lucris-rs.");
    Ok(())
}
