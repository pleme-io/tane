//! Autocommand registration helpers.

use nvim_oxi::api;
use nvim_oxi::types::{AutocmdCallbackArgs, CreateAutocmdOpts};

/// Builder for a Neovim autocommand.
pub struct Autocmd {
    events: Vec<String>,
    pattern: Option<String>,
    group: Option<String>,
    desc: Option<String>,
    once: bool,
}

impl Autocmd {
    /// Create an autocommand for the given event(s).
    ///
    /// ```ignore
    /// Autocmd::on(&["BufEnter", "BufWritePost"])
    ///     .pattern("*.rs")
    ///     .group("my_plugin")
    ///     .register(|args| { /* ... */ Ok(false) })?;
    /// ```
    #[must_use]
    pub fn on(events: &[&str]) -> Self {
        Self {
            events: events.iter().map(|e| (*e).to_string()).collect(),
            pattern: None,
            group: None,
            desc: None,
            once: false,
        }
    }

    /// File pattern to match (e.g., `"*.rs"`, `"*.lua"`).
    #[must_use]
    pub fn pattern(mut self, pattern: &str) -> Self {
        self.pattern = Some(pattern.to_string());
        self
    }

    /// Autocommand group name. Creates the group if it doesn't exist.
    #[must_use]
    pub fn group(mut self, group: &str) -> Self {
        self.group = Some(group.to_string());
        self
    }

    /// Description for the autocommand.
    #[must_use]
    pub fn desc(mut self, desc: &str) -> Self {
        self.desc = Some(desc.to_string());
        self
    }

    /// Only fire this autocommand once.
    #[must_use]
    pub fn once(mut self) -> Self {
        self.once = true;
        self
    }

    /// Register the autocommand with a callback.
    ///
    /// The callback receives `AutocmdCallbackArgs` and returns `Ok(true)`
    /// to delete the autocommand after firing, or `Ok(false)` to keep it.
    pub fn register<F>(self, callback: F) -> crate::Result<()>
    where
        F: Fn(AutocmdCallbackArgs) -> crate::Result<bool> + 'static,
    {
        // Create group if specified.
        if let Some(group_name) = &self.group {
            let group_opts = nvim_oxi::types::CreateAugroupOpts::builder()
                .clear(false)
                .build();
            api::create_augroup(group_name, &group_opts)?;
        }

        let mut opts = CreateAutocmdOpts::builder();

        if let Some(pattern) = &self.pattern {
            opts.patterns([pattern.as_str()]);
        }
        if let Some(group) = &self.group {
            opts.group(group.as_str());
        }
        if let Some(desc) = &self.desc {
            opts.desc(desc.as_str());
        }
        opts.once(self.once);

        let events: Vec<&str> = self.events.iter().map(String::as_str).collect();

        opts.callback(move |args| {
            callback(args).unwrap_or(false)
        });

        let opts = opts.build();
        api::create_autocmd(events, &opts)?;

        Ok(())
    }
}
