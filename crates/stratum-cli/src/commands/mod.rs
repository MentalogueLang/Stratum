pub mod install;
pub mod r#use;
pub mod pin;
pub mod list;
pub mod remove;
pub mod which;

pub fn dispatch(args: &[String]) -> Result<(), String> {
    let Some((command, rest)) = args.split_first() else {
        return Err(usage());
    };

    match command.as_str() {
        "install" => install::run(rest),
        "use" => r#use::run(rest),
        "pin" => pin::run(rest),
        "list" => list::run(rest),
        "remove" => remove::run(rest),
        "which" => which::run(rest),
        "help" | "--help" | "-h" => Err(usage()),
        other => Err(format!("unknown command `{other}`\n\n{}", usage())),
    }
}

pub fn usage() -> String {
    [
        "usage:",
        "  stratum install <version>",
        "  stratum use <version>",
        "  stratum pin <version>",
        "  stratum list",
        "  stratum remove <version>",
        "  stratum which",
    ]
    .join("\n")
}
