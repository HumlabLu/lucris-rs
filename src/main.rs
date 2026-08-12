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
use log::{debug, error, info, trace, LevelFilter};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
mod errors;
mod uuid_map;
use std::fs::File;
use std::io::{BufRead, BufReader};
use uuid_map::UuidMap;
mod filter;
use csv;
use filter::{
    filter_research_by_abstract, filter_research_by_keyword, filter_research_by_person, FilterMode,
};
use std::io;

#[derive(Parser)]
#[command(version, about, long_about = "Reading data.")]
struct Cli {
    /// Research info jsonl file
    #[arg(
        short,
        long,
        help = "The file containing the cleaned research-outputs."
    )]
    research: Option<String>,

    /// Persons info jsonl file
    #[arg(short, long, help = "The file containing the cleaned persons.")]
    persons: Option<String>,

    /// Fingerprint info jsonl file
    #[arg(short, long, help = "The file containing the cleaned fingerprints.")]
    fingerprints: Option<String>,

    /// Concept info jsonl file
    #[arg(short, long, help = "The file containing the cleaned concepts.")]
    concepts: Option<String>,

    /// OrgUnit info jsonl file
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

    #[arg(
        short = 'n',
        long = "names",
        help = "The file containing the names to keep."
    )]
    names: Option<String>,

    #[arg(
        short = 'k',
        long = "keywords",
        help = "The file containing the keywords to keep."
    )]
    keywords: Option<String>,

    #[arg(
        short = 'a',
        long = "abstract",
        help = "The file containing the abstract words to keep."
    )]
    // abstract is a reserved word...
    abstractterms: Option<String>,

    #[arg(
        long = "filtermode",
        value_enum,
        default_value_t = FilterMode::KeepMatching
        )]
    filtermode: FilterMode,

    /// Sets the locale for the extracted texts.
    #[arg(short, long, default_value = "en_GB")]
    locale: String,

    /// Sets the level of logging;
    /// error, warn, info, debug, or trace
    #[arg(long = "ll", default_value = "warn")]
    log_level: String,

    /// Output one JSON object per line instead of text.
    #[arg(short = 'j', long)]
    jsonl: bool,
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

