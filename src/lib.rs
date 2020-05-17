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
