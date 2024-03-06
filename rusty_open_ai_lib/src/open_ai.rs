pub mod open_ai {
    use serde::Deserialize;
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
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
        object: String,
        pub data: Vec<Model>,
    }

    impl ModelListResponse {
        pub fn new() -> ModelListResponse {
            ModelListResponse {
                object: "".to_string(),
                data: vec![],
            }
        }
    }

    pub struct OpenAi {
        pub _open_ai_key: String,
        pub _model: String,
    }

    impl OpenAi {

        pub fn new(open_ai_key: String, model: String) -> OpenAi {
            OpenAi {
                _open_ai_key: open_ai_key,
                _model: model,
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
    }
}