use leptos::mount::mount_to_body;
use leptos::prelude::*;
use wasm_bindgen::JsCast; // for dyn_into()
use wasm_bindgen_test::*;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::{window, Document};

use wasm_knowledge_chatbot_rs::features::crm::CRMPanel;

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> Document {
    window().unwrap().document().unwrap()
}

#[wasm_bindgen_test]
fn create_stage_customer_deal_and_see_on_board() {
    // Mount fresh instance
    mount_to_body(|| view! { <CRMPanel/> });

    let d = doc();

    // 1) Add a stage
    let tab_stages = d.get_element_by_id("tab-stages").unwrap();
    tab_stages
        .dispatch_event(&Event::new("click").unwrap())
        .unwrap();
    // Find stage input by placeholder
    let stage_input_el = d
        .query_selector("#crm-stages input[placeholder='New stage name']")
        .unwrap()
        .unwrap();
    let stage_input: HtmlInputElement = stage_input_el.dyn_into().unwrap();
    stage_input.set_value("Stage A");
    stage_input
        .dispatch_event(&Event::new("input").unwrap())
        .unwrap();
    // Click Add
    let stage_add_btn = d.query_selector("#crm-stages .btn").unwrap().unwrap();
    stage_add_btn
        .dispatch_event(&Event::new("click").unwrap())
        .unwrap();

    // 2) Add a customer
    let tab_customers = d.get_element_by_id("tab-customers").unwrap();
    tab_customers
        .dispatch_event(&Event::new("click").unwrap())
        .unwrap();
    let cust_input_el = d
        .query_selector("#crm-customers input[placeholder='New customer name']")
        .unwrap()
        .unwrap();
    let cust_input: HtmlInputElement = cust_input_el.dyn_into().unwrap();
    cust_input.set_value("Acme, Inc.");
    cust_input
        .dispatch_event(&Event::new("input").unwrap())
        .unwrap();
    let cust_add_btn = d.query_selector("#crm-customers .btn").unwrap().unwrap();
    cust_add_btn
        .dispatch_event(&Event::new("click").unwrap())
        .unwrap();

    // 3) Add a deal (requires 1 customer + 1 stage)
    let tab_deals = d.get_element_by_id("tab-deals").unwrap();
    tab_deals
        .dispatch_event(&Event::new("click").unwrap())
        .unwrap();
    let deal_input_el = d
        .query_selector(
            "#crm-deals input[placeholder='New deal title (requires 1 customer + stage)']",
        )
        .unwrap()
        .unwrap();
    let deal_input: HtmlInputElement = deal_input_el.dyn_into().unwrap();
    deal_input.set_value("Important Deal");
    deal_input
        .dispatch_event(&Event::new("input").unwrap())
        .unwrap();
    let deal_add_btn = d.query_selector("#crm-deals .btn").unwrap().unwrap();
    deal_add_btn
        .dispatch_event(&Event::new("click").unwrap())
        .unwrap();

    // 4) Go to Board and assert at least one deal card exists
    let tab_board = d.get_element_by_id("tab-board").unwrap();
    tab_board
        .dispatch_event(&Event::new("click").unwrap())
        .unwrap();
    let board_present = d.query_selector("#crm-board").unwrap().is_some();
    assert!(board_present, "board should be present");
    let some_card = d
        .query_selector("#crm-board .card.bg-base-100")
        .unwrap()
        .is_some();
    assert!(
        some_card,
        "at least one deal card should be visible on the board"
    );
}

#[wasm_bindgen_test]
fn deep_link_hash_activates_board_tab() {
    // Set hash before mounting
    let w = window().unwrap();
    let loc = w.location();
    loc.set_hash("board").unwrap();

    // Mount
    mount_to_body(|| view! { <CRMPanel/> });

    let d = doc();
    let board = d.get_element_by_id("tab-board").unwrap();
    let board_class = board.get_attribute("class").unwrap_or_default();
    assert!(
        board_class.contains("tab-active"),
        "board tab should be active via deep link"
    );
    assert!(
        d.query_selector("#crm-board").unwrap().is_some(),
        "board container should exist via deep link"
    );
}

#[wasm_bindgen_test]
fn switching_tabs_activates_board_tab() {
    // Mount fresh instance
    mount_to_body(|| view! { <CRMPanel/> });

    let d = doc();

    let board = d.get_element_by_id("tab-board").unwrap();
    let customers = d.get_element_by_id("tab-customers").unwrap();

    // Initially, customers should be active
    let cust_class = customers.get_attribute("class").unwrap_or_default();
    assert!(
        cust_class.contains("tab-active"),
        "customers tab should be active initially"
    );

    // Click board tab
    let evt = Event::new("click").unwrap();
    board.dispatch_event(&evt).unwrap();

    // After click, board should be active
    let board_class = board.get_attribute("class").unwrap_or_default();
    assert!(
        board_class.contains("tab-active"),
        "board tab should become active after click"
    );

    // Board container should render
    assert!(
        d.query_selector("#crm-board").unwrap().is_some(),
        "board container should exist after switching to board tab"
    );
}

#[wasm_bindgen_test]
fn mount_crm_panel_and_tabs_exist() {
    mount_to_body(|| view! { <CRMPanel/> });

    let d = doc();

    // Tabs exist
    assert!(
        d.query_selector("#tab-customers").unwrap().is_some(),
        "customers tab present"
    );
    assert!(
        d.query_selector("#tab-leads").unwrap().is_some(),
        "leads tab present"
    );
    assert!(
        d.query_selector("#tab-deals").unwrap().is_some(),
        "deals tab present"
    );
    assert!(
        d.query_selector("#tab-stages").unwrap().is_some(),
        "stages tab present"
    );
    assert!(
        d.query_selector("#tab-board").unwrap().is_some(),
        "board tab present"
    );
}
