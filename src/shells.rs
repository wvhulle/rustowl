use std::{
    env,
    fmt::{self, Display},
    io,
    path::Path,
    str::FromStr,
};

use clap::ValueEnum;
use clap_complete::{Generator, shells};
use clap_complete_nushell::Nushell;

/// Shell with auto-generated completion script available.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
#[non_exhaustive]
#[value(rename_all = "lower")]
pub enum Shell {
    /// Bourne Again `SHell` (bash)
    Bash,
    /// Elvish shell
    Elvish,
    /// Friendly Interactive `SHell` (fish)
    Fish,
    /// `PowerShell`
    Power,
    /// Z `SHell` (zsh)
    Zsh,
    /// `Nushell`
    Nushell,
}

impl Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for Shell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::value_variants()
            .iter()
            .find(|variant| variant.to_possible_value().unwrap().matches(s, false))
            .copied()
            .ok_or_else(|| format!("invalid variant: {s}"))
    }
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Self::Bash => shells::Bash.file_name(name),
            Self::Elvish => shells::Elvish.file_name(name),
            Self::Fish => shells::Fish.file_name(name),
            Self::Power => shells::PowerShell.file_name(name),
            Self::Zsh => shells::Zsh.file_name(name),
            Self::Nushell => Nushell.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn io::Write) {
        match self {
            Self::Bash => shells::Bash.generate(cmd, buf),
            Self::Elvish => shells::Elvish.generate(cmd, buf),
            Self::Fish => shells::Fish.generate(cmd, buf),
            Self::Power => shells::PowerShell.generate(cmd, buf),
            Self::Zsh => shells::Zsh.generate(cmd, buf),
            Self::Nushell => Nushell.generate(cmd, buf),
        }
    }
}

impl Shell {
    /// Parse a shell from a path to the executable for the shell
    ///
    /// # Examples
    ///
    /// ```
    /// use clap_complete::shells::Shell;
    ///
    /// assert_eq!(Shell::from_shell_path("/bin/bash"), Some(Shell::Bash));
    /// assert_eq!(Shell::from_shell_path("/usr/bin/zsh"), Some(Shell::Zsh));
    /// assert_eq!(Shell::from_shell_path("/opt/my_custom_shell"), None);
    /// ```
    pub fn from_shell_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        parse_shell_from_path(path.as_ref())
    }

    /// Determine the user's current shell from the environment
    ///
    /// This will read the SHELL environment variable and try to determine which
    /// shell is in use from that.
    ///
    /// If SHELL is not set, then on windows, it will default to powershell, and
    /// on other operating systems it will return `None`.
    ///
    /// If SHELL is set, but contains a value that doesn't correspond to one of
    /// the supported shell types, then return `None`.
    ///
    /// # Example:
    ///
    /// ```no_run
    /// # use clap::Command;
    /// use clap_complete::{generate, shells::Shell};
    /// # fn build_cli() -> Command {
    /// #     Command::new("compl")
    /// # }
    /// let mut cmd = build_cli();
    /// generate(
    ///     Shell::from_env().unwrap_or(Shell::Bash),
    ///     &mut cmd,
    ///     "myapp",
    ///     &mut std::io::stdout(),
    /// );
    /// ```
    pub fn from_env() -> Option<Self> {
        env::var_os("SHELL")
            .and_then(Self::from_shell_path)
            .or_else(|| cfg!(windows).then_some(Self::Power))
    }
}

// use a separate function to avoid having to monomorphize the entire function
// due to from_shell_path being generic
fn parse_shell_from_path(path: &Path) -> Option<Shell> {
    let name = path.file_stem()?.to_str()?;
    match name {
        "bash" => Some(Shell::Bash),
        "zsh" => Some(Shell::Zsh),
        "fish" => Some(Shell::Fish),
        "elvish" => Some(Shell::Elvish),
        "powershell" | "powershell_ise" => Some(Shell::Power),
        "nushell" => Some(Shell::Nushell),
        _ => None,
    }
}
