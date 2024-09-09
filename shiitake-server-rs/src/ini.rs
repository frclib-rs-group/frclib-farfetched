use std::{path::PathBuf, collections::HashMap, io::Write};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IniError {
    #[error("Failed to read file")]
    FileReadError(#[from] std::io::Error),
    #[error("Failed to parse int")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Failed to parse float")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Custom")]
    CustomError(&'static str),
    #[error("Key doesn't exist")]
    KeyDoesntExist,
    #[error("Section doesn't exist")]
    SectionDoesntExist,
}

pub type IniResult<T> = Result<T, IniError>;

#[derive(Debug, PartialEq, Clone)]
pub enum IniTypes {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool)
}

impl IniTypes {
    pub fn to_string(self) -> Option<String> {
        match self {
            IniTypes::String(value) => Some(value),
            _ => None
        }
    }

    pub fn to_integer(self) -> Option<i64> {
        match self {
            IniTypes::Integer(value) => Some(value),
            _ => None
        }
    }

    pub fn to_float(self) -> Option<f64> {
        match self {
            IniTypes::Float(value) => Some(value),
            _ => None
        }
    }

    pub fn to_boolean(self) -> Option<bool> {
        match self {
            IniTypes::Boolean(value) => Some(value),
            _ => None
        }
    }

    pub fn to_boolean_from_string(self) -> Option<bool> {
        match self {
            IniTypes::String(value) => {
                let l_value = value.to_lowercase();
                if l_value == "true" {
                    Some(true)
                } else if l_value == "false" {
                    Some(false)
                } else {
                    None
                }
            },
            _ => None
        }
    }

    pub fn types_match(&self, other: &Self) -> bool {
        match self {
            IniTypes::String(_) => matches!(other, IniTypes::String(_)),
            IniTypes::Integer(_) => matches!(other, IniTypes::Integer(_)),
            IniTypes::Float(_) => matches!(other, IniTypes::Float(_)),
            IniTypes::Boolean(_) => matches!(other, IniTypes::Boolean(_)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ini {
    path: PathBuf,
    sections: HashMap<String, IniSection>,
}

impl Ini {
    pub fn get(&self, section: &str) -> Option<&IniSection> {
        self.sections.get(section)
    }

    pub fn get_mut(&mut self, section: &str) -> Option<&mut IniSection> {
        self.sections.get_mut(section)
    }

    pub fn set(&mut self, section: IniSection) -> IniResult<()> {
        if self.sections.contains_key(&section.name) {
            Err(IniError::CustomError("Section already exists"))
        } else {
            self.sections.insert(section.name.clone(), section);
            Ok(())
        }
    }

    pub fn create_and_set(&mut self, section: IniSection) {
        self.sections.insert(section.name.clone(), section);
    }

    pub fn save(&self) -> Result<(), IniError> {
        write_ini(self.clone())
    }

    pub fn finish(self) -> Result<(), IniError> {
        write_ini(self)
    }
}


#[derive(Debug, Clone)]
pub struct IniSection {
    name: String,
    keys: HashMap<String, IniTypes>,
}

impl IniSection {
    pub fn get(&self, key: &str) -> Option<&IniTypes> {
        self.keys.get(key)
    }

    pub fn set(&mut self, key: &str, value: IniTypes) -> IniResult<()>{
        if self.keys.contains_key(key) {
            self.keys.insert(key.to_string(), value);
            Ok(())
        } else {
            Err(IniError::KeyDoesntExist)
        }
    }

    pub fn create_and_set(&mut self, key: &str, value: IniTypes) {
        self.keys.insert(key.to_string(), value);
    }

    pub fn new(name: &str) -> IniSection {
        IniSection{name: name.to_string(), keys: HashMap::new()}
    }
}

pub fn read_ini(path: PathBuf) -> IniResult<Ini> {
    let file = std::fs::read_to_string(path.clone())?;
    let mut sections = HashMap::new();
    let mut current_section = String::new();
    for line in file.lines() {
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            sections.insert(current_section.clone(), IniSection { name: current_section.clone(), keys: HashMap::new() });
        } else if let Some((key, value)) = line.replace(" ", "").split_once('=') {
            if value.starts_with('"') && value.ends_with('"') {
                sections.get_mut(&current_section).unwrap().keys.insert(key.to_string(), IniTypes::String(value[1..value.len()-1].to_string()));
            } else if value.parse::<i64>().is_ok() {
                sections.get_mut(&current_section).unwrap().keys.insert(key.to_string(), IniTypes::Integer(value.parse::<i64>().unwrap()));
            } else if value.parse::<f64>().is_ok() {
                sections.get_mut(&current_section).unwrap().keys.insert(key.to_string(), IniTypes::Float(value.parse::<f64>().unwrap()));
            } else if value == "true" || value == "false" {
                sections.get_mut(&current_section).unwrap().keys.insert(key.to_string(), IniTypes::Boolean(value.parse::<bool>().unwrap()));
            } else {
                sections.get_mut(&current_section).unwrap().keys.insert(key.to_string(), IniTypes::String(value.to_string()));
            }
        }
    }
    Ok(Ini{sections, path})
}

pub fn read_ini_field(path: PathBuf, section: &str, key: &str) -> IniResult<IniTypes> {
    let ini = read_ini(path)?;
    if let Some(section) = ini.get(section) {
        if let Some(key) = section.get(key) {
            return Ok(key.clone());
        } else {
            Err(IniError::KeyDoesntExist)
        }
    } else {
        Err(IniError::SectionDoesntExist)
    }
}

pub fn write_ini(ini: Ini) -> IniResult<()> {
    let mut file = std::fs::File::create(ini.path)?;
    for section in ini.sections.values() {
        file.write_all(format!("[{}]\n", section.name).as_bytes())?;
        for (key, value) in section.keys.iter() {
            match value {
                IniTypes::String(value) => file.write_all(format!("{} = \"{}\"\n", key, value).as_bytes())?,
                IniTypes::Integer(value) => file.write_all(format!("{} = {}\n", key, value).as_bytes())?,
                IniTypes::Float(value) => file.write_all(format!("{} = {}\n", key, value).as_bytes())?,
                IniTypes::Boolean(value) => file.write_all(format!("{} = {}\n", key, value).as_bytes())?
            }
        }
        file.write_all(b"\n")?;
    }
    Ok(())
}