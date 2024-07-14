use std::path::PathBuf;

use clap::{
    Parser,
    Args,
    Subcommand,
};


#[derive(Debug, Parser)]
#[clap(version="0.0.0 BETA", author="D.A.", about="multi purpose alarm program")]
pub struct Argument {
    #[clap(subcommand)]
    pub action: Action,

    #[clap(short='c', long="config", global=true)]
    pub config: Option<PathBuf>,

    #[clap(long, global=true)]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    Set(SetOpts),
    Start(StartOpts),
    Stop(StopOpts),
    Del(DelOpts),
    DelAll,
}

#[derive(Debug, Args)]
pub struct SetOpts {
    pub alarm_name: String,

    pub alarm_type: String,

    #[clap(short='r', long="refresh", global=true)]
    pub refresh: i64,

    #[clap(short='P', long, global=true)]
    pub prohibit_autostart: bool,
}

#[derive(Debug, Args)]
pub struct StartOpts {
    pub alarm_name: String,
}

#[derive(Debug, Args)]
pub struct StopOpts {
    pub alarm_name: String,
}

#[derive(Debug, Args)]
pub struct DelOpts {
    pub alarm_name: String,
}

