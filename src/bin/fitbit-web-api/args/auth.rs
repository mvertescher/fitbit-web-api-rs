//! Argument parsing for auth commands

use crate::Command;

use clap::{App, Arg, ArgMatches};

pub(super) const BASE: &'static str = "auth";

pub(super) fn app() -> App<'static, 'static> {
    App::new(BASE)
        .about("Fetch an OAuth token from Fitbit")
        .arg(Arg::with_name("client_id")
             .help("The client ID of your personal app")
             .required(true))
        .arg(Arg::with_name("client_secret")
             .help("The client secret of your personal app")
             .required(true))
}

pub(super) fn get_command(matches: &ArgMatches) -> Command {
    let id = matches.value_of("client_id").unwrap().to_string();
    let secret = matches.value_of("client_secret").unwrap().to_string();
    Command::GetAuthToken(id, secret)
}