// Helper function for names list.
fn read_names(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let names = reader
        .lines()
        .map(|line| line.map(|name| name.trim().to_owned()))
        .collect::<Result<Vec<String>, _>>()?
        .into_iter()
        .filter(|name| !name.is_empty())
        .collect();

    Ok(names)
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
    let mut umap = {
        let mut map = UuidMap::new();
        if let Some(filename) = cli.optout.as_deref() {
            info!("Reading optout file {:?}", filename);
            let count = map.read_optouts(filename)?;
            info!("Read {} optout UUIDs.", count);
            info!("Mappings {}.", map);
        }
        map
    };

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

                // FILTER FORBIDDEN HERE? Done in person/research?

                // Convert the ResearchJson to ResearchClean, keeping only the
                // relevant fields.
                match ResearchClean::try_from_with_locale_umap(entry, &cli.locale, &mut umap) {
                    Ok(research_des) => {
                        let json_output = serde_json::to_string_pretty(&research_des).unwrap();
                        trace!("\n{}", json_output);
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
    let persons_data: Option<Vec<PersonJson>> = cli.persons.as_ref().and_then(|persons_filename| {
        info!("Reading persons file {:?}.", persons_filename);
        match read_persons_jsonl(persons_filename, &umap) {
            Ok(data) => {
                info!("Person data contains {} elements.", data.len());
                match serde_json::to_string_pretty(&data) {
                    Ok(s) => trace!("\n{}", s),
                    Err(e) => eprintln!("Cannot pretty print persons JSON: {:?}", e),
                }
                Some(data)
            }
            Err(e) => {
                eprintln!("Failed to read PersonJson: {:?}", e);
                None
            }
        }
    });

    // Save a mapping from uuid to data, so we can combine later. PersonClean
    // is a simpler/cleaner version of PersonJson with only the fields we are
    // interested in.
    let person_map: HashMap<String, PersonClean> = persons_data
        .as_deref()
        .map(|data| {
            data.iter()
                .filter_map(|entry| {
                    let uuid = entry.get_uuid()?;
                    if let Some((first, last)) = entry.get_first_and_last_name() {
                        trace!("Name: {} {} {}", first, last, uuid);
                    } else {
                        error!("First or last name not found for {}", uuid);
                    }
                    trace!("{:?}", entry.get_all_education_pure_ids());

                    //let info_texts = entry.get_profile_information_texts_for_locale(&cli.locale);
                    let info_texts = extract_texts_with_formatting(
                        &entry.get_profile_information_texts_for_locale(&cli.locale),
                    );
                    trace!("{:?}", info_texts);

                    // Convert to PersonClean structures.
                    match PersonClean::try_from_with_locale_umap(entry, &cli.locale, &mut umap) {
                        Ok(person_des) => {
                            if let Ok(json_output) = serde_json::to_string(&person_des) {
                                trace!("{}", json_output);
                            }
                            Some((uuid.to_string(), person_des))
                        }
                        Err(e) => {
                            error!("Failed to convert PersonJson ({}): {:?}", uuid, e);
                            None
                        }
                    }
                })
                .collect()
        })
        .unwrap_or_else(|| {
            debug!("No persons data available.");
            HashMap::new()
        });

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
    let fingerprints_data: Option<Vec<FingerprintJson>> =
        cli.fingerprints.as_ref().and_then(|fingerprints_filename| {
            info!("Reading fingerprint file {:?}.", fingerprints_filename);
            match read_fingerprint_jsonl(fingerprints_filename) {
                Ok(data) => {
                    info!("Fingerprint data contains {} elements.", data.len());
                    match ::serde_json::to_string_pretty(&data) {
                        Ok(s) => trace!("\n{}", s),
                        Err(e) => eprintln!("Cannot pretty print fingerprint JSON: {:?}", e),
                    }
                    for fp in &data {
                        if let Some((id, uuids)) = fp.id_and_concepts() {
                            //_uuids() {
                            trace!("{id}: {:?}", uuids);
                        }
                    }
                    Some(data)
                }
                Err(e) => {
                    eprintln!("Error reading FingerprintJSON: {:?}", e);
                    None
                }
            }
        });

    // ------------------------------------------------------------------------

    // Parse the concepts JSON. Each struct is pushed into
    // a vector.
    let concepts_data: Option<Vec<ConceptJson>> =
        cli.concepts.as_ref().and_then(|concepts_filename| {
            info!("Reading concepts file {:?}.", concepts_filename);
            match read_concept_jsonl(concepts_filename) {
                Ok(data) => {
                    info!("Concepts data contains {} elements.", data.len());
                    match ::serde_json::to_string_pretty(&data) {
                        Ok(s) => trace!("\n{}", s),
                        Err(e) => eprintln!("Cannot pretty print concept JSON: {:?}", e),
                    }
                    for c in &data {
                        if let Some((id, text)) = c.id_and_text_for_locale(&cli.locale) {
                            trace!("{id}: {text}");
                        }
                    }
                    Some(data)
                }
                Err(e) => {
                    eprintln!("Error reading ConceptJSON: {:?}", e);
                    None
                }
            }
        });
    // ------------------------------------------------------------------------

    // Parse the orgunits JSON. Each struct is pushed into
    // a vector.
    let orgunits_data: Option<Vec<OrgUnitJson>> =
        cli.orgunits.as_ref().and_then(|orgunits_filename| {
            info!("Reading organisational-units file {:?}.", orgunits_filename);
            match read_orgunits_jsonl(orgunits_filename) {
                Ok(data) => {
                    info!("Orgunits data contains {} elements.", data.len());
                    match ::serde_json::to_string_pretty(&data) {
                        Ok(s) => trace!("\n{}", s),
                        Err(e) => eprintln!("Cannot pretty print orgunits JSON: {:?}", e),
                    }
                    Some(data)
                }
                Err(e) => {
                    eprintln!("Error reading OrgunitsJSON: {:?}", e);
                    None
                }
            }
        });

    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    // Filter before we output.
    // is this the right spot? Combined might contain a filtered list,
    // while the other datastrcuts are unfiltered. FIXME
    // ------------------------------------------------------------------------

    // We need the Keep/Delete option as well. TODO
    if let Some(names_filename) = cli.names {
        info!("Before names file {} items.", research_map.len());
        let names_list = read_names(&names_filename)?;
        filter_research_by_person(&mut research_map, names_list, cli.filtermode);
        info!("After names file {} items.", research_map.len());
    }

    // We need the Keep/Delete option as well. TODO
    // FIXME this looks in keywords, not in abstracts!
    if let Some(keywords_filename) = cli.keywords {
        info!("Before keywords files {} items.", research_map.len());
        let keywords_list = read_names(&keywords_filename)?;
        filter_research_by_keyword(&mut research_map, keywords_list, cli.filtermode);
        info!("After keywords file {} items.", research_map.len());
    }

    // Abstracts filter.
    if let Some(abstract_filename) = cli.abstractterms {
        info!("Before keywords files {} items.", research_map.len());
        let keywords_list = read_names(&abstract_filename)?;
        let _ = filter_research_by_abstract(&mut research_map, keywords_list, cli.filtermode);
        info!("After keywords file {} items.", research_map.len());
    }

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

    // Print the output to stdout. Simple format,
    // NAME: ... (multiple names)
    // TITLE: ...
    // KEYWORDS: ...
    // CREATED: ...
    // PUBLISHED: ...
    // ABSTRACT: ...
    if !cli.jsonl {
        // We really need a CSV output mode as well...
        /*
        for r in combined.research.values() {
            debug!("research clean uuid={:?}", r.get_uuid());
            trace!("{:?}", r);
            // If empty abstract skip?
            let names: Vec<_> = r
                .persons
                .iter()
                //.filter(|p| p.is_internal()) // Filter by the `inex` variable
                .map(|p| p.get_name())
                .collect();
            if names.is_empty() {
                eprintln!("No names! {}", r.get_title());
            } else {
                // TODO Check the type of research (journal, etc).
                /*
                println!("NAMES:{}", names.join(","));
                println!("TITLE:{}", r.get_title());
                println!("KEYWORDS:{}", r.get_keywords().join(","));
                println!("CREATED:{}", r.get_creation_date());
                println!("PUBLISHED:{}", r.get_publication_date());
                let s = r.get_abstract();
                /*
                s.split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" ");
                */
                //println!("ABSTRACT:{}", r.get_abstract());
                println!("ABSTRACT:{}", s);
                */
                print!("\"{}\"\t", names.join(","));
                print!("\"{}\"\t", r.get_title());
                print!("\"{}\"\t", r.get_keywords().join(","));
                print!("\"{}\"\t", r.get_publication_date());
                let s = r.get_abstract();
                println!("\"{}\"", escape(&s));
            }
        }
        */
    } else {
        let stdout = std::io::stdout();
        let mut output = stdout.lock();

        let mut research: Vec<&ResearchClean> = combined.research.values().collect();
        research.sort_by_key(|item| item.get_publication_date()); // Even get_keywords() works here.

        for item in research {
            serde_json::to_writer(&mut output, item)?;
            writeln!(output)?;
        }
    }

    if (cli.jsonl == false) && (combined.research.len() > 0) {
        let stdout = io::stdout();
        let mut output = stdout.lock();
        // UTF-8 BOM
        output.write_all(b"\xEF\xBB\xBF")?;
        let mut writer = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .from_writer(output);
        writer.write_record(["names", "title", "keywords", "pubdate", "abstract"])?;

        for r in combined.research.values() {
            let names: Vec<_> = r
                .persons
                .iter()
                //.filter(|p| p.is_internal()) // Filter by the `inex` variable
                .map(|p| p.get_name())
                .collect();
            let keywords = r.get_keywords().join(",");
            let abstract_text = r
                .get_abstract()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            if abstract_text.len() > 0 {
                // skip empty abstract
                let abstract_text = extract_text_with_formatting(&abstract_text);
                writer.write_record([
                    names.join(","),
                    r.get_title().to_string(),
                    keywords.to_string(),
                    r.get_publication_date().to_string(),
                    abstract_text,
                ])?;
            }
        }

        writer.flush()?;
    }

    // ------------------------------------------------------------------------

    /*
    let id = Uuid::new_v4();
    println!("{} {}", id, id.urn());
    */

    info!("Ending lucris-rs.");
    Ok(())
}
