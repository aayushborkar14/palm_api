use serde::{Deserialize, Serialize};
use std::io::Read;

const ENDPOINT: &str = "https://generativelanguage.googleapis.com";

/// A client configured with a PaLM API key and an API endpoint
pub struct PalmClient {
    api_key: String,
    endpoint: String,
}

/// Creates a PalmClient
///
/// # Arguments
///
/// * `api_key` - A string that holds the PaLM API key from Google
///
/// # Example
/// ```
/// const API_KEY: &str = "api key here";
/// let client = palm_api::palm::create_client(API_KEY.to_string());
/// ```
pub fn create_client(api_key: String) -> PalmClient {
    PalmClient {
        api_key: api_key,
        endpoint: ENDPOINT.to_string(),
    }
}

/// Information about any model
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// Required. The resource name of the Model
    ///
    /// # Format
    /// models/{model} with a {model} naming convention of "{baseModelId}-{version}"
    ///
    /// # Example
    /// models/chat-bison-001
    pub name: String,
    /// Required. The version number of the model
    /// This represents the major version
    pub version: String,
    /// The human-readable name of the model. E.g. "Chat Bison"
    /// The name can be up to 128 characters long and can consist of any UTF-8 characters
    pub display_name: String,
    /// A short description of the model
    pub description: String,
    /// Maximum number of input tokens allowed for this model
    pub input_token_limit: u32,
    /// Maximum number of output tokens available for this model
    pub output_token_limit: u32,
    /// The model's supported generation methods
    /// The method names are defined as Pascal case strings, such as generateMessage which correspond to API methods
    pub supported_generation_methods: Vec<String>,
    /// Controls the randomness of the output
    /// Values can range over [0.0,1.0], inclusive
    /// A value closer to 1.0 will produce responses that are more varied, while a value closer to 0.0 will typically result in less surprising responses from the model
    /// This value specifies default to be used by the backend while making the call to the model
    pub temperature: Option<f64>,
    /// For Nucleus sampling
    /// Nucleus sampling considers the smallest set of tokens whose probability sum is at least topP
    /// This value specifies default to be used by the backend while making the call to the model
    pub top_p: Option<f64>,
    /// For Top-k sampling
    /// Top-k sampling considers the set of topK most probable tokens
    /// This value specifies default to be used by the backend while making the call to the model
    pub top_k: Option<i32>,
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

/// JSON Payload for POST request required to generate message (chat)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatBody {
    prompt: MessagePrompt,
    temperature: f64,
    candidate_count: u32,
    top_p: f64,
    top_k: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Example {
    input: Message,
    output: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct MessagePrompt {
    context: String,
    examples: Vec<Example>,
    messages: Vec<Message>,
}

/// Content filtering metadata associated with processing a single request
#[derive(Serialize, Deserialize, Debug)]
pub struct ContentFilter {
    /// The reason content was blocked during request processing
    pub reason: String,
}

/// Message response to generate message (chat)
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageRes {
    /// Optional. The author of this Message
    /// This serves as a key for tagging the content of this Message when it is fed to the model as text
    /// The author can be any alphanumeric string
    pub author: String,
    /// Required. The text content of the structured Message
    pub content: String,
}

// Response to generate message (chat)
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRes {
    /// The conversation history used by the model
    pub messages: Vec<MessageRes>,
    /// A set of content filtering metadata for the prompt and response text
    /// This indicates which SafetyCategory(s) blocked a candidate from this response, the lowest HarmProbability that triggered a block, and the HarmThreshold setting for that category
    /// This indicates the smallest change to the SafetySettings that would be necessary to unblock at least 1 response
    /// The blocking is configured by the SafetySettings in the request (or the default SafetySettings of the API)
    ///
    /// # Example
    /// ```
    /// println!("{}",chat_res.filters.unwrap()[0].reason);
    /// ```
    pub filters: Option<Vec<ContentFilter>>,
    /// Candidate response messages from the model
    ///
    /// # Example
    /// ```
    /// println!("{}",chat_res.candidates.unwrap()[0].content);
    /// ```
    pub candidates: Option<Vec<MessageRes>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TextPrompt {
    text: String,
}

/// Safety setting, affecting the safety-blocking behavior
/// Passing a safety setting for a category changes the allowed proability that content is blocked
#[derive(Serialize, Deserialize, Debug)]
pub struct SafetySetting {
    /// Required. The category for this setting
    pub category: String,
    /// Required. Controls the probability threshold at which harm is blocked
    pub threshold: String,
}

/// The request body for generate_text() function
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
    top_k: i32,
}

/// Safety rating for a piece of content
/// The safety rating contains the category of harm and the harm probability level in that category for a piece of content
/// Content is classified for safety across a number of harm categories and the probability of the harm classification is included here
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
        mut chat_body: ChatBody,
    ) -> Result<(reqwest::blocking::Response, String), Box<dyn std::error::Error>> {
        let model_info = self.get_model(model.to_string()).expect("err");
        if chat_body.temperature == -1.0 {
            chat_body.temperature = model_info.temperature.unwrap();
        }
        if chat_body.top_p == -1.0 {
            chat_body.top_p = model_info.top_p.unwrap();
        }
        if chat_body.top_k == -1 {
            chat_body.top_k = model_info.top_k.unwrap();
        }
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
        chat_body: ChatBody,
    ) -> Result<ChatRes, Box<dyn std::error::Error>> {
        let (res, body) = self
            .post_chat_req(&model, chat_body)
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
        if text_body.temperature == -1.0 {
            text_body.set_temperature(model_info.temperature.unwrap());
        }
        if text_body.top_p == -1.0 {
            text_body.set_top_p(model_info.top_p.unwrap());
        }
        if text_body.top_k == -1 {
            text_body.set_top_k(model_info.top_k.unwrap());
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

fn new_message_prompt() -> MessagePrompt {
    let messages: Vec<Message> = Vec::new();
    let examples: Vec<Example> = Vec::new();
    MessagePrompt {
        context: "".to_string(),
        messages: messages,
        examples: examples,
    }
}

pub fn new_chat_body() -> ChatBody {
    let prompt = new_message_prompt();
    let temperature = -1.0;
    let candidate_count = 1;
    let top_p = -1.0;
    let top_k = -1;
    ChatBody {
        prompt: prompt,
        temperature: temperature,
        candidate_count: candidate_count,
        top_p: top_p,
        top_k: top_k,
    }
}

impl ChatBody {
    pub fn append_example(&mut self, input: String, output: String) {
        let in_message = Message { content: input };
        let out_message = Message { content: output };
        let example = Example {
            input: in_message,
            output: out_message,
        };
        self.prompt.examples.push(example);
    }

    pub fn append_message(&mut self, content: String) {
        let message = Message { content: content };
        self.prompt.messages.push(message);
    }

    pub fn set_context(&mut self, context: String) {
        self.prompt.context = context;
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
    }

    pub fn set_candidate_count(&mut self, candidate_count: u32) {
        self.candidate_count = candidate_count;
    }

    pub fn set_top_p(&mut self, top_p: f64) {
        self.top_p = top_p;
    }

    pub fn set_top_k(&mut self, top_k: i32) {
        self.top_k = top_k;
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
    let top_k = -1;
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

    pub fn set_top_k(&mut self, top_k: i32) {
        self.top_k = top_k;
    }
}
