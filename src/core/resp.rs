use serde_json::Value;
use std::time::Duration;
use super::req::Req;
use reqwest::{Response, Version};

#[derive(Debug)]
pub struct Resp {
    pub response: Response,
    pub duration: Duration,
    pub req: Req,
    text: Option<String>,
    value: Option<Value>,
}

impl Resp {

    pub fn new(response: Response, duration: Duration, req: Req) -> Self {
        Self {response, duration, req, text: None, value: None}
    }

    pub fn value(&mut self) -> Result<Value, String> {
        if self.value.is_some() {
            return Ok(self.value.clone().unwrap());
        }

        let text = self.text();
        match serde_json::from_str(text.as_str()) {
            Ok(value) => Ok(value),
            Err(_) => Err(html2text::from_read(text.as_bytes(), text.len())),
        }
    }

    pub fn text(&mut self) -> String {
        if self.text.is_none() {
            self.text.replace(self.response.text().unwrap());
        }
        self.text.clone().unwrap()
    }

    pub fn result(&mut self) -> Result<Value, String> {
        match self.value() {
            Ok(value) => match value.is_object() {
                true => match value.as_object().unwrap().contains_key("result") {
                    true => Ok(value["result"].clone()),
                    false => Err(serde_json::to_string_pretty(&value).unwrap()),
                },
                false => Err(serde_json::to_string_pretty(&value).unwrap()),
            },
            Err(value) => Err(value)
        }
    }

    // TODO buffer file type should be switched between json and text before write to buf
    pub fn result_lines(&mut self) -> Vec<String> {
        match self.value() {
            Ok(value) => serde_json::to_string_pretty(&value).unwrap(),
            Err(text) => text,
        }.split("\n").map(|a|String::from(a)).collect()
    }

    pub fn detail_lines(&mut self) -> Vec<String> {
        let mut lines = Vec::new();

        lines.push(format!("Duration: {}ms", self.duration.as_millis()));
        lines.push("".into());

        lines.push(format!("Status: {}", self.response.status()));
        lines.push("".into());

        lines.push(format!("HTTP: v{}", match self.response.version() {
            Version::HTTP_09 => "0.9",
            Version::HTTP_10 => "1.0",
            Version::HTTP_11 => "1.1",
            Version::HTTP_2 => "2.0",
        }));
        lines.push("".into());

        lines.push(format!("URL: {}", self.response.url()));
        lines.push("".into());

        lines.push("Resp headers:".into());
        for (key, value) in self.response.headers().into_iter() {
            lines.push(format!("    {}: {}", key, value.to_str().unwrap_or("<ERR_>")));
        }
        lines.push("".into());

        lines.push("Request body:".into());
        for jp in self.req_str().split("\n") {
            lines.push(format!("    {}", jp));
        }

        lines.push("Raw response body:".into());
        for jp in self.text().split("\n") {
            lines.push(format!("    {}", jp));
        }

        lines
    }

    fn req_str(&self) -> String {
        serde_json::to_string_pretty(&self.req).unwrap()
    }
}
