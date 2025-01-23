//
// ---------------------------------------------------------------------------
// TODO:
// Output format...
// ---------------------------------------------------------------------------
//
use clap::Parser;
mod json_person;
use json_person::{read_persons_jsonl, PersonClean, PersonJson};
mod json_research;
use json_research::{read_research_jsonl, ResearchClean, ResearchJson};
mod json_fingerprint;
use json_fingerprint::{read_fingerprint_jsonl, FingerprintJson};
mod json_concepts;
use json_concepts::{read_concept_jsonl, ConceptJson};
mod json_orgunits;
use json_orgunits::{read_orgunits_jsonl, OrgUnitJson};
mod combined;
use combined::Combined;
mod formatting;
use flexi_logger::{DeferredNow, Record};
use flexi_logger::{Duplicate, FileSpec, LogSpecification, Logger, WriteMode};
use formatting::{extract_text_with_formatting, extract_texts_with_formatting};
use log::{debug, error, info, trace, warn, LevelFilter};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
mod errors;
mod uuid_map;
use uuid_map::UuidMap;

#[derive(Parser)]
#[command(version, about, long_about = "Reading data.")]
struct Cli {
    /// Research info jasonl file
    #[arg(
        short,
        long,
        help = "The file containing the cleaned research-outputs."
    )]
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
    #[arg(
        short,
        long,
        help = "The file containing the cleaned organisational-units."
    )]
    orgunits: Option<String>,

    /// Opt-out uuids.
    #[arg(
        short = 'u',
        long = "optout",
        help = "The file containing the opt-out uuids."
    )]
    optout: Option<String>,

    /// Sets the locale for the extracted texts.
    #[arg(short, long, default_value = "en_GB")]
    locale: String,

    /// Sets the level of logging;
    /// error, warn, info, debug, or trace
    #[arg(long = "ll", default_value = "warn")]
    log_level: String,
}

