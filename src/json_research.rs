#![allow(non_snake_case)]
use crate::errors::CleanError;
use crate::uuid_map::UuidMap;
use log::{debug, error, info, trace, warn};
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::File as FSFile;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

/// JSON as it is read from the AIML cleaned data.
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

/// Simplified struct for output. Keep only relevant fields.
#[derive(Debug, Serialize, Clone)]
pub struct ResearchClean {
    uuid: String,
    title: String,
    #[serde(rename = "abstract")]
    abstract_text: String,
    pub persons: Vec<PersonRef>, // Or PersonClean?
}

/// Whether a researcher is internal (we have info in persons.jsonl) or external.
#[derive(Debug, Serialize, Clone, PartialEq)]
enum PersonType {
    Internal,
    External,
    Unknown,
}

/// Pointer to the data in persons.jsonl.
#[derive(Debug, Serialize, Clone)]
pub struct PersonRef {
    idx: u32,
    pub uuid: String, // Can be used to lookup in the person_map data.
    name: String,
    inex: PersonType, // Needs a better name...
}

impl fmt::Display for PersonRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.name)
    }
}

impl PersonRef {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_internal(&self) -> bool {
        self.inex == PersonType::Internal
    }
}

impl fmt::Display for ResearchClean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.title)?;
        let counts = self
            .persons // Count number of internal/external authors.
            .iter()
            .map(|p| match p.inex {
                PersonType::Internal => (1, 0, 0),
                PersonType::External => (0, 1, 0),
                PersonType::Unknown => (0, 0, 1),
            })
            .fold((0, 0, 0), |acc, (in_count, ex_count, uk_count)| {
                (acc.0 + in_count, acc.1 + ex_count, acc.2 + uk_count)
            });
        write!(f, " [{}/{}/{}]", counts.0, counts.1, counts.2)?;
        /*for p in &self.persons {
            write!(f, "/{}", p)?;
        }*/
        Ok(())
    }
}

// Getters
impl ResearchClean {
    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_abstract(&self) -> &str {
        &self.abstract_text
    }

    pub fn get_uuid(&self) -> &str {
        &self.uuid
    }
}

// This one takes a locale string and extracts the information for the specified locale.
impl ResearchClean {
    pub fn try_from_with_locale_umap(
        value: &ResearchJson,
        locale: &str,
        umap: &mut UuidMap,
    ) -> Result<Self, CleanError> {
        let uuid = value.uuid.as_ref().ok_or(CleanError::MissingUUID)?;

        let (abstract_title, abstract_text) = value.get_title_abstract(locale); // returns &str, &str
        let mut persons: Vec<PersonRef> = vec![];
        let person_names = value.get_internal_person_names(); // People responsible for the research.
        let mut c = 0;
        for (first_name, last_name, uuid) in person_names.iter() {
            if umap.optout_contains(uuid) {
                warn!("Opt-out internal person uuid in research!");
            } else {
                let safe_uuid = umap.get_uuid_as_str(uuid);
                // Often more than one.
                let person = PersonRef {
                    idx: c,
                    uuid: safe_uuid,
                    name: format!("{} {}", first_name, last_name),
                    inex: PersonType::Internal,
                };
                persons.push(person);
                c += 1;
            }
        }

        let external_person_names = value.get_external_person_names();
        for (full_name, uuid) in external_person_names.iter() {
            let safe_uuid = umap.get_uuid_as_str(uuid);
            if umap.optout_contains(uuid) {
                warn!("Opt-out external person uuid in research!");
            } else {
                let person = PersonRef {
                    idx: c,
                    uuid: safe_uuid,
                    name: full_name.to_string(),
                    inex: PersonType::External,
                };
                persons.push(person);
                c += 1;
            }
        }

        // Some journals (?) have a different persons sections, without
        // uuids. (They do have pure_ids however, but these are unused at the
        // moment). This extracts those names without uuids.
        // Also unpublished works?
        if persons.is_empty() {
            // HACK
            // FIX this will allow opt-out persons because we do not have uuid?
            warn!("Empty persons in {}.", uuid);
            for full_name in value.get_names_umap(umap) {
                trace!("full_name: {}", full_name);
                // We can generate a "fake" uuid, which will not be present
                // in the persons data. Not sure if good or bad...
                let safe_uuid = umap.get_uuid_as_str(&full_name); // Maybe add uuid.
                let person = PersonRef {
                    idx: c,
                    uuid: safe_uuid,
                    name: full_name,
                    inex: PersonType::Unknown,
                };
                persons.push(person);
                c += 1;
            }
        }

        // safe_uuid for the research structure itself.
        let safe_uuid = umap.get_uuid_as_str(uuid);

        // We have come this far, return the new struct.
        Ok(ResearchClean {
            uuid: safe_uuid,
            title: abstract_title.to_string(),
            abstract_text: abstract_text.to_string(),
            persons,
        })
    }
}

