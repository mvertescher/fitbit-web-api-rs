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
    GetProfile,
}

fn main() {
    let command = args::parse();

    match command {
        Command::GetActivityGoals => client::get_activity_goals(),
        Command::GetActivityLifetimeStats => client::get_activity_lifetime_stats(),
        Command::GetActivitySummary => client::get_activity_summary(),
        Command::GetActivityTimeSeries(resource) => client::get_activity_time_series(resource),
        Command::GetAuthToken(id, secret) => client::get_auth_token(id, secret),
        Command::GetBadges => client::get_badges(),
        Command::GetDevices => client::get_devices(),
        Command::GetProfile => client::get_profile(),
    }
}
