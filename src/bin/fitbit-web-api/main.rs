//! Fitbit Web API client

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
    GetBodyFatLogs,
    GetBodyGoals,
    GetBodyTimeSeries,
    GetBodyWeightLogs,
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
        Command::GetBodyFatLogs => client::body::get_fat_logs(),
        Command::GetBodyGoals => client::body::get_goals(),
        Command::GetBodyTimeSeries => client::body::get_time_series(),
        Command::GetBodyWeightLogs => client::body::get_weight_logs(),
        Command::GetDevices => client::get_devices(),
        Command::GetHrTimeSeries => client::heart_rate::get_time_series(),
        Command::GetHrIntradayTimeSeries => client::heart_rate::get_intraday_time_series(),
        Command::GetProfile => client::get_profile(),
        Command::GetSleepGoal => client::sleep::get_goal(),
        Command::GetSleepList => client::sleep::get_list(),
        Command::GetSleepLog => client::sleep::get_log(),
    }
}
