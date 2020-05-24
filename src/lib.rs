use serde::Deserialize;

pub mod api;

use std::io::Write;

use env_logger::{DEFAULT_FILTER_ENV, DEFAULT_WRITE_STYLE_ENV};

// Cluster node information
// https://www.elastic.co/guide/en/elasticsearch/reference/current/cat-nodes.html
#[derive(Deserialize, Debug)]
pub struct ESNode {
    ip: String,
    port: String,
    #[serde(alias = "heapPercent")]
    heap_percent: String,
    name: String,
}

pub fn setup_logger() -> dotenv::Result<()> {
    let log_level: String = dotenv::var(DEFAULT_FILTER_ENV)?;
    let log_style: String = dotenv::var(DEFAULT_WRITE_STYLE_ENV)?;
    std::env::set_var(DEFAULT_FILTER_ENV, log_level);
    std::env::set_var(DEFAULT_WRITE_STYLE_ENV, log_style);
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .format_indent(Some(4))
        .format(
            |buf: &mut env_logger::fmt::Formatter, record: &log::Record| {
                let ts = buf.timestamp();
                writeln!(buf, "[{}] {}", ts, record.level())?;
                writeln!(buf, "{}", record.args())
            },
        )
        // .write_style(env_logger::WriteStyle::Auto)
        .init();
    Ok(())
}
