use crate::ticket::Ticket;

pub struct Storage {
    tickets: Vec<Ticket>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn get_tickets(&self) -> &Vec<Ticket> {
        &self.tickets
    }

    pub fn get_ticket(&self, title: &str) -> Option<&Ticket> {
        self.tickets.iter().find(|ticket| ticket.title() == title)
    }

    pub fn remove_ticket(&mut self, title: &str) -> Option<Ticket> {
        let position = self.tickets.iter().position(|ticket| ticket.title() == title);
        match position {
            Some(index) => Some(self.tickets.remove(index)),
            None => None,
        }
    }

    pub fn change_status(&mut self, title: &str, status: &str) {
        if let Some(ticket) = self.tickets.iter_mut().find(|ticket| ticket.title() == title) {
            ticket.set_status(status.to_string());
        }
    }

    pub fn assign_to(&mut self, title: &str, assigned_to: &str) {
        if let Some(ticket) = self.tickets.iter_mut().find(|ticket| ticket.title() == title) {
            ticket.assign_to(assigned_to.to_string());
        }
    }

    pub fn estimated_hours(&mut self, title: &str, estimated_hours: u16) {
        if let Some(ticket) = self.tickets.iter_mut().find(|ticket| ticket.title() == title) {
            ticket.set_estimated_hours(estimated_hours);
        }
    }

    pub fn resolved_by(&mut self, title: &str, resolved_by: &str) {
        if let Some(ticket) = self.tickets.iter_mut().find(|ticket| ticket.title() == title) {
            ticket.set_resolved_by(resolved_by.to_string());
        }
    }

    pub fn logged_hours(&mut self, title: &str, logged_hours: u16) {
        if let Some(ticket) = self.tickets.iter_mut().find(|ticket| ticket.title() == title) {
            ticket.set_logged_hours(logged_hours);
        }
    }
}
