use crate::components::ui_primitives::{Button, Input};
use leptos::ev;
use leptos::prelude::*;
use std::rc::Rc;

// ---------- Facoltativo: tipi per future estensioni ----------
// #[derive(Clone, PartialEq)]
// pub enum Role {
//     User,
//     Assistant,
// }
// #[derive(Clone)]
// pub struct Message {
//     pub id: String,
//     pub role: Role,
//     pub content: String,
//     // timestamp, ecc.
// }
// ------------------------------------------------------------

#[component]
pub fn InputArea(
    input_value: ReadSignal<String>,
    set_input_value: WriteSignal<String>,
    on_send: Rc<dyn Fn(ev::MouseEvent)>,
    knowledge_enabled: ReadSignal<bool>,
    set_knowledge_enabled: WriteSignal<bool>,
    is_loading: ReadSignal<bool>,
    set_status_message: WriteSignal<String>,
) -> impl IntoView {
    let handle_keypress = {
        let on_send_key = on_send.clone();
        move |ev: ev::KeyboardEvent| {
            if ev.key() == "Enter" && !ev.shift_key() && !is_loading.get() {
                ev.prevent_default();
                let mouse_ev = ev::MouseEvent::new("click").unwrap();
                on_send_key(mouse_ev);
                set_status_message.set("Message sent".into());
            }
        }
    };

    // Avoid overwriting the global status message when toggling Knowledge.
    let disabled_sig =
        Signal::derive(move || input_value.get().trim().is_empty() || is_loading.get());

    view! {
        <div class="flex items-center gap-4 px-2 py-2 w-full">
            // Knowledge switch (simple daisyUI toggle)
            <label class="flex items-center gap-2">
                <input
                    type="checkbox"
                    class="toggle toggle-primary"
                    prop:checked=move || knowledge_enabled.get()
                    on:change=move |ev| set_knowledge_enabled.set(event_target_checked(&ev))
                />
                <span class="text-sm">{"Knowledge"}</span>
            </label>

            // Input expands to fill the row
            <div class="flex-1 min-w-0">
                <Input
                    value=input_value
                    set_value=set_input_value
                    placeholder=Signal::derive(|| "Write a message...".to_string())
                    on_keypress=Box::new(handle_keypress)
                    size=Signal::derive(|| "input-lg".to_string())
                    disabled=Signal::derive(move || is_loading.get())
                />
            </div>

            // Icon-only send button
            <Button
                // No text
                label=Signal::derive(|| "".to_string())
                on_click=Box::new({
                    let on_send = on_send.clone();
                    move || {
                        if !is_loading.get() {
                            let mouse_ev = ev::MouseEvent::new("click").unwrap();
                            on_send(mouse_ev);
                            set_status_message.set("Message sent".into());
                        }
                    }
                })
                variant=Signal::derive(|| "btn-primary".to_string())
                icon=Signal::derive(|| "send".to_string())
                icon_position=Signal::derive(|| "right".to_string())
                disabled=disabled_sig
            />
        </div>
    }
}
