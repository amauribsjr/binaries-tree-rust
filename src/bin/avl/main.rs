mod avl;

use avl::AVLTree;
use std::io::{self, Write};

fn read_i32() -> Result<i32, String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    input.trim().parse::<i32>().map_err(|e| e.to_string())
}

fn main() {
    let mut avl = AVLTree::new();
    let mut choice = -1;

    println!("AVL Tree");

    while choice != 0 {
        println!(
            "\n[1] Insert node \
             \n[2] Search value \
             \n[3] Calculate height \
             \n[4] Remove node \
             \n[5] Print tree by level \
             \n[6] Print tree in-order \
             \n[0] Exit"
        );

        print!("> ");
        io::stdout().flush().unwrap();

        match read_i32() {
            Ok(value) => choice = value,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }

        match choice {
            0 => {}

            1 => {
                print!("Enter node value: ");
                io::stdout().flush().unwrap();
            
                match read_i32() {
                    Ok(value) => {
                        if !avl.insert(value) {
                            println!("Value already exists. It will not be inserted.");
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }

            2 => {
                print!("Enter value to search: ");
                io::stdout().flush().unwrap();

                match read_i32() {
                    Ok(value) => {
                        if avl.search(value) {
                            println!("Value {} found.", value);
                        } else {
                            println!("Value {} not found.", value);
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }

            3 => {
                println!("Tree height: {}", avl.calculate_height());
            }

            4 => {
                print!("Enter value to remove: ");
                io::stdout().flush().unwrap();

                match read_i32() {
                    Ok(value) => avl.remove(value),
                    Err(e) => println!("Error: {}", e),
                }
            }

            5 => {
                avl.print_by_level();
                println!();
            }

            6 => {
                println!();
                avl.print_tree();
                println!();
            }

            _ => println!("Invalid option."),
        }
    }
}