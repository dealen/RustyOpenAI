
use open_ai_lib::{images::Images, open_ai::OpenAi, text_to_speech::speech::Speech, transctiption::Transcription};

#[tokio::main]
async fn main() {
    
    let api_key = "".to_string();

    // Creating Open_AI instance
    // let mut open_ai = OpenAi::new(api_key.clone(), "gpt-4o-mini".to_string(), true);

    // Downloading list of available models
    // show_available_models(&open_ai).await;

    // asking question to OpenAI API chat completion model
    // let previous_messages: &mut Vec<String> = &mut Vec::new();
    // let mut result = ask_ai_some_questions(&open_ai, previous_messages).await;

    // Changing used model
    // open_ai.change_model("gpt-4o".to_string());

    // let current_model = open_ai.model.to_string();
    // println!("Current model: {:?}", current_model);

    // let result2 = ask_ai_some_questions(&open_ai, &mut result).await;
    // println!("Result: {:?}", result2);

    // Generating audio file from text
    let text_to_speech = Speech::new(api_key.clone());
    let path = text_to_speech.get_audio("What lovely weather have we today.", "Onyx").await;
    println!("Path: to audio file: {path:?}");

    // Generating image from text
    // let images = Images::new(api_key.clone());
    // let image_path = images.get_image("Small dinosaur wearing sunglasses.").await;
    // println!("Path to image file: {image_path:?}");

    // Generating transcription from audio file
    let transcriptions = Transcription::new(api_key.clone());
    let transcription = transcriptions.transcript(&path).await;

    println!("Transcription: {transcription:?}");
}

async fn ask_ai_some_questions(open_ai: &OpenAi, previous_messages: &mut Vec<String>) -> Vec<String> {

    let system_message = "You are helpful and answer briefly.".to_string();
    let message = "What if capital of Poland? Give me top 5 most interesting things about it".to_string();

    match open_ai.ask_ai(system_message, message, previous_messages.to_vec()).await {
        Ok(response) => {
            let content = response.split("content\": \"").collect::<Vec<&str>>()[1].to_string();
            println!("Response: {:?}", content);
            previous_messages.push(content);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    previous_messages.to_vec()
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