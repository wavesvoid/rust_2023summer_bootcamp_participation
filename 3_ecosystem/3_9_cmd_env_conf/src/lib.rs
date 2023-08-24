mod cli;
mod config;
mod serde_defaults;

use self::config::PrintableSettings;


pub struct PrintableConfig(PrintableSettings);
impl PrintableConfig {
    pub fn new() -> Self {
        Self(PrintableSettings::new().unwrap())
    }
    pub fn print(&self) {
        println!("{:#?}", self.0);
    }
}