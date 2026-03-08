//! Plugin configuration via Neovim global variables.
//!
//! Plugins store their config in `vim.g.<plugin_name>` as a Lua table.
//! This module provides typed access to those values.

use nvim_oxi::api;
use nvim_oxi::Object;

/// Configuration reader for a plugin's global variable.
#[derive(Debug, Clone)]
pub struct Config {
    prefix: String,
}

impl Config {
    /// Create a config reader for `vim.g.<name>`.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            prefix: name.to_string(),
        }
    }

    /// Read the plugin's config table from `vim.g.<name>`.
    /// Returns `None` if the variable is not set.
    pub fn from_global(name: &str) -> crate::Result<Option<Self>> {
        let var: std::result::Result<Object, _> = api::get_var(name);
        match var {
            Ok(_) => Ok(Some(Self::new(name))),
            Err(_) => Ok(None),
        }
    }

    /// Get a string value from the config.
    pub fn get_string(&self, key: &str) -> crate::Result<Option<String>> {
        let full_key = format!("{}_{}", self.prefix, key);
        match api::get_var::<String>(&full_key) {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }

    /// Get a boolean value from the config.
    pub fn get_bool(&self, key: &str) -> crate::Result<Option<bool>> {
        let full_key = format!("{}_{}", self.prefix, key);
        match api::get_var::<bool>(&full_key) {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }

    /// Get an integer value from the config.
    pub fn get_int(&self, key: &str) -> crate::Result<Option<i64>> {
        let full_key = format!("{}_{}", self.prefix, key);
        match api::get_var::<i64>(&full_key) {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }

    /// The config prefix (plugin name).
    #[must_use]
    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}
