//! Neovim namespace management.
//!
//! Namespaces isolate highlights, signs, and virtual text per plugin.

use nvim_oxi::api;

/// A Neovim namespace scoped to a plugin.
#[derive(Debug, Clone, Copy)]
pub struct Namespace {
    id: u32,
}

impl Namespace {
    /// Create or get a namespace by name.
    ///
    /// Convention: use your plugin's crate name as the namespace name.
    pub fn create(name: &str) -> crate::Result<Self> {
        let id = api::create_namespace(name);
        Ok(Self { id })
    }

    /// The numeric namespace ID for use with nvim-oxi APIs.
    #[must_use]
    pub const fn id(self) -> u32 {
        self.id
    }
}
