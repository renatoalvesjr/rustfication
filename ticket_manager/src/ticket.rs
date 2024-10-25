use crate::enums::Status;

#[derive(Debug)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[allow(dead_code)]
impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Ticket, String> {
        Ok(Ticket {
            title: validate_title(title),
            description: match validate_description(description) {
                Ok(description) => description,
                Err(e) => e,
            },
            status: validate_status(status),
        })
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn status(&self) -> &str {
        match &self.status {
            Status::ToDo { .. } => "To-do",
            Status::InProgress { .. } => "In Progress",
            Status::Done { .. } => "Done",
        }
    }
    pub fn set_title(&mut self, title: String) {
        self.title = validate_title(title);
    }
    pub fn set_description(&mut self, description: String) {
        self.description = match validate_description(description) {
            Ok(description) => description,
            Err(e) => e,
        };
    }
    pub fn set_status(&mut self, status: String) {
        self.status = validate_status(status);
    }

    fn is_done(&self) -> bool {
        match self {
            Ticket { status: Status::Done { .. }, .. } => true,
            _ => false,
        }
    }

    fn assigned_to(&self) -> Option<&str> {
        if let Status::InProgress { assigned_to } = &self.status { Some(assigned_to) } else { None }
    }
}
//Custom equality implementation
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.status == other.status
    }
    // fn ne(&self, other: &Self) -> bool {
    //     self.title != other.title || (self.status !=  other.status)
    // }
}

fn validate_title(title: String) -> String {
    match title.split_whitespace().next() {
        Some("Fix") | Some("Feature") | Some("Bug") | Some("Task") => if
            title.chars().count() <= 50
        {
            title
        } else {
            panic!("Title should be at most 50 bytes")
        }
        _ => panic!("Invalid title"),
    }
}

fn validate_description(description: String) -> Result<String, String> {
    if description.chars().count() <= 500 {
        Ok(description)
    } else {
        Err(String::from("Description not provided"))
    }
}
fn validate_status(status: String) -> Status {
    match status.as_str() {
        "To-do" =>
            Status::ToDo {
                estimated_hours: 0,
            },
        "In Progress" =>
            Status::InProgress {
                assigned_to: String::from(""),
            },
        "Done" =>
            Status::Done {
                resolved_by: String::from(""),
                logged_hours: 0,
            },
        _ => panic!("Invalid status"),
    }
}