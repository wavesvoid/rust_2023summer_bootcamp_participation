use crate::config::LogLevel;


pub  fn mode_debug() -> bool { false }

pub  fn server_external_url() -> String { "http://127.0.0.1".to_owned() }
pub  fn server_http_port() -> usize { 8081 }
pub  fn server_grpc_port() -> usize { 8082 }
pub  fn server_healthz_port() -> usize { 10025 }
pub  fn server_metrics_port() -> usize { 9199 }

pub  fn mysql_host() -> String { "127.0.0.1".into() }
pub  fn mysql_port() -> usize { 3306 }
pub  fn mysql_user() -> String { "root".into() }
pub  fn mysql_dating() -> String { "default".into() }
pub  fn dbconn_max_conn() -> usize { 30 }

pub  fn applog_level() -> LogLevel { LogLevel::INFO }

pub  fn watchdog_period() -> String { "5s".into() }
pub  fn watchdog_limit() -> usize { 10 }
pub  fn watchdog_lock_timeout() -> String { "4s".into() }