//
// ---------------------------------------------------------------------------
// TODO:
// Output format...
// ---------------------------------------------------------------------------
//
use clap::{Parser};
mod json_person;
use json_person::{read_persons_jsonl, PersonJson, PersonJsonDes};
mod json_research;
use json_research::{ResearchJson, ResearchJsonDes, read_research_jsonl, dump_titles};
mod json_fingerprint;
use json_fingerprint::{read_fingerprint_jsonl, FingerprintJson};
mod json_concepts;
use json_concepts::{read_concept_jsonl, ConceptJson};
mod json_orgunits;
use json_orgunits::{read_orgunits_jsonl, OrgUnitJson};
mod combined;
use combined::Combined;
mod formatting;
use formatting::{extract_text_with_formatting, extract_texts_with_formatting};
use std::collections::HashMap;
use uuid::Uuid;
use log::{debug, error, info, trace, warn, LevelFilter};
use flexi_logger::{FileSpec, Logger, WriteMode, AdaptiveFormat, Duplicate, LogSpecification};
use std::str::FromStr;
use flexi_logger::{DeferredNow, Record};
use std::io::Write;
use std::path::Path;
mod errors;

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

    /// Sets the locale for the extracted texts.
    #[arg(short, long, default_value = "en_GB")]
    locale: String,

    /// Sets the level of logging;
    /// error, warn, info, debug, or trace
    #[arg(long = "ll", default_value = "warn")]
    log_level: String,
}

fn log_format(w: &mut dyn Write, now: &mut DeferredNow, record: &Record) -> Result<(), std::io::Error> {
    let file_path = record.file().unwrap_or("<unknown>");
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("<unknown>");
    let line = record.line().unwrap_or(0);
    write!(
        w,
        "{} [{}] {}:{} - {}",
        now.format("%Y-%m-%d %H:%M:%S"), // Custom timestamp format without timezone.
        record.level(),
        file_name,
        line,
        &record.args()
    )
}

// TODO: Better error handling.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // This switches off logging from html5 and other crates.
    let level_filter = LevelFilter::from_str(&cli.log_level).unwrap_or(LevelFilter::Off);
    let log_spec = LogSpecification::builder()
        .module("html5ever", LevelFilter::Off)
        .module("lucris", level_filter) // Sets our level to the one on the cli.
        .build();
    
    let _logger = Logger::with(log_spec)
        .format(log_format)
        .log_to_file(
            FileSpec::default()
                .suppress_timestamp()
                .basename("lucris")
                .suffix("log")
        )
        .append()
        .duplicate_to_stderr(Duplicate::All)
        .write_mode(WriteMode::BufferAndFlush)
        .start()?;
    
    info!("Starting lucris-rs.");

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
        if &cli.log_level == "trace" {
            dump_titles(research_data.as_ref().unwrap(), &cli.locale);
        }
    }
    
    // All the uuids are uniq (should be...). We could make a map
    // with uuids->data to connect it to the other data.
    let mut uuids: HashMap<String, u64> = HashMap::new();
    if let Some(data) = research_data {
        for entry in &data {
            if let Some(uuid) = entry.get_uuid() {
                if uuids.contains_key(uuid) == true {
                    warn!("Repeating research uuid: {}", uuid);
                }
                uuids.insert(uuid.to_string(), 0);
                //let comb = Combined::from(entry);
                //println!("{:?}", comb);
                /*
                let person_names = entry.get_person_names(); // People responsible for the research.
                for (i, (first_name, last_name, uuid)) in person_names.iter().enumerate() {
                    trace!("Person {}: {} {} {}", i, first_name, last_name, uuid);
                    // Often more than one.
                    println!("PERSON{}: {} {} {}", i, first_name, last_name, uuid);
                }
                // Lookup uuid in person_data below. Connect. Does that give extra
                // research info? Profile information?
                //
                // The abstract, cleaned because it often contains HTML.
                let (abstract_title, abstract_text) = entry.get_title_abstract(&cli.locale);
                let abstract_text = extract_text_with_formatting(abstract_text);
                println!("TITLE: {}", abstract_title);
                println!("ABSTRACT: {}", abstract_text);
                */
                
                // TEST
                match ResearchJsonDes::try_from_with_locale(entry, &cli.locale) {
                Ok(research_des) => {
                    let json_output = serde_json::to_string(&research_des).unwrap();
                    println!("{}\n", json_output);
                }
                Err(e) => {
                    panic!("Failed to convert ResearchJson: {:?}", e);
                }
            }
            // TEST

            } else {
                error!("Research JSON does not contain uuid.");
            }
        } // for entry
        
        let foo:Vec<ResearchJsonDes> = data.iter()
            .map(|x| ResearchJsonDes::try_from_with_locale(x, &cli.locale).unwrap())
            .collect();
        
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
                    warn!("Repeating person uuid: {}", uuid);
                }
                persons_uuids.insert(uuid.to_string(), 0);
            } else {
                error!("Research JSON does not contain uuid.");
            }
            if let Some((first_name, last_name)) = entry.get_first_and_last_name() {
                trace!("Name: {} {}", first_name, last_name);
            } else {
                error!("First or last name not found.");
            }
            trace!("{:?}", entry.get_all_education_pure_ids());
            let info_texts = entry.get_profile_information_texts_for_locale(&cli.locale);
            let info_texts = extract_texts_with_formatting(&info_texts);
            trace!("{:?}", info_texts);
            //println!("{:?}",info_texts);

            // TEST
            match PersonJsonDes::try_from_with_locale(entry, &cli.locale) {
                Ok(person_des) => {
                    let json_output = serde_json::to_string(&person_des).unwrap();
                    println!("{}", json_output);
                }
                Err(e) => {
                    panic!("Failed to convert PersonJson: {:?}", e);
                }
            }
            // TEST
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
    
    info!("Ending lucris-rs.");
    Ok(())
}

