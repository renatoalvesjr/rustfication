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

impl Status {
    pub fn to_string(&self) -> String {
        (
            match self {
                Status::ToDo { .. } =>
                    "To-do\n\tEstimated hours: ".to_string() +
                        &self.estimated_hours().unwrap().to_string(),
                Status::InProgress { .. } =>
                    "In progress\n\tAssigned to: ".to_string() + &self.assigned_to().unwrap(),
                Status::Done { .. } =>
                    "Done\n\tResolved by: ".to_string() +
                        &self.resolved_by().unwrap() +
                        "\n\tLogged hours: " +
                        &self.logged_hours().unwrap().to_string(),
            }
        ).to_string()
    }
    fn estimated_hours(&self) -> Option<u16> {
        match self {
            Status::ToDo { estimated_hours } => Some(*estimated_hours),
            _ => None,
        }
    }
    fn assigned_to(&self) -> Option<&String> {
        match self {
            Status::InProgress { assigned_to } => Some(assigned_to),
            _ => None,
        }
    }
    pub fn resolved_by(&self) -> Option<&String> {
        match self {
            Status::Done { resolved_by, .. } => Some(resolved_by),
            _ => None,
        }
    }
    fn logged_hours(&self) -> Option<u16> {
        match self {
            Status::Done { logged_hours, .. } => Some(*logged_hours),
            _ => None,
        }
    }
}
