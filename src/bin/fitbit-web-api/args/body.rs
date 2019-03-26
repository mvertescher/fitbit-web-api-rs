//! Argument parsing for body commands

use crate::Command;

use clap::{App, ArgMatches, SubCommand, AppSettings};

pub(super) const BASE: &'static str = "body";
pub(super) const BASE_FAT_LOGS: &'static str = "fat";
pub(super) const BASE_GOALS: &'static str = "log";
pub(super) const BASE_TIME_SERIES: &'static str = "ts";
pub(super) const BASE_WEIGHT_LOGS: &'static str = "weight";

pub(super) fn app() -> App<'static, 'static> {
    App::new(BASE)
        .about("Sleep data commands")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name(BASE_FAT_LOGS)
            .about("Print the user's body fat logs"))
        .subcommand(SubCommand::with_name(BASE_GOALS)
            .about("Print the user's body goals"))
        .subcommand(SubCommand::with_name(BASE_TIME_SERIES)
            .about("Print the user's body time series data"))
        .subcommand(SubCommand::with_name(BASE_WEIGHT_LOGS)
            .about("Print the user's body weight logs"))
}

pub(super) fn get_command(matches: &ArgMatches) -> Command {
    match matches.subcommand() {
        (BASE_FAT_LOGS, _) => Command::GetBodyFatLogs,
        (BASE_GOALS, _) => Command::GetBodyGoals,
        (BASE_TIME_SERIES, _) => Command::GetBodyTimeSeries,
        (BASE_WEIGHT_LOGS, _) => Command::GetBodyWeightLogs,
        ("", None) => super::invalid_command_exit(),
        _ => unreachable!(),
    }
}
