//! User command registration helpers.

use nvim_oxi::api;
use nvim_oxi::types::{CommandArgs, CommandNArgs, CreateCommandOpts};

/// Builder for a Neovim user command.
pub struct UserCommand {
    name: String,
    desc: Option<String>,
    nargs: CommandNArgs,
    bang: bool,
    bar: bool,
    buffer: Option<u32>,
}

impl UserCommand {
    /// Create a new user command.
    ///
    /// Command names must start with an uppercase letter.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            desc: None,
            nargs: CommandNArgs::Zero,
            bang: false,
            bar: false,
            buffer: None,
        }
    }

    /// Set the description.
    #[must_use]
    pub fn desc(mut self, desc: &str) -> Self {
        self.desc = Some(desc.to_string());
        self
    }

    /// Accept zero or one argument.
    #[must_use]
    pub fn optional_arg(mut self) -> Self {
        self.nargs = CommandNArgs::ZeroOrOne;
        self
    }

    /// Accept exactly one argument.
    #[must_use]
    pub fn one_arg(mut self) -> Self {
        self.nargs = CommandNArgs::One;
        self
    }

    /// Accept any number of arguments.
    #[must_use]
    pub fn any_args(mut self) -> Self {
        self.nargs = CommandNArgs::Any;
        self
    }

    /// Accept one or more arguments.
    #[must_use]
    pub fn at_least_one_arg(mut self) -> Self {
        self.nargs = CommandNArgs::OneOrMore;
        self
    }

    /// Allow the `!` modifier.
    #[must_use]
    pub fn bang(mut self) -> Self {
        self.bang = true;
        self
    }

    /// Allow `|` command chaining.
    #[must_use]
    pub fn bar(mut self) -> Self {
        self.bar = true;
        self
    }

    /// Scope to a specific buffer.
    #[must_use]
    pub fn buffer(mut self, buf: u32) -> Self {
        self.buffer = Some(buf);
        self
    }

    /// Register the command with a callback.
    pub fn register<F>(self, callback: F) -> crate::Result<()>
    where
        F: Fn(CommandArgs) -> crate::Result<()> + 'static,
    {
        let mut opts = CreateCommandOpts::builder();
        opts.nargs(self.nargs);
        opts.bang(self.bang);
        opts.bar(self.bar);
        if let Some(desc) = &self.desc {
            opts.desc(desc.as_str());
        }
        let opts = opts.build();

        api::create_user_command(
            &self.name,
            move |args| {
                callback(args).unwrap_or(());
            },
            &opts,
        )?;

        Ok(())
    }
}