fn log_format(
    w: &mut dyn Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
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
                .suffix("log"),
        )
        .append()
        .duplicate_to_stderr(Duplicate::All)
        .write_mode(WriteMode::BufferAndFlush)
        .start()?;

    info!("Starting lucris-rs.");

    // ------------------------------------------------------------------------

    // The map. This translates uuids to "safe" uuids.
    // them somewhere.
    let mut umap = UuidMap::new();

    if let Some(optout_filename) = cli.optout {
        info!("Reading optout file {:?}.", optout_filename);
        let optout_count = umap.read_optouts(&optout_filename)?;
        info!("Read {} optout_count UUIDs.", optout_count);
        info!("Mappings {}.", umap);
    }

    // Parse the research data, structures are pushed
    // into a vector. Reads the research.jsonl and creates the
    // person->[research, ...] vector.
    let mut research_data: Option<Vec<ResearchJson>> = None;
    let mut person_research: Option<HashMap<String, Vec<String>>> = None;

    if let Some(research_filename) = cli.research {
        info!("Reading research file {:?}.", research_filename);
        match read_research_jsonl(&research_filename, &umap) {
            Ok((res_data, pers_data)) => {
                research_data = Some(res_data);
                info!(
                    "Research data contains {} elements.",
                    research_data
                        .as_ref() // Converts &Option<T> to Option<&T>.
                        .expect("No research data")
                        .len()
                );
                person_research = Some(pers_data);
                info!(
                    "Person-research contains {} elements.",
                    person_research
                        .as_ref()
                        .expect("No person-research data")
                        .len()
                );
            }
            Err(e) => eprintln!("Error reading JSON: {}", e),
        }
    }

    // Save a mapping from uuid to data, so we can combine later.
    let mut research_map: HashMap<String, ResearchClean> = HashMap::new();

    // All the uuids are uniq (should be...). We could make a map
    // with uuids->data to connect it to the other data.
    info!("Convert ResearchJSON to ResearchClean.");
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

                // FILTER FORBIDDEN HERE?

                // Convert the ResearchJson to ResearchClean, keeping only the
                // relevant fields.
                match ResearchClean::try_from_with_locale_umap(entry, &cli.locale, &mut umap) {
                    Ok(research_des) => {
                        let json_output = serde_json::to_string(&research_des).unwrap();
                        trace!("{}", json_output);
                        let safe_uuid = umap.get_uuid_as_str(uuid);
                        research_map.insert(safe_uuid, research_des);
                    }
                    Err(e) => {
                        panic!("Failed to convert ResearchJson: {:?}", e);
                    }
                }
            } else {
                error!("Research JSON does not contain uuid.");
            }
        } // for entry

    /*
    let foo:Vec<ResearchJsonDes> = data.iter()
        .map(|x| ResearchJsonDes::try_from_with_locale(x, &cli.locale).unwrap())
        .collect();
    */
    } else {
        debug!("No research data available.");
    }

    info!("Mappings {}.", &umap);

    for v in research_map.values() {
        trace!("{}", v);
    }

    // ------------------------------------------------------------------------

    // Parse the persons JSON file. Each struct is pushed into
    // a vector.
    let mut persons_data: Option<Vec<PersonJson>> = None;
    if let Some(persons_filename) = cli.persons {
        info!("Reading persons file {:?}.", persons_filename);
        match read_persons_jsonl(&persons_filename, &umap) {
            Ok(data) => {
                persons_data = Some(data);
                info!(
                    "Person data contains {} elements.",
                    persons_data
                        .as_ref() // Converts Option<T> to Option<&T>.
                        .expect("No persons data")
                        .len()
                );
            }
            Err(e) => {
                panic!("Failed to read PersonJson: {:?}", e)
            }
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
                    println!("Name: {} {} {}", first_name, last_name, uuid);
                } else {
                    error!("First or last name not found.");
                }
                trace!("{:?}", entry.get_all_education_pure_ids());
                let info_texts = entry.get_profile_information_texts_for_locale(&cli.locale);
                let info_texts = extract_texts_with_formatting(&info_texts);
                trace!("{:?}", info_texts);
                //println!("{:?}",info_texts);

                // Convert to PersonClean structures.
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
            } else {
                error!("Research JSON does not contain uuid.");
            }
        }
    } else {
        debug!("No persons data available.");
    }

    info!("Mappings {}.", &umap);

    for v in person_map.values() {
        trace!("{}", v);
    }

    /*for v in research_map.values() {
        println!("research: {}", v);
        for p in &v.persons {
            let uuid = &p.uuid;
            if let Some(value) = person_map.get(uuid) {
                println!("-> person in research: {}", value);
            }
        }
    }*/

    // Go through the research_map, extracts the person-uuids and look them up in the
    // person_map. Print/store/save/...

    // ------------------------------------------------------------------------

    // Parse the fingerprints JSON. Each struct is pushed into
    // a vector.
    let mut fingerprints_data: Option<Vec<FingerprintJson>> = None;
    if let Some(fingerprints_filename) = cli.fingerprints {
        info!("Reading fingerprint file {:?}.", fingerprints_filename);
        match read_fingerprint_jsonl(&fingerprints_filename) {
            Ok(data) => {
                fingerprints_data = Some(data);
                info!(
                    "Fingerprint data contains {} elements.",
                    fingerprints_data
                        .as_ref() // Converts Option<T> to Option<&T>.
                        .expect("No fingerprints data")
                        .len()
                );
                if let Some(fd) = &fingerprints_data {
                    for fp in fd {
                        //println!("\n{:?}", fp);
                    }
                }
            }
            Err(e) => eprintln!("Error reading FingerprintJSON: {:?}", e),
        }
    }
    trace!("{:?}", fingerprints_data);

    // ------------------------------------------------------------------------

    // Parse the concepts JSON. Each struct is pushed into
    // a vector.
    let mut concepts_data: Option<Vec<ConceptJson>> = None;
    if let Some(concepts_filename) = cli.concepts {
        info!("Reading concepts file {:?}.", concepts_filename);
        match read_concept_jsonl(&concepts_filename) {
            Ok(data) => {
                concepts_data = Some(data);
                info!(
                    "Concepts data contains {} elements.",
                    concepts_data
                        .as_ref() // Converts Option<T> to Option<&T>.
                        .expect("No concepts data")
                        .len()
                );
                if let Some(cd) = &concepts_data {
                    for c in cd {
                        //println!("\n{:?}", c);
                    }
                }
            }
            Err(e) => eprintln!("Error reading ConceptJSON: {:?}", e),
        }
    }
    trace!("{:?}", concepts_data);

    // ------------------------------------------------------------------------

    // Parse the orgunits JSON. Each struct is pushed into
    // a vector.
    let mut orgunits_data: Option<Vec<OrgUnitJson>> = None;
    if let Some(orgunits_filename) = cli.orgunits {
        info!("Reading organisational-units file {:?}.", orgunits_filename);
        match read_orgunits_jsonl(&orgunits_filename) {
            Ok(data) => {
                orgunits_data = Some(data);
                info!(
                    "Orgunits data contains {} elements.",
                    orgunits_data
                        .as_ref() // Converts Option<T> to Option<&T>.
                        .expect("No orgunits data")
                        .len()
                );
            }
            Err(e) => eprintln!("Error reading OrgunitsJSON: {:?}", e),
        }
    }
    trace!("{:?}", orgunits_data);

    // ------------------------------------------------------------------------

    // TODO: How to connect everything?
    // Use Combined.
    // Note that the person_reseach is the Option<...> returned from read_research_jsonl(...)
    // without processing.
    // If we don't read the research data, this will fail!
    // Should this thing include "optout" uuids? We need to keep them somewhere.
    // But the uuids have already been translated to "safe"... We can translate them too...
    // umap is an arg to the functions, could be there too?
    info!("Creating Combined.");
    let optout_uuids = vec![];
    let combined = Combined::new(
        research_map,
        person_map,
        person_research.expect("No person_research data?"),
        optout_uuids,
    );
    info!("{}", combined);
    //trace!("{:?}", &combined);

    // dd0ce568-96e7-449b-9a59-9ee857f79a13 (ok in research_1.jsonl)
    // 147e206b-b9d5-49a6-bc83-ddec9ff21af1 (ok in research_10.jasonl)
    // dd0ce568-96e7-449b-9a59-9ee857f79a13 (err in research_10.jasonl)
    //
    /*
    match combined.get_research_from_uuid_ref("dd0ce568-96e7-449b-9a59-9ee857f79a13") {
        Ok((research, persons)) => {
            println!("Research: {:?}", research);
            for person in persons {
                println!("{} / {}", research, person);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
    */

    //combined.output_test();

    /*
    println!("combined.get_research_for_person_uuid(...)");
    match combined.get_research_for_person_uuid("61781b1a-c069-4971-bb76-b18ed231a453") {
        Ok(res) => {
            for r in res {
                println!("-> {}", r);
            }
        },
        _ => ()
    }
    */

    // Output name, research title & abstract (nta format for haystack_research.py).
    // What we want is maybe a HayStack compatible (JSON) string.
    //   doc = Document(content=page.content, meta={"title": page.title, "url": page.url})
    // for (person_uuid, person) in &combined.persons {
    //     //println!("\n{}", person.get_name());
    //     if let Ok(res) = combined.get_research_for_person_uuid(person_uuid) {
    //         for r in res {
    //             /*println!("{}, {}, {}",
    //             person.get_name(),
    //             r.get_title(),
    //             r.get_abstract()
    //             );*/
    //             // TODO there is a get_internal_person_names() too.
    //             // Better to iterate over research instead of persons.
    //             println!(
    //                 "NAME:{}\nTITLE:{}\nABSTRACT:{}",
    //                 person.get_name(),
    //                 r.get_title(),
    //                 r.get_abstract()
    //             );
    //         }
    //     }
    // }

    for r in combined.research.values() {
        debug!("research clean uuid={:?}", r.get_uuid());
        trace!("{:?}", r);
        let names: Vec<_> = r
            .persons
            .iter()
            //.filter(|p| p.is_internal()) // Filter by the `inex` variable
            .map(|p| p.get_name())
            .collect();
        if names.is_empty() {
            println!("No names! {}", r.get_title());
        }
        // TODO Check the type of research (journal, etc).
        println!("NAMES:{}", names.join(","));
        println!("TITLE:{}", r.get_title());
        println!("ABSTRACT:{}", r.get_abstract());
    }

    // ------------------------------------------------------------------------

    /*
    let id = Uuid::new_v4();
    println!("{} {}", id, id.urn());
    */

    info!("Ending lucris-rs.");
    Ok(())
}
