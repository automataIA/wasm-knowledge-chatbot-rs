use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast; // necessario per dyn_into

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
    // testo dell'utente
    input_value: ReadSignal<String>,
    set_input_value: WriteSignal<String>,

    // callback di invio
    on_send: impl Fn(ev::MouseEvent) + 'static + Copy,

    // toggle Knowledge
    knowledge_enabled: ReadSignal<bool>,
    set_knowledge_enabled: WriteSignal<bool>,

    // stato di caricamento (es. chiamata rete in corso)
    is_loading: ReadSignal<bool>,

    // messaggi da mostrare in StatusBar
    set_status_message: WriteSignal<String>,
) -> impl IntoView {
    // Invio con [Enter] (Shift+Enter = nuova riga non più necessario):
    let handle_keypress = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() && !is_loading.get() {
            ev.prevent_default();
            let mouse_ev = ev::MouseEvent::new("click").unwrap();
            on_send(mouse_ev);

            set_status_message.set("Messaggio inviato".into());
        }
    };

    // Gestione toggle Knowledge
    let handle_toggle_change = move |ev: ev::Event| {
        // cast sicuro da EventTarget a HtmlInputElement
        let checked = ev
            .target()
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .checked();

        set_knowledge_enabled.set(checked);
        let msg = if checked {
            "Knowledge base attivata"
        } else {
            "Knowledge base disattivata"
        };
        set_status_message.set(msg.into());
    };

    // ---------- MARKUP ----------
    view! {
        // Barra inferiore stile ChatArea.tsx
        <div class="w-full flex items-center gap-3 px-4 py-3 bg-base-200 border-t border-base-300 rounded-b-2xl">

            <div class="tooltip tooltip-right" data-tip="Toggle Knowledge Base">
                <label class="flex items-center gap-2 cursor-pointer">
                    <input
                        type="checkbox"
                        class="toggle toggle-primary"
                        checked=knowledge_enabled
                        on:change=handle_toggle_change
                    />
                    <i
                        data-lucide="network"
                        class=move || {
                            if knowledge_enabled.get() {
                                "w-5 h-5 text-primary"
                            } else {
                                "w-5 h-5 text-base-content/60"
                            }
                        }
                    ></i>
                </label>
            </div>

            <input
                class="input input-bordered flex-1 text-base placeholder:text-base-content/60 focus:outline-none focus:ring-2 focus:ring-primary/30 transition-colors disabled:opacity-60"
                placeholder="Scrivi un messaggio..."
                prop:value=input_value
                on:input=move |ev| set_input_value.set(event_target_value(&ev))
                on:keypress=handle_keypress
                prop:disabled=is_loading
                autocomplete="off"
            />

            <button
                class=move || {
                    if input_value.get().trim().is_empty() || is_loading.get() {
                        "btn btn-primary btn-square btn-disabled opacity-60"
                    } else {
                        "btn btn-primary btn-square hover:scale-105 transition-transform"
                    }
                }
                on:click=on_send
                disabled=move || input_value.get().trim().is_empty() || is_loading.get()
            >
                <Show
                    when=move || !is_loading.get()
                    fallback=|| view! { <span class="loading loading-spinner loading-xs"></span> }
                >

                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="18"
                        height="18"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <path d="m22 2-7 20-4-9-9-4Z" />
                        <path d="M22 2 11 13" />
                    </svg>
                </Show>
            </button>
        </div>
    }
}
