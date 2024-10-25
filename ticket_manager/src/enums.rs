#[derive(Debug, PartialEq)]
pub enum Status {
    ToDo {
        estimated_hours: u16,
    },
    InProgress {
        assigned_to: String,
    },
    Done {
        resolved_by: String,
        logged_hours: u16,
    },
}

#[derive(Debug)]
pub enum TicketNewError {
    TitleError(String),
    StatusError(String),
}
