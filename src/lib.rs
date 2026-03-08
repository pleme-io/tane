//! Tane (種) — plugin SDK for building Rust-native Neovim plugins.
//!
//! Provides high-level abstractions over `nvim-oxi` for common plugin
//! patterns: configuration, autocommands, keymaps, highlights, and
//! user commands.
//!
//! # Quick Start
//!
//! ```ignore
//! use tane::prelude::*;
//!
//! #[nvim_oxi::plugin]
//! fn my_plugin() -> oxi::Result<()> {
//!     let config = Config::from_global("my_plugin")?;
//!     Keymap::normal("<leader>mp", "MyPluginAction")
//!         .desc("Run my plugin")
//!         .register()?;
//!     Ok(())
//! }
//! ```

pub mod autocmd;
pub mod config;
pub mod highlight;
pub mod keymap;
pub mod namespace;
pub mod usercmd;

/// Re-exports for convenient plugin authoring.
pub mod prelude {
    pub use crate::autocmd::Autocmd;
    pub use crate::config::Config;
    pub use crate::highlight::Highlight;
    pub use crate::keymap::Keymap;
    pub use crate::namespace::Namespace;
    pub use crate::usercmd::UserCommand;
    pub use nvim_oxi as oxi;
}

/// Errors specific to the tane SDK.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("nvim-oxi error: {0}")]
    Oxi(#[from] nvim_oxi::Error),

    #[error("config key not found: {0}")]
    ConfigKeyNotFound(String),

    #[error("config type mismatch for key '{key}': expected {expected}")]
    ConfigTypeMismatch { key: String, expected: String },

    #[error("{0}")]
    Custom(String),
}

impl From<nvim_oxi::api::Error> for Error {
    fn from(err: nvim_oxi::api::Error) -> Self {
        Self::Oxi(nvim_oxi::Error::from(err))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
