#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};

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
pub struct Period {
    pub startDate: Option<Date>,
    pub endDate: Option<Date>,
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

    // Getter function for the Person struct
    pub fn get_first_and_last_name(&self) -> Option<(String, String)> {
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

pub fn test_person() {
    let data = r#"{"pureId":141649,"externallyManaged":true,"uuid":"34347c3f-5f08-4412-a9f8-573a58acc46e","name":{"firstName":"Torbjörn","lastName":"Hjort"},"fte":0.0,"info":{"createdDate":"2016-03-31T14:02:53.048+0200","modifiedDate":"2023-06-30T01:40:44.964+0200","portalUrl":"https://portal.research.lu.se/en/persons/34347c3f-5f08-4412-a9f8-573a58acc46e","prettyURLIdentifiers":["torbjörn-hjort"]},"visibility":{"key":"FREE","value":{"formatted":false,"text":[{"locale":"en_GB","value":"Public - No restriction"},{"locale":"sv_SE","value":"Allmänt tillgänglig - Inga restriktioner"}]}},"titles":[{"pureId":141693,"externallyManaged":true,"value":{"formatted":false,"text":[{"locale":"en_GB","value":"senior lecturer"},{"locale":"sv_SE","value":"Universitetslektor"}]},"type":{"pureId":4872,"uri":"/dk/atira/pure/person/titles/generic","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Title"},{"locale":"sv_SE","value":"Titel"}]}}}],"profileInformations":[{"pureId":7253612,"value":{"formatted":true,"text":[{"locale":"en_GB","value":"<p>My research interests concerns sustenance problems and economic vulnerability. One area involves the questions of what implications economic vulnerability may have in a society characterized by consumption. Another area is focused on how the welfare state relates to economic vulnerability and how notions of what is a reasonable standard of living are created and manifested. In a third area, I study how society’s financialization affects the individual, in particular groups with limited financial resources. I am also involved in research concerning how the role of the citizen changes in relation to an increased freedom of choice regarding welfare services.</p>"},{"locale":"sv_SE","value":"<p>Mina forskningsområden berör på olika sätt försörjningsproblem och ekonomisk utsatthet. Ett område fokuserar på vad ekonomisk utsatthet kan ha för innebörder i ett samhälle präglat av konsumtion. Ett annat område handlar om hur välfärdsstaten förhåller sig till ekonomisk utsatthet och om hur föreställningar kring skälig levnadsnivå skapas och tar sig uttryck. I ett tredje område studerar jag hur samhällets finansialisering påverkar den enskilde, företrädesvis grupper med knapp ekonomi. Vidare är jag involverad i forskning som rör hur medborgarrollen förändras i takt med en ökad valfrihet gällande välfärdstjänster.</p>"}]},"type":{"pureId":106829375,"uri":"/dk/atira/pure/person/customfields/researchinterests","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Research"},{"locale":"sv_SE","value":"Forskning"}]}}},{"pureId":7253613,"value":{"formatted":true,"text":[{"locale":"en_GB","value":"<p>I am mainly involved in the fifth, sixth and seventh semesters of the BSc in Social work programme, including supervising and examining bachelor’s theses. I am also involved in the master's programme and have been course director for the course Social Work with Poverty and Maintenance Problems.</p>"},{"locale":"sv_SE","value":"<p>Jag arbetar huvudsakligen på terminerna 5, 6 och 7 på socionomprogrammet där jag bland annat handleder och examinerar c-uppsatser. Jag är också involverad i masterprogrammet och har varit kursföreståndare för kursen Socialt arbete med fattigdom och försörjningsproblem.</p>"}]},"type":{"pureId":4830,"uri":"/dk/atira/pure/person/customfields/teaching","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Teaching"},{"locale":"sv_SE","value":"Undervisning"}]}}}],"staffOrganisationAssociations":[{"pureId":141668,"externallyManaged":true,"person":{"uuid":"34347c3f-5f08-4412-a9f8-573a58acc46e","link":{"ref":"content","href":"https://lucris.lub.lu.se/ws/api/524/persons/34347c3f-5f08-4412-a9f8-573a58acc46e"},"externallyManaged":true,"name":{"formatted":false,"text":[{"value":"Torbjörn Hjort"}]}},"period":{"startDate":"2015-09-21T12:00:00.000+0200"},"isPrimaryAssociation":false,"organisationalUnit":{"uuid":"069c947f-fe86-4b45-b1de-f7eb56cbef44","link":{"ref":"content","href":"https://lucris.lub.lu.se/ws/api/524/organisational-units/069c947f-fe86-4b45-b1de-f7eb56cbef44"},"externallyManaged":true,"name":{"formatted":false,"text":[{"locale":"en_GB","value":"School of Social Work"},{"locale":"sv_SE","value":"Socialhögskolan"}]},"type":{"pureId":6028,"uri":"/dk/atira/pure/organisation/organisationtypes/organisation/department","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Department"},{"locale":"sv_SE","value":"Institution"}]}}},"staffType":{"pureId":8523,"uri":"/dk/atira/pure/person/personstafftype/academic","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Academic"},{"locale":"sv_SE","value":"!!Academic"}]}},"jobDescription":{"formatted":false,"text":[{"locale":"en_GB","value":"Associate professor, Senior lecturer"},{"locale":"sv_SE","value":"Docent, Universitetslektor"}]},"keywordGroups":[{"pureId":141682,"externallyManaged":true,"logicalName":"exofficio","type":{"uri":"/dk/atira/pure/person/staff/exofficio","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Ex Officio"},{"locale":"sv_SE","value":"Verksamhetsroller"}]}},"keywordContainers":[{"pureId":141683,"structuredKeyword":{"pureId":5324,"uri":"/dk/atira/pure/person/staff/exofficio/sen_lecturer","term":{"formatted":false,"text":[{"locale":"en_GB","value":"senior lecturer"},{"locale":"sv_SE","value":"universitetslektor"}]}}},{"pureId":91199305,"structuredKeyword":{"pureId":4976,"uri":"/dk/atira/pure/person/staff/exofficio/reader","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Associate Professor"},{"locale":"sv_SE","value":"docent"}]}}}]}]},{"pureId":129698080,"externallyManaged":true,"person":{"uuid":"34347c3f-5f08-4412-a9f8-573a58acc46e","link":{"ref":"content","href":"https://lucris.lub.lu.se/ws/api/524/persons/34347c3f-5f08-4412-a9f8-573a58acc46e"},"externallyManaged":true,"name":{"formatted":false,"text":[{"value":"Torbjörn Hjort"}]}},"period":{"startDate":"2022-11-29T12:00:00.000+0100"},"isPrimaryAssociation":false,"organisationalUnit":{"uuid":"0c396fd2-1eb2-4356-a95e-fe18c666c946","link":{"ref":"content","href":"https://lucris.lub.lu.se/ws/api/524/organisational-units/0c396fd2-1eb2-4356-a95e-fe18c666c946"},"externallyManaged":true,"name":{"formatted":false,"text":[{"locale":"en_GB","value":"Social Vulnerability and Inequality"},{"locale":"sv_SE","value":"Social utsatthet och ojämlikhet"}]},"type":{"pureId":6055,"uri":"/dk/atira/pure/organisation/organisationtypes/organisation/groupingnode","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Grouping node"},{"locale":"sv_SE","value":"grupperingsnod"}]}}},"staffType":{"pureId":8523,"uri":"/dk/atira/pure/person/personstafftype/academic","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Academic"},{"locale":"sv_SE","value":"!!Academic"}]}},"jobDescription":{"formatted":false,"text":[{"locale":"en_GB","value":"Senior lecturer"},{"locale":"sv_SE","value":"Universitetslektor"}]},"keywordGroups":[{"pureId":129698083,"externallyManaged":true,"logicalName":"exofficio","type":{"uri":"/dk/atira/pure/person/staff/exofficio","term":{"formatted":false,"text":[{"locale":"en_GB","value":"Ex Officio"},{"locale":"sv_SE","value":"Verksamhetsroller"}]}},"keywordContainers":[{"pureId":129698084,"structuredKeyword":{"pureId":5324,"uri":"/dk/atira/pure/person/staff/exofficio/sen_lecturer","term":{"formatted":false,"text":[{"locale":"en_GB","value":"senior lecturer"},{"locale":"sv_SE","value":"universitetslektor"}]}}}]}]}]}
    "#;

    let root: PersonJson = serde_json::from_str(data).expect("Err");
    println!("{:#?}", root);
}
