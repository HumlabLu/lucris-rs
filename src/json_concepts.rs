#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConceptJson {
    pub conceptId: Option<String>,
    pub concepts: Option<Vec<Concept>>,
    pub idf: Option<f64>,
    pub info: Option<Info>,
    pub name: Option<Name>,
    pub pureId: Option<u64>,
    pub terms: Option<Vec<Term>>,
    pub thesauri: Option<Thesauri>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Concept {
    pub link: Option<Link>,
    pub name: Option<Name>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub href: Option<String>,
    #[serde(rename = "ref")]
    pub ref_field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub formatted: Option<bool>,
    pub text: Option<Vec<LocaleText>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocaleText {
    pub locale: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub createdBy: Option<String>,
    pub createdDate: Option<String>,
    pub modifiedBy: Option<String>,
    pub modifiedDate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Term {
    pub locale: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thesauri {
    pub link: Option<Link>,
    pub name: Option<Name>,
    pub uuid: Option<String>,
}

// ----------------------------------------------------------

pub fn read_concept_jsonl(file_path: &str) -> Result<Vec<ConceptJson>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let data = Arc::new(Mutex::new(vec![]));
    let failed_count = Arc::new(Mutex::new(0));
    
    reader
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()   // parallelise
        .for_each(|line: String| {
            match serde_json::from_str::<ConceptJson>(&line) {
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
