use serde::{Deserialize, Serialize};
use std::io::Read;

const ENDPOINT: &str = "https://generativelanguage.googleapis.com";

pub struct PalmClient {
    api_key: String,
    endpoint: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub name: String,
    pub version: String,
    pub display_name: String,
    pub description: String,
    pub input_token_limit: u32,
    pub output_token_limit: u32,
    pub supported_generation_methods: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListRes {
    models: Vec<Model>,
}

impl PalmClient {
    fn fetch_models(
        &self,
    ) -> Result<(reqwest::blocking::Response, String), Box<dyn std::error::Error>> {
        let mut res = reqwest::blocking::get(format!(
            "{}/v1beta2/models?key={}",
            self.endpoint, self.api_key
        ))?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok((res, body))
    }

    fn parse_body(&self, body: String) -> serde_json::Result<ListRes> {
        let parsed_body = serde_json::from_str(&body.as_str())?;
        Ok(parsed_body)
    }

    pub fn list_models(&self) -> Result<Vec<Model>, Box<dyn std::error::Error>> {
        let (res, body) = self.fetch_models().expect("msg");
        match res.status() {
            reqwest::StatusCode::OK => {
                let parsed_body = self.parse_body(body)?;
                return Ok(parsed_body.models);
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                panic!("API Key Invalid")
            }
            reqwest::StatusCode::FORBIDDEN => {
                panic!("API Key Invalid")
            }
            other => {
                panic!("Something unexpected happened: {}", other)
            }
        };
    }
}

pub fn create_client(api_key: String) -> PalmClient {
    PalmClient {
        api_key: api_key,
        endpoint: String::from(ENDPOINT),
    }
}
