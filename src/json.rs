use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

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
    externally_managed: bool,
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
pub struct OneJson {
    pure_id: u64,
    uuid: String,
    title: Title,
    peer_review: bool,
    managing_organisational_unit: OrganisationalUnit,
    confidential: bool,
    info: Info,
    total_scopus_citations: u32,
    pages: String,
    volume: String,
    journal_association: JournalAssociation,
    journal_number: String,
    publication_statuses: Vec<PublicationStatus>,
    #[serde(flatten)]
    other: Other,
}

// Function to read the JSON file
pub fn read_json(file_path: &str) -> Result<OneJson, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let data: OneJson = serde_json::from_reader(reader)?;
    Ok(data)
}
pub fn read_json_all(file_path: &str) -> Result<Vec<OneJson>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let data = vec![];
    reader
        .lines()        // split to lines serially
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()   // parallelize
        .filter_map(|line: String| serde_json::from_str(&line).ok()) // filter out bad lines
        .for_each(|v: OneJson| {
           // do some processing (in parallel)
            println!("title={:?}", v.title);
        });
    Ok(data)
}
