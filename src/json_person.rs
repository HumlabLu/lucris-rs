use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

// Catch-all type for undefined fields in the structures.
// These are caught by "#[serde(flatten)]".
type Other = serde_json::Map<String, serde_json::Value>;

// Define the structs based on the JSON structure
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    firstName: Option<String>,
    lastName: Option<String>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    createdDate: String,
    modifiedDate: String,
    portalUrl: String,
    prettyURLIdentifiers: Vec<String>,
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
    pureId: u64,
    externallyManaged: bool,
    value: TitleValue,
    title_type: Option<LocaleTexts>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInformation {
    pureId: u64,
    value: LocaleTexts,
    title_type: Option<LocaleTexts>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StaffAssociation {
    pureId: u64,
    externallyManaged: bool,
    person: Person,
    period: Period,
    isPrimaryAssociation: bool,
    organisationalUnit: OrganisationalUnit,
    staffType: StaffType,
    jobDescription: LocaleTexts,
    keywordGroups: Vec<KeywordGroup>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    uuid: String,
    link: Link,
    externallyManaged: bool,
    name: Name,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    startDate: String,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrganisationalUnit {
    uuid: String,
    link: Link,
    externallyManaged: bool,
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
    pureId: u64,
    uri: String,
    term: LocaleTexts,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeywordGroup {
    pureId: u64,
    externallyManaged: bool,
    logicalName: String,
    keyword_type: Option<LocaleTexts>,
    keywordContainers: Vec<KeywordContainer>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeywordContainer {
    pureId: u64,
    structuredKeyword: StructuredKeyword,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StructuredKeyword {
    pureId: u64,
    uri: String,
    term: LocaleTexts,
    #[serde(flatten)]
    other: Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OnePerson {
    pureId: u64,
    externallyManaged: bool,
    uuid: String,
    name: Name,
    fte: f32,
    info: Info,
    visibility: Visibility,
    titles: Vec<Title>,
    profileInformations: Vec<ProfileInformation>,
    staffOrganisationAssociations: Vec<StaffAssociation>,
    #[serde(flatten)]
    other: Other,
}

// Function to read the JSON file into OnePerson structure
pub fn read_json(file_path: &str) -> Result<OnePerson, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let person_data: OnePerson = serde_json::from_reader(reader)?;
    Ok(person_data)
}

