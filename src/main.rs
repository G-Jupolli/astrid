use clap::{Arg, ArgAction, Command};
use uuid_command::generate_uuid;

mod uuid_command;

fn cli() -> Command {
    Command::new("astrid")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("uuid")
                .about("Generates a Uuidv4 in lowercase")
                .arg(
                    Arg::new("uppercase")
                        .long("uppercase")
                        .short('u')
                        .action(ArgAction::SetTrue),
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("uuid", sub_matches)) => {
            let upper_case = sub_matches
                .get_one::<bool>("uppercase")
                .map(|b| b.clone())
                .unwrap_or(false);

            generate_uuid(upper_case)
        }
        _ => unreachable!(),
    }
}
