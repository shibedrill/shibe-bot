use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};

use serde::de::Deserialize;
use serde::ser::Serialize;

/// A utility structure to manage a settings structure.
pub struct SettingsManager<T: Default + Serialize + for<'a> Deserialize<'a>> {
    internal: T,
    path: String,
}

impl<T: Default + Serialize + for<'a> Deserialize<'a>> SettingsManager<T> {
    /// Instantiate new self if the path contains a valid serialization of
    /// the settings structure.
    pub fn load(path: &str) -> Option<Self> {
        let mut file = std::fs::File::open(path).ok()?;
        let mut data = String::new();
        file.read_to_string(&mut data).ok()?;
        let settings = serde_json::from_str(&data).ok()?;
        Some(Self {
            internal: settings,
            path: String::from(path),
        })
    }
    /// Update the data stored in the settings, if it has been modified on the
    /// disk but not in memory. Because this is a stupid method, it will most
    /// likely go unused by most.
    #[allow(dead_code)]
    pub fn update(&mut self) -> Option<()> {
        let mut file = std::fs::File::open(self.path.clone()).ok()?;
        let mut data = String::new();
        file.read_to_string(&mut data).ok()?;
        self.internal = serde_json::from_str(&data).ok()?;
        Some(())
    }
    /// Serialize settings structure to the stored path. Returns None if
    /// unsuccessful.
    pub fn store(&self) -> Option<()> {
        let data = serde_json::to_string_pretty(&self.internal).ok()?;
        let mut file = std::fs::File::create(&self.path).ok()?;
        let _ = file.write(data.as_bytes());
        Some(())
    }
    /// Create a new manager, passing in the path, and a structure to manage.
    /// We cannot initialize a settings manager without fully initialized settings.
    pub fn manage(path: &str, intake: T) -> Self {
        Self {
            internal: intake,
            path: path.to_string(),
        }
    }
}

impl<T: Default + Serialize + for<'a> Deserialize<'a>> Deref for SettingsManager<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T: Default + Serialize + for<'a> Deserialize<'a>> DerefMut for SettingsManager<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal
    }
}
