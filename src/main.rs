use clap::{Parser};
mod json_person;
use json_person::{read_persons_jsonl, PersonJson};
mod json_research;
use json_research::{ResearchJson, read_research_jsonl, dump_titles};
mod json_fingerprint;
use json_fingerprint::{read_fingerprint_jsonl, FingerprintJson};
mod json_concepts;
use json_concepts::{read_concept_jsonl, ConceptJson};
mod json_orgunits;
use json_orgunits::{read_orgunits_jsonl, OrgUnitJson};
mod combined;
use combined::Combined;
use std::collections::HashMap;
use uuid::Uuid;

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

    /// Fingerprint info jasonl file
    #[arg(short, long, help = "The file containing the cleaned fingerprints.")]
    fingerprints: Option<String>,

    /// Concept info jasonl file
    #[arg(short, long, help = "The file containing the cleaned concepts.")]
    concepts: Option<String>,

    /// OrgUnit info jasonl file
    #[arg(short, long, help = "The file containing the cleaned organisational-units.")]
    orgunits: Option<String>,

    /// Sets the level of logging;
    /// error, warn, info, debug, or trace
    #[arg(short, long, default_value = "warn")]
    log_level: String,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    #[cfg(not(debug_assertions))]
    let config = LogConfigBuilder::builder()
        .path("./lucris.log")
        .size(1024) // MB
        .roll_count(10)
        .output_file()
        .level(&cli.log_level)?
        .time_format("%Y-%m-%d %H:%M:%S")
        .output_console()
        .build();
    #[cfg(debug_assertions)]
    let config = LogConfigBuilder::builder()
        .path("./lucris.log")
        .size(1024) // MB
        .roll_count(10)
        .output_file()
        .level(&cli.log_level)?
        .time_format("%Y-%m-%d %H:%M:%S.%f")
        .output_console()
        .build();
    simple_log::new(config)?;
    debug!("Starting lucris-rs.");

    // ------------------------------------------------------------------------
    
    // Parse the research data, structures are pushed
    // into a vector.
    let mut research_data: Option<Vec<ResearchJson>> = None;
    if let Some(research_filename) = cli.research {
        info!("Reading research file {:?}.", research_filename);
        match read_research_jsonl(&research_filename) {
            Err(e) => eprintln!("Error reading JSON: {}", e),
            Ok(data) => {
                research_data = Some(data);
            },
        }
    }

    // .as_ref() produces &T inside Option<T>.
    if let Some(ref data) = research_data {
        // This dumps the authors, titles and abstracts
        // to stdout.
        if &cli.log_level == "debug" {
            dump_titles(research_data.as_ref().unwrap());
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
                //let comb = Combined::from(entry);
                //println!("{:?}", comb);
                let person_names = entry.get_person_names();
                for (i, (first_name, last_name, uuid)) in person_names.iter().enumerate() {
                    println!("Person {}: {} {} {}", i, first_name, last_name, uuid);
                }
                // Lookup uuid in person_data below. Connect. Does that give extra
                // research info? Profile information?
            } else {
                error!("Research JSON does not contain uuid.");
            }
        }
    } else {
        debug!("No research data available.");
    }

    // ------------------------------------------------------------------------
    
    // Parse the persons JSON. Each struct is pushed into
    // a vector. 
    let mut persons_data: Option<Vec<PersonJson>> = None;
    if let Some(persons_filename) = cli.persons {
        info!("Reading persons file {:?}.", persons_filename);
        match read_persons_jsonl(&persons_filename) {
            Err(e) => eprintln!("Error reading JSON: {}", e),
            Ok(data) => {
                persons_data = Some(data);
            },
        }
    }

    // If we successfully parsed person data, we extract
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
            /*
            if let Some((first_name, last_name)) = entry.get_first_and_last_name() {
                trace!("Name: {} {}", first_name, last_name);
            } else {
                error!("First or last name not found.");
            }
            trace!("{:?}", entry.get_all_education_pure_ids());
            */
            /*
            if let Some((first_name, last_name)) = entry.get_first_and_last_name() {
                println!("Name: {} {}", first_name, last_name);
            }
            let foo = entry.get_profile_information_texts_for_locale("en_GB");
            println!("{:?}", foo);
            let foo = entry.get_profile_information_texts_for_locale("sv_SE");
            println!("{:?}", foo);
            */
        }
    } else {
        debug!("No persons data available.");
    }

    // ------------------------------------------------------------------------
    
    // Parse the fingerprints JSON. Each struct is pushed into
    // a vector. 
    let mut fingerprints_data: Option<Vec<FingerprintJson>> = None;
    if let Some(fingerprints_filename) = cli.fingerprints {
        info!("Reading fingerprint file {:?}.", fingerprints_filename);
        match read_fingerprint_jsonl(&fingerprints_filename) {
            Err(e) => eprintln!("Error reading JSON: {}", e),
            Ok(data) => {
                fingerprints_data = Some(data);
            },
        }
    }

    // ------------------------------------------------------------------------
    
    // Parse the concepts JSON. Each struct is pushed into
    // a vector. 
    let mut concepts_data: Option<Vec<ConceptJson>> = None;
    if let Some(concepts_filename) = cli.concepts {
        info!("Reading concepts file {:?}.", concepts_filename);
        match read_concept_jsonl(&concepts_filename) {
            Err(e) => eprintln!("Error reading JSON: {}", e),
            Ok(data) => {
                concepts_data = Some(data);
            },
        }
    }

    // ------------------------------------------------------------------------
    
    // Parse the orgunits JSON. Each struct is pushed into
    // a vector. 
    let mut orgunits_data: Option<Vec<OrgUnitJson>> = None;
    if let Some(orgunits_filename) = cli.orgunits {
        info!("Reading organisational-units file {:?}.", orgunits_filename);
        match read_orgunits_jsonl(&orgunits_filename) {
            Err(e) => eprintln!("Error reading JSON: {}", e),
            Ok(data) => {
                orgunits_data = Some(data);
            },
        }
    }
    
    // ------------------------------------------------------------------------

    // TODO: How to connect everything?
    
    // ------------------------------------------------------------------------

    /*
    let id = Uuid::new_v4();
    println!("{} {}", id, id.urn());
    */
    
    debug!("Ending lucris-rs.");
    Ok(())
}
