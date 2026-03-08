//! Keymap registration helpers.
//!
//! Ergonomic builder for setting Neovim keymaps from Rust plugins.

use nvim_oxi::api;
use nvim_oxi::api::opts::SetKeymapOpts;
use nvim_oxi::api::types::Mode;

/// Builder for a Neovim keymap.
pub struct Keymap {
    mode: Mode,
    lhs: String,
    rhs: String,
    desc: Option<String>,
    silent: bool,
    nowait: bool,
    buffer: Option<u32>,
}

impl Keymap {
    /// Create a normal-mode keymap.
    #[must_use]
    pub fn normal(lhs: &str, rhs: &str) -> Self {
        Self {
            mode: Mode::Normal,
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            desc: None,
            silent: true,
            nowait: false,
            buffer: None,
        }
    }

    /// Create an insert-mode keymap.
    #[must_use]
    pub fn insert(lhs: &str, rhs: &str) -> Self {
        Self {
            mode: Mode::Insert,
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            desc: None,
            silent: true,
            nowait: false,
            buffer: None,
        }
    }

    /// Create a visual-mode keymap.
    #[must_use]
    pub fn visual(lhs: &str, rhs: &str) -> Self {
        Self {
            mode: Mode::Visual,
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            desc: None,
            silent: true,
            nowait: false,
            buffer: None,
        }
    }

    /// Set a description for the keymap (shown in which-key, etc.).
    #[must_use]
    pub fn desc(mut self, desc: &str) -> Self {
        self.desc = Some(desc.to_string());
        self
    }

    /// Make the keymap non-silent (echo commands).
    #[must_use]
    pub fn loud(mut self) -> Self {
        self.silent = false;
        self
    }

    /// Set nowait (don't wait for longer key sequences).
    #[must_use]
    pub fn nowait(mut self) -> Self {
        self.nowait = true;
        self
    }

    /// Scope this keymap to a specific buffer.
    #[must_use]
    pub fn buffer(mut self, buf: u32) -> Self {
        self.buffer = Some(buf);
        self
    }

    /// Register this keymap with Neovim.
    pub fn register(self) -> crate::Result<()> {
        let mut opts = SetKeymapOpts::builder();
        opts.silent(self.silent);
        opts.nowait(self.nowait);
        if let Some(desc) = &self.desc {
            opts.desc(desc.as_str());
        }
        let opts = opts.build();

        api::set_keymap(self.mode, &self.lhs, &self.rhs, &opts)?;
        Ok(())
    }
}
