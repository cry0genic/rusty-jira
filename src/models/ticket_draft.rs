use crate::models::Title;

#[derive(PartialEq, Debug, Clone)]
pub struct TicketDraft {
    pub title: Title,
    pub description: String,
}
