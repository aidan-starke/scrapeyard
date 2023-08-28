use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs,
};

#[derive(Debug)]
pub enum ReadWriteError {
    Read(String),
    #[allow(dead_code)]
    Write(String),
    JsonParse(String),
    #[allow(dead_code)]
    JsonSerialize(String),
}

impl Display for ReadWriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadWriteError::JsonParse(location) => {
                write!(f, "Unable to parse json for {}", location)
            }
            ReadWriteError::JsonSerialize(location) => {
                write!(f, "Unable to serialize json for {}", location)
            }
            ReadWriteError::Read(location) => write!(f, "Unable to read file for {}", location),
            ReadWriteError::Write(location) => write!(f, "Unable to write file for {}", location),
        }
    }
}

impl Error for ReadWriteError {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Surf {
    pub model: String,
    pub count: u32,
    pub links: Vec<String>,
}

pub fn read_surfs(location: String) -> Result<Vec<Surf>, Box<dyn Error>> {
    let previous = fs::read_to_string(format!("{}.json", "data/".to_string() + location.as_str()))
        .map_err(|_| ReadWriteError::Read(location.clone()))?;

    Ok(serde_json::from_str::<Vec<Surf>>(&previous)
        .map_err(|_| ReadWriteError::JsonParse(location))?)
}
