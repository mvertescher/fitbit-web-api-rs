//! Argument parsing

use super::Command;

use clap::{crate_name, crate_description, crate_version, crate_authors};
use clap::{App, Arg, SubCommand, AppSettings};

pub(super) fn parse() -> super::Command {
    const ACTIVITY: &'static str = "activity";
    const ACTIVITY_GOALS: &'static str = "goals";
    const ACTIVITY_LIFETIME_STATS: &'static str = "lifetime_stats";
    const ACTIVITY_SUMMARY: &'static str = "summary";
    const ACTIVITY_TS: &'static str = "time_series";
    const ACTIVITY_TS_CALORIES: &'static str = "calories";
    const ACTIVITY_TS_CALORIES_BMR: &'static str = "calories_bmr";
    const ACTIVITY_TS_STEPS: &'static str = "steps";
    const ACTIVITY_TS_DISTANCE: &'static str = "distance";
    const ACTIVITY_TS_FLOORS: &'static str = "floors";
    const ACTIVITY_TS_ELEVATION: &'static str = "elevation";
    const ACTIVITY_TS_SEDENTARY: &'static str = "sedentary";
    const ACTIVITY_TS_LIGHTLY_ACTIVE: &'static str = "lightly_active";
    const ACTIVITY_TS_FAIRLY_ACTIVE: &'static str = "fairly_active";
    const ACTIVITY_TS_VERY_ACTIVE: &'static str = "very_active";
    const ACTIVITY_TS_ACTIVITY_CALORIES: &'static str = "activity_calories";
    const AUTH: &'static str = "auth";
    const BADGES: &'static str = "badges";
    const DEVICES: &'static str = "devices";
    const PROFILE: &'static str = "profile";

    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(SubCommand::with_name(ACTIVITY)
                    .about("User activity data commands")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommand(SubCommand::with_name(ACTIVITY_GOALS)
                        .about("Print a summary of the user's activity goals"))
                    .subcommand(SubCommand::with_name(ACTIVITY_LIFETIME_STATS)
                        .about("Print a summary of the user's lifetime activity statistics"))
                    .subcommand(SubCommand::with_name(ACTIVITY_SUMMARY)
                        .about("Print a summary of the user's recent activities"))
                    .subcommand(SubCommand::with_name(ACTIVITY_TS)
                        .about("User activity time series data commands")
                        .setting(AppSettings::SubcommandRequiredElseHelp)
                        .subcommand(SubCommand::with_name(ACTIVITY_TS_CALORIES)
                            .about("Print calories time series data"))
                        .subcommand(SubCommand::with_name(ACTIVITY_TS_CALORIES_BMR)
                            .about("Print calories BMR time series data"))
                        .subcommand(SubCommand::with_name(ACTIVITY_TS_STEPS)
                            .about("Print steps time series data"))
                        .subcommand(SubCommand::with_name(ACTIVITY_TS_DISTANCE)
                            .about("Print distance time series data"))
                        .subcommand(SubCommand::with_name(ACTIVITY_TS_FLOORS)
                            .about("Print floors time series data"))
                        .subcommand(SubCommand::with_name(ACTIVITY_TS_ELEVATION)
                            .about("Print elevation time series data"))
                        .subcommand(SubCommand::with_name(ACTIVITY_TS_SEDENTARY)
                            .about("Print minutes sedentary time series data"))
                        .subcommand(SubCommand::with_name(ACTIVITY_TS_LIGHTLY_ACTIVE)
                            .about("Print lightly active time series data"))
                        .subcommand(SubCommand::with_name(ACTIVITY_TS_FAIRLY_ACTIVE)
                            .about("Print fairly active time series data"))
                        .subcommand(SubCommand::with_name(ACTIVITY_TS_VERY_ACTIVE)
                            .about("Print very active time series data"))
                        .subcommand(SubCommand::with_name(ACTIVITY_TS_ACTIVITY_CALORIES)
                            .about("Print activity calories time series data"))
                        )
                    )
        .subcommand(SubCommand::with_name(AUTH)
                    .about("Fetch an OAuth token from Fitbit")
                    .arg(Arg::with_name("client_id")
                         .help("The client ID of your personal app")
                         .required(true))
                    .arg(Arg::with_name("client_secret")
                         .help("The client secret of your personal app")
                         .required(true)))
        .subcommand(SubCommand::with_name(BADGES)
                    .about("Prints a list of user badges"))
        .subcommand(SubCommand::with_name("devices")
                    .about("Prints a list of currently paired devices"))
        .subcommand(SubCommand::with_name(PROFILE)
                    .about("Print user profile information"))
        .get_matches();

    match matches.subcommand() {
        (ACTIVITY, Some(activity_matches)) => {
            match activity_matches.subcommand() {
                (ACTIVITY_GOALS, Some(_)) => Command::GetActivityGoals,
                (ACTIVITY_LIFETIME_STATS, Some(_)) => Command::GetActivityLifetimeStats,
                (ACTIVITY_SUMMARY, Some(_)) => Command::GetActivitySummary,
                (ACTIVITY_TS, Some(_activity_ts_matches)) => invalid_command_exit(),
                ("", None) => invalid_command_exit(),
                _ => unreachable!(),
            }
        }
        (AUTH, Some(auth_matches)) => {
            let id = auth_matches.value_of("client_id").unwrap().to_string();
            let secret = auth_matches.value_of("client_secret").unwrap().to_string();
            Command::GetAuthToken(id, secret)
        }
        (BADGES, Some(_)) => Command::GetBadges,
        (DEVICES, Some(_)) => Command::GetDevices,
        (PROFILE, Some(_)) => Command::GetProfile,
        ("", None) => invalid_command_exit(),
        _ => unreachable!(),
    }
}

fn invalid_command_exit() -> ! {
    eprintln!("Please enter a valid command! See `-h` for more info!");
    std::process::exit(1);
}
