use serde::Deserialize;
use clap::Parser;
use config::{Config, ConfigError, Environment, File};

use crate::cli;



#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct Mode {
    #[serde(default)]
    debug: bool,
}


#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct Server {
    #[serde(default = "crate::serde_defaults::server_external_url")]
    external_url: String,

    #[serde(default = "crate::serde_defaults::server_http_port")]
    http_port: usize,

    #[serde(default = "crate::serde_defaults::server_grpc_port")]
    grpc_port: usize,

    #[serde(default = "crate::serde_defaults::server_healthz_port")]
    healthz_port: usize,

    #[serde(default = "crate::serde_defaults::server_metrics_port")]
    metrics_port: usize,
}


#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct Db {
    mysql: Mysql,
}

#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct Mysql {
    #[serde(default = "crate::serde_defaults::mysql_host")]
    host: String,

    #[serde(default = "crate::serde_defaults::mysql_port")]
    port: usize,
    
    #[serde(default = "crate::serde_defaults::mysql_user")]
    user: String,
    
    #[serde(default)]
    pass: String,

    #[serde(default = "crate::serde_defaults::mysql_dating")]
    dating: String,
    connections: DbConnections,
}

#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct DbConnections {
    #[serde(default = "crate::serde_defaults::dbconn_max_conn")]
    max_idle: usize,

    #[serde(default = "crate::serde_defaults::dbconn_max_conn")]
    max_open: usize,
}


#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct Log {
    app: AppLog,
}

#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct AppLog {
    #[serde(default = "crate::serde_defaults::applog_level")]
    level: LogLevel,
}
#[derive(Debug, Deserialize, Default)]
pub enum LogLevel {
    ERROR,
    WARN,
    #[default]
    INFO,
    DEBUG,
    TRACE,
}


#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct Background {
    watchdog: Watchdog,
}


// WILL THERE BE REMARKS CONSIDERING proper types and validation?????????????????
#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct Watchdog {
    #[serde(default = "crate::serde_defaults::watchdog_period")]
    period: String,

    #[serde(default = "crate::serde_defaults::watchdog_limit")]
    limit: usize,

    #[serde(default = "crate::serde_defaults::watchdog_lock_timeout")]
    lock_timeout: String,
}



#[derive(Debug, Deserialize, Default)]
// #[serde(deny_unknown_fields, default)]
#[allow(unused)]
pub struct PrintableSettings {
    #[serde(default)]
    mode: Mode,

    #[serde(default)]
    server: Server,

    #[serde(default)]
    db: Db,
    
    #[serde(default)]
    log: Log,

    #[serde(default)]
    background: Background,
}

impl PrintableSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let from_cli = cli::CliArgs::parse();

        let k = 12;
        let mut c = Config::builder()
            .add_source(File::with_name(&from_cli.conf).required(false))
            .set_override_option("mode.debug", from_cli.debug.into())?
            .add_source(Environment::with_prefix("CONF").separator("_"));
        
        c.build()?.try_deserialize()
    }
}