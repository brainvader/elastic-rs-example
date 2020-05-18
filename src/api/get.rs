use serde::Deserialize;

pub struct ResponseBody<T> {
    index: String,
    r#type: String,
    id: String,
    version: i64,
    seq_no: i64,
    primary_term: i64,
    source: T,
}
