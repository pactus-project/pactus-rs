use anyhow::{Context, Error};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};
use toml;

/// Loads TOML-encoded file.
pub fn load_config_file<P, T>(path: P) -> Result<T, Error>
where
    T: for<'r> Deserialize<'r>,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let res = toml_load(path).with_context(|| format!("loading config from {}", path.display()))?;
    Ok(res)
}

/// Loads Json-encoded file.
pub fn load_genesis_file<P, T>(path: P) -> Result<T, Error>
where
    T: for<'r> Deserialize<'r>,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let res =
        json_load(path).with_context(|| format!("loading genesis from {}", path.display()))?;
    Ok(res)
}

/// Loads Json-encoded file.
pub fn load_json_file<P, T>(path: P) -> Result<T, Error>
where
    T: for<'r> Deserialize<'r>,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let res = json_load(path).with_context(|| format!("loading json from {}", path.display()))?;
    Ok(res)
}

/// Saves TOML-encoded file.
///
/// Creates directory if needed.
pub fn save_config_file<P, T>(value: &T, path: P) -> Result<(), Error>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    toml_save(value, path).with_context(|| format!("saving config to {}", path.display()))?;
    Ok(())
}

/// Saves Json-encoded file.
///
/// Creates directory if needed.
pub fn save_genesis_file<P, T>(value: &T, path: P) -> Result<(), Error>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    save_json(value, path).with_context(|| format!("saving genesis to {}", path.display()))?;
    Ok(())
}

/// Saves Json-encoded file.
///
/// Creates directory if needed.
pub fn save_json_file<P, T>(value: &T, path: P) -> Result<(), Error>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    save_json(value, path).with_context(|| format!("saving json to {}", path.display()))?;
    Ok(())
}

fn toml_load<T: DeserializeOwned>(path: &Path) -> Result<T, Error> {
    let mut file = File::open(path)?;
    let mut toml = String::new();
    file.read_to_string(&mut toml)?;
    Ok(toml::de::from_str(&toml)?)
}

pub fn json_load<T: DeserializeOwned>(path: &Path) -> Result<T, Error> {
    let file = File::open(path)?;
    let json_data = serde_json::from_reader(file)?;
    Ok(json_data)
}
fn toml_save<T: Serialize>(value: &T, path: &Path) -> Result<(), Error> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut file = File::create(path)?;
    let value_toml = toml::Value::try_from(value)?;
    file.write_all(value_toml.to_string().as_bytes())?;
    Ok(())
}

fn save_json<T: Serialize>(value: &T, path: &Path) -> Result<(), Error> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut file = File::create(path)?;
    let json_value = serde_json::to_string(&value).unwrap();
    file.write_all(json_value.as_bytes())?;
    Ok(())
}
