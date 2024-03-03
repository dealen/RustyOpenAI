
pub mod open_ai {
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
    }
}
