use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read};

const ENDPOINT: &str = "https://generativelanguage.googleapis.com";

pub struct PalmClient {
    api_key: String,
    endpoint: String,
}

pub fn create_client(api_key: String) -> PalmClient {
    PalmClient {
        api_key: api_key,
        endpoint: ENDPOINT.to_string(),
    }
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
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListRes {
    models: Vec<Model>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TokenRes {
    token_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenBody {
    prompt: Messages,
}

#[derive(Serialize, Deserialize, Debug)]
struct Messages {
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EmbedBody {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EmbedRes {
    embedding: EmbedValue,
}

#[derive(Serialize, Deserialize, Debug)]
struct EmbedValue {
    value: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChatBody {
    prompt: MessagePrompt,
    temperature: f64,
    candidate_count: u32,
    top_p: f64,
    top_k: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Example {
    input: Message,
    output: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePrompt {
    context: String,
    examples: Vec<Example>,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentFilter {
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageRes {
    pub author: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub author: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRes {
    pub messages: Vec<MessageRes>,
    pub filters: Option<Vec<ContentFilter>>,
    pub candidates: Option<Vec<Candidate>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TextPrompt {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SafetySetting {
    pub category: String,
    pub threshold: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextBody {
    prompt: TextPrompt,
    safety_settings: Vec<SafetySetting>,
    stop_sequences: Vec<String>,
    temperature: f64,
    candidate_count: u32,
    max_output_tokens: u32,
    top_p: f64,
    top_k: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextCompletion {
    pub output: String,
    pub safety_ratings: Vec<SafetyRating>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SafetyFeedback {
    pub rating: SafetyRating,
    pub setting: SafetySetting,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextRes {
    pub candidates: Option<Vec<TextCompletion>>,
    pub filters: Option<Vec<ContentFilter>>,
    pub safety_feedback: Option<Vec<SafetyFeedback>>,
}

impl PalmClient {
    // functions for list_models
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

    fn parse_models(&self, body: String) -> serde_json::Result<ListRes> {
        let parsed_models = serde_json::from_str(&body.as_str())?;
        Ok(parsed_models)
    }

    pub fn list_models(&self) -> Result<Vec<Model>, Box<dyn std::error::Error>> {
        let (res, body) = self
            .fetch_models()
            .expect("Error occured while sending GET request");
        match res.status() {
            reqwest::StatusCode::OK => {
                let parsed_models = self.parse_models(body)?;
                return Ok(parsed_models.models);
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

    // functions for get_model
    fn fetch_model(
        &self,
        model: &String,
    ) -> Result<(reqwest::blocking::Response, String), Box<dyn std::error::Error>> {
        let mut res = reqwest::blocking::get(format!(
            "{}/v1beta2/models/{}?key={}",
            self.endpoint, model, self.api_key
        ))?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok((res, body))
    }

    fn parse_model(&self, body: String) -> serde_json::Result<Model> {
        let parsed_model = serde_json::from_str(&body.as_str())?;
        Ok(parsed_model)
    }

    pub fn get_model(&self, model: String) -> Result<Model, Box<dyn std::error::Error>> {
        let (res, body) = self
            .fetch_model(&model)
            .expect("Error occured while sending GET request");
        match res.status() {
            reqwest::StatusCode::OK => {
                let model = self.parse_model(body)?;
                return Ok(model);
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                panic!("API Key Invalid")
            }
            reqwest::StatusCode::FORBIDDEN => {
                panic!("API Key Invalid")
            }
            reqwest::StatusCode::NOT_FOUND => {
                return Err(format!("Model {} doesn't exist", &model).into());
            }
            other => {
                panic!("Something unexpected happened: {}", other)
            }
        };
    }

    // functions for count_message_tokens
    fn post_count_req(
        &self,
        model: &String,
        message_list: Vec<String>,
    ) -> Result<(reqwest::blocking::Response, String), Box<dyn std::error::Error>> {
        let mut messages_vec: Vec<Message> = Vec::new();
        for message_text in message_list {
            let message = Message {
                content: message_text,
            };
            messages_vec.push(message);
        }
        let messages = Messages {
            messages: messages_vec,
        };
        let token_body = TokenBody { prompt: messages };
        let client = reqwest::blocking::Client::new();
        let mut res = client
            .post(format!(
                "{}/v1beta2/models/{}:countMessageTokens?key={}",
                self.endpoint, model, self.api_key
            ))
            .json(&token_body)
            .send()?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok((res, body))
    }

    fn parse_token(&self, body: String) -> serde_json::Result<TokenRes> {
        let parsed_token = serde_json::from_str(&body.as_str())?;
        Ok(parsed_token)
    }

    pub fn count_message_tokens(
        &self,
        model: String,
        message_list: Vec<String>,
    ) -> Result<u32, Box<dyn std::error::Error>> {
        let (res, body) = self
            .post_count_req(&model, message_list)
            .expect("Error occured while sending POST request");
        match res.status() {
            reqwest::StatusCode::OK => {
                let parsed_token = self.parse_token(body)?;
                return Ok(parsed_token.token_count);
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                panic!("API Key Invalid")
            }
            reqwest::StatusCode::FORBIDDEN => {
                panic!("API Key Invalid")
            }
            reqwest::StatusCode::NOT_FOUND => {
                return Err(format!("Model {} not supported", &model).into());
            }
            reqwest::StatusCode::BAD_REQUEST => {
                return Err("Message not found".to_string().into());
            }
            other => {
                panic!("Something unexpected happened: {}", other)
            }
        };
    }

    // functions for generate_embeddings
    fn post_embed_req(
        &self,
        model: &String,
        text: String,
    ) -> Result<(reqwest::blocking::Response, String), Box<dyn std::error::Error>> {
        let embed_body = EmbedBody { text: text };
        let client = reqwest::blocking::Client::new();
        let mut res = client
            .post(format!(
                "{}/v1beta2/models/{}:embedText?key={}",
                self.endpoint, model, self.api_key
            ))
            .json(&embed_body)
            .send()?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok((res, body))
    }

    fn parse_embeddings(&self, body: String) -> serde_json::Result<EmbedRes> {
        let parsed_embeddings = serde_json::from_str(&body.as_str())?;
        Ok(parsed_embeddings)
    }

    pub fn generate_embeddings(
        &self,
        model: String,
        text: String,
    ) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let (res, body) = self
            .post_embed_req(&model, text)
            .expect("Error occured while sending POST request");
        match res.status() {
            reqwest::StatusCode::OK => {
                let parsed_embeddings = self.parse_embeddings(body)?;
                return Ok(parsed_embeddings.embedding.value);
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                panic!("API Key Invalid")
            }
            reqwest::StatusCode::FORBIDDEN => {
                panic!("API Key Invalid")
            }
            reqwest::StatusCode::NOT_FOUND => {
                return Err(format!("Model {} not supported", &model).into());
            }
            reqwest::StatusCode::BAD_REQUEST => {
                return Err("Message not found".to_string().into());
            }
            other => {
                panic!("Something unexpected happened: {}", other)
            }
        };
    }

    // functions for chat
    fn post_chat_req(
        &self,
        model: &String,
        message_prompt: MessagePrompt,
        config: HashMap<String, String>,
    ) -> Result<(reqwest::blocking::Response, String), Box<dyn std::error::Error>> {
        let temperature: f64;
        let candidate_count: u32;
        let top_p: f64;
        let top_k: u32;
        let model_info = self.get_model(model.to_string()).expect("err");
        if config.contains_key(&"temperature".to_string()) {
            temperature = match config["temperature"].trim().parse() {
                Ok(num) => num,
                Err(_) => return Err("Invalid temperature".into()),
            };
        } else {
            temperature = model_info.temperature.unwrap();
        }
        if config.contains_key(&"candidate_count".to_string()) {
            candidate_count = match config["candidate_count"].trim().parse() {
                Ok(num) => num,
                Err(_) => return Err("Invalid Candidate Count".into()),
            };
        } else {
            candidate_count = 1;
        }
        if config.contains_key(&"top_p".to_string()) {
            top_p = match config["top_p"].trim().parse() {
                Ok(num) => num,
                Err(_) => return Err("Invalid top_p".into()),
            };
        } else {
            top_p = model_info.top_p.unwrap();
        }
        if config.contains_key(&"top_k".to_string()) {
            top_k = match config["top_k"].trim().parse() {
                Ok(num) => num,
                Err(_) => return Err("Invalid top_k".into()),
            };
        } else {
            top_k = model_info.top_k.unwrap();
        }
        let chat_body = ChatBody {
            prompt: message_prompt,
            temperature: temperature,
            candidate_count: candidate_count,
            top_p: top_p,
            top_k: top_k,
        };
        let client = reqwest::blocking::Client::new();
        let mut res = client
            .post(format!(
                "{}/v1beta2/models/{}:generateMessage?key={}",
                self.endpoint, model, self.api_key
            ))
            .json(&chat_body)
            .send()?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok((res, body))
    }

    fn parse_chat(&self, body: String) -> serde_json::Result<ChatRes> {
        let parsed_chat = serde_json::from_str(&body.as_str())?;
        Ok(parsed_chat)
    }

    pub fn chat(
        &self,
        model: String,
        message_prompt: MessagePrompt,
        config: HashMap<String, String>,
    ) -> Result<ChatRes, Box<dyn std::error::Error>> {
        let (res, body) = self
            .post_chat_req(&model, message_prompt, config)
            .expect("Error occured while sending POST request");
        match res.status() {
            reqwest::StatusCode::OK => {
                let parsed_chats = self.parse_chat(body)?;
                return Ok(parsed_chats);
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                panic!("API Key Invalid")
            }
            reqwest::StatusCode::FORBIDDEN => {
                panic!("API Key Invalid")
            }
            reqwest::StatusCode::NOT_FOUND => {
                return Err(format!("Model {} not supported", &model).into());
            }
            reqwest::StatusCode::BAD_REQUEST => {
                return Err("Bad Request".to_string().into());
            }
            other => {
                panic!("Something unexpected happened: {}", other)
            }
        };
    }

    // functions for generate_text
    fn post_text_req(
        &self,
        model: &String,
        mut text_body: TextBody,
    ) -> Result<(reqwest::blocking::Response, String), Box<dyn std::error::Error>> {
        let model_info = self.get_model(model.to_string()).expect("err");
        let temperature: f64 = model_info.temperature.unwrap();
        let top_p: f64 = model_info.top_p.unwrap();
        if text_body.temperature == -1.0 {
            text_body.set_temperature(temperature);
        }
        if text_body.top_p == -1.0 {
            text_body.set_top_p(top_p);
        }
        let client = reqwest::blocking::Client::new();
        let mut res = client
            .post(format!(
                "{}/v1beta2/models/{}:generateText?key={}",
                self.endpoint, model, self.api_key
            ))
            .json(&text_body)
            .send()?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        Ok((res, body))
    }

    fn parse_text(&self, body: String) -> serde_json::Result<TextRes> {
        let parsed_text = serde_json::from_str(&body.as_str())?;
        Ok(parsed_text)
    }

    pub fn generate_text(
        &self,
        model: String,
        text_body: TextBody,
    ) -> Result<TextRes, Box<dyn std::error::Error>> {
        let (res, body) = self
            .post_text_req(&model, text_body)
            .expect("Error occured while sending POST request");
        match res.status() {
            reqwest::StatusCode::OK => {
                let parsed_text = self.parse_text(body)?;
                return Ok(parsed_text);
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                panic!("API Key Invalid")
            }
            reqwest::StatusCode::FORBIDDEN => {
                panic!("API Key Invalid")
            }
            reqwest::StatusCode::NOT_FOUND => {
                return Err(format!("Model {} not supported", &model).into());
            }
            reqwest::StatusCode::BAD_REQUEST => {
                return Err("Bad Request".to_string().into());
            }
            other => {
                panic!("Something unexpected happened: {}", other)
            }
        };
    }
}

pub fn new_message_prompt() -> MessagePrompt {
    let messages: Vec<Message> = Vec::new();
    let examples: Vec<Example> = Vec::new();
    MessagePrompt {
        context: "".to_string(),
        messages: messages,
        examples: examples,
    }
}

impl MessagePrompt {
    pub fn append_example(&mut self, input: String, output: String) {
        let in_message = Message { content: input };
        let out_message = Message { content: output };
        let example = Example {
            input: in_message,
            output: out_message,
        };
        self.examples.push(example);
    }

    pub fn append_message(&mut self, content: String) {
        let message = Message { content: content };
        self.messages.push(message);
    }

    pub fn set_context(&mut self, context: String) {
        self.context = context;
    }
}

pub fn new_text_body() -> TextBody {
    let text_prompt = TextPrompt {
        text: "".to_string(),
    };
    let safety_settings: Vec<SafetySetting> = Vec::new();
    let stop_sequences: Vec<String> = Vec::new();
    let temperature = -1.0;
    let candidate_count = 1;
    let max_output_tokens = 64;
    let top_p = -1.0;
    let top_k = 40;
    TextBody {
        prompt: text_prompt,
        safety_settings: safety_settings,
        stop_sequences: stop_sequences,
        temperature: temperature,
        candidate_count: candidate_count,
        max_output_tokens: max_output_tokens,
        top_p: top_p,
        top_k: top_k,
    }
}

impl TextBody {
    pub fn set_text_prompt(&mut self, text: String) {
        self.prompt.text = text;
    }

    pub fn append_safety_setting(&mut self, category: String, threshold: String) {
        self.safety_settings.push(SafetySetting {
            category: category,
            threshold: threshold,
        });
    }

    pub fn append_stop_sequence(&mut self, stop_sequence: String) {
        self.stop_sequences.push(stop_sequence);
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
    }

    pub fn set_candidate_count(&mut self, candidate_count: u32) {
        self.candidate_count = candidate_count;
    }

    pub fn set_max_output_tokens(&mut self, max_output_tokens: u32) {
        self.max_output_tokens = max_output_tokens;
    }

    pub fn set_top_p(&mut self, top_p: f64) {
        self.top_p = top_p;
    }

    pub fn set_top_k(&mut self, top_k: u32) {
        self.top_k = top_k;
    }
}
