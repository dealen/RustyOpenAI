mod core;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_open_ai() {
        let open_ai = core::open_ai::OpenAi::new("key".to_string(), "model".to_string());
        assert_eq!(open_ai._open_ai_key, "key");
        assert_eq!(open_ai._model, "model");
        assert!(open_ai.get_bearer_key().contains("Bearer key"));
    }

    #[tokio::test]
    async fn can_get_model_list() {
        let open_ai = core::open_ai::OpenAi::new("myKey".to_string(), "model".to_string());
        let model_list = open_ai.get_model_list().await;
        assert!(model_list.is_ok());
    }
}