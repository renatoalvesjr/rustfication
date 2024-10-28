use crate::ticket::Ticket;
use crate::storage::Storage;
use std::io::{ self, Write };

pub fn show_ticket(storage: &Storage, title: &str) {
    match storage.get_ticket(title) {
        Some(ticket) => {
            println!("Title: {}", ticket.title());
            println!("Description: {}", ticket.description());
            println!("Status: {}", ticket.status());
        }
        None => println!("Ticket not found"),
    }
}

pub fn show_all_tickets(storage: &Storage) {
    for ticket in storage.get_tickets() {
        println!("Title: {}", ticket.title());
        println!("Description: {}", ticket.description());
        println!("Status: {}", ticket.status());
    }
}

pub fn insert_ticket(storage: &mut Storage) {
    let mut title = String::new();
    let mut description = String::new();

    print!("Enter the title: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    print!("Enter the description: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut description).unwrap();
    let description = description.trim();

    let status = "To-do";
    //Set estimated hours
    print!("Enter the estimated hours: ");
    io::stdout().flush().unwrap();
    let mut estimated_hours = String::new();
    io::stdin().read_line(&mut estimated_hours).unwrap();
    let estimated_hours: u16 = estimated_hours.trim().parse().unwrap();

    match Ticket::new(title.to_string(), description.to_string(), status.to_string()) {
        Ok(ticket) => {
            storage.add_ticket(ticket);
            storage.estimated_hours(title, estimated_hours);
            println!("Ticket inserted successfully");
        }
        Err(e) => println!("Failed to create ticket: {}", e),
    }
}

pub fn update_ticket_status(storage: &mut Storage) {
    let mut title = String::new();
    let mut status = String::new();

    print!("Enter the title of the ticket: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    print!("Enter the new status: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut status).unwrap();
    let status = status.trim();
    match status {
        "In Progress" => {
            print!("Enter the name of the person assigned to the ticket: ");
            io::stdout().flush().unwrap();
            let mut assigned_to = String::new();
            io::stdin().read_line(&mut assigned_to).unwrap();
            let assigned_to = assigned_to.trim();
            storage.change_status(title, status);
            storage.assign_to(title, assigned_to);
        }
        "Done" => {
            print!("Enter the name of the person who resolved the ticket: ");
            io::stdout().flush().unwrap();
            let mut resolved_by = String::new();
            io::stdin().read_line(&mut resolved_by).unwrap();
            let resolved_by = resolved_by.trim();

            print!("Enter the number of hours logged: ");
            io::stdout().flush().unwrap();
            let mut logged_hours = String::new();
            io::stdin().read_line(&mut logged_hours).unwrap();
            let logged_hours: u16 = logged_hours.trim().parse().unwrap();

            storage.change_status(title, status);
            storage.resolved_by(title, resolved_by);
            storage.logged_hours(title, logged_hours);
        }
        _ => {
            panic!("Invalid status");
        }
    }

    println!("Ticket status updated successfully");
}

pub fn remove_ticket(storage: &mut Storage) {
    let mut title = String::new();

    print!("Enter the title of the ticket: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    match storage.remove_ticket(title) {
        Some(_) => println!("Ticket removed successfully"),
        None => println!("Ticket not found"),
    }
}
