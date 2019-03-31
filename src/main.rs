#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

mod commands;
mod jira;
mod utils;

use clap::App;
use utils::config::Config;

fn main() {
    env_logger::init();
    let yaml = load_yaml!("cli_args.yml");
    let app = App::from_yaml(&yaml);
    let matches = app.get_matches();
    let working_directory = matches.value_of("path").unwrap_or(".");

    // Init is a special case, we don't want to load any config if we fire init;
    if let ("init", Some(flags)) = matches.subcommand() {
        return commands::init::start(flags.value_of("PATH"));
    };

    let mut path = std::path::PathBuf::from(working_directory);
    if path.is_relative() {
        // prepend the working directory;
        let current_dir =
            std::env::current_dir().expect("Unable to get reference to working directory");
        path = std::path::PathBuf::from(current_dir);
        path.push(working_directory);
    }

    // attempt to canonicalize the path;
    let path_string = path
        .canonicalize()
        .expect(
            "Provided path does not exist, please check that you have access to the set directory",
        )
        .to_str()
        .expect("Unable to decode path string. Please make sure your path is valid unicode")
        .to_owned();

    let config = Config::load(&path_string);

    match matches.subcommand() {
        ("completions", Some(flags)) => {
            let mut app = App::from_yaml(&yaml);
            app.gen_completions_to(
                "jira",
                flags.value_of("SHELL").expect("Shell type is required").parse().unwrap(),
                &mut std::io::stdout(),
            )
        }
        ("fetch", Some(flags)) => match flags.subcommand_name() {
            Some("boards") => commands::fetch::boards(config),
            Some("issues") => commands::fetch::issues(config),
            _ => {}
        },
        ("list", Some(flags)) => match flags.subcommand() {
            ("issues", Some(issue_flags)) => commands::list::issues(
                &config,
                commands::list::Options {
                    machine_ready: issue_flags.is_present("silent"),
                },
            )
            .unwrap_or(()),
            _ => {}
        },
        _ => {}
    };
}
