use std::{collections::BTreeSet, path::PathBuf};

use clap::{command, Parser};
use serde::{Deserialize, Serialize};

pub mod educator_model;

/// A model for describing users of the tool.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    /// User's name. Should be full, because it will be written in the beginning of the letter.
    pub name: String,
    /// IDs of educators that user watches.
    pub watch_educators: BTreeSet<u32>,
    /// IDs of groups that user watches.
    pub watch_groups: BTreeSet<u32>,
    /// User's email address to which they will receive letters.
    pub email: String,
}

/// A model for describing ARGS of the tool.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to user.json, that provides the info about users who will receive notifications and the list of watched educators for each one of them.
    #[arg(long, value_name = "FILE", default_value = "users.json")]
    pub users_json_path: PathBuf,
    /// Path to config.json, that contains email sender configuration parameters.
    #[arg(long, value_name = "FILE", default_value = "config.json")]
    pub config_json_path: PathBuf,
    /// Path to previous_events.json, that contains the information about schedule state at the time of the last Geraltt's launch.
    #[arg(long, value_name = "FILE", default_value = "previous_events.json")]
    pub previous_events_json_path: PathBuf,
}

/// A model for describing configuration of the tool.
#[derive(Deserialize)]
pub struct Config {
    /// SMTP server address
    pub email_relay: String,
    /// Email address from which the letters will be sent
    pub email_sender_username: String,
    /// Email sender display name, that will be shown in the letter
    pub email_sender_fullname: String,
    /// Password for email account from which the letters will be sent
    pub email_sender_password: String,
}
