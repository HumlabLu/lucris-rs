#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::fs::File as FSFile;
use std::io::BufReader;
use std::io::BufRead;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ResearchJson {
    #[serde(rename = "abstract")]
    pub abstract_field: Option<FormattedText>,
    pub additionalFiles: Option<Vec<AdditionalFile>>,
    pub additionalLinks: Option<Vec<AdditionalLink>>,
    pub articleNumber: Option<String>,
    pub awardedDate: Option<String>,
    pub awardingInstitutions: Option<Vec<AwardingInstitution>>,
    pub bibliographicalNote: Option<FormattedText>,
    pub category: Option<Category>,
    pub chapter: Option<String>,
    pub commissioningBody: Option<Organisation>,
    pub confidential: Option<bool>,
    pub country: Option<Country>,
    pub date: Option<String>,
    pub edition: Option<String>,
    pub electronicIsbns: Option<Vec<String>>,
    pub electronicVersions: Option<Vec<ElectronicVersion>>,
    pub embargoEndDate: Option<String>,
    pub event: Option<Event>,
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externalOrganisations: Option<Vec<Organisation>>,
    pub fieldWeightedCitationImpact: Option<f64>,
    pub hostPublicationEditors: Option<Vec<Name>>,
    pub hostPublicationSubTitle: Option<FormattedValue>,
    pub hostPublicationTitle: Option<FormattedValue>,
    pub info: Option<Info>,
    pub ipc: Option<String>,
    pub isbns: Option<Vec<String>>,
    pub journalAssociation: Option<JournalAssociation>,
    pub journalNumber: Option<String>,
    pub keywordGroups: Option<Vec<KeywordGroup>>,
    pub language: Option<Language>,
    pub managingOrganisationalUnit: Option<OrganisationalUnit>,
    pub numberOfPages: Option<u32>,
    pub number: Option<String>,
    pub openAccessPermission: Option<OpenAccessPermission>,
    pub organisationalUnits: Option<Vec<OrganisationalUnit>>,
    pub outputMedia: Option<OutputMedia>,
    pub pages: Option<String>,
    pub patentNumber: Option<String>,
    pub peerReview: Option<bool>,
    pub personAssociations: Option<Vec<PersonAssociation>>,
    pub placeOfPublication: Option<String>,
    pub priorityDate: Option<String>,
    pub priorityNumber: Option<String>,
    pub publicationSeries: Option<Vec<PublicationSeries>>,
    pub publicationStatuses: Option<Vec<PublicationStatus>>,
    pub publisher: Option<Organisation>,
    pub pureId: Option<u64>,
    pub qualification: Option<Qualification>,
    pub relatedActivities: Option<Vec<RelatedActivity>>,
    pub relatedEquipment: Option<Vec<RelatedEquipment>>,
    pub relatedPrizes: Option<Vec<RelatedPrize>>,
    pub relatedProjects: Option<Vec<RelatedProject>>,
    pub relatedResearchOutputs: Option<Vec<RelatedResearchOutput>>,
    pub scopusMetrics: Option<Vec<ScopusMetric>>,
    pub size: Option<String>,
    pub sponsors: Option<Vec<Organisation>>,
    pub submissionYear: Option<u32>,
    pub subTitle: Option<FormattedValue>,
    pub supervisorExternalOrganisations: Option<Vec<Organisation>>,
    pub supervisorOrganisationalUnits: Option<Vec<OrganisationalUnit>>,
    pub supervisors: Option<Vec<Supervisor>>,
    pub title: Option<FormattedValue>,
    pub totalNumberOfAuthors: Option<u32>,
    pub totalScopusCitations: Option<u32>,
    pub translatedHostPublicationSubTitle: Option<TranslatedText>,
    pub translatedHostPublicationTitle: Option<TranslatedText>,
    pub translatedSubTitle: Option<TranslatedText>,
    pub translatedTitle: Option<TranslatedText>,
    pub typeDescription: Option<FormattedText>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>, // Prolly doesnae have to be an Option.
    pub visibility: Option<Visibility>,
    pub volume: Option<String>,
    pub workflow: Option<Workflow>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FormattedText {
    pub formatted: bool,
    pub text: Vec<LocaleValue>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LocaleValue {
    pub locale: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[derive(Clone)]
pub struct FormattedValue {
    pub formatted: bool,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AdditionalFile {
    pub accessType: Option<AccessType>,
    pub created: Option<String>,
    pub creator: Option<String>,
    pub embargoEndDate: Option<String>,
    pub embargoStartDate: Option<String>,
    pub file: Option<File>,
    pub licenseType: Option<LicenseType>,
    pub pureId: Option<u64>,
    pub title: Option<String>,
    pub userDefinedLicense: Option<String>,
    pub visibleOnPortalDate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AccessType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Term {
    pub formatted: bool,
    pub text: Vec<LocaleValue>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct File {
    pub fileName: Option<String>,
    pub fileURL: Option<String>,
    pub mimeType: Option<String>,
    pub pureId: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LicenseType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AdditionalLink {
    pub description: Option<Term>,
    pub linkType: Option<LinkType>,
    pub pureId: Option<u64>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LinkType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AwardingInstitution {
    pub externalOrganisationalUnit: Option<OrganisationalUnit>,
    pub organisationalUnit: Option<OrganisationalUnit>,
    pub pureId: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OrganisationalUnit {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Link {
    pub href: Option<String>,
    #[serde(rename = "ref")]
    pub ref_field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TypeField {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Organisation {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Country {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ElectronicVersion {
    pub accessType: Option<AccessType>,
    pub created: Option<String>,
    pub creator: Option<String>,
    pub doi: Option<String>,
    pub embargoPeriod: Option<EmbargoPeriod>,
    pub file: Option<File>,
    pub licenseType: Option<LicenseType>,
    pub link: Option<String>,
    pub pureId: Option<u64>,
    pub title: Option<String>,
    pub userDefinedLicense: Option<String>,
    pub versionType: Option<VersionType>,
    pub visibleOnPortalDate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EmbargoPeriod {
    pub endDate: Option<String>,
    pub startDate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VersionType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Info {
    pub additionalExternalIds: Option<Vec<AdditionalExternalId>>,
    pub createdBy: Option<String>,
    pub createdDate: Option<String>,
    pub modifiedBy: Option<String>,
    pub modifiedDate: Option<String>,
    pub portalUrl: Option<String>,
    pub prettyURLIdentifiers: Option<Vec<String>>,
    pub previousUuids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AdditionalExternalId {
    pub idSource: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JournalAssociation {
    pub issn: Option<Issn>,
    pub journal: Option<Journal>,
    pub pureId: Option<u64>,
    pub title: Option<JournalTitle>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Issn {
    pub endDate: Option<String>,
    pub startDate: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Journal {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JournalTitle {
    pub endDate: Option<String>,
    pub startDate: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KeywordGroup {
    pub keywordContainers: Option<Vec<KeywordContainer>>,
    pub logicalName: Option<String>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KeywordContainer {
    pub freeKeywords: Option<Vec<FreeKeyword>>,
    pub pureId: Option<u64>,
    pub structuredKeyword: Option<StructuredKeyword>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FreeKeyword {
    pub freeKeywords: Option<Vec<String>>,
    pub locale: Option<String>,
    pub pureId: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StructuredKeyword {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Language {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OpenAccessPermission {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OutputMedia {
    String(String),
    Struct(OutputMediaStruct),
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OutputMediaStruct {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PersonAssociation {
    pub authorCollaboration: Option<AuthorCollaboration>,
    pub country: Option<Country>,
    pub externalOrganisations: Option<Vec<Organisation>>,
    pub externalPerson: Option<ExternalPerson>,
    pub isHidden: Option<bool>,
    pub name: Option<Name>,
    pub organisationalUnits: Option<Vec<OrganisationalUnit>>,
    pub person: Option<Person>,
    pub personRole: Option<PersonRole>,
    pub pureId: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthorCollaboration {
    pub link: Option<Link>,
    pub name: Option<Term>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExternalPerson {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Name {
    pub firstName: Option<String>,
    pub lastName: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Person {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PersonRole {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicationSeries {
    pub electronicIssn: Option<String>,
    pub issn: Option<String>,
    pub name: Option<FormattedValue>,
    pub no: Option<String>,
    pub publisherName: Option<String>,
    pub pureId: Option<u64>,
    pub volume: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicationStatus {
    pub current: Option<bool>,
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub publicationDate: Option<PublicationDate>,
    pub publicationStatus: Option<PublicationStatusField>,
    pub pureId: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicationDate {
    pub day: Option<u32>,
    pub month: Option<u32>,
    pub year: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicationStatusField {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Qualification {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RelatedActivity {
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RelatedEquipment {
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RelatedPrize {
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RelatedProject {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RelatedResearchOutput {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ScopusMetric {
    pub value: Option<f64>,
    pub year: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Supervisor {
    pub externalOrganisations: Option<Vec<Organisation>>,
    pub externalPerson: Option<ExternalPerson>,
    pub name: Option<Name>,
    pub organisationalUnits: Option<Vec<OrganisationalUnit>>,
    pub person: Option<Person>,
    pub personRole: Option<PersonRole>,
    pub pureId: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TranslatedText {
    pub formatted: bool,
    pub text: Vec<LocaleValue>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Visibility {
    pub key: Option<String>,
    pub value: Option<Term>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Workflow {
    pub value: Option<Term>,
    pub workflowStep: Option<String>,
}

// ----------------------------------------------------------------------------

impl ResearchJson {
    pub fn get_uuid(&self) -> Option<&str> {
        self.uuid.as_deref()
    }

    pub fn get_title_value(&self) -> Option<&str> {
        self.title.as_ref().map(|fv| fv.value.as_str())
    }

    pub fn get_abstract_text_for_locale(&self, locale: &str) -> Option<&str> {
        self.abstract_field.as_ref()?.text.iter()
            .find_map(|locale_value| {
                if locale_value.locale.as_deref() == Some(locale) {
                    locale_value.value.as_deref()
                } else {
                    None
                }
            })
    }
}

// ----------------------------------------------------------------------------

// Mostly for testing purposes, dumps the uuid, title and abstract
// from the research JSON.
pub fn dump_titles(research_data: &Vec<ResearchJson>) {
    for entry in research_data {
        if let Some(uuid) = entry.get_uuid() {
            println!("{}", uuid);
        } else {
            println!("No uuid");
        }
        if let Some(title) = entry.get_title_value() {
            println!("{}", title);
        } else {
            println!("No title");
        }        
        if let Some(abstract_field) = entry.get_abstract_text_for_locale("en_GB") {
            println!("{}\n", abstract_field);
        } else {
            println!("No abstract\n");
        }
    }
}

// ----------------------------------------------------------------------------

pub fn read_research_jsonl(file_path: &str) -> Result<Vec<ResearchJson>, Box<dyn std::error::Error>> {
    let file = FSFile::open(file_path)?;
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
                    //trace!("title={:?}", json.title.clone().unwrap().value);
                    trace!("uuid={:?}", json.uuid);
                    
                    // Add it to the data vector.
                    let mut data = data.lock().unwrap();
                    data.push(json);
                },
                Err(e) => {
                    error!("{}", e);
                    //error!("{}", line);

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
    info!("Extracted {} entries.", extracted_data.len());
    Ok(extracted_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::fs;
    
    fn read_test_data(file_name: &str) -> String {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let data_path = Path::new(project_root)
            .join("tests")
            .join("data")
            .join(file_name);
        fs::read_to_string(&data_path).expect("Unable to read test data file")
    }

    fn make_test_path(file_name: &str) -> PathBuf {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let data_path = Path::new(project_root)
            .join("tests")
            .join("data")
            .join(file_name);
        data_path
    }

    #[test]
    fn test_read_research_one() {
        let data_path = make_test_path("research_one.jsonl");
        println!("{:?}", data_path);
        let foo = read_research_jsonl(data_path.to_str().expect("Test data not found!"));
        let foo = foo.unwrap();
        let foo = &foo[0];
        assert_eq!(foo.get_uuid(), Some("1d136ffd-6d08-444a-9c50-76c0e5aec513"));
    }

    #[test]
    fn test_read_research_one_err() {
        let data_path = make_test_path("research_one_err.jsonl");
        println!("{:?}", data_path);
        let foo = read_research_jsonl(data_path.to_str().expect("Test data not found!"));
        let foo = foo.unwrap();
        assert_eq!(foo, []);            
    }
    
    #[test]
    pub fn test_research_uuid() {
        let data = r#"
{"pureId":2940508,"uuid":"1d136ffd-6d08-444a-9c50-76c0e5aec513"}
    "#;
        let research: ResearchJson = serde_json::from_str(data).expect("Err");
        assert!(research.uuid.as_deref() == Some("1d136ffd-6d08-444a-9c50-76c0e5aec513"));
    }
}
