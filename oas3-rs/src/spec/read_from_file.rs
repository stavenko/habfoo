use std::path::Path;
use std::fs::File;

use serde::{ de::DeserializeOwned};

use crate::{spec::RefError, Spec};

use super::FromRef;


pub fn read_from_file< P, T>(spec: &Spec, path: P) -> Result<T, RefError>
where
    P: AsRef<Path>,
    T: DeserializeOwned + FromRef,
{
    let file_path = spec
        .root_directory
        .as_ref()
        .map(|root| {
            let p: &Path = root.as_ref();
            p.join(path)
        })
        .expect("WFT");

    let schema = File::open(&file_path)
        .map_err(|e| RefError::UnableToReadFromFile(file_path.display().to_string(), e.to_string()))
        .and_then(|reader| {
            serde_yaml::from_reader::<_, T>(reader).map_err(|e| {
                RefError::UnableToReadFromFile(file_path.display().to_string(), e.to_string())
            })
        });
    match schema {
        Ok(schema) => {
            let oor = super::ObjectOrReference::Object(schema);
            oor.resolve(spec)
        }
        Err(err) => Err(RefError::UnableToReadFromFile(
            file_path.display().to_string(),
            err.to_string(),
        )),
    }
}
