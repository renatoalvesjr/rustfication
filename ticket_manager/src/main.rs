mod ticket;
mod enums;

use ticket::Ticket;

fn main() {
    let mut x: Ticket = Ticket::new(
        String::from("Fix the current bug"),
        String::from("Description of the current ticket"),
        String::from("In Progress")
    ).expect("Could not create ticket");

    println!("{}", x.title());
    println!("{}", x.description());
    println!("{}", x.status());

    //Changing the title of the ticket
    x.set_title(String::from("Fix the current bug in the code"));

    println!("{}", x.title());
    println!("{}", x.description());
    println!("{}", x.status());

    //Ticket y is equal to ticket x if they have the same title and status
    let y: Ticket = Ticket::new(
        String::from("Fix the current bug in the code"),
        String::from("Description of the current ticket"),
        String::from("In Progress")
    ).expect("Could not create ticket");
    println!("{}", x == y);
}
