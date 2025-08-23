use crate::models::app::AppError;
use crate::models::crm::{Customer, Deal, Lead, PipelineStage};
use crate::utils::storage::StorageUtils;
use leptos::prelude::*;

const CUSTOMERS_KEY: &str = "crm_customers";
const LEADS_KEY: &str = "crm_leads";
const DEALS_KEY: &str = "crm_deals";
const STAGES_KEY: &str = "crm_stages";

#[derive(Clone)]
pub struct CRMStateContext {
    customers: RwSignal<Vec<Customer>>,
    leads: RwSignal<Vec<Lead>>,
    deals: RwSignal<Vec<Deal>>,
    stages: RwSignal<Vec<PipelineStage>>,
    last_error: RwSignal<Option<AppError>>,
}

impl CRMStateContext {
    pub fn new() -> Self {
        let ctx = Self {
            customers: RwSignal::new(Vec::new()),
            leads: RwSignal::new(Vec::new()),
            deals: RwSignal::new(Vec::new()),
            stages: RwSignal::new(Vec::new()),
            last_error: RwSignal::new(None),
        };
        ctx.load_from_storage();
        ctx
    }

    pub fn customers_now(&self) -> Vec<Customer> {
        self.customers.get_untracked()
    }
    pub fn leads_now(&self) -> Vec<Lead> {
        self.leads.get_untracked()
    }
    pub fn deals_now(&self) -> Vec<Deal> {
        self.deals.get_untracked()
    }
    pub fn stages_now(&self) -> Vec<PipelineStage> {
        self.stages.get_untracked()
    }
    pub fn last_error_now(&self) -> Option<AppError> {
        self.last_error.get_untracked()
    }

    pub fn load_from_storage(&self) {
        // Load lists from localStorage. On error, keep empty and set last_error.
        match StorageUtils::retrieve_local::<Vec<Customer>>(CUSTOMERS_KEY) {
            Ok(Some(v)) => self.customers.set(v),
            Ok(None) => {}
            Err(e) => self.last_error.set(Some(e)),
        }
        match StorageUtils::retrieve_local::<Vec<Lead>>(LEADS_KEY) {
            Ok(Some(v)) => self.leads.set(v),
            Ok(None) => {}
            Err(e) => self.last_error.set(Some(e)),
        }
        match StorageUtils::retrieve_local::<Vec<Deal>>(DEALS_KEY) {
            Ok(Some(v)) => self.deals.set(v),
            Ok(None) => {}
            Err(e) => self.last_error.set(Some(e)),
        }
        match StorageUtils::retrieve_local::<Vec<PipelineStage>>(STAGES_KEY) {
            Ok(Some(v)) => self.stages.set(v),
            Ok(None) => {}
            Err(e) => self.last_error.set(Some(e)),
        }
    }

    fn persist_all(&self) {
        // Persist vectors; on error capture last_error
        if let Err(e) = StorageUtils::store_local(CUSTOMERS_KEY, &self.customers.get_untracked()) {
            self.last_error.set(Some(e));
        }
        if let Err(e) = StorageUtils::store_local(LEADS_KEY, &self.leads.get_untracked()) {
            self.last_error.set(Some(e));
        }
        if let Err(e) = StorageUtils::store_local(DEALS_KEY, &self.deals.get_untracked()) {
            self.last_error.set(Some(e));
        }
        if let Err(e) = StorageUtils::store_local(STAGES_KEY, &self.stages.get_untracked()) {
            self.last_error.set(Some(e));
        }
    }

    // Customers CRUD
    pub fn upsert_customer(&self, customer: Customer) {
        self.customers.update(|v| {
            if let Some(idx) = v.iter().position(|c| c.id == customer.id) {
                v[idx] = customer;
            } else {
                v.push(customer);
            }
        });
        self.persist_all();
    }

    pub fn delete_customer(&self, id: &str) {
        self.customers.update(|v| v.retain(|c| c.id != id));
        self.persist_all();
    }

    // Leads CRUD
    pub fn upsert_lead(&self, lead: Lead) {
        self.leads.update(|v| {
            if let Some(idx) = v.iter().position(|c| c.id == lead.id) {
                v[idx] = lead;
            } else {
                v.push(lead);
            }
        });
        self.persist_all();
    }

    pub fn delete_lead(&self, id: &str) {
        self.leads.update(|v| v.retain(|c| c.id != id));
        self.persist_all();
    }

    // Deals CRUD
    pub fn upsert_deal(&self, deal: Deal) {
        self.deals.update(|v| {
            if let Some(idx) = v.iter().position(|c| c.id == deal.id) {
                v[idx] = deal;
            } else {
                v.push(deal);
            }
        });
        self.persist_all();
    }

    pub fn delete_deal(&self, id: &str) {
        self.deals.update(|v| v.retain(|c| c.id != id));
        self.persist_all();
    }

    // Stages CRUD
    pub fn upsert_stage(&self, stage: PipelineStage) {
        self.stages.update(|v| {
            if let Some(idx) = v.iter().position(|c| c.id == stage.id) {
                v[idx] = stage;
            } else {
                v.push(stage);
            }
        });
        self.persist_all();
    }

    pub fn delete_stage(&self, id: &str) {
        self.stages.update(|v| v.retain(|c| c.id != id));
        self.persist_all();
    }
}

#[component]
pub fn CRMStateProvider(children: Children) -> impl IntoView {
    let ctx = CRMStateContext::new();
    provide_context(ctx);
    view! { {children()} }
}

pub fn use_crm_state() -> CRMStateContext {
    expect_context::<CRMStateContext>()
}

impl Default for CRMStateContext {
    fn default() -> Self {
        Self::new()
    }
}
