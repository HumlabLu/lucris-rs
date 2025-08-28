#![allow(non_snake_case)]
use log::{debug, error, info, trace, warn};
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
pub struct FingerprintJson {
    pub concepts: Option<Vec<Concept>>,
    pub contentFamily: Option<String>,
    pub contentId: Option<u64>,
    pub contentUuid: Option<String>,
    pub contentVersion: Option<u64>,
    pub info: Option<Info>,
    pub pureId: Option<u64>,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Concept {
    pub frequency: Option<f64>,
    pub rank: Option<f64>,
    pub uuid: Option<String>,
    pub weightedRank: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub createdBy: Option<String>,
    pub createdDate: Option<String>,
    pub modifiedBy: Option<String>,
    pub modifiedDate: Option<String>,
}

// ----------------------------------------------------------

impl FingerprintJson {
    /// Return (content id, list of concept uuids) where id = contentUuid or fallback uuid
    pub fn _id_and_concept_uuids(&self) -> Option<(String, Vec<String>)> {
        let id = self.contentUuid.as_ref().or(self.uuid.as_ref())?.clone();

        let concept_ids = self
            .concepts
            .as_ref()?
            .iter()
            .filter_map(|c: &Concept| c.uuid.clone())
            .collect::<Vec<_>>();

        Some((id, concept_ids))
    }

    pub fn id_and_concept_uuids(&self) -> Option<(String, Vec<(String, f64)>)> {
        let id = self.contentUuid.as_ref().or(self.uuid.as_ref())?.clone();

        let concepts = self.concepts.as_ref()?;
        let concept_pairs = concepts
            .iter()
            .filter_map(|c| {
                let uuid = c.uuid.as_ref()?;
                let rank = c.weightedRank.unwrap_or(0.0) as f64;
                Some((uuid.clone(), rank))
            })
            .collect::<Vec<_>>();

        Some((id, concept_pairs))
    }

    pub fn id_and_concepts(&self) -> Option<(String, Vec<Concept>)> {
        let id = self.contentUuid.as_ref().or(self.uuid.as_ref())?.clone();

        let concepts = self.concepts.as_ref()?.clone();
        Some((id, concepts))
    }

    pub fn id_and_concepts_ref(&self) -> Option<(&str, &[Concept])> {
        let id = self.contentUuid.as_deref().or(self.uuid.as_deref())?;
        let concepts = self.concepts.as_deref()?;
        Some((id, concepts))
    }
}

pub fn read_fingerprint_jsonl(
    file_path: &str,
) -> Result<Vec<FingerprintJson>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let data = Arc::new(Mutex::new(vec![]));
    let failed_count = Arc::new(Mutex::new(0));

    reader
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge() // parallelise
        .for_each(|line: String| {
            match serde_json::from_str::<FingerprintJson>(&line) {
                Ok(json) => {
                    debug!("uuid={:?}", json.uuid);

                    // Add it to the data vector.
                    let mut data = data.lock().unwrap();
                    data.push(json);
                }
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
