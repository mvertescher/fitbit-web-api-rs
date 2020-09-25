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

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let command = args::parse();

    match command {
        Command::GetActivityGoals => client::activity::get_goals().await,
        Command::GetActivityLifetimeStats => client::activity::get_lifetime_stats().await,
        Command::GetActivitySummary => client::activity::get_summary().await,
        Command::GetActivityTimeSeries(resource) => client::activity::get_time_series(resource).await,
        Command::GetAuthToken(id, secret) => client::get_auth_token(id, secret),
        Command::GetBadges => client::get_badges().await,
        Command::GetBodyFatLogs => client::body::get_fat_logs(),
        Command::GetBodyGoals => client::body::get_goals(),
        Command::GetBodyTimeSeries => client::body::get_time_series(),
        Command::GetBodyWeightLogs => client::body::get_weight_logs(),
        Command::GetDevices => client::get_devices().await,
        Command::GetHrTimeSeries => client::heart_rate::get_time_series().await,
        Command::GetHrIntradayTimeSeries => client::heart_rate::get_intraday_time_series().await,
        Command::GetProfile => client::get_profile().await,
        Command::GetSleepGoal => client::sleep::get_goal().await,
        Command::GetSleepList => client::sleep::get_list().await,
        Command::GetSleepLog => client::sleep::get_log().await,
    }
}
