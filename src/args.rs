//! Argument parsing

extern crate clap;

use clap::{crate_name, crate_description, crate_version, crate_authors};
use clap::{App, Arg, SubCommand};

pub(super) fn parse() -> super::Command {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(SubCommand::with_name("auth")
                    .about("Fetch an OAuth token from Fitbit")
                    .arg(Arg::with_name("client_id")
                         .help("The client ID of your personal app")
                         .required(true))
                    .arg(Arg::with_name("client_secret")
                         .help("The client secret of your personal app")
                         .required(true)))
        .subcommand(SubCommand::with_name("badges")
                    .about("Prints a list of user badges"))
        .subcommand(SubCommand::with_name("devices")
                    .about("Prints a list of currently paired devices"))
        .subcommand(SubCommand::with_name("profile")
                    .about("Print user profile information"))
        .get_matches();

    match matches.subcommand() {
        ("auth", Some(auth_matches)) => {
            let id = auth_matches.value_of("client_id").unwrap().to_string();
            let secret = auth_matches.value_of("client_secret").unwrap().to_string();
            super::Command::GetAuthToken(id, secret)
        }
        ("badges", Some(_)) => super::Command::GetBadges,
        ("devices", Some(_)) => super::Command::GetDevices,
        ("profile", Some(_)) => super::Command::GetProfile,
        ("", None) => {
            eprintln!("Please enter a valid command! See `-h` for more info!");
            std::process::exit(1);
        }
        _ => unreachable!(),
    }
}
