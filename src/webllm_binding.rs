use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;
use log::{info, error};

// Simple WebLLM bindings
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "webllm"])]
    fn CreateMLCEngine(model: &str, config: JsValue) -> js_sys::Promise;
}

/// Initialize WebLLM with a specific model
pub async fn init_webllm(model_id: &str) -> Result<JsValue, JsValue> {
    info!("Initializing WebLLM with model: {}", model_id);
    
    let config = js_sys::Object::new();
    
    let promise = CreateMLCEngine(model_id, config.into());
    match JsFuture::from(promise).await {
        Ok(engine) => {
            info!("WebLLM engine initialized successfully with model: {}", model_id);
            Ok(engine)
        }
        Err(e) => {
            error!("Failed to initialize WebLLM with model {}: {:?}", model_id, e);
            Err(e)
        }
    }
}

/// Send a message to the WebLLM engine and get a response
pub async fn send_message_to_llm(engine: &JsValue, messages: Vec<crate::models::Message>) -> Result<String, JsValue> {
    info!("Sending message to WebLLM with {} messages", messages.len());
    
    // Create messages array manually
    let messages_array = js_sys::Array::new();
    for msg in messages {
        let message_obj = js_sys::Object::new();
        let role = match msg.role {
            crate::models::MessageRole::User => "user",
            crate::models::MessageRole::Assistant => "assistant",
        };
        js_sys::Reflect::set(&message_obj, &"role".into(), &role.into())?;
        js_sys::Reflect::set(&message_obj, &"content".into(), &msg.content.into())?;
        messages_array.push(&message_obj);
    }
    
    // Create request object
    let request = js_sys::Object::new();
    js_sys::Reflect::set(&request, &"messages".into(), &messages_array)?;
    js_sys::Reflect::set(&request, &"stream".into(), &false.into())?;
    js_sys::Reflect::set(&request, &"max_tokens".into(), &512.into())?;
    js_sys::Reflect::set(&request, &"temperature".into(), &0.7.into())?;
    
    // Call WebLLM API using reflection to access nested methods
    let chat_completion = js_sys::Reflect::get(engine, &"chat".into())
        .map_err(|e| {
            error!("Failed to get chat object: {:?}", e);
            e
        })?;
    
    let completions = js_sys::Reflect::get(&chat_completion, &"completions".into())
        .map_err(|e| {
            error!("Failed to get completions object: {:?}", e);
            e
        })?;
    
    let create_fn = js_sys::Reflect::get(&completions, &"create".into())
        .map_err(|e| {
            error!("Failed to get create function: {:?}", e);
            e
        })?;
    
    let args = js_sys::Array::of1(&request);
    let promise = js_sys::Reflect::apply(&create_fn.into(), &completions, &args)
        .map_err(|e| {
            error!("Failed to call create function: {:?}", e);
            e
        })?;
    
    let result = JsFuture::from(js_sys::Promise::from(promise)).await
        .map_err(|e| {
            error!("WebLLM API call failed: {:?}", e);
            e
        })?;
    
    // Extract the response
    let choices = js_sys::Reflect::get(&result, &"choices".into())
        .map_err(|e| {
            error!("Failed to get choices from response: {:?}", e);
            e
        })?;
    
    let first_choice = js_sys::Reflect::get(&choices, &0_u32.into())
        .map_err(|e| {
            error!("Failed to get first choice: {:?}", e);
            e
        })?;
    
    let message = js_sys::Reflect::get(&first_choice, &"message".into())
        .map_err(|e| {
            error!("Failed to get message from choice: {:?}", e);
            e
        })?;
    
    let content = js_sys::Reflect::get(&message, &"content".into())
        .map_err(|e| {
            error!("Failed to get content from message: {:?}", e);
            e
        })?;
    
    let response_text = content.as_string().unwrap_or_else(|| {
        error!("Response content is not a string");
        "Errore nella risposta".to_string()
    });
    
    info!("WebLLM response received: {} characters", response_text.len());
    Ok(response_text)
}