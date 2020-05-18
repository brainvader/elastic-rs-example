use serde::Deserialize;

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

// https://www.elastic.co/guide/en/elasticsearch/reference/7.7/docs-index_.html#docs-index-api-response-body
#[derive(Deserialize, Debug)]
struct IndexResponseShard {
    failed: i64,
    successful: i64,
    total: i64,
}

#[derive(Deserialize, Debug)]
pub struct IndexResponseBody {
    #[serde(alias = "_index")]
    index: String,
    #[serde(alias = "_type")]
    r#type: String,
    #[serde(alias = "_id")]
    id: String,
    #[serde(alias = "_version")]
    version: i64,
    result: String,
    #[serde(alias = "_shards")]
    shards: IndexResponseShard,
    #[serde(alias = "_seq_no")]
    seq_no: i64,
    #[serde(alias = "_primary_term")]
    primary_term: i64,
}
