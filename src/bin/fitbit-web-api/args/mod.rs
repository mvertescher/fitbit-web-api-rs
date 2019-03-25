//! Argument parsing

use super::Command;

use clap::{crate_name, crate_description, crate_version, crate_authors};
use clap::{App, SubCommand};

mod activity;
mod auth;
mod body;
mod heart_rate;
mod sleep;

pub(super) fn parse() -> super::Command {
    const BADGES: &'static str = "badges";
    const DEVICES: &'static str = "devices";
    const PROFILE: &'static str = "profile";

    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(activity::app())
        .subcommand(auth::app())
        .subcommand(SubCommand::with_name(BADGES)
                    .about("Prints a list of user badges"))
        .subcommand(body::app())
        .subcommand(SubCommand::with_name("devices")
                    .about("Prints a list of currently paired devices"))
        .subcommand(heart_rate::app())
        .subcommand(SubCommand::with_name(PROFILE)
                    .about("Print user profile information"))
        .subcommand(sleep::app())
        .get_matches();

    match matches.subcommand() {
        (activity::BASE, Some(m)) => activity::get_command(m),
        (auth::BASE, Some(m)) => auth::get_command(m),
        (BADGES, Some(_)) => Command::GetBadges,
        (DEVICES, Some(_)) => Command::GetDevices,
        (heart_rate::BASE, Some(m)) => heart_rate::get_command(m),
        (PROFILE, Some(_)) => Command::GetProfile,
        (sleep::BASE, Some(m)) => sleep::get_command(m),
        ("", None) => invalid_command_exit(),
        _ => unreachable!(),
    }
}

pub(crate) fn invalid_command_exit() -> ! {
    eprintln!("Please enter a valid command! See `-h` for more info!");
    std::process::exit(1);
}
