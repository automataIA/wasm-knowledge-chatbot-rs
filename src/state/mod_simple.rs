// Simplified state management modules for Leptos 0.8 compatibility

use crate::state::app_state_simple;
use crate::state::webllm_state_simple;
use crate::state::conversation_state_simple;
use crate::state::actions_simple;

// Re-export the simplified contexts and providers
pub use app_state_simple::{AppStateContext, AppStateProvider, use_app_state};
pub use webllm_state_simple::{WebLLMStateContext, WebLLMStateProvider, use_webllm_state};
pub use conversation_state_simple::{ConversationStateContext, ConversationStateProvider, use_conversation_state};
pub use actions_simple::{WebLLMActions, ConversationActions, AppActions};

/// Combined state provider that provides all state contexts
#[leptos::component]
pub fn GlobalStateProvider(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    use leptos::prelude::*;
    
    view! {
        <AppStateProvider>
            <WebLLMStateProvider>
                <ConversationStateProvider>
                    {children()}
                </ConversationStateProvider>
            </WebLLMStateProvider>
        </AppStateProvider>
    }
}

/// Hook to get all state contexts at once
pub fn use_all_states() -> (AppStateContext, WebLLMStateContext, ConversationStateContext) {
    (
        use_app_state(),
        use_webllm_state(),
        use_conversation_state(),
    )
}
