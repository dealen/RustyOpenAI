use rusty_open_ai_lib::open_ai::OpenAi;

#[tokio::main]
async fn main() {

    let open_ai = OpenAi::new("key".to_string(), "gpt-3.5-turbo-0125".to_string());

    show_available_models(open_ai).await;

    println!("Hello, world!");
}

async fn show_available_models(open_ai: OpenAi) {
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