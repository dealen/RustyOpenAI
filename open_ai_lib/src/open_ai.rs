use crate::{completions, moderation};
use actix_web::Error;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use serde_json::Result as JsonResult;

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

impl Default for ModelListResponse {
    fn default() -> Self {
        ModelListResponse {
            value: Option::from(String::new()),
            data: vec![],
        }
    }
}

impl ModelListResponse {
    #[must_use]
    pub fn new() -> ModelListResponse {
        ModelListResponse {
            value: Option::from(String::new()),
            data: vec![],
        }
    }

    #[must_use]
    pub fn get_value(&self) -> Option<String> {
        self.value.clone()
    }
}

pub struct OpenAi {
    pub open_ai_key: String,
    pub model: String,
    ask_moderation: bool,
}

impl OpenAi {
    #[must_use]
    pub fn new(open_ai_key: String, model: String, ask_moderation: bool) -> OpenAi {
        OpenAi {
            open_ai_key,
            model,
            ask_moderation,
        }
    }

    #[must_use]
    pub fn get_bearer_key(&self) -> String {
        format!("Bearer {}", self.open_ai_key)
    }

    /// # Panics
    ///
    /// Will panic if result is not successful
    /// # Errors
    ///
    /// Will return an error if the request to `OpenAI` fails
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
            panic!(
                "Request to OpenAI failed with status: {}",
                response.status()
            );
        }
    }

    pub fn change_model(&mut self, model: String) {
        self.model = model;
    }

    /// # Panics
    ///
    /// Will panic if there is no connection to `OpenAI` or could not get result
    /// # Errors
    ///
    /// Will return an error if the request to `OpenAI` fails
    pub async fn ask_ai(
        &self,
        system_message: String,
        message: String,
        previous_messages: Vec<String>,
    ) -> Result<String, Error> {
        let mut chat =
            completions::chat::Chat::new(self.open_ai_key.to_string(), self.model.to_string());
        let moderation = moderation::Moderation::new();

        if self.ask_moderation {
            if let Some(value) = self.ask_moderation(moderation, &message).await {
                return value;
            }
        }

        let mut previous_messages_new: Vec<String> = vec![];

        for msg in previous_messages {
            chat.add_user_message(&msg);
            previous_messages_new.push(msg.to_string());
        }

        chat.add_system_message(&system_message);
        previous_messages_new.push(system_message.clone());

        let user_message = message.to_string();
        let response = chat.perform_conversation(message).await;
        previous_messages_new.push(user_message.to_string());

        match response {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    async fn ask_moderation(
        &self,
        moderation: moderation::Moderation,
        message: &str,
    ) -> Option<Result<String, Error>> {
        let is_message_flagged = moderation
            .ask_moderation(self.get_bearer_key(), message.to_string())
            .await;

        if is_message_flagged {
            return Some(Err(actix_web::error::ErrorBadRequest("Message is flagged")));
        }
        None
    }
}
