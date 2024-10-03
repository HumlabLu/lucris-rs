use clap::{Parser};
mod json;
use json::{read_jsonl, ResearchJson};
mod json_person;
use json_person::{read_persons_jsonl, PersonJson, test_person};
mod json_research;
use json_research::{test};

#[macro_use]
extern crate simple_log;
use simple_log::LogConfigBuilder;

#[derive(Parser)]
#[command(version, about, long_about = "Reading data.")]
struct Cli {
    /// Research info jasonl file
    #[arg(short, long, help = "The file containing the cleaned research-outputs.")]
    research: Option<String>,

    /// Persons info jasonl file
    #[arg(short, long, help = "The file containing the cleaned persons.")]
    persons: Option<String>,

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

    let mut research_data: Option<Vec<ResearchJson>> = None;
    if let Some(research_filename) = cli.research {
        info!("Research file {:?}.", research_filename);
        match read_jsonl(&research_filename) {
            Err(e) => eprintln!("Error reading JSON: {}", e),
            Ok(data) => {
                info!("We got {:?}", data.len());
                research_data = Some(data);
            },
        }
    }

    // All the uuids are uniq (should be...). We could make a map
    // with uuids->data to connect it to the other data.
    if let Some(data) = research_data {
        for entry in &data {
            // Do something with each entry
            //println!("{:?}", entry.uuid);
            println!("{:?}\n", entry);
        }
    } else {
        println!("No research data available.");
    }

    let mut persons_data: Option<Vec<PersonJson>> = None;
    if let Some(persons_filename) = cli.persons {
        info!("Research file {:?}.", persons_filename);
        match read_persons_jsonl(&persons_filename) {
            Err(e) => eprintln!("Error reading JSON: {}", e),
            Ok(data) => {
                info!("We got {:?}", data.len());
                persons_data = Some(data);
            },
        }
    }

    if let Some(data) = persons_data {
        for entry in &data {
            // Do something with each entry
            println!("{:?}\n", entry);
        }
    } else {
        println!("No persons data available.");
    }

    test();
    test_person();
    
    debug!("Ending lucris-rs.");
    Ok(())
}
