use std::io::{self, IsTerminal, Write};

use owo_colors::OwoColorize;

pub fn is_color_enabled() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if std::env::var("CLICOLOR") == Ok("0".to_owned()) {
        return false;
    }
    io::stdout().is_terminal()
}

pub fn with_color(enabled: bool) -> ColorMode {
    ColorMode { enabled }
}

#[derive(Debug, Clone, Copy)]
pub struct ColorMode {
    enabled: bool,
}

impl ColorMode {
    pub fn success(&self, text: &str) -> String {
        if self.enabled {
            text.green().bold().to_string()
        } else {
            text.to_owned()
        }
    }

    pub fn error(&self, text: &str) -> String {
        if self.enabled {
            text.red().bold().to_string()
        } else {
            text.to_owned()
        }
    }

    pub fn info(&self, text: &str) -> String {
        if self.enabled {
            text.cyan().to_string()
        } else {
            text.to_owned()
        }
    }

    pub fn highlight(&self, text: &str) -> String {
        if self.enabled {
            text.yellow().bold().to_string()
        } else {
            text.to_owned()
        }
    }

    pub fn dim(&self, text: &str) -> String {
        if self.enabled {
            text.dimmed().to_string()
        } else {
            text.to_owned()
        }
    }

    pub fn id(&self, text: &str) -> String {
        if self.enabled {
            text.blue().to_string()
        } else {
            text.to_owned()
        }
    }
}

pub fn print_success(msg: &str) -> io::Result<()> {
    let color = with_color(is_color_enabled());
    writeln!(io::stdout(), "{}", color.success(msg))
}

pub fn print_error(msg: &str) -> io::Result<()> {
    let color = with_color(is_color_enabled());
    writeln!(io::stderr(), "{}", color.error(msg))
}

pub fn print_info(msg: &str) -> io::Result<()> {
    let color = with_color(is_color_enabled());
    writeln!(io::stdout(), "{}", color.info(msg))
}

pub fn read_line(prompt: &str) -> io::Result<String> {
    let color = with_color(is_color_enabled());
    write!(io::stdout(), "{}", color.info(prompt))?;
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_owned())
}
