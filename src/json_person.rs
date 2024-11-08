#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};
use log::{debug, error, info, trace, warn};
use std::convert::TryFrom;
use crate::errors::{CleanError};
use std::fmt;
use crate::uuid_map::{UuidMap};

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonJson {
    pub educations: Option<Vec<Education>>,
    pub employeeEndDate: Option<String>,
    pub employeeStartDate: Option<String>,
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub externalPositions: Option<Vec<ExternalPosition>>,
    pub fte: Option<f64>,
    pub honoraryStaffOrganisationAssociations: Option<Vec<OrganisationAssociation>>,
    pub ids: Option<Vec<Id>>,
    pub info: Option<Info>,
    pub keywordGroups: Option<Vec<KeywordGroup>>,
    pub links: Option<Vec<LinkItem>>,
    pub name: Option<Name>,
    pub nameVariants: Option<Vec<NameVariant>>,
    pub orcid: Option<String>,
    pub professionalQualifications: Option<Vec<ProfessionalQualification>>,
    pub profileInformations: Option<Vec<ProfileInformation>>,
    pub profilePhotos: Option<Vec<ProfilePhoto>>,
    pub pureId: Option<u64>,
    pub staffOrganisationAssociations: Option<Vec<StaffOrganisationAssociation>>,
    pub startDateAsResearcher: Option<String>,
    pub supervisedByRelations: Option<Vec<SupervisedByRelation>>,
    pub supervisorForRelations: Option<Vec<SupervisorForRelation>>,
    pub titles: Option<Vec<Title>>,
    pub uuid: Option<String>,
    pub visibility: Option<Visibility>,
    pub visitingScholarOrganisationAssociations: Option<Vec<OrganisationAssociation>>,
}

// Simplified struct for output. Keep only relevant fields.
#[derive(Debug, Serialize)]
pub struct PersonClean {
    uuid: String,
    name: String,
    profile_info: String,
}

// We may need a mapping from (safe-)uuid to PersonClean?

impl fmt::Display for PersonClean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.name)
    }
}

impl TryFrom<&PersonJson> for PersonClean {
    type Error = CleanError;

    fn try_from(value: &PersonJson) -> Result<Self, Self::Error> {
        let uuid = value.uuid.as_ref().ok_or(CleanError::MissingUUID)?;

        // Extract name field as a reference.
        let name_struct = value.name.as_ref().ok_or(CleanError::MissingNameField)?;

        // Extract 'firstName' and 'lastName' as references, combine.
        let first_name = name_struct.firstName.as_ref().ok_or(CleanError::MissingFirstName)?;
        let last_name = name_struct.lastName.as_ref().ok_or(CleanError::MissingLastName)?;
        let full_name = format!("{} {}", first_name, last_name);

        // Create the PersonClean.
        Ok(PersonClean {
            uuid: uuid.to_string(),
            name: full_name,
            profile_info: "".to_string(), // Take a default en_GB locale?
        })
    }
}

// Another try_from, but this one takes a locale string and extracts the
// profile_information for the specified locale.
//
// Can we map the uuid already here? Probably not, there could be unknown
// uuids at this point?
impl PersonClean {
    pub fn try_from_with_locale(value: &PersonJson, locale: &str) -> Result<Self, CleanError> {
        let uuid = value.uuid.as_ref().ok_or(CleanError::MissingUUID)?;

        let name_struct = value.name.as_ref().ok_or(CleanError::MissingNameField)?;
        let first_name = name_struct.firstName.as_ref().ok_or(CleanError::MissingFirstName)?;
        let last_name = name_struct.lastName.as_ref().ok_or(CleanError::MissingLastName)?;
        let full_name = format!("{} {}", first_name, last_name);

        // Extract profile informations using locale? The function returns a vec,
        // which can be empty ([]).
        let profile_info_text = value.get_profile_information_texts_for_locale(locale);
        let profile_info_text = profile_info_text
            .first() // First element of the vector (it should only contain one?).
            .copied() // Dereferences &&str to &str.
            .unwrap_or("There is no profile_information.");

        let titles = value.get_title_for_locale(locale).unwrap();
        println!("TITLES {:?}", titles);

        let keywords = value.get_keywords_for_locale(locale);
        println!("KEYWORDS {:?}", keywords);

        // We have come this far, return the new struct.
        Ok(PersonClean {
            uuid: uuid.to_string(),
            name: full_name,
            profile_info: profile_info_text.to_string(),
        })
    }

