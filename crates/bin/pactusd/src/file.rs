use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};
use toml;

/// Loads TOML-encoded file.
pub fn load_toml_file<P, T>(path: P) -> Result<T>
where
    T: for<'r> Deserialize<'r>,
    P: AsRef<Path>,
{
    let content = read_file(path.as_ref())?;
    Ok(toml::de::from_str(&content)?)
}

/// Loads Json-encoded file.
pub fn load_json_file<P, T>(path: P) -> Result<T>
where
    T: for<'r> Deserialize<'r>,
    P: AsRef<Path>,
{
    let content = read_file(path.as_ref())?;
    Ok(serde_json::de::from_str(&content)?)
}

/// Loads plain-encoded file.
pub fn load_text_file<P, T>(path: P) -> Result<T>
where
    T: for<'r> Deserialize<'r>,
    P: AsRef<Path>,
{
    let content = read_file(path.as_ref())?;
    Ok(serde_plain::from_str(&content)?)
}

/// Saves TOML-encoded file.
///
/// Creates directory if needed.
pub fn save_toml_file<P, T>(value: &T, path: P) -> Result<()>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let content = toml::to_string(value)?;
    save_file(path.as_ref(), &content)
}

/// Saves Json-encoded file.
///
/// Creates directory if needed.
pub fn save_json_file<P, T>(value: &T, path: P) -> Result<()>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let content = serde_json::to_string(&value)?;
    save_file(path.as_ref(), &content)
}

/// Saves plain-encoded file.
///
/// Creates directory if needed.
pub fn save_text_file<P, T>(value: &T, path: P) -> Result<()>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let content = serde_plain::to_string(&value)?;
    save_file(path.as_ref(), &content)
}

fn read_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn save_file(path: &Path, content: &str) -> Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    struct Foo {
        bar: String,
    }

    #[test]
    fn test_json() {
        let foo1 = Foo {
            bar: "test".to_string(),
        };

        let file = tempfile::NamedTempFile::new().unwrap();
        save_json_file(&foo1, file.path()).unwrap();
        let foo2: Foo = load_json_file(file.path()).unwrap();
        assert_eq!(foo1, foo2);
    }

    #[test]
    fn test_toml() {
        let foo1 = Foo {
            bar: "test".to_string(),
        };

        let file = tempfile::NamedTempFile::new().unwrap();
        save_toml_file(&foo1, file.path()).unwrap();
        let foo2: Foo = load_toml_file(file.path()).unwrap();
        assert_eq!(foo1, foo2);
    }

    #[test]
    fn test_plain() {
        let foo1 = "test".to_string();

        let file = tempfile::NamedTempFile::new().unwrap();
        save_text_file(&foo1, file.path()).unwrap();
        let foo2: String = load_text_file(file.path()).unwrap();
        assert_eq!(foo1, foo2);
    }
}
