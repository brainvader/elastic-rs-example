# Examples for elasticsearch-rs

This is my private project for [elasticsearch-rs](https://github.com/elastic/elasticsearch-rs).

## What is elasticserch-rs

It is an offical client for Elasticsearch.

## Usage

Before running a example, you need start your elasticsearch (es) server up and running.

```bash
cargo make up
```

It's also better to check the status of es server.

```bash
http://127.0.0.1:9200/_cat/health?v
```

in browser serchbar or enter the following command;

```bash
 curl -X GET "localhost:9200/_cat/health?v"
```

Is it running your server? It's time to play with any examples you want.

```bash
# run ex1-cat-call.rs
cargo make example ex1-cat-call.rs
```

## Clean-up

If you no need the es server, you should stop docker instance.

```bash
cargo make down
```

## Examples

examples correspond to [Elasticsearch Reference](https://www.elastic.co/guide/en/elasticsearch/reference/current/index.html) sections.

### ex2-index-document.rs

[Index some documents](https://www.elastic.co/guide/en/elasticsearch/reference/current/getting-started-index.html)
