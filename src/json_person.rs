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

// Define the structs based on the JSON structure
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    first_name: Option<String>,
    last_name: Option<String>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    created_date: String,
    modified_date: String,
    portal_url: String,
    // Alias because the snake_case version is not recognised...
    #[serde(alias = "prettyURLIdentifiers")]
    pretty_url_identifiers: Option<Vec<String>>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocaleText {
    locale: String,
    value: String,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Visibility {
    key: String,
    value: LocaleTexts,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocaleTexts {
    formatted: bool,
    text: Vec<LocaleText>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TitleValue {
    formatted: bool,
    text: Vec<LocaleText>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pure_id: u64,
    externally_managed: Option<bool>,
    value: TitleValue,
    title_type: Option<LocaleTexts>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInformation {
    pure_id: u64,
    value: LocaleTexts,
    title_type: Option<LocaleTexts>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StaffAssociation {
    pure_id: u64,
    externally_managed: Option<bool>,
    person: Person,
    period: Period,
    is_primary_association: bool,
    organisational_unit: OrganisationalUnit,
    staff_type: StaffType,
    job_description: Option<LocaleTexts>,
    keyword_groups: Option<Vec<KeywordGroup>>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    uuid: String,
    link: Link,
    externally_managed: Option<bool>,
    name: Name,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    start_date: String,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrganisationalUnit {
    uuid: String,
    link: Link,
    externally_managed: Option<bool>,
    name: LocaleTexts,
    unit_type: Option<LocaleTexts>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    ref_: String,
    href: String,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StaffType {
    pure_id: u64,
    uri: String,
    term: LocaleTexts,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeywordGroup {
    pure_id: u64,
    externally_managed: Option<bool>,
    logical_name: String,
    keyword_type: Option<LocaleTexts>,
    keyword_containers: Vec<KeywordContainer>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeywordContainer {
    pure_id: u64,
    structured_keyword: StructuredKeyword,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StructuredKeyword {
    pure_id: u64,
    uri: String,
    term: LocaleTexts,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PersonJson {
    pure_id: u64,
    externally_managed: Option<bool>,
    uuid: String,
    pub name: Name,
    fte: f32,
    pub info: Info,
    visibility: Visibility,
    titles: Option<Vec<Title>>,
    profile_informations: Option<Vec<ProfileInformation>>,
    staff_organisation_associations: Option<Vec<StaffAssociation>>,
    #[serde(flatten)]
    other: Other,
}

pub fn read_persons_jsonl(file_path: &str) -> Result<Vec<PersonJson>, Box<dyn std::error::Error>> {
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
            match serde_json::from_str::<PersonJson>(&line) {
                Ok(json) => {
                    trace!("uuid={:?}", json.uuid);

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
