use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub status: CustomerStatus,
    pub created_at: f64,
    pub updated_at: f64,
    pub tags: Vec<String>,
    pub custom_fields: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CustomerStatus {
    Active,
    Inactive,
    Prospect,
    Churned,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lead {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub source: LeadSource,
    pub status: LeadStatus,
    pub score: Option<u32>, // Lead scoring 0-100
    pub assigned_to: Option<String>,
    pub created_at: f64,
    pub updated_at: f64,
    pub notes: Vec<Note>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LeadSource {
    Website,
    Email,
    Social,
    Referral,
    Advertisement,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Proposal,
    Negotiation,
    Closed,
    Lost,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub job_title: Option<String>,
    pub company_id: Option<String>,
    pub customer_id: Option<String>,
    pub created_at: f64,
    pub updated_at: f64,
    pub interactions: Vec<Interaction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub id: String,
    pub interaction_type: InteractionType,
    pub subject: String,
    pub content: String,
    pub timestamp: f64,
    pub duration_minutes: Option<u32>,
    pub outcome: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InteractionType {
    Email,
    Call,
    Meeting,
    Note,
    Task,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deal {
    pub id: String,
    pub title: String,
    pub customer_id: String,
    pub stage_id: String,
    pub value: f64,
    pub currency: String,
    pub probability: f32, // 0.0 to 1.0
    pub expected_close_date: Option<f64>,
    pub actual_close_date: Option<f64>,
    pub status: DealStatus,
    pub assigned_to: Option<String>,
    pub created_at: f64,
    pub updated_at: f64,
    pub activities: Vec<Activity>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DealStatus {
    Open,
    Won,
    Lost,
    Cancelled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineStage {
    pub id: String,
    pub name: String,
    pub order: u32,
    pub probability: f32, // Default probability for deals in this stage
    pub color: Option<String>,
    pub is_closed: bool, // Whether this stage represents a closed deal
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub activity_type: ActivityType,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<f64>,
    pub completed_at: Option<f64>,
    pub assigned_to: Option<String>,
    pub priority: Priority,
    pub created_at: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActivityType {
    Call,
    Email,
    Meeting,
    Task,
    FollowUp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub content: String,
    pub created_at: f64,
    pub created_by: Option<String>,
    pub tags: Vec<String>,
}

impl Customer {
    pub fn new(name: String) -> Self {
        let timestamp = js_sys::Date::now();
        Self {
            id: format!("cust_{}", timestamp),
            name,
            email: None,
            phone: None,
            company: None,
            status: CustomerStatus::Prospect,
            created_at: timestamp,
            updated_at: timestamp,
            tags: Vec::new(),
            custom_fields: HashMap::new(),
        }
    }
}

impl Lead {
    pub fn new(name: String, source: LeadSource) -> Self {
        let timestamp = js_sys::Date::now();
        Self {
            id: format!("lead_{}", timestamp),
            name,
            email: None,
            phone: None,
            company: None,
            source,
            status: LeadStatus::New,
            score: None,
            assigned_to: None,
            created_at: timestamp,
            updated_at: timestamp,
            notes: Vec::new(),
        }
    }
}

impl Contact {
    pub fn new(first_name: String, last_name: String) -> Self {
        let timestamp = js_sys::Date::now();
        Self {
            id: format!("contact_{}", timestamp),
            first_name,
            last_name,
            email: None,
            phone: None,
            job_title: None,
            company_id: None,
            customer_id: None,
            created_at: timestamp,
            updated_at: timestamp,
            interactions: Vec::new(),
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

impl Deal {
    pub fn new(title: String, customer_id: String, stage_id: String, value: f64) -> Self {
        let timestamp = js_sys::Date::now();
        Self {
            id: format!("deal_{}", timestamp),
            title,
            customer_id,
            stage_id,
            value,
            currency: "USD".to_string(),
            probability: 0.5,
            expected_close_date: None,
            actual_close_date: None,
            status: DealStatus::Open,
            assigned_to: None,
            created_at: timestamp,
            updated_at: timestamp,
            activities: Vec::new(),
        }
    }
}
