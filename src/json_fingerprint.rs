#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
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

#[derive(Debug, Serialize, Deserialize)]
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

pub fn read_fingerprint_jsonl(file_path: &str) -> Result<Vec<FingerprintJson>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let data = Arc::new(Mutex::new(vec![]));
    let failed_count = Arc::new(Mutex::new(0));
    
    reader
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()   // parallelise
        .for_each(|line: String| {
            match serde_json::from_str::<FingerprintJson>(&line) {
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
