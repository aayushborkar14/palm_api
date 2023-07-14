use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;

const ENDPOINT: &str = "https://generativelanguage.googleapis.com";

#[derive(Serialize, Deserialize, Debug)]
struct ListRes {
    models: Vec<HashMap<String, String>>,
}

pub struct PalmClient {
    api_key: String,
    endpoint: String,
}

impl PalmClient {
    fn fetch_models(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut res = reqwest::blocking::get(format!(
            "{}/v1beta2/models?key={}",
            self.endpoint, self.api_key
        ))?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok(body)
    }
    pub fn list_models(&self) -> serde_json::Result<()> {
        let body = self.fetch_models().expect("msg");
        println!("{}",body);
        // let parsed_body = serde_json::from_str(&body.as_str())?;
        // println!("{}",parsed_body.models.len());
        Ok(())
    }
}

pub fn create_client(api_key: String) -> PalmClient {
    PalmClient {
        api_key: api_key,
        endpoint: String::from(ENDPOINT),
    }
}