    pub fn try_from_with_locale_umap(value: &PersonJson, locale: &str, umap: &mut UuidMap) -> Result<Self, CleanError> {
        let uuid = value.uuid.as_ref().ok_or(CleanError::MissingUUID)?;
        let safe_uuid = umap.get_uuid_as_str(&uuid);

        let name_struct = value.name.as_ref().ok_or(CleanError::MissingNameField)?;
        let first_name = name_struct.firstName.as_ref().ok_or(CleanError::MissingFirstName)?;
        let last_name = name_struct.lastName.as_ref().ok_or(CleanError::MissingLastName)?;
        let full_name = format!("{} {}", first_name, last_name);

        // Extract profile informations using locale? The function returns a vec,
        // which can be empty ([]).
        let profile_info_text = value.get_profile_information_texts_for_locale(locale);
        let profile_info_text = profile_info_text
            .first() // First element of the vector (it should only contain one?).
            .copied() // Dereferences &&str to &str.
            .unwrap_or("There is no profile_information.");

        let titles = value.get_title_for_locale(locale).unwrap();
        trace!("TITLES {:?}", titles);

        let keywords = value.get_keywords_for_locale(locale);
        trace!("KEYWORDS {:?}", keywords);

        // We have come this far, return the new struct.
        Ok(PersonClean {
            uuid: uuid.to_string(),
            name: full_name,
            profile_info: profile_info_text.to_string(),
        })
    }
}

