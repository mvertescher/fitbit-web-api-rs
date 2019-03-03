//! Fitbit Web API client

#![deny(warnings)]

mod args;
mod client;

/// Possible commands for the client to execute.
#[derive(PartialEq)]
pub(crate) enum Command {
    GetActivityGoals,
    GetActivityLifetimeStats,
    GetActivitySummary,
    GetActivityTimeSeries(fitbit_web_api::activity::time_series::Resource),
    GetAuthToken(String, String),
    GetBadges,
    GetDevices,
    GetHrIntradayTimeSeries,
    GetHrTimeSeries,
    GetProfile,
    GetSleepGoal,
    GetSleepList,
    GetSleepLog,
}

fn main() {
    pretty_env_logger::init();
    let command = args::parse();

    match command {
        Command::GetActivityGoals => client::activity::get_goals(),
        Command::GetActivityLifetimeStats => client::activity::get_lifetime_stats(),
        Command::GetActivitySummary => client::activity::get_summary(),
        Command::GetActivityTimeSeries(resource) => client::activity::get_time_series(resource),
        Command::GetAuthToken(id, secret) => client::get_auth_token(id, secret),
        Command::GetBadges => client::get_badges(),
        Command::GetDevices => client::get_devices(),
        Command::GetHrTimeSeries => client::get_hr_time_series(),
        Command::GetHrIntradayTimeSeries => client::get_hr_intraday_time_series(),
        Command::GetProfile => client::get_profile(),
        _ => panic!("unsupported command"),
    }
}
