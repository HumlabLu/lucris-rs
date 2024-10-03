#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::fs::File as FSFile;
use std::io::BufReader;
use std::io::BufRead;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedText {
    pub formatted: bool,
    pub text: Vec<LocaleValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocaleValue {
    pub locale: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct FormattedValue {
    pub formatted: bool,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Term {
    pub formatted: bool,
    pub text: Vec<LocaleValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub fileName: Option<String>,
    pub fileURL: Option<String>,
    pub mimeType: Option<String>,
    pub pureId: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalLink {
    pub description: Option<Term>,
    pub linkType: Option<LinkType>,
    pub pureId: Option<u64>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AwardingInstitution {
    pub externalOrganisationalUnit: Option<OrganisationalUnit>,
    pub organisationalUnit: Option<OrganisationalUnit>,
    pub pureId: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub href: Option<String>,
    #[serde(rename = "ref")]
    pub ref_field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeField {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbargoPeriod {
    pub endDate: Option<String>,
    pub startDate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalExternalId {
    pub idSource: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalAssociation {
    pub issn: Option<Issn>,
    pub journal: Option<Journal>,
    pub pureId: Option<u64>,
    pub title: Option<JournalTitle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issn {
    pub endDate: Option<String>,
    pub startDate: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Journal {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalTitle {
    pub endDate: Option<String>,
    pub startDate: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeywordGroup {
    pub keywordContainers: Option<Vec<KeywordContainer>>,
    pub logicalName: Option<String>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeywordContainer {
    pub freeKeywords: Option<Vec<FreeKeyword>>,
    pub pureId: Option<u64>,
    pub structuredKeyword: Option<StructuredKeyword>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FreeKeyword {
    pub freeKeywords: Option<Vec<String>>,
    pub locale: Option<String>,
    pub pureId: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructuredKeyword {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAccessPermission {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutputMedia {
    String(String),
    Struct(OutputMediaStruct),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OutputMediaStruct {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorCollaboration {
    pub link: Option<Link>,
    pub name: Option<Term>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPerson {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub firstName: Option<String>,
    pub lastName: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonRole {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicationSeries {
    pub electronicIssn: Option<String>,
    pub issn: Option<String>,
    pub name: Option<FormattedValue>,
    pub no: Option<String>,
    pub publisherName: Option<String>,
    pub pureId: Option<u64>,
    pub volume: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicationStatus {
    pub current: Option<bool>,
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub publicationDate: Option<PublicationDate>,
    pub publicationStatus: Option<PublicationStatusField>,
    pub pureId: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicationDate {
    pub day: Option<u32>,
    pub month: Option<u32>,
    pub year: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicationStatusField {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Qualification {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedActivity {
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedEquipment {
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedPrize {
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedProject {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedResearchOutput {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub link: Option<Link>,
    pub name: Option<Term>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopusMetric {
    pub value: Option<f64>,
    pub year: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Supervisor {
    pub externalOrganisations: Option<Vec<Organisation>>,
    pub externalPerson: Option<ExternalPerson>,
    pub name: Option<Name>,
    pub organisationalUnits: Option<Vec<OrganisationalUnit>>,
    pub person: Option<Person>,
    pub personRole: Option<PersonRole>,
    pub pureId: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslatedText {
    pub formatted: bool,
    pub text: Vec<LocaleValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Visibility {
    pub key: Option<String>,
    pub value: Option<Term>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub value: Option<Term>,
    pub workflowStep: Option<String>,
}

// ----

impl ResearchJson {
    pub fn get_uuid(&self) -> Option<&str> {
        self.uuid.as_deref()
    }
}

// ----

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
    
    #[test]

    pub fn test_full() {
        let data = r#"
{"pureId":2940508,"uuid":"1d136ffd-6d08-444a-9c50-76c0e5aec513","title":{"formatted":true,"value":"A microcalorimetric method of studying mould activity as a function of water activity"},"peerReview":true,"managingOrganisationalUnit":{"uuid":"282ec847-a11d-47ca-9824-8c0e736743ff","link":{"ref":"content","href":"https://lucris.lub.lu.se/ws/api/524/organisational-units/282ec847-a11d-47ca-9824-8c0e736743ff"},"externallyManaged":true,"name":{"formatted":false,"text":[{"locale":"en_GB","value":"Division of Building Materials"},{"locale":"sv_SE","value":"Avdelningen för Byggnadsmaterial"}]},"type":{"pureId":6031,"uri":"/dk/atira/pure/organisation/organisationtypes/organisation/division","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Division"},{"locale":"sv_SE","value":"avdelning"}]}}},"confidential":false,"info":{"createdDate":"2016-04-01T12:28:51.367+0200","modifiedDate":"2023-03-18T19:37:15.211+0100","portalUrl":"https://portal.research.lu.se/en/publications/1d136ffd-6d08-444a-9c50-76c0e5aec513","prettyURLIdentifiers":["a-microcalorimetric-method-of-studying-mould-activity-as-a-functi"],"additionalExternalIds":[{"value":"0031787431","idSource":"Scopus"}]},"totalScopusCitations":8,"fieldWeightedCitationImpact":0,"pages":"25-28","volume":"42","journalAssociation":{"pureId":2940509,"title":{"value":"International Biodeterioration & Biodegradation"},"issn":{"value":"1879-0208"},"journal":{"uuid":"f9dc196e-e9aa-4513-830b-30ffbb4d6cad","link":{"ref":"content","href":"https://lucris.lub.lu.se/ws/api/524/journals/f9dc196e-e9aa-4513-830b-30ffbb4d6cad"},"name":{"formatted":false,"text":[{"value":"International Biodeterioration and Biodegradation"}]},"type":{"pureId":989,"uri":"/dk/atira/pure/journal/journaltypes/journal/journal","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Journal"},{"locale":"sv_SE","value":"Tidskrift"}]}}}},"journalNumber":"1","type":{"pureId":4488,"uri":"/dk/atira/pure/researchoutput/researchoutputtypes/contributiontojournal/article","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Article"},{"locale":"sv_SE","value":"Artikel i vetenskaplig tidskrift"}]}},"category":{"pureId":4454,"uri":"/dk/atira/pure/researchoutput/category/research","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Research"},{"locale":"sv_SE","value":"Forskning"}]}},"language":{"pureId":4694,"uri":"/dk/atira/pure/core/languages/en_GB","term":{"formatted":false,"text":[{"locale":"en_GB","value":"English"},{"locale":"sv_SE","value":"engelska"}]}},"abstract":{"formatted":true,"text":[{"locale":"en_GB","value":"This paper presents a new method of studying mould activity as a function of water activity by measuring the heat produced by the fungal metabolism. During a measurement a small sample (&lt;1 g) of a moulded substrate is moved between a humidity generator, where it is conditioned to a certain water activity, and a microcalorimeter, where the heat production rate is measured. This is repeated for different water activities. A conditioning and the subsequent thermal measurement takes approximately one day for each water activity. Results are presented from a measurement with Penicillium mould growing on a bread substrate. The results correlate well with literature data indicating that the present method is a rapid way of assessing mould activity as a function of water activity."}]},"totalNumberOfAuthors":2,"openAccessPermission":{"pureId":2924,"uri":"/dk/atira/pure/researchoutput/openaccesspermission/closed","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Closed"},{"locale":"sv_SE","value":"Closed"}]}},"visibility":{"key":"FREE","value":{"formatted":false,"text":[{"locale":"en_GB","value":"Public - No restriction"},{"locale":"sv_SE","value":"Allmänt tillgänglig - Inga restriktioner"}]}},"workflow":{"workflowStep":"approved","value":{"formatted":false,"text":[{"locale":"en_GB","value":"Validated"},{"locale":"sv_SE","value":"Granskad"}]}},"publicationStatuses":[{"pureId":2940508,"current":true,"publicationDate":{"year":1998},"publicationStatus":{"pureId":869,"uri":"/dk/atira/pure/researchoutput/status/published","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Published"},{"locale":"sv_SE","value":"Published"}]}}}],"personAssociations":[{"pureId":2940514,"externalPerson":{"uuid":"f3551f5f-bf49-46df-ae6b-c14e79a65d58","link":{"ref":"content","href":"https://lucris.lub.lu.se/ws/api/524/external-persons/f3551f5f-bf49-46df-ae6b-c14e79a65d58"},"name":{"formatted":false,"text":[{"value":"N Markova"}]},"type":{"pureId":5491,"uri":"/dk/atira/pure/externalperson/externalpersontypes/externalperson/externalperson","term":{"formatted":false,"text":[{"locale":"en_GB","value":"External person"},{"locale":"sv_SE","value":"Extern person"}]}}},"name":{"firstName":"N","lastName":"Markova"},"personRole":{"pureId":5570,"uri":"/dk/atira/pure/researchoutput/roles/contributiontojournal/author","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Author"},{"locale":"sv_SE","value":"författare"}]}}},{"pureId":2940515,"person":{"uuid":"4967eb17-7f37-47a0-a8f8-ec7b22af1567","link":{"ref":"content","href":"https://lucris.lub.lu.se/ws/api/524/persons/4967eb17-7f37-47a0-a8f8-ec7b22af1567"},"externallyManaged":true,"name":{"formatted":false,"text":[{"value":"Lars Wadsö"}]}},"name":{"firstName":"Lars","lastName":"Wadsö"},"personRole":{"pureId":5570,"uri":"/dk/atira/pure/researchoutput/roles/contributiontojournal/author","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Author"},{"locale":"sv_SE","value":"författare"}]}},"organisationalUnits":[{"uuid":"282ec847-a11d-47ca-9824-8c0e736743ff","link":{"ref":"content","href":"https://lucris.lub.lu.se/ws/api/524/organisational-units/282ec847-a11d-47ca-9824-8c0e736743ff"},"externallyManaged":true,"name":{"formatted":false,"text":[{"locale":"en_GB","value":"Division of Building Materials"},{"locale":"sv_SE","value":"Avdelningen för Byggnadsmaterial"}]},"type":{"pureId":6031,"uri":"/dk/atira/pure/organisation/organisationtypes/organisation/division","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Division"},{"locale":"sv_SE","value":"avdelning"}]}}}]}],"organisationalUnits":[{"uuid":"282ec847-a11d-47ca-9824-8c0e736743ff","link":{"ref":"content","href":"https://lucris.lub.lu.se/ws/api/524/organisational-units/282ec847-a11d-47ca-9824-8c0e736743ff"},"externallyManaged":true,"name":{"formatted":false,"text":[{"locale":"en_GB","value":"Division of Building Materials"},{"locale":"sv_SE","value":"Avdelningen för Byggnadsmaterial"}]},"type":{"pureId":6031,"uri":"/dk/atira/pure/organisation/organisationtypes/organisation/division","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Division"},{"locale":"sv_SE","value":"avdelning"}]}}}],"electronicVersions":[{"pureId":2940510,"doi":"https://doi.org/10.1016/S0964-8305(98)00042-0","accessType":{"pureId":111,"uri":"/dk/atira/pure/core/openaccesspermission/closed","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Closed"},{"locale":"sv_SE","value":"Sluten"}]}},"versionType":{"pureId":14550,"uri":"/dk/atira/pure/researchoutput/electronicversion/versiontype/publishersversion","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Final published version"},{"locale":"sv_SE","value":"Publicerad version"}]}}}],"keywordGroups":[{"pureId":2940511,"logicalName":"uka_full","type":{"uri":"uka_full","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Subject classification (UKÄ)"},{"locale":"sv_SE","value":"Ämnesklassifikation (UKÄ)"}]}},"keywordContainers":[{"pureId":2940512,"structuredKeyword":{"pureId":17764,"uri":"uka_full/2/205","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Materials Engineering"},{"locale":"sv_SE","value":"Materialteknik"}]}}}]}]}
    "#;
        let research: ResearchJson = serde_json::from_str(data).expect("Err");
        assert!(research.uuid.as_deref() == Some("1d136ffd-6d08-444a-9c50-76c0e5aec513"));
    }
}
