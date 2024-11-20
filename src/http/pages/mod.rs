pub mod base_page;
pub mod employee_pages;
pub mod tech_pages;
pub mod transfer_pages;
pub mod unit_pages;
pub mod welcome;
use crate::model::dao::employee::Employee;
use chrono::NaiveDate;
use maud::{html, Markup};

pub enum MessageKind {
    Success,
    Error,
    Notify,
    None
}

pub struct Message {
    pub kind: MessageKind,
    pub content: String,
}

pub struct Session {
    pub tech_id: i32,
    pub unit_id: i32,
    pub room_id: i32,
    pub date: NaiveDate,
    pub accountable_employees: Vec<Employee>,
}

impl Message {
    pub fn new(content: &str, kind: MessageKind) -> Self {
        Self {
            kind,
            content: content.to_string(),
        }
    }

    pub fn none() -> Self {
        Self {
            kind: MK::None,
            content: "".to_string()
        }
    }

    pub fn to_html(&self) -> Markup {
        match self.kind {
            MK::Notify => html! { div class="notify-message mt-3" {
                (self.content)
            } },
            MK::Success => html! { div class="alert alert-success mt-3" {
                (self.content)
            }},
            MK::Error => html! { div class="alert alert-danger mt-3" {
                (self.content)
            }},
            MK::None => html! {}
        }
    }
}

fn get_elem_class(message_kind: MessageKind) -> String {
    match message_kind {
        MessageKind::Success => "success".into(),
        MessageKind::Notify => "notify".into(),
        MessageKind::Error => "error".into(),
        MK::None => "".into()
    }
}

pub use MessageKind as MK;
