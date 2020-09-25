//! Activity argument parsing

use crate::Command;

use clap::{App, AppSettings, ArgMatches, SubCommand};

pub(super) const BASE: &'static str = "activity";
pub(super) const BASE_GOALS: &'static str = "goals";
pub(super) const BASE_LIFETIME_STATS: &'static str = "lifetime_stats";
pub(super) const BASE_SUMMARY: &'static str = "summary";
pub(super) const BASE_TS: &'static str = "time_series";
pub(super) const BASE_TS_CALORIES: &'static str = "calories";
pub(super) const BASE_TS_CALORIES_BMR: &'static str = "calories_bmr";
pub(super) const BASE_TS_STEPS: &'static str = "steps";
pub(super) const BASE_TS_DISTANCE: &'static str = "distance";
pub(super) const BASE_TS_FLOORS: &'static str = "floors";
pub(super) const BASE_TS_ELEVATION: &'static str = "elevation";
pub(super) const BASE_TS_SEDENTARY: &'static str = "sedentary";
pub(super) const BASE_TS_LIGHTLY_ACTIVE: &'static str = "lightly_active";
pub(super) const BASE_TS_FAIRLY_ACTIVE: &'static str = "fairly_active";
pub(super) const BASE_TS_VERY_ACTIVE: &'static str = "very_active";
pub(super) const BASE_TS_ACTIVITY_CALORIES: &'static str = "activity_calories";

pub(super) fn app() -> App<'static, 'static> {
    App::new(BASE)
        .about("User activity data commands")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name(BASE_GOALS).about("Print a summary of the user's activity goals"),
        )
        .subcommand(
            SubCommand::with_name(BASE_LIFETIME_STATS)
                .about("Print a summary of the user's lifetime activity statistics"),
        )
        .subcommand(
            SubCommand::with_name(BASE_SUMMARY)
                .about("Print a summary of the user's recent activities"),
        )
        .subcommand(
            SubCommand::with_name(BASE_TS)
                .about("User activity time series data commands")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name(BASE_TS_CALORIES)
                        .about("Print calories time series data"),
                )
                .subcommand(
                    SubCommand::with_name(BASE_TS_CALORIES_BMR)
                        .about("Print calories BMR time series data"),
                )
                .subcommand(
                    SubCommand::with_name(BASE_TS_STEPS).about("Print steps time series data"),
                )
                .subcommand(
                    SubCommand::with_name(BASE_TS_DISTANCE)
                        .about("Print distance time series data"),
                )
                .subcommand(
                    SubCommand::with_name(BASE_TS_FLOORS).about("Print floors time series data"),
                )
                .subcommand(
                    SubCommand::with_name(BASE_TS_ELEVATION)
                        .about("Print elevation time series data"),
                )
                .subcommand(
                    SubCommand::with_name(BASE_TS_SEDENTARY)
                        .about("Print minutes sedentary time series data"),
                )
                .subcommand(
                    SubCommand::with_name(BASE_TS_LIGHTLY_ACTIVE)
                        .about("Print lightly active time series data"),
                )
                .subcommand(
                    SubCommand::with_name(BASE_TS_FAIRLY_ACTIVE)
                        .about("Print fairly active time series data"),
                )
                .subcommand(
                    SubCommand::with_name(BASE_TS_VERY_ACTIVE)
                        .about("Print very active time series data"),
                )
                .subcommand(
                    SubCommand::with_name(BASE_TS_ACTIVITY_CALORIES)
                        .about("Print activity calories time series data"),
                ),
        )
}

pub(super) fn get_command(matches: &ArgMatches) -> Command {
    match matches.subcommand() {
        (BASE_GOALS, Some(_)) => Command::GetActivityGoals,
        (BASE_LIFETIME_STATS, Some(_)) => Command::GetActivityLifetimeStats,
        (BASE_SUMMARY, Some(_)) => Command::GetActivitySummary,
        (BASE_TS, Some(activity_ts_matches)) => {
            use fitbit_web_api::activity::time_series::Resource;
            match activity_ts_matches.subcommand() {
                (BASE_TS_CALORIES, Some(_)) => Command::GetActivityTimeSeries(Resource::Calories),
                (BASE_TS_CALORIES_BMR, Some(_)) => {
                    Command::GetActivityTimeSeries(Resource::CaloriesBMR)
                }
                (BASE_TS_STEPS, Some(_)) => Command::GetActivityTimeSeries(Resource::Steps),
                (BASE_TS_DISTANCE, Some(_)) => Command::GetActivityTimeSeries(Resource::Distance),
                (BASE_TS_FLOORS, Some(_)) => Command::GetActivityTimeSeries(Resource::Floors),
                (BASE_TS_ELEVATION, Some(_)) => Command::GetActivityTimeSeries(Resource::Elevation),
                (BASE_TS_SEDENTARY, Some(_)) => Command::GetActivityTimeSeries(Resource::Sedentary),
                (BASE_TS_LIGHTLY_ACTIVE, Some(_)) => {
                    Command::GetActivityTimeSeries(Resource::LightlyActive)
                }
                (BASE_TS_FAIRLY_ACTIVE, Some(_)) => {
                    Command::GetActivityTimeSeries(Resource::FairlyActive)
                }
                (BASE_TS_VERY_ACTIVE, Some(_)) => {
                    Command::GetActivityTimeSeries(Resource::VeryActive)
                }
                (BASE_TS_ACTIVITY_CALORIES, Some(_)) => {
                    Command::GetActivityTimeSeries(Resource::ActivityCalories)
                }
                ("", None) => super::invalid_command_exit(),
                _ => unreachable!(),
            }
        }
        ("", None) => super::invalid_command_exit(),
        _ => unreachable!(),
    }
}
