use clap::{Parser};
mod json_person;
use json_person::{read_persons_jsonl, PersonJson};
mod json_research;
use json_research::{ResearchJson, read_research_jsonl};
use std::collections::HashMap;

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
        .size(1024) // MB
        .roll_count(10)
        .output_file()
        .level(cli.log_level)?
        .time_format("%Y-%m-%d %H:%M:%S")
        .output_console()
        .build();
    simple_log::new(config)?;
    debug!("Starting lucris-rs.");
    
    // Parse the research data, structures are pushed
    // into a vector.
    let mut research_data: Option<Vec<ResearchJson>> = None;
    if let Some(research_filename) = cli.research {
        info!("Research file {:?}.", research_filename);
        match read_research_jsonl(&research_filename) {
            Err(e) => eprintln!("Error reading JSON: {}", e),
            Ok(data) => {
                research_data = Some(data);
            },
        }
    }

    // All the uuids are uniq (should be...). We could make a map
    // with uuids->data to connect it to the other data.
    let mut uuids: HashMap<String, u64> = HashMap::new();
    if let Some(data) = research_data {
        for entry in &data {
            if let Some(uuid) = entry.get_uuid() {
                //println!("{}", uuid);
                if uuids.contains_key(uuid) == true {
                    warn!("Repeating uuid: {}", uuid);
                }
                uuids.insert(uuid.to_string(), 0);
            } else {
                error!("Research JSON does not contain uuid.");
            }
        }
    } else {
        debug!("No research data available.");
    }

    // Parse the persons JSON. Each struct is pushed into
    // a vector. 
    let mut persons_data: Option<Vec<PersonJson>> = None;
    if let Some(persons_filename) = cli.persons {
        info!("Persons file {:?}.", persons_filename);
        match read_persons_jsonl(&persons_filename) {
            Err(e) => eprintln!("Error reading JSON: {}", e),
            Ok(data) => {
                persons_data = Some(data);
            },
        }
    }

    // If we successfully parsed person data, we extracts
    // the uuids.
    let mut persons_uuids: HashMap<String, u64> = HashMap::new();
    if let Some(data) = persons_data {
        for entry in &data {
            // Do something with each entry
            //println!("{:?}\n", entry);
            if let Some(uuid) = entry.get_uuid() {
                //println!("{}", uuid);
                if persons_uuids.contains_key(uuid) == true {
                    warn!("Repeating uuid: {}", uuid);
                }
                persons_uuids.insert(uuid.to_string(), 0);
            } else {
                error!("Research JSON does not contain uuid.");
            }
            if let Some((first_name, last_name)) = entry.get_first_and_last_name() {
                //println!("Name: {} {}", first_name, last_name);
            } else {
                error!("First or last name not found.");
            }
        }
    } else {
        debug!("No persons data available.");
    }

    debug!("Ending lucris-rs.");
    Ok(())
}
