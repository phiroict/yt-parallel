use chrono::Local;
use env_logger::Builder;
use log::{LevelFilter, trace};
use std::{io::Write, str::FromStr};
use uuid::Uuid;

pub fn initialize_logging(log_level: String) {
    let request_id = Uuid::new_v4();
    let level = LevelFilter::from_str(log_level.as_str()).unwrap_or(LevelFilter::Trace);
    Builder::new()
        .format(move |buf, record| {
            writeln!(
                buf,
                "{} [{}] '{}' - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                &request_id,
                record.args()
            )
        })
        .filter(None, level)
        .init();
    trace!("Logging system set up.")
}
