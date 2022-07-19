use crate::models::Title;

#[derive(PartialEq, Debug, Clone)]
pub struct TicketPatch {
    pub title: Option<Title>,
    pub description: Option<String>,
}
