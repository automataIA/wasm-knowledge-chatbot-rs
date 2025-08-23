//! WASM tests for ConversationStorage (serves as knowledge storage persistence)

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use wasm_bindgen_test::wasm_bindgen_test_configure;
wasm_bindgen_test_configure!(run_in_browser);

use wasm_knowledge_chatbot_rs::storage::conversation_storage::ConversationStorage;
use wasm_knowledge_chatbot_rs::models::{Message, MessageRole};

fn clear_local_storage() {
    if let Some(win) = web_sys::window() {
        if let Ok(Some(storage)) = win.local_storage() {
            // Ignore errors, best-effort cleanup
            let _ = storage.remove_item("wasm_llm_conversations");
        }
    }
}

#[wasm_bindgen_test]
fn create_and_list_conversations() {
    clear_local_storage();
    let storage = ConversationStorage::new().expect("init storage");

    let id1 = storage
        .create_conversation("First convo".to_string())
        .expect("create convo 1");
    let id2 = storage
        .create_conversation("Second convo".to_string())
        .expect("create convo 2");

    assert_ne!(id1, id2);

    let list = storage.list_conversations().expect("list convos");
    assert_eq!(list.len(), 2);
    assert!(list.iter().any(|c| c.id == id1));
    assert!(list.iter().any(|c| c.id == id2));
}

#[wasm_bindgen_test]
fn save_and_load_messages() {
    clear_local_storage();
    let storage = ConversationStorage::new().expect("init storage");

    let convo_id = storage
        .create_conversation("Chat".to_string())
        .expect("create convo");

    // No messages yet => load_conversation should return None
    let loaded = storage
        .load_conversation(&convo_id)
        .expect("load convo (empty)");
    assert!(loaded.is_none());

    // Save a user message
    let msg_user = Message::new(MessageRole::User, "Hello".to_string());
    storage
        .save_message(&convo_id, &msg_user)
        .expect("save user msg");

    // Now load should return Some(vec)
    let loaded = storage
        .load_conversation(&convo_id)
        .expect("load convo (1 msg)")
        .expect("should have messages");
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].content, "Hello");
    matches!(loaded[0].role, MessageRole::User);

    // Save assistant reply
    let msg_asst = Message::new(MessageRole::Assistant, "Hi!".to_string());
    storage
        .save_message(&convo_id, &msg_asst)
        .expect("save asst msg");

    let loaded = storage
        .load_conversation(&convo_id)
        .expect("load convo (2 msgs)")
        .expect("should have messages");
    assert_eq!(loaded.len(), 2);
    assert_eq!(loaded[1].content, "Hi!");
}

#[wasm_bindgen_test]
fn update_and_delete_conversation() {
    clear_local_storage();
    let storage = ConversationStorage::new().expect("init storage");

    let convo_id = storage
        .create_conversation("Original".to_string())
        .expect("create convo");

    // Update title
    storage
        .update_conversation_title(&convo_id, "Renamed".to_string())
        .expect("update title");

    let list = storage.list_conversations().expect("list convos");
    let info = list.iter().find(|c| c.id == convo_id).expect("exists");
    assert_eq!(info.title, "Renamed");

    // Delete conversation
    storage
        .delete_conversation(&convo_id)
        .expect("delete convo");

    let list = storage.list_conversations().expect("list convos again");
    assert!(list.iter().all(|c| c.id != convo_id));
}

#[wasm_bindgen_test]
fn export_import_replace_and_merge() {
    clear_local_storage();
    let storage = ConversationStorage::new().expect("init storage");

    // Create two conversations and add a message to first
    let id1 = storage
        .create_conversation("First".to_string())
        .expect("create 1");
    let id2 = storage
        .create_conversation("Second".to_string())
        .expect("create 2");

    let msg = Message::new(MessageRole::User, "hello".to_string());
    storage
        .save_message(&id1, &msg)
        .expect("save msg 1");

    // Export bundle
    let bundle = storage.export_json().expect("export json");

    // Clear and import with replace
    clear_local_storage();
    let storage2 = ConversationStorage::new().expect("init storage2");
    storage2
        .import_json(&bundle, false)
        .expect("import replace");
    let list = storage2.list_conversations().expect("list after import");
    assert_eq!(list.len(), 2);
    assert!(list.iter().any(|c| c.id == id1));
    assert!(list.iter().any(|c| c.id == id2));

    // Prepare a bundle with an updated conversation (higher updated_at)
    // Achieve this by adding another message to id1 in current storage, re-export
    let msg2 = Message::new(MessageRole::Assistant, "hi".to_string());
    storage2
        .save_message(&id1, &msg2)
        .expect("save msg 2");
    let newer_bundle = storage2.export_json().expect("export newer json");

    // Import via merge into a store that has an older version
    clear_local_storage();
    let storage3 = ConversationStorage::new().expect("init storage3");
    storage3.import_json(&bundle, false).expect("seed old");
    storage3
        .import_json(&newer_bundle, true)
        .expect("merge newer");

    // Now id1 should reflect two messages
    let loaded = storage3
        .load_conversation(&id1)
        .expect("load merged")
        .expect("some msgs");
    assert_eq!(loaded.len(), 2);
}
