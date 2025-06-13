use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader};
use serde_json;
use text_io::read;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Item {
    name: String,
    quantity: u32,
}

fn main() {
    let mut list_of_items: Vec<Item> = Vec::new();

    loop {
        println!("\n=== Welcome to our store! ===");
        println!("1. Add an item to store");
        println!("2. Show all items");
        println!("3. Show total quantity");
        println!("4. Show items from JSON file");
        println!("5. Exit");
        print!("Enter your choice: ");

        let choice: u8 = read!();

        match choice {
            1 => {
                println!("Please enter your item name: ");
                let item_name: String = read!();
                println!("Please enter your item quantity: ");
                let item_quantity: u32 = read!();

                let item = Item {
                    name: item_name,
                    quantity: item_quantity,
                };

                list_of_items.push(item);
                let mut item_file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("items.txt")
                    .unwrap();
                for item in &list_of_items {
                    writeln!(item_file, "{} - {}", item.name, item.quantity).unwrap();
                }

                let item_file_json = File::create("items.json").unwrap();
                serde_json::to_writer_pretty(item_file_json, &list_of_items).unwrap();
            }

            2 => {
                if list_of_items.is_empty() {
                    println!("📭 No items in the store yet.");
                } else {
                    println!("📝 List of Items:");
                    for item in &list_of_items {
                        println!("{} => {}", item.name, item.quantity);
                    }
                }
            }

            3 => {
                let total: u32 = list_of_items.iter().map(|item| item.quantity).sum();
                println!("📦 Total quantity: {}", total);
            }

            4 => {
                let file = File::open("items.json").unwrap();
                let reader = BufReader::new(file);
                let file_info: Vec<Item> = serde_json::from_reader(reader).unwrap();

                println!("📄 Items from JSON file:");
                for item in file_info {
                    println!("{:?}", item);
                }
            }

            5 => {
                println!("👋 Goodbye!");
                break;
            }

            _ => {
                println!("⚠️ Please enter a valid choice!");
            }
        }
    }
}
