mod avl;

use avl::AVLTree;
use avl::read_i32;

fn main() {
    let mut avl = AVLTree::new();
    let mut choice = -1;

    println!("Arvore AVL");

    while choice != 0 {
        println!(
            "\n[1]- Adicionar nó \
             \n[2]- Procurar valor de nó \
             \n[3]- Calcular altura \
             \n[4]- Remover nó \
             \n[5]- Mostrar arvore completa (aumenta muito de tamanho a partir de altura > 5) \
             \n[6]- Mostrar arvore em ordem crescente \
             \n[0]- Sair"
        );

        print!("> ");
        io::stdout().flush().unwrap();

        match read_i32() {
            Ok(value) => choice = value,
            Err(e) => {
                println!("erro encontrado {}", e);
                continue;
            }
        }

        match choice {
            0 => {}

            1 => {
                print!("Insira o valor do nó: ");
                io::stdout().flush().unwrap();

                match read_i32() {
                    Ok(value) => avl.insert(value),
                    Err(e) => println!("erro encontrado {}", e),
                }
            }

            2 => {
                print!("Insira o valor que você quer procurar: ");
                io::stdout().flush().unwrap();

                match read_i32() {
                    Ok(value) => match avl.search(value) {
                        Some(found_node) => println!("valor encontrado: {}", found_node.key),
                        None => println!("valor nao encontrado"),
                    },
                    Err(e) => println!("erro encontrado {}", e),
                }
            }

            3 => {
                println!("altura da arvore: {}", avl.calculate_height());
            }

            4 => {
                print!("Insira o valor do nó que você quer remover: ");
                io::stdout().flush().unwrap();

                match read_i32() {
                    Ok(value) => avl.remove(value),
                    Err(e) => println!("erro encontrado {}", e),
                }
            }

            5 => avl.print_by_level(),

            6 => {
                println!();
                avl.print_tree();
                println!();
            }

            _ => println!("Insira um valor válido."),
        }
    }
}