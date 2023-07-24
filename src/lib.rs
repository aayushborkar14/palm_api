//! # palm_api
//!
//! The palm_api crate provides a wrapper for Google's large language model PaLM API
//!
//! ## Generating text
//! Use `PalmClient`'s `generate_text()` method to have the model complete some initial text.
//! ```rust
//! use palm_api::palm::{create_client, new_text_body};
//!
//! let client = create_client(API_KEY.to_string());
//! let mut text_body = new_text_body();
//! text_body.set_text_prompt("The opposite of hot is".to_string());
//! let response = client
//!     .generate_text("text-bison-001".to_string(), text_body)
//!     .expect("An error has occured.");
//! println!("{}", response.candidates.unwrap()[0].output);
//! ```
//!
//! ## Generating message
//! Use `PalmClient`'s `chat()` method to have a discussion with a model.
//! ```rust
//! use palm_api::palm::{create_client, new_chat_body};
//!
//! let client = create_client(API_KEY.to_string());
//! let mut chat_body = new_chat_body();
//! chat_body.append_message("Hello.".to_string());
//! let response = client
//!     .chat("chat-bison-001".to_string(), chat_body)
//!     .expect("An error has occured.");
//! println!("{}", response.candidates.unwrap()[0].content);
//! ```
//!

pub mod palm;

#[cfg(test)]
mod tests {
    use crate::palm::{create_client, new_chat_body, new_text_body};

    #[test]
    fn list_models_works() {
        let my_client = create_client("".to_string());
        let models_list = my_client.list_models().expect("err");
        assert!(models_list.len() > 0);
    }

    #[test]
    fn get_model_works() {
        let my_client = create_client("".to_string());
        let model = my_client
            .get_model("text-bison-001".to_string())
            .expect("err");
        assert_eq!(model.name, "models/text-bison-001");
    }

    #[test]
    fn count_token_works() {
        let my_client = create_client("".to_string());
        let token_count = my_client
            .count_message_tokens(
                "chat-bison-001".to_string(),
                vec![
                    "How many tokens?".to_string(),
                    "For this whole conversation?".to_string(),
                ],
            )
            .expect("err");
        assert!(token_count > 0);
    }

    #[test]
    fn generate_embed_works() {
        let my_client = create_client("".to_string());
        let embeddings = my_client
            .generate_embeddings(
                "embedding-gecko-001".to_string(),
                "say something cool and nice!".to_string(),
            )
            .expect("err");
        assert!(embeddings.len() > 0);
    }

    #[test]
    fn generate_message_works() {
        let my_client = create_client("".to_string());
        let mut chat_body = new_chat_body();
        chat_body.append_example(
            "How are you doing?".to_string(),
            "I am doing absolutely fine!".to_string(),
        );
        chat_body.append_message("How are you doing?".to_string());
        chat_body.set_context("Reply in english".to_string());
        chat_body.set_temperature(0.8);
        chat_body.set_top_p(0.56);
        chat_body.set_candidate_count(2);
        let chat_res = my_client
            .chat("chat-bison-001".to_string(), chat_body)
            .expect("err");
        assert!(chat_res.candidates.unwrap().len() > 0);
    }

    #[test]
    fn generate_text_works() {
        let my_client = create_client("".to_string());
        let mut text_body = new_text_body();
        text_body.append_safety_setting(
            "HARM_CATEGORY_TOXICITY".to_string(),
            "BLOCK_LOW_AND_ABOVE".to_string(),
        );
        text_body.set_candidate_count(2);
        text_body.set_temperature(1.0);
        text_body.set_text_prompt("Write a story about a magic backpack.".to_string());
        let text_res = my_client
            .generate_text("text-bison-001".to_string(), text_body)
            .expect("err");
        assert!(text_res.candidates.unwrap().len() > 0);
    }
}
