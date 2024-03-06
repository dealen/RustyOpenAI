use rusty_open_ai_lib::open_ai::OpenAi;

fn main() {

    let open_ai = OpenAi::new("key".to_string(), "model".to_string());

    println!("Hello, world!");
}
