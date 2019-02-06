//! Argument parsing

use super::Command;

use clap::{crate_name, crate_description, crate_version, crate_authors};
use clap::{App, Arg, SubCommand, AppSettings};

pub(super) fn parse() -> super::Command {
    const ACTIVITY: &'static str = "activity";
    const ACTIVITY_SUMMARY: &'static str = "summary";
    const AUTH: &'static str = "auth";
    const BADGES: &'static str = "badges";
    const DEVICES: &'static str = "devices";
    const PROFILE: &'static str = "profile";

    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(SubCommand::with_name(ACTIVITY)
                    .about("User activity data commands")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommand(SubCommand::with_name(ACTIVITY_SUMMARY)
                        .about("Print a summary of the user's recent activities")))
        .subcommand(SubCommand::with_name(AUTH)
                    .about("Fetch an OAuth token from Fitbit")
                    .arg(Arg::with_name("client_id")
                         .help("The client ID of your personal app")
                         .required(true))
                    .arg(Arg::with_name("client_secret")
                         .help("The client secret of your personal app")
                         .required(true)))
        .subcommand(SubCommand::with_name(BADGES)
                    .about("Prints a list of user badges"))
        .subcommand(SubCommand::with_name("devices")
                    .about("Prints a list of currently paired devices"))
        .subcommand(SubCommand::with_name(PROFILE)
                    .about("Print user profile information"))
        .get_matches();

    match matches.subcommand() {
        (ACTIVITY, Some(activity_matches)) => {
            match activity_matches.subcommand() {
                (ACTIVITY_SUMMARY, Some(_)) => Command::GetActivitySummary,
                ("", None) => invalid_command_exit(),
                _ => unreachable!(),
            }
        }
        (AUTH, Some(auth_matches)) => {
            let id = auth_matches.value_of("client_id").unwrap().to_string();
            let secret = auth_matches.value_of("client_secret").unwrap().to_string();
            Command::GetAuthToken(id, secret)
        }
        (BADGES, Some(_)) => Command::GetBadges,
        (DEVICES, Some(_)) => Command::GetDevices,
        (PROFILE, Some(_)) => Command::GetProfile,
        ("", None) => invalid_command_exit(),
        _ => unreachable!(),
    }
}

fn invalid_command_exit() -> ! {
    eprintln!("Please enter a valid command! See `-h` for more info!");
    std::process::exit(1);
}
