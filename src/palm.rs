use std::io::Read;

const ENDPOINT: &str = "https://generativelanguage.googleapis.com";

pub struct PalmClient {
    api_key: String,
    endpoint: String,
}

impl PalmClient {
    pub fn list_models(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut res = reqwest::blocking::get(format!(
            "{}/v1beta2/models?key={}",
            self.endpoint, self.api_key
        ))?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;

        println!("{}",body);

        Ok(())
    }
}

pub fn create_client(api_key: String) -> PalmClient {
    PalmClient {
        api_key: api_key,
        endpoint: String::from(ENDPOINT),
    }
}
