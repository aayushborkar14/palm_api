pub mod palm;
pub use palm::PalmClient;

pub fn create_client(api_key: String) -> PalmClient {
    palm::create_client(api_key)
}

#[cfg(test)]
mod tests {
    use crate::palm::{new_chat_body, new_text_body};

    use super::*;

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
        assert_eq!(token_count, 23);
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
        assert_eq!(chat_res.candidates.unwrap().len(), 2);
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
        assert_eq!(text_res.candidates.unwrap().len(), 2);
    }
}
