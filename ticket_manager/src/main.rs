mod ticket;
mod enums;
mod storage;
mod controls;

use std::io::Write;

use storage::Storage;

fn main() {
    let mut storage: Storage = Storage::new();

    //main menu
    loop {
        println!("1. Show all tickets");
        println!("2. Show a ticket");
        println!("3. Insert a ticket");
        println!("4. Update ticket status");
        println!("5. Remove a ticket");
        println!("6. Exit");

        let mut option = String::new();
        std::io::stdin().read_line(&mut option).expect("Could not read line");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        match option {
            1 => controls::show_all_tickets(&storage),
            2 => {
                let mut title = String::new();
                print!("Enter the title of the ticket: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut title).unwrap();
                let title = title.trim();
                controls::show_ticket(&storage, title);
            }
            3 => controls::insert_ticket(&mut storage),
            4 => controls::update_ticket_status(&mut storage),
            5 => {
                controls::remove_ticket(&mut storage);
            }
            6 => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
}
