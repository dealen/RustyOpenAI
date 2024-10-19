pub mod open_ai {
    use actix_web::Error;
    use serde::Deserialize;
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
    use serde_json::Result as JsonResult;
    use crate::{completions, moderation};

    #[derive(Deserialize, Debug)]
    pub struct Model {
        pub id: String,
        pub object: String,
        pub created: i64,
        pub owned_by: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct ModelListResponse {
        value: Option<String>,
        pub data: Vec<Model>,
    }

    impl ModelListResponse {
        pub fn new() -> ModelListResponse {
            ModelListResponse {
                value: Option::from("".to_string()),
                data: vec![],
            }
        }

        pub fn get_value(&self) -> Option<String> {
            self.value.clone()
        }
    }

    pub struct OpenAi {
        pub _open_ai_key: String,
        pub _model: String,
        _ask_moderation: bool,
    }

    impl OpenAi {

        pub fn new(open_ai_key: String, model: String, ask_moderation: bool) -> OpenAi {
            OpenAi {
                _open_ai_key: open_ai_key,
                _model: model,
                _ask_moderation: ask_moderation,
            }
        }

        pub fn get_bearer_key(&self) -> String {
            format!("Bearer {}", self._open_ai_key)
        }

        pub async fn get_model_list(&self) -> JsonResult<ModelListResponse> {
            let client = reqwest::Client::new();
            let mut headers = HeaderMap::new();

            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&self.get_bearer_key()).expect("Could not create header"),
            );

            let response = client
                .get("https://api.openai.com/v1/models")
                .headers(headers)
                .send()
                .await
                .expect("Could not get model list");

            if response.status().is_success() {
                let model_list_response = response
                    .json::<ModelListResponse>()
                    .await
                    .expect("Failed to deserialize response");
                Ok(model_list_response)
            } else {
                panic!("Request to OpenAI failed with status: {}", response.status());
            }
        }

        pub fn change_model(&mut self, model: String) {
            self._model = model;
        }

        pub async fn ask_ai(&self, system_message: String, message: String, previous_messages: Vec<String>) -> Result<String, Error> {
            let mut chat = completions::chat::Chat::new(self._open_ai_key.to_string(), self._model.to_string());
            let moderation = moderation::moderation::Moderation::new();

            let is_message_flagged = moderation.ask_moderation(self.get_bearer_key(), message.clone()).await;

            if is_message_flagged {
                return Err(actix_web::error::ErrorBadRequest("Message is flagged"));
            }

            let mut previous_messages_new: Vec<String> = vec![];

            for msg in previous_messages {
                chat.add_user_message(msg.to_string());
                previous_messages_new.push(msg.to_string());
            }

            chat.add_system_message(system_message.clone());
            previous_messages_new.push(system_message.clone());

            let user_message = message.to_string();
            let response = chat.perform_conversation(message).await;   
            previous_messages_new.push(user_message.to_string());

            if response.is_ok() {
                Ok(response.unwrap())
            } else {
                Err(response.err().unwrap())
            }
        }
    }
}