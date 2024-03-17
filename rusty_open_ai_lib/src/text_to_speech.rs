pub mod speech {

    pub struct Speech {
        pub _open_ai_key: String,
        pub _model: String,
    }

    impl Speech {
        pub fn new(open_ai_key: String, model: String) -> Speech {
            Speech {
                _open_ai_key: open_ai_key,
                _model: model,
            }
        }
    }

}