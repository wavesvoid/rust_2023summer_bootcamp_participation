/// Priority:
/// 1) Default values declared directly in Rust sources;
/// 2) Values read from TOML file;
/// 3) Values set by environment variables with CONF_ prefix.




use step_3_9::PrintableConfig;



fn main() {
    let config = PrintableConfig::new();
    config.print();
}
