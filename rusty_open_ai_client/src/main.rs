use rusty_open_ai_lib::open_ai::OpenAi;

#[tokio::main]
async fn main() {
    let open_ai = OpenAi::new("key".to_string(), "gpt-3.5-turbo-0125".to_string());

    show_available_models(&open_ai).await;

    let previous_messages: Vec<String> = vec![];
    ask_ai_some_questions(&open_ai, previous_messages).await;
}

async fn ask_ai_some_questions(open_ai: &OpenAi, previous_messages: Vec<String>) {

    let system_message = "You are helpful and answer briefly.".to_string();
    let message = "What if capital of Poland? Give me top 5 most interesting things about it".to_string();
    
    match open_ai.ask_ai(system_message, message, previous_messages).await {
        Ok(response) => {
            let content = response.split("content\": \"").collect::<Vec<&str>>()[1];
            println!("Response: {:?}", content);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

async fn show_available_models(open_ai: &OpenAi) {
    match open_ai.get_model_list().await {
        Ok(list) => {
            for model in list.data {
                println!("Model: {:?}", model);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}