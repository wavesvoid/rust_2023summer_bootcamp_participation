use clap::{Parser, ValueEnum};


#[derive(Parser, Debug)]
#[command(version, about = "Does awesome things", long_about = None)]
pub struct CliArgs {
    /// Enables debug mode
    #[arg(short, long, help_heading = Some("CONFIG"))]
    pub debug: bool,

    /// Path to configuration file
    #[arg(short = 'c',
          long,
          value_name = "conf",
          default_value = "config.toml",
          env = "CONF_FILE",
          help_heading = Some("CONFIG"))]
    pub conf: String,
}

