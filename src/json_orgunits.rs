#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgUnitJson {
    pub addresses: Option<Vec<Address>>,
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub info: Option<Info>,
    pub keywordGroups: Option<Vec<KeywordGroup>>,
    pub links: Option<Vec<LinkItem>>,
    pub name: Option<Name>,
    pub nameVariants: Option<Vec<NameVariant>>,
    pub parents: Option<Vec<Parent>>,
    pub period: Option<Period>,
    pub phoneNumbers: Option<Vec<PhoneNumber>>,
    pub photos: Option<Vec<Photo>>,
    pub profileInformations: Option<Vec<ProfileInformation>>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub uuid: Option<String>,
    pub visibility: Option<Visibility>,
    pub webAddresses: Option<Vec<WebAddress>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub addressType: Option<AddressType>,
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
pub struct Info {
    pub createdBy: Option<String>,
    pub createdDate: Option<String>,
    pub modifiedBy: Option<String>,
    pub modifiedDate: Option<String>,
    pub portalUrl: Option<String>,
    pub prettyURLIdentifiers: Option<Vec<String>>,
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
pub struct TypeField {
    pub pureId: Option<u64>,
    pub term: Option<Term>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkItem {
    pub pureId: Option<u64>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub formatted: Option<bool>,
    pub text: Option<Vec<LocaleText>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameVariant {
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub value: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    pub formatted: Option<bool>,
    pub text: Option<Vec<LocaleText>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parent {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub link: Option<Link>,
    pub name: Option<Name>,
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
pub struct Period {
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
    pub value: Option<ValueWithValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueWithValue {
    pub formatted: Option<bool>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    pub filename: Option<String>,
    pub mimetype: Option<String>,
    pub pureId: Option<u64>,
    pub size: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileInformation {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub value: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Visibility {
    pub key: Option<String>,
    pub value: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebAddress {
    pub externalIdSource: Option<String>,
    pub externalId: Option<String>,
    pub externallyManaged: Option<bool>,
    pub pureId: Option<u64>,
    #[serde(rename = "type")]
    pub type_field: Option<TypeField>,
    pub value: Option<Value>,
}

// ----------------------------------------------------------

pub fn read_orgunits_jsonl(file_path: &str) -> Result<Vec<OrgUnitJson>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let data = Arc::new(Mutex::new(vec![]));
    let failed_count = Arc::new(Mutex::new(0));
    
    reader
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()   // parallelise
        .for_each(|line: String| {
            match serde_json::from_str::<OrgUnitJson>(&line) {
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
