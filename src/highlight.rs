//! Highlight group definition helpers.

use nvim_oxi::api;
use nvim_oxi::types::SetHighlightOpts;

/// Builder for a Neovim highlight group.
pub struct Highlight {
    name: String,
    fg: Option<String>,
    bg: Option<String>,
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    link: Option<String>,
    ns_id: u32,
}

impl Highlight {
    /// Create a new highlight group definition.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            link: None,
            ns_id: 0,
        }
    }

    /// Set the foreground color (hex like `"#ff0000"` or highlight name).
    #[must_use]
    pub fn fg(mut self, color: &str) -> Self {
        self.fg = Some(color.to_string());
        self
    }

    /// Set the background color.
    #[must_use]
    pub fn bg(mut self, color: &str) -> Self {
        self.bg = Some(color.to_string());
        self
    }

    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    #[must_use]
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    #[must_use]
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    #[must_use]
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Link this highlight group to another.
    #[must_use]
    pub fn link(mut self, target: &str) -> Self {
        self.link = Some(target.to_string());
        self
    }

    /// Set the namespace ID (0 = global).
    #[must_use]
    pub fn namespace(mut self, ns_id: u32) -> Self {
        self.ns_id = ns_id;
        self
    }

    /// Apply this highlight definition to Neovim.
    pub fn apply(self) -> crate::Result<()> {
        let mut opts = SetHighlightOpts::builder();

        if let Some(ref link) = self.link {
            opts.link(link.as_str());
        } else {
            if let Some(ref fg) = self.fg {
                opts.fg(fg.as_str());
            }
            if let Some(ref bg) = self.bg {
                opts.bg(bg.as_str());
            }
            opts.bold(self.bold);
            opts.italic(self.italic);
            opts.underline(self.underline);
            opts.strikethrough(self.strikethrough);
        }

        let opts = opts.build();
        api::set_hl(self.ns_id, &self.name, &opts)?;
        Ok(())
    }
}
