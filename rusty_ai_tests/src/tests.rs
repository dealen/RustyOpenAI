#![warn(clippy::all, clippy::pedantic)]

#[cfg(test)]
mod tests {
    use open_ai_lib::{completions::chat::Chat, open_ai::OpenAi};

    #[test]
    fn can_create_open_ai() {
        let open_ai = OpenAi::new("key".to_string(), "model".to_string(), true);
        assert_eq!(open_ai.open_ai_key, "key");
        assert_eq!(open_ai.model, "model");
        assert!(open_ai.get_bearer_key().contains("Bearer key"));
    }

    #[tokio::test]
    async fn can_ask_ai() {
        let chat = Chat::new("key".to_string(), "model".to_string());
        assert_eq!(chat.get_model(), "model");
    }

    #[tokio::test]
    async fn check_model_can_be_cahcnge() {
        let mut _open_ai = OpenAi::new("key".to_string(), "gpt-3.5-turbo-0125".to_string(), true);
        
        _open_ai.change_model("dall-e-3".to_string());
        assert_ne!(_open_ai.model, "gpt-3.5-turbo-0125");
        assert_eq!(_open_ai.model, "dall-e-3");
    }
}