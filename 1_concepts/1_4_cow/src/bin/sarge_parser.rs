use sarge::{ArgumentParser, arg, Tag};


fn main() {
    // TBH this is a crapy parser, but nevertheless
    // there was some expirience

    // Config instance
    let mut conf = step_1_4::config_utils::ConfigPath::new();

    // CLI arguments parsing
    let mut parser = ArgumentParser::new();
    parser.add(arg!(str, long, "conf"));
    parser.parse().expect("Cannot parse arguments");

    // Extracting needed argument
    let user_path = parser
        .arg(Tag::Long("conf".into()))
        .and_then(|arg| arg.val.to_owned())
        .map(|v| v.get_str());
    if let Some(v) = &user_path {
        let _ = conf.set_path(v);
    }

    conf.print_path();
}