// End simplified.

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    pub awardDate: Option<String>,
    pub fieldOfStudy: Option<FieldOfStudy>,
    pub organisationalUnits: Option<Vec<OrganisationalUnitAssociation>>,
    pub period: Option<Period>,
    pub projectTitle: Option<FormattedText>,
    pub pureId: Option<u64>,
    pub qualification: Option<Term>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldOfStudy {
    pub term: Option<Term>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Term {
    pub formatted: Option<bool>,
    pub text: Option<Vec<LocaleText>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocaleText {
    pub locale: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganisationalUnitAssociation {
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
    pub name: Option<FormattedText>,
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
pub struct FormattedText {
    pub formatted: Option<bool>,
    pub text: Option<Vec<LocaleText>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DateField {
    Struct(DateStruct),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateStruct {
    pub day: Option<u32>,
    pub month: Option<u32>,
    pub year: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Period {
    pub startDate: Option<DateField>,
    pub endDate: Option<DateField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    pub day: Option<u32>,
    pub month: Option<u32>,
    pub year: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPosition {
    pub appointmentValue: Option<FormattedText>,
    pub externalOrganisation: Option<Organisation>,
    pub period: Option<Period>,
    pub pureId: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organisation {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub link: Option<Link>,
    pub name: Option<FormattedText>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganisationAssociation {
    pub isPrimaryAssociation: Option<bool>,
    pub organisationalUnit: Option<OrganisationalUnit>,
    pub period: Option<Period>,
    pub person: Option<Person>,
    pub pureId: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub link: Option<Link>,
    pub name: Option<FormattedText>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub value: Option<FormattedValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedValue {
    pub formatted: Option<bool>,
    pub value: Option<String>,
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
pub struct LinkItem {
    pub description: Option<Term>,
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
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
pub struct Name {
    pub firstName: Option<String>,
    pub lastName: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameVariant {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub name: Option<Name>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionalQualification {
    pub abbreviatedQualification: Option<FormattedText>,
    pub period: Option<Period>,
    pub pureId: Option<u64>,
    pub qualification: Option<FormattedText>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileInformation {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub value: Option<FormattedText>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfilePhoto {
    pub filename: Option<String>,
    pub mimetype: Option<String>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub size: Option<u64>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaffOrganisationAssociation {
    pub addresses: Option<Vec<Address>>,
    pub affiliationId: Option<String>,
    pub contractType: Option<ContractType>,
    pub emails: Option<Vec<Email>>,
    pub employmentType: Option<EmploymentType>,
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub isPrimaryAssociation: Option<bool>,
    pub jobDescription: Option<FormattedText>,
    pub keywordGroups: Option<Vec<KeywordGroup>>,
    pub organisationalUnit: Option<OrganisationalUnit>,
    pub period: Option<PeriodSimple>,
    pub person: Option<Person>,
    pub phoneNumbers: Option<Vec<PhoneNumber>>,
    pub pureId: Option<u64>,
    pub staffType: Option<StaffType>,
    pub webAddresses: Option<Vec<WebAddress>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub addressLines: Option<String>,
    pub addressType: Option<AddressType>,
    pub building: Option<String>,
    pub city: Option<String>,
    pub country: Option<Country>,
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub geoLocation: Option<GeoLocation>,
    pub postalcode: Option<String>,
    pub pureId: Option<u64>,
    pub street: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoLocation {
    pub point: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub value: Option<FormattedValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodSimple {
    pub endDate: Option<String>,
    pub startDate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub value: Option<FormattedValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaffType {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebAddress {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub value: Option<FormattedText>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupervisedByRelation {
    pub period: Option<PeriodSimple>,
    pub pureId: Option<u64>,
    pub student: Option<StaffOrganisationAssociation>,
    pub supervisionPercentage: Option<f64>,
    pub supervisor: Option<Person>,
    pub supervisorRole: Option<SupervisorRole>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupervisorRole {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupervisorForRelation {
    pub period: Option<PeriodSimple>,
    pub pureId: Option<u64>,
    pub student: Option<StaffOrganisationAssociation>,
    pub supervisionPercentage: Option<f64>,
    pub supervisor: Option<Person>,
    pub supervisorRole: Option<SupervisorRole>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub value: Option<FormattedText>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Visibility {
    pub key: Option<String>,
    pub value: Option<Term>,
}

// ----

impl PersonJson {
    #[allow(dead_code)]
    pub fn get_all_education_pure_ids(&self) -> Vec<u64> {
        let mut ids = Vec::new();
        if let Some(educations) = &self.educations {
            for education in educations {
                if let Some(pure_id) = education.pureId {
                    ids.push(pure_id);
                }
            }
        }
        ids
    }

    // Getter function for first and last name.
    #[allow(dead_code)]
    pub fn get_first_and_last_name_old(&self) -> Option<(String, String)> {
        if let Some(name) = &self.name {
            if let (Some(first_name), Some(last_name)) = (&name.firstName, &name.lastName) {
                Some((first_name.clone(), last_name.clone()))
            } else {
                None
            }
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get_first_and_last_name(&self) -> Option<(&str, &str)> {
        Some((
        self.name.as_ref()?.firstName.as_deref()?,
        self.name.as_ref()?.lastName.as_deref()?,
        ))
    }

    // The uuid, should always be present. Return a slice.
    pub fn get_uuid(&self) -> Option<&str> {
        self.uuid.as_deref()
    }

    // Profile info text in difference locales. All values are Option<T> in the
    // struct, hence the large number of "if let Some(...)"s.
    /*
    "profileInformations": [
    {
      "value": {
        "text": [
          {
            "locale": "en_GB",
            "value": "..."
          }]},
    */
    pub fn get_profile_information_texts_for_locale(&self, locale: &str) -> Vec<&str> {
        let mut texts = Vec::new();
        if let Some(profile_informations) = &self.profileInformations {
            for profile_information in profile_informations {
                if let Some(value) = &profile_information.value {
                    if let Some(locale_texts) = &value.text {
                        for locale_text in locale_texts {
                            if let Some(text_locale) = &locale_text.locale {
                                if text_locale == locale {
                                    if let Some(text_value) = &locale_text.value {
                                        texts.push(text_value.as_ref());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        texts
    }

    pub fn get_title_for_locale(&self, locale: &str) -> Option<String> {
            self.titles.as_ref()?.iter()
                .filter_map(|title| title.value.as_ref())
                .filter_map(|formatted_text| formatted_text.text.as_ref())
                .flat_map(|texts| texts.iter())
                .find_map(|locale_text| {
                    if locale_text.locale.as_deref() == Some(locale) {
                        locale_text.value.clone()
                    } else {
                        None
                    }
                })
        }

        pub fn get_keywords_for_locale(&self, locale: &str) -> Vec<String> {
                let mut keywords = Vec::new();

                if let Some(keyword_groups) = &self.keywordGroups {
                    for group in keyword_groups {
                        if let Some(containers) = &group.keywordContainers {
                            for container in containers {
                                // Process freeKeywords
                                if let Some(free_keywords_list) = &container.freeKeywords {
                                    for free_keyword in free_keywords_list {
                                        if free_keyword.locale.as_deref() == Some(locale) {
                                            if let Some(free_keywords) = &free_keyword.freeKeywords {
                                                keywords.extend(free_keywords.clone());
                                            }
                                        }
                                    }
                                }
                                // Process structuredKeyword
                                if let Some(structured_keyword) = &container.structuredKeyword {
                                    if let Some(term) = &structured_keyword.term {
                                        if let Some(texts) = &term.text {
                                            for locale_text in texts {
                                                if locale_text.locale.as_deref() == Some(locale) {
                                                    if let Some(value) = &locale_text.value {
                                                        keywords.push(value.clone());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                keywords
            }
}

// ----

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
                    //panic!("{}", line);

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
    fn person_extract_pureid() {
        let data = r#"{"pureId":282828}"#;
        let person: PersonJson = serde_json::from_str(data).expect("Err");
        assert!(person.pureId == Some(282828));
    }

    #[test]
    fn empty_person() {
        let data = r#"{}"#;
        let person: PersonJson = serde_json::from_str(data).expect("Err");
        assert!(person.pureId == None);
    }

    #[test]
    fn test_person_parsing() {
        let data = r#"
        {
          "pureId": 282828,
          "externallyManaged": true,
          "uuid": "01234567-0123-0123-0123-0123456789ABC",
          "name": {
            "firstName": "Petrus",
            "lastName": "Berck"
          }
        }
        "#;
        let person: PersonJson = serde_json::from_str(data).expect("Err");
        assert_eq!(person.pureId, Some(282828));
        assert_eq!(person.externallyManaged, Some(true));
        assert_eq!(person.uuid.as_deref(), Some("01234567-0123-0123-0123-0123456789ABC"));
        if let Some(name) = person.name {
            assert_eq!(name.firstName.as_deref(), Some("Petrus"));
            assert_eq!(name.lastName.as_deref(), Some("Berck"));
        } else {
            panic!("Could not parse name struct.");
        }
    }

    #[test]
    fn test_person_des_ok() {
        let data = r#"
        {
          "pureId": 282828,
          "externallyManaged": true,
          "uuid": "01234567-0123-0123-0123-0123456789ABC",
          "name": {
            "firstName": "Quinten",
            "lastName": "Berck"
          }
        }
        "#;
        let person: PersonJson = serde_json::from_str(data).expect("Err");
        let person_des:PersonClean = PersonClean::try_from(&person).expect("Err");
        let person_des_jstr = serde_json::to_string(&person_des).unwrap();
        assert_eq!(person_des_jstr, r#"{"uuid":"01234567-0123-0123-0123-0123456789ABC","name":"Quinten Berck","profile_info":""}"#);
    }

    #[test]
    fn test_person_des_noname() {
        let data = r#"
        {
          "pureId": 282828,
          "externallyManaged": true,
          "uuid": "01234567-0123-0123-0123-0123456789ABC"
        }
        "#;
        let person: PersonJson = serde_json::from_str(data).expect("Err");
        let person_des = PersonClean::try_from(&person);
        println!("{:?}", person_des); // Err(MissingNameField)
        assert!(person_des.is_err());
    }

    #[test]
    fn test_person_des_nouuid() {
        let data = r#"
        {
          "pureId": 282828,
          "externallyManaged": true,
          "name": {
            "firstName": "Quinten",
            "lastName": "Berck"
          }
        }
        "#;
        let person: PersonJson = serde_json::from_str(data).expect("Err");
        let person_des = PersonClean::try_from(&person);
        println!("{:?}", person_des); // Err(MissingUUID)
        assert!(person_des.is_err());
    }

    #[test]
    fn test_person_des_profile_text() {
        let data = r#"
        {
          "pureId": 282828,
          "externallyManaged": true,
          "name": {
            "firstName": "Quinten",
            "lastName": "Berck"
          },
          "uuid": "01234567-0123-0123-0123-0123456789ABC",
          "profileInformations": [
            {
              "pureId": 37832137,
              "value": {
              "formatted": true,
              "text": [
                {
                  "locale": "en_GB",
                  "value": "Research Engineer, Lund University Humanities Lab"
                }
              ]
            }
           }
          ]
        }
        "#;
        let person: PersonJson = serde_json::from_str(data).expect("Err");
        let person_des = PersonClean::try_from_with_locale(&person, "en_GB").expect("Err");
        let person_des_jstr = serde_json::to_string(&person_des).unwrap();
        assert_eq!(person_des_jstr, r#"{"uuid":"01234567-0123-0123-0123-0123456789ABC","name":"Quinten Berck","profile_info":"Research Engineer, Lund University Humanities Lab"}"#);
    }

    #[test]
    fn test_date_parsing() {
        let data = r#"
        {
          "fte": 1.0,
          "info": {
            "createdDate": "1966-03-05T05:45:00.128+0100",
            "modifiedDate": "1966-03-05T05:45:00.128+0100",
            "portalUrl": "https://portal.research.lu.se/en/persons/01234567-0123-0123-0123-0123456789ABC",
            "prettyURLIdentifiers": [
              "petrus-berck"
            ]
          }
        }
        "#;
        let person: PersonJson = serde_json::from_str(data).expect("Err");
        assert_eq!(person.fte, Some(1.0));
        if let Some(info) = person.info {
            assert_eq!(info.createdDate.as_deref(), Some("1966-03-05T05:45:00.128+0100"));
            let expected = "https://portal.research.lu.se/en/persons/01234567-0123-0123-0123-0123456789ABC";
            assert_eq!(info.portalUrl.as_deref(), Some(expected));
            let expected = vec!["petrus-berck".to_string()];
            assert_eq!(info.prettyURLIdentifiers, Some(expected));
        } else {
            panic!("Could not parse info struct.");
        }
    }

}
