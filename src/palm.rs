const ENDPOINT: String = String::from("https://generativelanguage.googleapis.com");

pub struct PalmClient {
    api_key: String,
    endpoint: String,
}

impl PalmClient {
    pub fn list_models(&self) -> Result<()> {
        let body = reqwest::blocking::get(format!(
            "{}/v1beta2/models?key={}",
            self.endpoint, self.api_key
        ))?
        .text()?;
    }
}

pub fn create_client(api_key: String) -> PalmClient {
    PalmClient {
        api_key: api_key,
        endpoint: ENDPOINT,
    }
}
