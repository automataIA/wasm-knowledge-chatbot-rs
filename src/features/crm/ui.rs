#![allow(non_snake_case)]
use crate::models::crm::{Customer, Deal, Lead, LeadSource, PipelineStage};
use crate::state::{use_crm_state, CRMStateProvider};
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
fn DetailAlert(hash: &'static str, text: String) -> impl IntoView {
    let on_close = move |_| {
        let _ = web_sys::window().unwrap().location().set_hash(hash);
    };
    view! {
        <div class=move || "alert alert-info mb-2">
            <div class=move || "flex-1">
                <span class=move || "font-semibold">{text}</span>
            </div>
            <button class=move || "btn btn-xs" on:click=on_close>{"Close"}</button>
        </div>
    }
}

#[component]
pub fn CRMPanel() -> impl IntoView {
    // Provide local CRM state scope so panel can be dropped independently if desired
    let (tab, set_tab) = signal("customers".to_string());
    // Optional detail tuple: (kind, id) where kind is "customers" | "deals"
    let (detail, set_detail) = signal(None::<(String, String)>);
    // Initialize from location.hash if present
    if let Some(win) = web_sys::window() {
        if let Ok(loc) = win.location().hash() {
            let h = loc.trim_start_matches('#').to_string();
            // patterns: customers | leads | deals | stages | board | customers/<id> | deals/<id>
            if let Some((kind, id)) = h.split_once('/') {
                match kind {
                    "customers" => {
                        set_tab.set("customers".into());
                        set_detail.set(Some(("customers".into(), id.into())));
                    }
                    "deals" => {
                        set_tab.set("deals".into());
                        set_detail.set(Some(("deals".into(), id.into())));
                    }
                    _ => {}
                }
            } else {
                match h.as_str() {
                    "customers" | "leads" | "deals" | "stages" | "board" => {
                        set_tab.set(h);
                        set_detail.set(None);
                    }
                    _ => {}
                }
            }
        }
    }
    // Keep hash in sync on tab changes (best-effort)
    Effect::new({
        move |_| {
            if let Some(win) = web_sys::window() {
                let location = win.location();
                let _ = location.set_hash(&tab.get());
            }
        }
    });
    // Listen to hash changes (back/forward navigation)
    if let Some(win) = web_sys::window() {
        let set_tab_from_hash = set_tab;
        let set_detail_from_hash = set_detail;
        let cb = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            if let Some(w) = web_sys::window() {
                if let Ok(h) = w.location().hash() {
                    let h = h.trim_start_matches('#').to_string();
                    if let Some((kind, id)) = h.split_once('/') {
                        match kind {
                            "customers" => {
                                set_tab_from_hash.set("customers".into());
                                set_detail_from_hash.set(Some(("customers".into(), id.into())));
                            }
                            "deals" => {
                                set_tab_from_hash.set("deals".into());
                                set_detail_from_hash.set(Some(("deals".into(), id.into())));
                            }
                            _ => {}
                        }
                    } else {
                        match h.as_str() {
                            "customers" | "leads" | "deals" | "stages" | "board" => {
                                set_tab_from_hash.set(h);
                                set_detail_from_hash.set(None);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        let _ = win.add_event_listener_with_callback("hashchange", cb.as_ref().unchecked_ref());
        cb.forget(); // leak to keep listener alive for app lifetime (Send + Sync not satisfied for on_cleanup)
    }
    view! {
        <CRMStateProvider>
            <div class="w-full min-w-[320px] max-w-full">
                <div class="tabs tabs-boxed mb-3 gap-2">
                    <button class=move || if tab.get() == "customers" { "tab tab-active" } else { "tab" } id="tab-customers" on:click=move |_| set_tab.set("customers".into())>"Customers"</button>
                    <button class=move || if tab.get() == "leads" { "tab tab-active" } else { "tab" } id="tab-leads" on:click=move |_| set_tab.set("leads".into())>"Leads"</button>
                    <button class=move || if tab.get() == "deals" { "tab tab-active" } else { "tab" } id="tab-deals" on:click=move |_| set_tab.set("deals".into())>"Deals"</button>
                    <button class=move || if tab.get() == "stages" { "tab tab-active" } else { "tab" } id="tab-stages" on:click=move |_| set_tab.set("stages".into())>"Stages"</button>
                    <button class=move || if tab.get() == "board" { "tab tab-active" } else { "tab" } id="tab-board" on:click=move |_| set_tab.set("board".into())>"Board"</button>
                </div>
                <Show when=move || tab.get() == "customers">
                    <CustomersView detail=detail />
                </Show>
                <Show when=move || tab.get() == "leads">
                    <LeadsView />
                </Show>
                <Show when=move || tab.get() == "deals">
                    <DealsView detail=detail />
                </Show>
                <Show when=move || tab.get() == "stages">
                    <StagesView />
                </Show>
                <Show when=move || tab.get() == "board">
                    <PipelineBoardView />
                </Show>
            </div>
        </CRMStateProvider>
    }
}

#[component]
fn CustomersView(detail: ReadSignal<Option<(String, String)>>) -> impl IntoView {
    let crm = use_crm_state();
    let (name, set_name) = signal(String::new());

    let crm_add = crm.clone();
    let add = move |_| {
        let n = name.get();
        if n.trim().is_empty() {
            return;
        }
        crm_add.upsert_customer(Customer::new(n));
        set_name.set(String::new());
    };

    let crm_for_customers = crm.clone();
    view! {
        <div id="crm-customers" class="mb-6">
            // Optional detail pane
            <Show when=move || detail.get().as_ref().is_some_and(|(k, _)| k == "customers")>
                {{
                    let crm_for_customers_clone = crm_for_customers.clone();
                    move || {
                        let text = detail.with(|d| {
                            if let Some((_, id)) = d.clone() {
                                if let Some(c) = crm_for_customers_clone
                                    .customers_now()
                                    .into_iter()
                                    .find(|c| c.id == id)
                                {
                                    format!("Customer: {}", c.name.clone())
                                } else {
                                    "Customer not found".to_string()
                                }
                            } else {
                                String::new()
                            }
                        });
                        view! { <DetailAlert hash="customers" text=text /> }
                    }
                }}
            </Show>
            <div class="flex items-center gap-2 mb-2">
                <input
                    class="input input-sm input-bordered w-full"
                    prop:value=name
                    on:input=move |e| set_name.set(event_target_value(&e))
                    placeholder="New customer name"
                />
                <button class="btn btn-sm" on:click=add>
                    "Add"
                </button>
            </div>
            <ul class="menu bg-base-200 rounded-box">
                {move || {
                    let crm_ctx = crm.clone();
                    crm_ctx
                        .customers_now()
                        .into_iter()
                        .map(|c| {
                            let id = c.id.clone();
                            let crm_item = crm_ctx.clone();
                            view! {
                                <li class="flex items-center justify-between">
                                    <button class="btn btn-ghost btn-xs" on:click={
                                        let id = id.clone();
                                        move |_| { let _ = web_sys::window().unwrap().location().set_hash(&format!("customers/{}", id)); }
                                    }>{c.name.clone()}</button>
                                    <button
                                        class="btn btn-ghost btn-xs"
                                        on:click=move |_| crm_item.delete_customer(&id)
                                    >
                                        "✕"
                                    </button>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </div>
    }
}

#[component]
fn PipelineBoardView() -> impl IntoView {
    let crm = use_crm_state();

    // Helpers to move a deal to adjacent stage
    let move_deal = {
        let crm_ctx = crm.clone();
        move |deal_id: String, direction: i32| {
            // capture fresh data from signals
            let mut stages = crm_ctx.stages_now();
            stages.sort_by_key(|s| s.order);
            if stages.is_empty() {
                return;
            }

            if let Some(mut deal) = crm_ctx.deals_now().into_iter().find(|d| d.id == deal_id) {
                if let Some((idx, _)) = stages
                    .iter()
                    .enumerate()
                    .find(|(_, s)| s.id == deal.stage_id)
                {
                    let new_idx = if direction < 0 {
                        idx.saturating_sub(1)
                    } else {
                        (idx + 1).min(stages.len().saturating_sub(1))
                    };
                    if new_idx != idx {
                        deal.stage_id = stages[new_idx].id.clone();
                        crm_ctx.upsert_deal(deal);
                    }
                }
            }
        }
    };

    // Inline add stage + reorder helpers
    let (new_stage, set_new_stage) = signal(String::new());
    let add_stage = {
        let crm_add = crm.clone();
        move |_| {
            let n = new_stage.get();
            if n.trim().is_empty() {
                return;
            }
            let ts = js_sys::Date::now();
            crm_add.upsert_stage(PipelineStage {
                id: format!("stage_{}", ts),
                name: n,
                order: 0,
                probability: 0.2,
                color: None,
                is_closed: false,
            });
            set_new_stage.set(String::new());
        }
    };

    let reorder_stage = {
        let crm_ctx = crm.clone();
        move |stage_id: String, delta: i32| {
            let mut stages = crm_ctx.stages_now();
            stages.sort_by_key(|s| s.order);
            if let Some((idx, _)) = stages.iter().enumerate().find(|(_, s)| s.id == stage_id) {
                let new_idx = if delta < 0 {
                    idx.saturating_sub(1)
                } else {
                    (idx + 1).min(stages.len().saturating_sub(1))
                };
                if new_idx != idx {
                    stages.swap(idx, new_idx);
                    for (i, mut s) in stages.into_iter().enumerate() {
                        s.order = i as u32;
                        crm_ctx.upsert_stage(s);
                    }
                }
            }
        }
    };

    // Render board
    view! {
        <div id="crm-board" class="overflow-x-auto">
            <div class="flex items-center gap-2 mb-3">
                <input class="input input-sm input-bordered w-full" prop:value=new_stage on:input=move |e| set_new_stage.set(event_target_value(&e)) placeholder="Add new stage" />
                <button class="btn btn-sm" on:click=add_stage>{"Add Stage"}</button>
            </div>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 min-w-[360px]">
                {move || {
                    let mut stages = crm.stages_now();
                    stages.sort_by_key(|s| s.order);
                    let deals = crm.deals_now();

                    stages.into_iter().map(|stage| {
                        let stage_id = stage.id.clone();
                        let title = stage.name.clone();
                        let stage_deals: Vec<_> = deals.iter().filter(|d| d.stage_id == stage_id).cloned().collect();
                        let crm_move_left = move_deal.clone();
                        let crm_move_right = move_deal.clone();
                        view! {
                            <div class="card bg-base-200">
                                <div class="card-body p-3">
                                    <div class="flex items-center justify-between mb-2">
                                        <div class="font-semibold">{format!("{} ({})", title, stage_deals.len())}</div>
                                        <div class="flex gap-1">
                                            <button class="btn btn-xs" on:click={
                                                let f = reorder_stage.clone(); let id = stage_id.clone(); move |_| f(id.clone(), -1)
                                            }>{"↑"}</button>
                                            <button class="btn btn-xs" on:click={
                                                let f = reorder_stage.clone(); let id = stage_id.clone(); move |_| f(id.clone(), 1)
                                            }>{"↓"}</button>
                                        </div>
                                    </div>
                                    <div class="space-y-2">
                                        {stage_deals.into_iter().map(|d| {
                                            let id_left = d.id.clone();
                                            let id_right = d.id.clone();
                                            view! {
                                                <div class="card bg-base-100 shadow-sm">
                                                    <div class="card-body p-3">
                                                        <div class="flex items-center justify-between gap-2">
                                                            <div class="text-sm truncate">{d.title.clone()}</div>
                                                            <div class="flex gap-1">
                                                                <button class="btn btn-xs" on:click={
                                                                    let f = crm_move_left.clone();
                                                                    let id = id_left.clone();
                                                                    move |_| f(id.clone(), -1)
                                                                }>
                                                                    "←"
                                                                </button>
                                                                <button class="btn btn-xs" on:click={
                                                                    let f = crm_move_right.clone();
                                                                    let id = id_right.clone();
                                                                    move |_| f(id.clone(), 1)
                                                                }>
                                                                    "→"
                                                                </button>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

#[component]
fn LeadsView() -> impl IntoView {
    let crm = use_crm_state();
    let (name, set_name) = signal(String::new());

    let crm_add = crm.clone();
    let add = move |_| {
        let n = name.get();
        if n.trim().is_empty() {
            return;
        }
        crm_add.upsert_lead(Lead::new(n, LeadSource::Website));
        set_name.set(String::new());
    };

    view! {
        <div id="crm-leads" class="mb-6">
            <div class="flex items-center gap-2 mb-2">
                <input
                    class="input input-sm input-bordered w-full"
                    prop:value=name
                    on:input=move |e| set_name.set(event_target_value(&e))
                    placeholder="New lead name"
                />
                <button class="btn btn-sm" on:click=add>
                    "Add"
                </button>
            </div>
            <ul class="menu bg-base-200 rounded-box">
                {move || {
                    let crm_ctx = crm.clone();
                    crm_ctx
                        .leads_now()
                        .into_iter()
                        .map(|l| {
                            let id = l.id.clone();
                            let crm_item = crm_ctx.clone();
                            view! {
                                <li class="flex items-center justify-between">
                                    <span>{l.name.clone()}</span>
                                    <button
                                        class="btn btn-ghost btn-xs"
                                        on:click=move |_| crm_item.delete_lead(&id)
                                    >
                                        "✕"
                                    </button>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </div>
    }
}

#[component]
fn DealsView(detail: ReadSignal<Option<(String, String)>>) -> impl IntoView {
    let crm = use_crm_state();
    let (title, set_title) = signal(String::new());

    // For minimal scaffold, if no stages exist, create a default stage
    let crm_effect = crm.clone();
    Effect::new(move |_| {
        if crm_effect.stages_now().is_empty() {
            crm_effect.upsert_stage(PipelineStage {
                id: "stage_default".into(),
                name: "New".into(),
                order: 0,
                probability: 0.2,
                color: None,
                is_closed: false,
            });
        }
    });

    let crm_add = crm.clone();
    let add = move |_| {
        let t = title.get();
        if t.trim().is_empty() {
            return;
        }
        let stage = crm_add.stages_now().first().cloned();
        let cust = crm_add.customers_now().first().cloned();
        if let (Some(s), Some(c)) = (stage, cust) {
            crm_add.upsert_deal(Deal::new(t, c.id, s.id, 1000.0));
            set_title.set(String::new());
        }
    };

    let crm_for_deals = crm.clone();
    view! {
        <div id="crm-deals" class="mb-6">
            // Optional detail pane
            <Show when=move || detail.get().as_ref().is_some_and(|(k, _)| k == "deals")>
                {{
                    let crm_for_deals_clone = crm_for_deals.clone();
                    move || {
                        let text = detail.with(|d| {
                            if let Some((_, id)) = d.clone() {
                                if let Some(deal) = crm_for_deals_clone
                                    .deals_now()
                                    .into_iter()
                                    .find(|d| d.id == id)
                                {
                                    format!("Deal: {}", deal.title.clone())
                                } else {
                                    "Deal not found".to_string()
                                }
                            } else {
                                String::new()
                            }
                        });
                        view! { <DetailAlert hash="deals" text=text /> }
                    }
                }}
            </Show>
            <div class="flex items-center gap-2 mb-2">
                <input
                    class="input input-sm input-bordered w-full"
                    prop:value=title
                    on:input=move |e| set_title.set(event_target_value(&e))
                    placeholder="New deal title (requires 1 customer + stage)"
                />
                <button
                    class="btn btn-sm"
                    on:click=add
                    disabled={
                        let crm_disable = crm.clone();
                        move || crm_disable.customers_now().is_empty() || crm_disable.stages_now().is_empty()
                    }
                >
                    "Add"
                </button>
            </div>
            <ul class="menu bg-base-200 rounded-box">
                {move || {
                    let crm_ctx = crm.clone();
                    crm_ctx
                        .deals_now()
                        .into_iter()
                        .map(|d| {
                            let id = d.id.clone();
                            let crm_item = crm_ctx.clone();
                            view! {
                                <li class="flex items-center justify-between">
                                    <button class="btn btn-ghost btn-xs" on:click={
                                        let id = id.clone();
                                        move |_| { let _ = web_sys::window().unwrap().location().set_hash(&format!("deals/{}", id)); }
                                    }>{d.title.clone()}</button>
                                    <button
                                        class="btn btn-ghost btn-xs"
                                        on:click=move |_| crm_item.delete_deal(&id)
                                    >
                                        "✕"
                                    </button>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </div>
    }
}

#[component]
fn StagesView() -> impl IntoView {
    let crm = use_crm_state();
    let (name, set_name) = signal(String::new());

    let crm_add = crm.clone();
    let add = move |_| {
        let n = name.get();
        if n.trim().is_empty() {
            return;
        }
        let ts = js_sys::Date::now();
        crm_add.upsert_stage(PipelineStage {
            id: format!("stage_{}", ts),
            name: n,
            order: 0,
            probability: 0.2,
            color: None,
            is_closed: false,
        });
        set_name.set(String::new());
    };

    view! {
        <div id="crm-stages" class="mb-2">
            <div class="flex items-center gap-2 mb-2">
                <input
                    class="input input-sm input-bordered w-full"
                    prop:value=name
                    on:input=move |e| set_name.set(event_target_value(&e))
                    placeholder="New stage name"
                />
                <button class="btn btn-sm" on:click=add>
                    "Add"
                </button>
            </div>
            <ul class="menu bg-base-200 rounded-box">
                {move || {
                    let crm_ctx = crm.clone();
                    crm_ctx
                        .stages_now()
                        .into_iter()
                        .map(|s| {
                            let id = s.id.clone();
                            let crm_item = crm_ctx.clone();
                            view! {
                                <li class="flex items-center justify-between">
                                    <span>{s.name.clone()}</span>
                                    <button
                                        class="btn btn-ghost btn-xs"
                                        on:click=move |_| crm_item.delete_stage(&id)
                                    >
                                        "✕"
                                    </button>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </div>
    }
}
