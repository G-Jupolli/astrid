use clap::{Arg, ArgAction, Command};
use run_command::run_yarn;
use uuid_command::generate_uuid;

mod on_line_command;
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
                )
                .arg(
                    Arg::new("amount")
                        .long("amount")
                        .short('n')
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new("comma")
                        .long("comma")
                        .short('c')
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
        .subcommand(
            Command::new("oLine")
                .about("Runs 'yarn install && yarn dev' in the terminal")
                .arg(
                    Arg::new("file_path")
                        .long("file_path")
                        .short('f')
                        .action(ArgAction::Set)
                        .required(true)
                        .help("Select a file path"),
                )
                .arg(
                    Arg::new("prefix")
                        .long("prefix")
                        .short('p')
                        .action(ArgAction::Set)
                        .help("Append the prefix to the start of each line"),
                )
                .arg(
                    Arg::new("suffix")
                        .long("suffix")
                        .short('s')
                        .action(ArgAction::Set)
                        .help("Append the suffix to the end of each line"),
                )
                .arg(
                    Arg::new("minify")
                        .long("minify")
                        .short('m')
                        .action(ArgAction::SetTrue)
                        .help("Minify to one line"),
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

            let count = sub_matches
                .get_one::<String>("amount")
                .map(|i| i.parse().expect("Invalid Number"))
                .unwrap_or(1);

            let comma = sub_matches
                .get_one::<bool>("comma")
                .map(|b| b.clone())
                .unwrap_or(false);

            generate_uuid(count, upper_case, comma)
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
        Some(("oLine", sub_matches)) => {
            let file_path = sub_matches
                .get_one::<String>("file_path")
                .map(|b| b.clone())
                .unwrap();

            let prefix = sub_matches.get_one::<String>("prefix").map(|b| b.clone());

            let suffix = sub_matches.get_one::<String>("suffix").map(|b| b.clone());

            let (prefix, suffix) = match (&prefix, &suffix) {
                (None, None) => {
                    println!("  - \x1b[91mFailed\x1b[0m: Prefix -p or Suffix -s required");
                    std::process::exit(1);
                }
                _ => (
                    prefix.unwrap_or("".to_string()),
                    suffix.unwrap_or("".to_string()),
                ),
            };

            let no_new_line = sub_matches
                .get_one::<bool>("minify")
                .map(|b| b.clone())
                .unwrap_or(false);

            on_line_command::on_line_command(file_path, prefix, suffix, no_new_line);
        }
        _ => unreachable!(),
    }
}
