//
// ---------------------------------------------------------------------------
// TODO:
// Output format...
// ---------------------------------------------------------------------------
//
use clap::{Parser};
mod json_person;
use json_person::{read_persons_jsonl, PersonJson, PersonClean};
mod json_research;
use json_research::{ResearchJson, ResearchClean, read_research_jsonl, dump_titles};
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
mod uuid_map;
use uuid_map::{UuidMap};

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
        now.format("%Y-%m-%d %H:%M:%S"), // Format without standard timezone.
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

    // The map. This translates uuids to "safe" uuids.
    let mut umap = UuidMap::new();

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
            //dump_titles(research_data.as_ref().unwrap(), &cli.locale);
        }
    }

    // Save a mapping from uuid to data, so we can combine later.
    let mut research_map: HashMap<String, ResearchClean> = HashMap::new();

    // All the uuids are uniq (should be...). We could make a map
    // with uuids->data to connect it to the other data.
    if let Some(data) = research_data {
        for entry in &data {
            if let Some(uuid) = entry.get_uuid() {
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
                match ResearchClean::try_from_with_locale(entry, &cli.locale) {
                    Ok(research_des) => {
                        let json_output = serde_json::to_string(&research_des).unwrap();
                        trace!("{}", json_output);
                        research_map.insert(uuid.to_string(), research_des);
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

        /*
        let foo:Vec<ResearchJsonDes> = data.iter()
            .map(|x| ResearchJsonDes::try_from_with_locale(x, &cli.locale).unwrap())
            .collect();
        */

        let mut foo = Vec::new();
        for x in data.iter() {
            let res = ResearchClean::try_from_with_locale_umap(x, &cli.locale, &mut umap).unwrap();
            foo.push(res);
        }

    } else {
        debug!("No research data available.");
    }

    info!("Mappings {}.", &umap.count());

    for (k, v) in &research_map {
        trace!("{}", v);
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

    // Save a mapping from uuid to data, so we can combine later. PersonClean
    // is a simpler/cleaner version of PersonJson with only the fields we are
    // interested in.
    let mut person_map: HashMap<String, PersonClean> = HashMap::new();

    if let Some(data) = persons_data {
        for entry in &data {
            // Do something with each entry
            //println!("{:?}\n", entry);
            if let Some(uuid) = entry.get_uuid() {
                // Check in uuid_map.
                //println!("--> {}", umap.get_uuid_as_str(uuid));

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
                match PersonClean::try_from_with_locale_umap(entry, &cli.locale, &mut umap) {
                    Ok(person_des) => {
                        let json_output = serde_json::to_string(&person_des).unwrap();
                        trace!("{}", json_output);
                        person_map.insert(uuid.to_string(), person_des);
                    }
                    Err(e) => {
                        panic!("Failed to convert PersonJson: {:?}", e);
                    }
                }
                // TEST
            } else {
                error!("Research JSON does not contain uuid.");
            }

        }

    } else {
        debug!("No persons data available.");
    }

    info!("Mappings {}.", &umap.count());

    for (k, v) in &person_map {
        trace!("{}", v);
    }

    for (k, v) in &research_map {
        println!("research: {}", v);
        for p in &v.persons {
            let uuid = &p.uuid;
            if let Some(value) = person_map.get(uuid) {
                println!("-> person in research: {}", value);
            }
        }
    }

    let combined = Combined::new(research_map, person_map);
    trace!("{:?}", &combined);

    // dd0ce568-96e7-449b-9a59-9ee857f79a13 (ok in research_1.jsonl)
    // 147e206b-b9d5-49a6-bc83-ddec9ff21af1 (ok in research_10.jasonl)
    // dd0ce568-96e7-449b-9a59-9ee857f79a13 (err in research_10.jasonl)
    match combined.get_research_from_uuid_ref("dd0ce568-96e7-449b-9a59-9ee857f79a13") {
        Ok((research, persons)) => {
            println!("Research: {:?}", research);
            for person in persons {
                println!("{} / {}", research, person);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }

    let all_uuids = combined.get_all_research_uuids();
    for uuid in all_uuids {
        println!("-------- {}", uuid);
        match combined.get_research_from_uuid_ref(uuid) {
            Ok((research, persons)) => {
                println!("Research: {:?}", research);
                for person in persons {
                    println!("{} / {}", research, person);
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
    // Go through the research_map, extracts the person-uuids and look them up in the
    // person_map. Print/store/save/...

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
    trace!("\n{:?}", fingerprints_data);

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
    trace!("\n{:?}", concepts_data);

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
    trace!("\n{:?}", orgunits_data);

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
