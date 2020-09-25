//! Argument parsing for sleep commands

use crate::Command;

use clap::{App, AppSettings, ArgMatches, SubCommand};

pub(super) const BASE: &'static str = "sleep";
pub(super) const BASE_GOAL: &'static str = "goal";
pub(super) const BASE_LIST: &'static str = "list";
pub(super) const BASE_LOG: &'static str = "log";

pub(super) fn app() -> App<'static, 'static> {
    App::new(BASE)
        .about("Sleep data commands")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name(BASE_GOAL).about("Print the user's current sleep goal"))
        .subcommand(SubCommand::with_name(BASE_LIST).about("Print the user's list of sleep logs"))
        .subcommand(
            SubCommand::with_name(BASE_LOG).about("Print the user's sleep log for the past week"),
        )
}

pub(super) fn get_command(matches: &ArgMatches) -> Command {
    match matches.subcommand() {
        (BASE_GOAL, _) => Command::GetSleepGoal,
        (BASE_LIST, _) => Command::GetSleepList,
        (BASE_LOG, _) => Command::GetSleepLog,
        ("", None) => super::invalid_command_exit(),
        _ => unreachable!(),
    }
}
