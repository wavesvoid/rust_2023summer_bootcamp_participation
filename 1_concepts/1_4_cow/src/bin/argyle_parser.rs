use argyle::{
    Argue,
    FLAG_HELP,
    FLAG_REQUIRED,
    FLAG_VERSION,
};

use step_1_4::config_utils::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conf = ConfigPath::new();

    // In production we would definitely like to handle this properly
    let args = Argue::new(FLAG_HELP | FLAG_REQUIRED | FLAG_VERSION)?;
    let option: Option<&[u8]> = args.option(b"--conf");

    let s = String::from_utf8(
        option.expect("Cannot parse argument").to_vec())
        .expect("Invalid CLI argument string");

    let _ = conf.set_path(&s);
    conf.print_path();

    Ok(())
}