// End simplified.

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
        self.abstract_field
            .as_ref()?
            .text
            .iter()
            .find_map(|locale_value| {
                if locale_value.locale.as_deref() == Some(locale) {
                    locale_value.value.as_deref()
                } else {
                    None
                }
            })
    }

    // Get the first and last names, plus associated uuid, from the
    // personAssociations data.
    pub fn get_internal_person_names(&self) -> Vec<(&str, &str, &str)> {
        // umap?
        //todo!("Fix opt-out check");
        self.personAssociations
            .as_ref()
            .map(|associations| {
                associations
                    .iter()
                    .filter_map(|association| {
                        let first_name = association.name.as_ref()?.firstName.as_deref()?;
                        let last_name = association.name.as_ref()?.lastName.as_deref()?;
                        let uuid = association.person.as_ref()?.uuid.as_deref()?;
                        Some((first_name, last_name, uuid))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_internal_person_names_umap(&self, umap: &UuidMap) -> Vec<(&str, &str, &str)> {
        //todo!("Fix opt-out check");
        self.personAssociations
            .as_ref()
            .map(|associations| {
                associations
                    .iter()
                    .filter_map(|association| {
                        let first_name = association.name.as_ref()?.firstName.as_deref()?;
                        let last_name = association.name.as_ref()?.lastName.as_deref()?;
                        let uuid = association.person.as_ref()?.uuid.as_deref()?;
                        if umap.optout_contains(uuid) {
                            None
                        } else {
                            Some((first_name, last_name, uuid))
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    // Get the name(s) and UUID of the externalPersons from the personAssociations data.
    pub fn get_external_person_names(&self) -> Vec<(&str, &str)> {
        self.personAssociations
            .as_ref()
            .map(|associations| {
                associations
                    .iter()
                    .filter_map(|association| {
                        let external_person = association.externalPerson.as_ref()?;
                        let term = external_person.name.as_ref()?;
                        let name_value = term
                            .text
                            .iter()
                            .filter_map(|locale_value| locale_value.value.as_deref())
                            .next()?;
                        let uuid = external_person.uuid.as_deref()?;
                        Some((name_value, uuid))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    // These are present sometimes as contributors to journals WITHOUT uuids etc.
    pub fn get_names_umap(&self, umap: &UuidMap) -> Vec<String> {
        self.personAssociations
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|association| {
                let name = association.name.as_ref()?;
                trace!("{:?}", name);
                if let (Some(first), Some(last)) =
                    (name.firstName.as_deref(), name.lastName.as_deref())
                {
                    let uuid = association
                        .person
                        .as_ref()
                        .and_then(|p| p.uuid.as_deref())
                        .or_else(|| association.externalPerson.as_ref()?.uuid.as_deref())
                        .unwrap_or("");
                    trace!("Uuid {{ {} }}", uuid); // {{ is an escaped {...
                    if umap.optout_contains(uuid) {
                        warn!("Opt-out person in data!");
                        None
                    } else {
                        Some(format!("{} {}", first, last))
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_names(&self) -> Vec<String> {
        self.personAssociations
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|association| association.name.as_ref())
            .filter_map(|name| {
                trace!("{:?}", name);
                if let (Some(first), Some(last)) =
                    (name.firstName.as_deref(), name.lastName.as_deref())
                {
                    Some(format!("{} {}", first, last))
                } else {
                    None
                }
            })
            .collect()
    }

    // We return an empty string if the info is not present. Could change to
    // Option<T> but seems overkill at the moment.
    pub fn get_title_abstract(&self, locale: &str) -> (&str, &str) {
        let title_text = self.get_title_value().unwrap_or("");
        // IT seems that some research contains the abstract string "[abstract missing]".
        let abstract_text = self.get_abstract_text_for_locale(locale).unwrap_or("");
        //let names = self.get_person_names();
        //let names = [names[0].0, " ", names[0].1].concat();
        (title_text, abstract_text)
    }
}

// ----------------------------------------------------------------------------

// Mostly for testing purposes, dumps the uuid, title and abstract
// from the research JSON.
pub fn _dump_titles(research_data: &Vec<ResearchJson>, locale: &str) {
    let mut abstract_counter = 0;
    let mut counter = 0;
    for entry in research_data {
        counter += 1;
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
        let person_names = entry.get_internal_person_names();
        // TODO check opt-out
        for (i, (first_name, last_name, uuid)) in person_names.iter().enumerate() {
            println!("Person {}: {} {} {}", i, first_name, last_name, uuid);
        }
        if let Some(abstract_field) = entry.get_abstract_text_for_locale(locale) {
            println!("{}\n", abstract_field);
            abstract_counter += 1;
        } else {
            println!("No abstract\n");
        }
    }
    println!(
        "counter, abstract_counter: {}, {} (missing {})",
        counter,
        abstract_counter,
        counter - abstract_counter
    );
}

// ----------------------------------------------------------------------------

pub fn read_research_jsonl(
    file_path: &str,
    umap: &UuidMap,
) -> Result<(Vec<ResearchJson>, HashMap<String, Vec<String>>), Box<dyn std::error::Error>> {
    let file = FSFile::open(file_path)?;
    let reader = BufReader::new(file);
    let data = Arc::new(Mutex::new(vec![]));
    let failed_count = Arc::new(Mutex::new(0));
    let person_research: Arc<Mutex<HashMap<String, Vec<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    reader
        .lines()
        .map_while(Result::ok)
        .par_bridge() // parallelise
        .for_each(|line: String| {
            match serde_json::from_str::<ResearchJson>(&line) {
                Ok(json) => {
                    //trace!("title={:?}", json.title.clone().unwrap().value);
                    debug!("research uuid={:?}", json.uuid);
                    trace!("{:?}", json); // This generates a lot of output...

                    // TODO check for optout uuid here? Ignore if it is?
                    // We might want to do this more dynamically later...

                    // Check persons for research reverse index?
                    let mut map = person_research.lock().unwrap();
                    let uuid = json.uuid.clone().unwrap();
                    let persons = json.get_internal_person_names();
                    trace!("{:?}", persons);
                    for (_first_name, _last_name, person_uuid) in persons {
                        if umap.optout_contains(person_uuid) {
                            warn!("Opt-out person uuid in research data!");
                        } else {
                            map.entry(person_uuid.to_string())
                                .or_default()
                                .push(uuid.clone());
                        }
                    }

                    // Also other persons? These are present sometimes as "contributed to journal"
                    // without uuids and other info.
                    // Save these in a "backup_names" field?
                    //let ex_persons = json.get_names();
                    //println!("{:?}", ex_persons);

                    // Add it to the data vector.
                    let mut data = data.lock().unwrap();
                    data.push(json);
                }
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

    // person_research is a hash which looks like:
    // {"862b1711-47e3-45ed-9330-a2071033c219": ["dd0ce568-96e7-449b-9a59-9ee857f79a13"],
    //  "61781b1a-c069-4971-bb76-b18ed231a453": ["dd0ce568-96e7-449b-9a59-9ee857f79a13"]}
    // Containing a person_id followed by [research_ids, ...]
    let extracted_pr = Arc::try_unwrap(person_research)
        .expect("Multiple references to person_research")
        .into_inner()
        .expect("Mutex was poisoned");
    trace!("extracted_pr: {:?}", extracted_pr);

    // Extract the data from Arc<Mutex<...>> and return it.
    let extracted_data = Arc::try_unwrap(data).unwrap().into_inner().unwrap();
    info!("Extracted {} entries.", extracted_data.len());
    Ok((extracted_data, extracted_pr))
}

// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

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
        let (foo, _bar) = foo.unwrap();
        let foo = &foo[0];
        assert_eq!(foo.get_uuid(), Some("1d136ffd-6d08-444a-9c50-76c0e5aec513"));
    }

    #[test]
    fn test_read_research_one_err() {
        let data_path = make_test_path("research_one_err.jsonl");
        println!("{:?}", data_path);
        let (foo, _bar) = read_research_jsonl(data_path.to_str().expect("Test data not found!"))
            .expect("Failed to read research JSONL data");
        assert_eq!(foo, []);
    }

    // Tests the output of ResearchClean, and as a side-effect
    // tests a file without "normal" internal/external persons.:w
    #[test]
    pub fn test_unknown_persons() {
        let data_path = make_test_path("journal.jsonl");
        println!("{:?}", data_path);
        let (foo, _bar) = read_research_jsonl(data_path.to_str().expect("Test data not found!"))
            .expect("Failed to read research JSONL data");
        let mut umap = UuidMap::new();
        let research_des: ResearchClean =
            ResearchClean::try_from_with_locale_umap(&foo[0], "en_GB", &mut umap).expect("Err");
        let output = format!("{}", research_des);
        assert_eq!(
            output,
            "Biodegradation of nonylphenol in a continuous packed-bed bioreactor. [0/0/0]"
        );
    }

    #[test]
    pub fn test_research_uuid() {
        let data = r#"
{"pureId":2940508,"uuid":"1d136ffd-6d08-444a-9c50-76c0e5aec513"}
    "#;
        let research: ResearchJson = serde_json::from_str(data).expect("Err");
        assert!(research.uuid.as_deref() == Some("1d136ffd-6d08-444a-9c50-76c0e5aec513"));
    }

    #[test]
    fn test_research_des_umap_ok() {
        let data = r#"
        {
          "uuid": "01234567-0123-0123-0123-0123456789AB",
          "title": {
            "formatted": true,
            "value": "A nice title."
          },
          "name": {
            "firstName": "Quinten",
            "lastName": "Berck"
          }
        }
        "#;
        let mut umap = UuidMap::new();
        // Create and save the safe_uuid so we can compare it later.
        let safe_uuid = umap.add_uuid("01234567-0123-0123-0123-0123456789AB");
        let answer = format!(
            r#"{{"uuid":"{}","title":"A nice title.","abstract":"","persons":[]}}"#,
            safe_uuid
        );
        let research: ResearchJson = serde_json::from_str(data).expect("Err");
        let research_des: ResearchClean =
            ResearchClean::try_from_with_locale_umap(&research, "en_GB", &mut umap).expect("Err");
        let research_des_jstr = serde_json::to_string(&research_des).unwrap();
        println!("{}", research_des_jstr);
        assert_eq!(research_des_jstr, answer);
    }
}
