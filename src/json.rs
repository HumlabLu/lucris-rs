use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};

// Catch-all type for undefined fields in the structures.
// These are caught by "#[serde(flatten)]".
type Other = serde_json::Map<String, serde_json::Value>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    formatted: Option<bool>,
    value: String,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NameText {
    locale: Option<String>,
    value: String,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    formatted: bool,
    text: Vec<NameText>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrganisationalUnit {
    uuid: String,
    externally_managed: Option<bool>,
    name: Name,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Journal {
    uuid: String,
    name: Name,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JournalAssociation {
    pure_id: u64,
    title: Title,
    journal: Journal,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    created_date: String,
    modified_date: String,
    portal_url: String,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicationStatus {
    pure_id: u64,
    current: bool,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResearchJson {
    pure_id: u64,
    pub uuid: String,
    title: Title,
    peer_review: Option<bool>,
    managing_organisational_unit: Option<OrganisationalUnit>,
    confidential: bool,
    info: Info,
    total_scopus_citations: Option<u32>,
    pages: Option<String>,
    volume: Option<String>,
    journal_association: Option<JournalAssociation>,
    journal_number: Option<String>,
    publication_statuses: Vec<PublicationStatus>,
    #[serde(flatten)]
    other: Other,
}

// Test function.
pub fn _read_json(file_path: &str) -> Result<ResearchJson, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let data: ResearchJson = serde_json::from_reader(reader)?;
    Ok(data)
}

pub fn read_jsonl(file_path: &str) -> Result<Vec<ResearchJson>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let data = Arc::new(Mutex::new(vec![]));
    let failed_count = Arc::new(Mutex::new(0));
    
    reader
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()   // parallelise
        // expect to check if it works, for prod use ok().
        //.filter_map(|line: String| serde_json::from_str(&line).expect("Err")) // filter out bad lines
        //.filter_map(|line: String| serde_json::from_str(&line).ok()) // filter out bad lines
        .for_each(|line: String| {
            match serde_json::from_str::<ResearchJson>(&line) {
                Ok(json) => {
                    trace!("title={:?}", json.title.value);
                    // Add it to the data vector.
                    let mut data = data.lock().unwrap();
                    data.push(json);
                },
                Err(e) => {
                    error!("{}", e);

                    // Increment the failure counter.
                    let mut failed = failed_count.lock().unwrap();
                    *failed += 1;
                }
            }
        });

    if *failed_count.lock().unwrap() > 0 {
        warn!("Failed to parse {} lines.", *failed_count.lock().unwrap());
    }
    
    // Extract the data from Arc<Mutex<...>> and return it.
    let extracted_data = Arc::try_unwrap(data).unwrap().into_inner().unwrap();
    info!("Parsed {} lines.", extracted_data.len());
    Ok(extracted_data)
}
