use crate::models::{Comment, DeletedTicket, Status, Ticket, TicketDraft, TicketId, TicketPatch};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct TicketStore {
    current_id: u64,
    data: HashMap<TicketId, Ticket>,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            current_id: 0,
            data: HashMap::new(),
        }
    }

    pub fn create(&mut self, draft: TicketDraft) -> TicketId {
        let id = self.generate_id();
        let ticket = Ticket {
            id,
            description: draft.description,
            title: draft.title,
            status: Status::ToDo,
            comments: Vec::new(),
        };
        self.data.insert(ticket.id, ticket);
        id
    }

    pub fn delete(&mut self, ticket_id: TicketId) -> Option<DeletedTicket> {
        self.data.remove(&ticket_id).map(DeletedTicket)
    }

    pub fn list(&self) -> Vec<&Ticket> {
        self.data.iter().map(|(_, ticket)| ticket).collect()
    }

    fn generate_id(&mut self) -> TicketId {
        self.current_id += 1;
        self.current_id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.data.get(&id)
    }

    pub fn update_ticket(&mut self, id: TicketId, patch: TicketPatch) -> Option<()> {
        self.data.get_mut(&id).map(|t| {
            if let Some(title) = patch.title {
                t.title = title;
            }
            if let Some(description) = patch.description {
                t.description = description;
            }
        })
    }

    pub fn update_ticket_status(&mut self, id: TicketId, status: Status) -> Option<()> {
        self.data.get_mut(&id).map(|t| t.status = status)
    }

    pub fn add_comment_to_ticket(&mut self, id: TicketId, comment: Comment) -> Option<()> {
        self.data.get_mut(&id).map(|t| t.comments.push(comment))
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{Comment, Status, Ticket, TicketDraft, TicketPatch, Title};
    use crate::store::TicketStore;
    use fake::{Fake, Faker};
    use std::collections::HashSet;

    #[test]
    fn create_ticket_test() {
        let draft = TicketDraft {
            title: Title::new(Faker.fake()).expect("Title should exist"),
            description: Faker.fake(),
        };

        let mut ticket_store = TicketStore::new();

        let ticket_id = ticket_store.create(draft.clone());

        let ticket = ticket_store
            .get(ticket_id)
            .expect("Failed to retrieve ticket.");
        assert_eq!(ticket.title, draft.title);
        assert_eq!(ticket.description, draft.description);
        assert_eq!(ticket.status, Status::ToDo);
    }

    #[test]
    fn delete_ticket_test() {
        let draft = TicketDraft {
            title: Title::new(Faker.fake()).expect("Title should exist"),
            description: Faker.fake(),
        };

        let mut ticket_store = TicketStore::new();
        let ticket_id = ticket_store.create(draft.clone());
        let inserted_ticket = ticket_store
            .get(ticket_id)
            .expect("Failed to retrieve ticket")
            .to_owned();

        let deleted_ticket = ticket_store
            .delete(ticket_id)
            .expect("There was no ticket to delete.");
        
        assert_eq!(deleted_ticket.0, inserted_ticket);
        let ticket = ticket_store.get(ticket_id);
        assert_eq!(ticket, None);
    }

    #[test]
    fn deleting_a_ticket_that_does_not_exist_returns_none() {
        let mut ticket_store = TicketStore::new();
        
        let deleted_ticket = ticket_store.delete(Faker.fake());
        
        assert_eq!(deleted_ticket, None);
    }

    #[test]
    fn listing_tickets_of_an_empty_store_returns_an_empty_collection() {
        let ticket_store = TicketStore::new();

        let tickets = ticket_store.list();

        assert!(tickets.is_empty())
    }

    #[test]
    fn listing_tickets_should_return_them_all() {
        let mut ticket_store = TicketStore::new();
        let n_tickets = Faker.fake::<u16>() as usize;
        let tickets: HashSet<_> = (0..n_tickets)
            .map(|_| generate_and_persist_ticket(&mut ticket_store))
            .collect();

        let retrieved_tickets = ticket_store.list();

        assert_eq!(retrieved_tickets.len(), n_tickets);
        let retrieved_tickets: HashSet<_> = retrieved_tickets
            .into_iter()
            .map(|t| t.to_owned())
            .collect();
        assert_eq!(tickets, retrieved_tickets);
    }

    fn generate_and_persist_ticket(store: &mut TicketStore) -> Ticket {
        let draft = TicketDraft {
            title: Title::new(Faker.fake()).expect("Failed to get a title"),
            description: Faker.fake(),
        };
        let ticket_id = store.create(draft);
        store
            .get(ticket_id)
            .expect("Failed to retrieve ticket")
            .to_owned()
    }

    #[test]
    fn updating_ticket_info_via_patch_should_update_ticket() {
        let mut ticket_store = TicketStore::new();

        let ticket = generate_and_persist_ticket(&mut ticket_store);

        let patch = TicketPatch {
            title: Some(Title::new(Faker.fake()).expect("Failed to get a title")),
            description: Some(Faker.fake()),
        };

        let expected = patch.clone();

        ticket_store.update_ticket(ticket.id, patch);

        let updated_ticket = ticket_store
            .get(ticket.id)
            .expect("Failed to retrieve ticket.");

        assert_eq!(
            updated_ticket.title,
            expected.title.expect("Failed to get a title")
        );

        assert_eq!(
            updated_ticket.description,
            expected.description.expect("Failed to get a Description")
        );
    }

    #[test]
    fn updating_ticket_with_no_patch_values_should_not_fail_or_change_values() {
        let draft = TicketDraft {
            title: Title::new(Faker.fake()).expect("Failed to get a title"),
            description: Faker.fake(),
        };

        let mut ticket_store = TicketStore::new();

        let ticket_id = ticket_store.create(draft.clone());

        let patch = TicketPatch {
            title: None,
            description: None,
        };

        ticket_store.update_ticket(ticket_id, patch);

        let updated_ticket = ticket_store
            .get(ticket_id)
            .expect("Failed to retrieve ticket.");

        assert_eq!(updated_ticket.title, draft.title);

        assert_eq!(updated_ticket.description, draft.description);
    }

    #[test]
    fn updating_ticket_status_should_change_ticket_to_new_status() {
        let mut ticket_store = TicketStore::new();

        let ticket = generate_and_persist_ticket(&mut ticket_store);

        ticket_store.update_ticket_status(ticket.id, Status::Done);

        let updated_ticket = ticket_store
            .get(ticket.id)
            .expect("Failed to retrieve ticket.");

        assert_eq!(updated_ticket.status, Status::Done)
    }

    #[test]
    fn add_comment_to_ticket() {
        let mut ticket_store = TicketStore::new();
        let ticket = generate_and_persist_ticket(&mut ticket_store);
        let comment = Comment::new("Test Comment".to_string()).unwrap();
        let expected = comment.clone();

        let result = ticket_store.add_comment_to_ticket(ticket.id, comment);
        
        assert!(result.is_some());
        let ticket = ticket_store.get(ticket.id).unwrap();
        assert_eq!(ticket.comments, vec![expected]);
    }

    #[test]
    fn add_comment_to_invalid_ticket_id_returns_none() {
        let faker = fake::Faker;

        let mut ticket_store = TicketStore::new();
        let comment = Comment::new("Test comment".to_string()).unwrap();

        let result = ticket_store.add_comment_to_ticket(faker.fake(), comment);

        assert!(result.is_none());
    }
}
