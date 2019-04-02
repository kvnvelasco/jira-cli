
extern crate serde_yaml;

extern crate fs;
extern crate jira;
mod utils;

// use clap::App;
use jira::client::Client;
use jira::{Context};

use utils::io::{pick_from_list, read_line};

fn main() -> Result<(), Box<std::error::Error>> {
    // let app = App::new("workflow");

    let working_directory = std::env::current_dir()?;
    let workspace =
        fs::Workspace::new(&working_directory).child_workspace(&String::from("./.workflow"));
    workspace.create_directories()?;

    let jira_workspace = workspace.child_workspace(&fs::PathBuf::from("./jira"));
    jira_workspace.create_directories()?;
    jira_workspace.create_file(&fs::PathBuf::from("./session.yml"));
    jira_workspace.create_file(&fs::PathBuf::from("./context.yml"));

    let jira_session_file = jira_workspace.read_file(&fs::Path::new("./session.yml"))?;

    let client: Client = match serde_yaml::from_str(&jira_session_file) {
        Ok(client) => client,
        Err(_) => {
            println!("You don't have a jira session set up yet");
            let domain = read_line("Provide your atlassian domain (e.g: company.atlassian.net)");
            let email = read_line("Enter your email");
            println!(
                "Enter your atlassian API key. \
                 \nFor instructions on how to generate an api key, visit \
                 \nhttps://confluence.atlassian.com/cloud/api-tokens-938839638.html"
            );

            let api_key = read_line("Your API key");
            let client = Client::new(&domain, &api_key, &email);

            // save it
            let string = serde_yaml::to_string(&client)?;
            jira_workspace.write_file(fs::Path::new("./session.yml"), &string)?;

            client
        }
    };

    let jira_context_file = jira_workspace.read_file(&fs::Path::new("./context.yml"))?;
    
    let _context: Context = match serde_yaml::from_str(&jira_context_file) {
        Ok(context) => context,
        Err(_) => {
            println!("You don't have an active context set up yet");
            let boards = jira::board::get_boards(&client)?;
            let board_index =
                pick_from_list("Select one project from the list Above", &boards.values)?;
            let board = &boards.values[board_index];
            let sprints = board.get_sprints(&client)?;
            let sprint_index =
                pick_from_list("Select one sprint from the list Above", &sprints.values)?;
            let sprint = &sprints.values[sprint_index];

            let context = Context {
                active_board: board.id,
                active_sprint: sprint.id,
                active_issue: None,
            };
            let string = serde_yaml::to_string(&context)?;
            jira_workspace.write_file(fs::Path::new("./context.yml"), &string)?;

            context
        }
    };

    // let boards = jira::board::get_boards(&client)?;
    // let board_index = pick_from_list("Select one from the list Above", &boards.values)?;
    // let board = &boards.values[board_index];
    // let sprints = board.get_sprints(&client)?;
    // let sprint_index = pick_from_list("Select one from the list Above", &sprints.values)?;
    // let sprint = &sprints.values[sprint_index];
    // let issues = sprint.get_issues(&client)?;
    // let issue_index = pick_from_list("Select one from the list Above", &issues.issues)?;

    // env_logger::init();
    // let yaml = load_yaml!("cli_args.yml");
    // let app = App::from_yaml(&yaml);
    // let matches = app.get_matches();
    // let working_directory = matches.value_of("path").unwrap_or(".");

    // // Init is a special case, we don't want to load any config if we fire init;
    // if let ("init", Some(flags)) = matches.subcommand() {
    //     return commands::init::start(flags.value_of("PATH"));
    // };

    // let mut path = std::path::PathBuf::from(working_directory);
    // if path.is_relative() {
    //     // prepend the working directory;
    //     let current_dir =
    //         std::env::current_dir().expect("Unable to get reference to working directory");
    //     path = std::path::PathBuf::from(current_dir);
    //     path.push(working_directory);
    // }

    // // attempt to canonicalize the path;
    // let path_string = path
    //     .canonicalize()
    //     .expect(
    //         "Provided path does not exist, please check that you have access to the set directory",
    //     )
    //     .to_str()
    //     .expect("Unable to decode path string. Please make sure your path is valid unicode")
    //     .to_owned();

    // let config = Config::load(&path_string);

    // match matches.subcommand() {
    //     ("completions", Some(flags)) => {
    //         let mut app = App::from_yaml(&yaml);
    //         app.gen_completions_to(
    //             "jira",
    //             flags.value_of("SHELL").expect("Shell type is required").parse().unwrap(),
    //             &mut std::io::stdout(),
    //         )
    //     }
    //     ("fetch", Some(flags)) => match flags.subcommand_name() {
    //         Some("boards") => commands::fetch::boards(config),
    //         Some("issues") => commands::fetch::issues(config),
    //         _ => {}
    //     },
    //     ("list", Some(flags)) => match flags.subcommand() {
    //         ("issues", Some(issue_flags)) => commands::list::issues(
    //             &config,
    //             commands::list::Options {
    //                 machine_ready: issue_flags.is_present("silent"),
    //             },
    //         )
    //         .unwrap_or(()),
    //         _ => {}
    //     },
    //     _ => {}
    // };

    Ok(())
}
