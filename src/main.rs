use std::env;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};

slint::include_modules!();


fn prepare_chat_messages(system_prompt: String, user_prompt: String) -> Vec<ChatCompletionMessage> {
    let messages = vec![
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some(system_prompt),
            name: None,
            function_call: None,
        },
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(user_prompt),
            name: None,
            function_call: None,
        }
    ];
    messages
}

async fn chat(model: &str, messages: Vec<ChatCompletionMessage>) -> ChatCompletionMessage {
    let chat_completion = ChatCompletion::builder(model, messages.clone())
        .create()
        .await
        .unwrap();
    let returned_message = chat_completion.choices.first().unwrap().message.clone();
    returned_message
}

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    set_key(env::var("OPENAI_API_KEY").unwrap());
    let app = AppWindow::new().unwrap();
    let app_weak: slint::Weak<AppWindow> = app.as_weak();

    app.on_send_message(move |system_prompt, user_prompt| {
        let weak_copy = app_weak.clone();
        slint::spawn_local(async move {
            let result = tokio::spawn(async move {
                let messages = prepare_chat_messages(system_prompt.to_string(), user_prompt.to_string());
                chat("gpt-3.5-turbo", messages).await
            }).await.unwrap();
            weak_copy.unwrap().set_completion(result.content.clone().unwrap().trim().into());
        }).unwrap();
    });
    
    app.run()
}