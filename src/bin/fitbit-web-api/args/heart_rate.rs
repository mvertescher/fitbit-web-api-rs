//! Argument parsing

use crate::Command;

use clap::{App, AppSettings, ArgMatches, SubCommand};

pub(super) const BASE: &'static str = "hr";
pub(super) const BASE_TS: &'static str = "ts";
pub(super) const BASE_INTRADAY: &'static str = "intraday";

pub(super) fn app() -> App<'static, 'static> {
    App::new(BASE)
        .about("Heart rate data commands")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name(BASE_TS).about("Print the user's heart rate time series data"),
        )
        .subcommand(
            SubCommand::with_name(BASE_INTRADAY)
                .about("Print the user's intraday heart rate time series data"),
        )
}

pub(super) fn get_command(matches: &ArgMatches) -> Command {
    match matches.subcommand() {
        (BASE_TS, _) => Command::GetHrTimeSeries,
        (BASE_INTRADAY, _) => Command::GetHrIntradayTimeSeries,
        ("", None) => super::invalid_command_exit(),
        _ => unreachable!(),
    }
}
