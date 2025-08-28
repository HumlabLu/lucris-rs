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

impl ConceptJson {
    /// Return (id, en_GB text) where id = conceptId if available, otherwise uuid.
    pub fn id_and_text_for_locale(&self, locale: &str) -> Option<(String, String)> {
        let _id = self.conceptId.as_ref().or(self.uuid.as_ref())?.clone();
        let id = self.uuid.as_ref().or(self.conceptId.as_ref())?.clone();
        let labels = self.name.as_ref()?.text.as_ref()?;
        let pick = |loc: &str| {
            labels
                .iter()
                .find(|lt| lt.locale.as_deref() == Some(loc))
                .and_then(|lt: &LocaleText| lt.value.as_ref().cloned())
        };
        let text = pick(locale)
            .or_else(|| pick("en_GB"))
            .or_else(|| pick("en"))
            .or_else(|| labels.iter().find_map(|lt| lt.value.clone()))?;
        Some((id, text))
    }
}

pub fn read_concept_jsonl(file_path: &str) -> Result<Vec<ConceptJson>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let data = Arc::new(Mutex::new(vec![]));
    let failed_count = Arc::new(Mutex::new(0));

    reader
        .lines()
        .map_while(Result::ok)
        .par_bridge() // parallelise
        .for_each(|line: String| {
            match serde_json::from_str::<ConceptJson>(&line) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concept_id_text() {
        let data = r#"{"pureId":103675163,"uuid":"ad91f1ae-3503-4cd5-9c3b-d126ea2ed999","thesauri":{"uuid":"ed51ac10-0e54-4833-97db-6503ceb8854c","link":{"ref":"content","href":"https://lucris.lub.lu.se/ws/api/524/thesauri/ed51ac10-0e54-4833-97db-6503ceb8854c"},"name":{"formatted":false,"text":[{"locale":"en_GB","value":"Chemical Compounds"},{"locale":"sv_SE","value":"Kemiska föreningar"}]}},"conceptId":"422263159","idf":1.0,"info":{"createdDate":"2021-10-12T09:15:01.001+0200","modifiedDate":"2021-10-12T09:15:01.001+0200"},"name":{"formatted":false,"text":[{"locale":"en_GB","value":"Phosphatidylethanolamine 40:9 Zwitterion"}]},"terms":[{"locale":"en","value":"Phosphatidylethanolamine 40:9 Zwitterion"},{"locale":"en","value":"Phosphatidyl-ethanolamine 40:9 Zwitterion"}]}"#;
        let concept: ConceptJson = serde_json::from_str(data).expect("Err in concept parsing");
        if let Some((id, txt)) = concept.id_and_text_for_locale("en_GB") {
            assert_eq!(id, "ad91f1ae-3503-4cd5-9c3b-d126ea2ed999");
            assert_eq!(txt, "Phosphatidylethanolamine 40:9 Zwitterion");
        }
    }
}
