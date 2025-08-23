use wasm_bindgen_test::*;
use wasm_knowledge_chatbot_rs::state::CRMStateContext;
use wasm_knowledge_chatbot_rs::models::crm::{Customer, Lead, LeadSource, Deal, PipelineStage};
use wasm_knowledge_chatbot_rs::utils::storage::StorageUtils;

wasm_bindgen_test_configure!(run_in_browser);

fn clear_crm_storage() {
    let _ = StorageUtils::remove_local("crm_customers");
    let _ = StorageUtils::remove_local("crm_leads");
    let _ = StorageUtils::remove_local("crm_deals");
    let _ = StorageUtils::remove_local("crm_stages");
}

#[wasm_bindgen_test]
fn crm_crud_and_persistence_roundtrip() {
    clear_crm_storage();

    // Create context and add entities
    let ctx = CRMStateContext::new();

    let mut cust = Customer::new("Acme Inc".to_string());
    cust.email = Some("sales@acme.test".to_string());
    ctx.upsert_customer(cust.clone());

    let lead = Lead::new("John Doe".to_string(), LeadSource::Website);
    ctx.upsert_lead(lead.clone());

    let stage = PipelineStage { id: "stage_new".into(), name: "New".into(), order: 1, probability: 0.1, color: None, is_closed: false };
    ctx.upsert_stage(stage.clone());

    let deal = Deal::new("Pilot".into(), cust.id.clone(), stage.id.clone(), 5000.0);
    ctx.upsert_deal(deal.clone());

    // Verify state contains items
    assert_eq!(ctx.customers_now().len(), 1);
    assert_eq!(ctx.leads_now().len(), 1);
    assert_eq!(ctx.deals_now().len(), 1);
    assert_eq!(ctx.stages_now().len(), 1);

    // Create new context (reload from localStorage)
    let ctx2 = CRMStateContext::new();
    assert_eq!(ctx2.customers_now().len(), 1);
    assert_eq!(ctx2.leads_now().len(), 1);
    assert_eq!(ctx2.deals_now().len(), 1);
    assert_eq!(ctx2.stages_now().len(), 1);

    // Delete and persist
    ctx2.delete_deal(&deal.id);
    ctx2.delete_lead(&lead.id);
    ctx2.delete_customer(&cust.id);
    ctx2.delete_stage(&stage.id);

    assert_eq!(ctx2.customers_now().len(), 0);
    assert_eq!(ctx2.leads_now().len(), 0);
    assert_eq!(ctx2.deals_now().len(), 0);
    assert_eq!(ctx2.stages_now().len(), 0);

    // Roundtrip again to ensure removals persisted
    let ctx3 = CRMStateContext::new();
    assert_eq!(ctx3.customers_now().len(), 0);
    assert_eq!(ctx3.leads_now().len(), 0);
    assert_eq!(ctx3.deals_now().len(), 0);
    assert_eq!(ctx3.stages_now().len(), 0);
}
