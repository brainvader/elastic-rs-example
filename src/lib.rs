use serde::Deserialize;

pub mod api;

use std::io::Write;

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

pub const RUST_LOG: &'static str = "RUST_LOG";
pub const RUST_LOG_STYLE: &'static str = "RUST_LOG_STYLE";

pub fn setup_logger() -> dotenv::Result<()> {
    let log_level: String = dotenv::var(RUST_LOG)?;
    let log_style: String = dotenv::var(RUST_LOG_STYLE)?;
    std::env::set_var(RUST_LOG, log_level);
    std::env::set_var(RUST_LOG_STYLE, log_style);
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
