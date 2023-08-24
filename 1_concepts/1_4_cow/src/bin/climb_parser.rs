use climb::*;
use step_1_4::config_utils::*;


fn main() {
    let _ = create_app!()
        .command(Command::new(
            "--conf",
            "desc",
            exec_fn
        ).alias("c").arg("conf")
    ).run();
}


fn exec_fn(inp: FunctionInput, _: FunctionOptions) -> FunctionResult {
    let mut conf = ConfigPath::new();

    if let Some(s) = inp.get(0) {
        let _ = conf.set_path(s);
    }
    conf.print_path();

    Ok(None)
}