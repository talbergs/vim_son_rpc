use reqwest::ClientBuilder;
use reqwest::Request;
use serde::Serialize;
use serde_json::Value;
use std::time::{Duration, Instant};
use super::resp::Resp;

#[derive(Debug, Serialize)]
pub struct Req {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: String,
    auth: Option<String>,
}

impl Req {
    pub fn new(method: String, params: Value, auth: Option<String>,) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            method,
            params,
            id: "1".into(),
            auth,
        }
    }

    pub fn send(self, url: &str) -> Resp {
        let serialized = serde_json::to_string(&self).unwrap();

        // TODO think about xdebug .. do we timeout then?
        let timeout = Duration::new(5, 0);
        let client = ClientBuilder::new()
            // TODO make this configurable
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true)
            .timeout(timeout)
            // .gzip(true)
            .build()
            .unwrap();

        let request = client
            .post(url)
            .header("Content-Type", "application/json")
            .body(serialized)
            .build()
            .unwrap();

        let now = Instant::now();

        let response = client.execute(request).unwrap();
        let duration = now.elapsed();

        Resp::new(response, duration, self)
    }
}
