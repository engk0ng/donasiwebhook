use serde::Deserialize;
use slog::{o, Drain, Logger};
use slog_async;
use slog_envlogger;
use slog_term;

#[derive(Deserialize)]
pub struct Config;

impl Config {
    pub fn configure_log() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_envlogger::new(console_drain);
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
    }
}