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

/// Initialize WebLLM with a specific model and progress callback
pub async fn init_webllm_with_progress<F>(model_id: &str, progress_callback: F) -> Result<JsValue, JsValue> 
where
    F: Fn(String, f64) + 'static,
{
    info!("Initializing WebLLM with model: {}", model_id);
    
    // Create JavaScript callback function
    let callback = Closure::wrap(Box::new(move |progress: JsValue| {
        // Parse progress object
        if let Ok(progress_obj) = progress.dyn_into::<js_sys::Object>() {
            // Extract progress information
            let text = js_sys::Reflect::get(&progress_obj, &"text".into())
                .unwrap_or_else(|_| "Loading...".into())
                .as_string()
                .unwrap_or_else(|| "Loading...".to_string());
            
            let progress_val = js_sys::Reflect::get(&progress_obj, &"progress".into())
                .unwrap_or_else(|_| 0.0.into())
                .as_f64()
                .unwrap_or(0.0);
            
            progress_callback(text, progress_val);
        }
    }) as Box<dyn Fn(JsValue)>);
    
    // Create config object with progress callback
    let config = js_sys::Object::new();
    js_sys::Reflect::set(&config, &"initProgressCallback".into(), callback.as_ref().unchecked_ref()).unwrap();
    
    let promise = CreateMLCEngine(model_id, config.into());
    let result = JsFuture::from(promise).await;
    
    // Keep callback alive until initialization is complete
    callback.forget();
    
    match result {
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

/// Initialize WebLLM with a specific model (backward compatibility)
#[allow(dead_code)]
pub async fn init_webllm(model_id: &str) -> Result<JsValue, JsValue> {
    init_webllm_with_progress(model_id, |_text, _progress| {
        // No-op callback for backward compatibility
    }).await
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
        "Error in response".to_string()
    });
    
    info!("WebLLM response received: {} characters", response_text.len());
    Ok(response_text)
}