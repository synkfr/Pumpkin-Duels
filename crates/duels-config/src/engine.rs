use crate::error::ConfigError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use toml_edit::{DocumentMut, Item, Table};

pub fn reconcile_tables(target: &mut Table, template: &Table) -> bool {
    let mut modified = false;

    for (key, template_item) in template.iter() {
        if !target.contains_key(key) {
            target.insert(key, template_item.clone());
            modified = true;
        } else if let (Some(target_table), Some(template_table)) = (
            target.get_mut(key).and_then(Item::as_table_like_mut),
            template_item.as_table_like(),
        ) {
            for (sub_key, sub_item) in template_table.iter() {
                if !target_table.contains_key(sub_key) {
                    target_table.insert(sub_key, sub_item.clone());
                    modified = true;
                }
            }
        }
    }

    modified
}

pub struct ConfigManager<T> {
    path: PathBuf,
    data: Arc<RwLock<T>>,
}

impl<T> ConfigManager<T>
where
    T: Serialize + DeserializeOwned + Default + Clone,
{
    pub fn load_or_create<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path_buf = path.as_ref().to_path_buf();

        if let Some(parent) = path_buf.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent).map_err(|source| ConfigError::Io {
                path: parent.to_path_buf(),
                source,
            })?;
        }

        let default_instance = T::default();
        let default_toml =
            toml::to_string_pretty(&default_instance).map_err(|source| ConfigError::Serialize {
                path: path_buf.clone(),
                source,
            })?;

        let template_doc: DocumentMut =
            default_toml
                .parse::<DocumentMut>()
                .map_err(|source| ConfigError::Parse {
                    path: path_buf.clone(),
                    source: Box::new(source),
                })?;

        let final_instance = if path_buf.exists() {
            let existing_raw =
                fs::read_to_string(&path_buf).map_err(|source| ConfigError::Io {
                    path: path_buf.clone(),
                    source,
                })?;

            let mut existing_doc: DocumentMut = existing_raw.parse::<DocumentMut>().map_err(
                |source| ConfigError::Parse {
                    path: path_buf.clone(),
                    source: Box::new(source),
                },
            )?;

            let was_updated =
                reconcile_tables(existing_doc.as_table_mut(), template_doc.as_table());

            let loaded_val: T = toml::from_str(&existing_doc.to_string()).map_err(|source| {
                ConfigError::Deserialize {
                    path: path_buf.clone(),
                    source,
                }
            })?;

            if was_updated {
                fs::write(&path_buf, existing_doc.to_string()).map_err(|source| {
                    ConfigError::Io {
                        path: path_buf.clone(),
                        source,
                    }
                })?;
            }

            loaded_val
        } else {
            fs::write(&path_buf, &default_toml).map_err(|source| ConfigError::Io {
                path: path_buf.clone(),
                source,
            })?;
            default_instance
        };

        Ok(Self {
            path: path_buf,
            data: Arc::new(RwLock::new(final_instance)),
        })
    }

    pub fn get(&self) -> T {
        self.data.read().unwrap().clone()
    }

    pub fn update<F>(&self, modifier: F) -> Result<T, ConfigError>
    where
        F: FnOnce(&mut T),
    {
        let mut guard = self.data.write().unwrap();
        modifier(&mut guard);
        let updated_val = guard.clone();

        let toml_string =
            toml::to_string_pretty(&updated_val).map_err(|source| ConfigError::Serialize {
                path: self.path.clone(),
                source,
            })?;

        fs::write(&self.path, toml_string).map_err(|source| ConfigError::Io {
            path: self.path.clone(),
            source,
        })?;

        Ok(updated_val)
    }

    pub fn reload(&self) -> Result<T, ConfigError> {
        let content = fs::read_to_string(&self.path).map_err(|source| ConfigError::Io {
            path: self.path.clone(),
            source,
        })?;

        let reloaded: T = toml::from_str(&content).map_err(|source| ConfigError::Deserialize {
            path: self.path.clone(),
            source,
        })?;

        let mut guard = self.data.write().unwrap();
        *guard = reloaded.clone();

        Ok(reloaded)
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconcile_tables_adds_missing_keys() {
        let mut target: DocumentMut = "existing_key = \"custom_value\"\n".parse().unwrap();
        let template: DocumentMut = "existing_key = \"default\"\nnew_key = \"added\"\n"
            .parse()
            .unwrap();

        let modified = reconcile_tables(target.as_table_mut(), template.as_table());
        assert!(modified);

        let output = target.to_string();
        assert!(output.contains("existing_key = \"custom_value\""));
        assert!(output.contains("new_key = \"added\""));
    }
}
