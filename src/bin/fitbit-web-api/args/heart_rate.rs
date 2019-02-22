//! Argument parsing

use clap::{App, SubCommand, AppSettings};

pub(super) const BASE: &'static str = "hr";
pub(super) const BASE_TS: &'static str = "ts";
pub(super) const BASE_INTRADAY: &'static str = "intraday";

pub(super) fn app() -> App<'static, 'static> {
    App::new(BASE)
        .about("Heart rate data commands")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name(BASE_TS)
            .about("Print the user's heart rate time series data"))
        .subcommand(SubCommand::with_name(BASE_INTRADAY)
            .about("Prin the user's intraday heart rate time series data"))
}
