use clap::{Arg, ArgAction, Command};
use run_command::run_yarn;
use uuid_command::generate_uuid;

mod run_command;
mod uuid_command;

fn cli() -> Command {
    Command::new("astrid")
        .author("Granit Jupolli <https://github.com/G-Jupolli>")
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
        .subcommand(
            Command::new("yrun")
                .about("Runs 'yarn install && yarn dev' in the terminal")
                .arg(
                    Arg::new("frontend")
                        .long("frontend")
                        .short('f')
                        .action(ArgAction::SetTrue)
                        .help("Default to running in the ./frontend directory"),
                )
                .arg(
                    Arg::new("directory")
                        .long("dir")
                        .short('d')
                        .action(ArgAction::Set)
                        .help("Specify which directory to run in from CWD"),
                )
                .arg(
                    Arg::new("clear_node_modules")
                        .long("clear_modules")
                        .short('c')
                        .action(ArgAction::SetTrue)
                        .help("Clear the existing node modules"),
                ),
        )
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("uuid", sub_matches)) => {
            let upper_case = sub_matches
                .get_one::<bool>("uppercase")
                .map(|b| b.clone())
                .unwrap_or(false);

            generate_uuid(upper_case)
        }
        Some(("yrun", sub_matches)) => {
            let front_end = sub_matches
                .get_one::<bool>("frontend")
                .map(|b| b.clone())
                .unwrap_or(false);

            let clear_node = sub_matches
                .get_one::<bool>("clear_node_modules")
                .map(|b| b.clone())
                .unwrap_or(false);

            let dir = sub_matches
                .get_one::<String>("directory")
                .map(|s| s.to_string())
                .unwrap_or("./".to_string());

            run_yarn(front_end, dir, clear_node).await;
        }
        _ => unreachable!(),
    }
}
