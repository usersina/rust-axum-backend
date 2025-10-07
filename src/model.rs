//! Simplistic Model Layer
//! (with mock-store layer)

use crate::{Error, Result, ctx::Ctx};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex}; // in memory store for now

// region: --- Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64, // creator user_id
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}
// endregion: --- Ticket Types

// region: --- Model Controller
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>, // FIXME: Will fill indefinitely
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD implementation
impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone())); // We will leave a None for deleted ones
        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        // Lock is exclusive anyway
        let store = self.tickets_store.lock().unwrap();

        // Filter out the None values
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

// endregion: --- Model Controller
